use anyhow::Result;
use collections::HashMap;
use gpui::{AsyncApp, Context, Entity, EventEmitter};
use proto::TypedEnvelope;
use rpc::AnyProtoClient;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct WorktreeEntry {
    pub id: u64,
    pub path: PathBuf,
    pub is_dir: bool,
    pub is_ignored: bool,
    pub is_hidden: bool,
}

#[derive(Debug, Clone)]
pub struct WorktreeInfo {
    pub id: u64,
    pub root_name: String,
    pub abs_path: String,
    pub entries: HashMap<u64, WorktreeEntry>,
}

#[derive(Debug, Clone)]
pub enum WebWorktreeStoreEvent {
    WorktreeUpdated { worktree_id: u64 },
}

pub struct WebWorktreeStore {
    worktrees: HashMap<u64, WorktreeInfo>,
    project_id: u64,
}

impl EventEmitter<WebWorktreeStoreEvent> for WebWorktreeStore {}

impl WebWorktreeStore {
    pub fn new(project_id: u64) -> Self {
        Self {
            worktrees: HashMap::default(),
            project_id,
        }
    }

    pub fn worktrees(&self) -> &HashMap<u64, WorktreeInfo> {
        &self.worktrees
    }

    /// Returns a sorted list of all entries across all worktrees, suitable for
    /// rendering a file tree. Entries are sorted by path within each worktree.
    pub fn sorted_entries(&self) -> Vec<(u64, Vec<&WorktreeEntry>)> {
        let mut result: Vec<_> = self
            .worktrees
            .iter()
            .map(|(&worktree_id, info)| {
                let mut entries: Vec<&WorktreeEntry> = info.entries.values().collect();
                entries.sort_by(|a, b| a.path.cmp(&b.path));
                (worktree_id, entries)
            })
            .collect();
        result.sort_by_key(|(id, _)| *id);
        result
    }

    pub fn register_message_handlers(
        client: &AnyProtoClient,
        store: &Entity<Self>,
    ) {
        client.add_entity_message_handler::<proto::UpdateWorktree, Self, _, _>(
            move |store: Entity<Self>,
                  envelope: TypedEnvelope<proto::UpdateWorktree>,
                  mut cx: AsyncApp| async move {
                store.update(&mut cx, |store, cx| {
                    store.handle_update_worktree(envelope.payload, cx);
                });
                Ok(())
            },
        );
    }

    fn handle_update_worktree(
        &mut self,
        message: proto::UpdateWorktree,
        cx: &mut Context<Self>,
    ) {
        let worktree = self
            .worktrees
            .entry(message.worktree_id)
            .or_insert_with(|| WorktreeInfo {
                id: message.worktree_id,
                root_name: message.root_name.clone(),
                abs_path: message.abs_path.clone(),
                entries: HashMap::default(),
            });

        worktree.root_name = message.root_name;
        worktree.abs_path = message.abs_path;

        for entry_proto in message.updated_entries {
            worktree.entries.insert(
                entry_proto.id,
                WorktreeEntry {
                    id: entry_proto.id,
                    path: PathBuf::from(&entry_proto.path),
                    is_dir: entry_proto.is_dir,
                    is_ignored: entry_proto.is_ignored,
                    is_hidden: entry_proto.is_hidden,
                },
            );
        }

        for removed_id in message.removed_entries {
            worktree.entries.remove(&removed_id);
        }

        cx.emit(WebWorktreeStoreEvent::WorktreeUpdated {
            worktree_id: message.worktree_id,
        });
        cx.notify();
    }
}
