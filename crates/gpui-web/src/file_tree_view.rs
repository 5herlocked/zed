use crate::web_buffer_store::WebBufferStore;
use crate::web_worktree_store::{WebWorktreeStore, WebWorktreeStoreEvent};
use collections::HashSet;
use gpui::{
    div, prelude::*, px, rgb, Context, Entity, InteractiveElement, IntoElement, ParentElement,
    Render, SharedString, Styled, Subscription, Window,
};
use rpc::AnyProtoClient;
use std::path::{Path, PathBuf};

pub struct FileTreeView {
    worktree_store: Entity<WebWorktreeStore>,
    buffer_store: Entity<WebBufferStore>,
    client: AnyProtoClient,
    expanded_dirs: HashSet<String>,
    _subscriptions: Vec<Subscription>,
}

impl FileTreeView {
    pub fn new(
        worktree_store: Entity<WebWorktreeStore>,
        buffer_store: Entity<WebBufferStore>,
        client: AnyProtoClient,
        cx: &mut Context<Self>,
    ) -> Self {
        let subscription = cx.subscribe(
            &worktree_store,
            |_this: &mut Self, _store, _event: &WebWorktreeStoreEvent, cx| {
                cx.notify();
            },
        );

        Self {
            worktree_store,
            buffer_store,
            client,
            expanded_dirs: HashSet::default(),
            _subscriptions: vec![subscription],
        }
    }

    fn toggle_dir(&mut self, path: String, cx: &mut Context<Self>) {
        if self.expanded_dirs.contains(&path) {
            self.expanded_dirs.remove(&path);
        } else {
            self.expanded_dirs.insert(path);
        }
        cx.notify();
    }

    fn open_file(&self, worktree_id: u64, path: String) {
        let request = proto::OpenBufferByPath {
            project_id: 0,
            worktree_id,
            path,
        };
        if let Err(error) = self.client.send(request) {
            web_sys::console::error_1(
                &format!("Failed to send OpenBufferByPath: {error:#}").into(),
            );
        }
    }

    fn is_ancestor_expanded(&self, path: &str) -> bool {
        if path.is_empty() {
            return true;
        }
        let path = Path::new(path);
        let mut current = PathBuf::new();
        for component in path.components() {
            current.push(component);
            let current_str = current.to_string_lossy().to_string();
            if !self.expanded_dirs.contains(&current_str) {
                return false;
            }
        }
        true
    }
}

struct TreeItem {
    worktree_id: u64,
    path: String,
    name: String,
    is_dir: bool,
    is_expanded: bool,
    depth: usize,
}

impl Render for FileTreeView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let sorted = self.worktree_store.read(cx).sorted_entries();

        let mut items = Vec::new();
        for (worktree_id, entries) in &sorted {
            for entry in entries {
                let path_str = entry.path.to_string_lossy().to_string();
                if path_str.is_empty() {
                    continue;
                }

                let depth = entry.path.components().count().saturating_sub(1);

                let visible = if depth == 0 {
                    true
                } else {
                    entry
                        .path
                        .parent()
                        .map(|p| self.is_ancestor_expanded(&p.to_string_lossy()))
                        .unwrap_or(true)
                };

                if !visible {
                    continue;
                }

                let is_expanded = entry.is_dir && self.expanded_dirs.contains(&path_str);

                items.push(TreeItem {
                    worktree_id: *worktree_id,
                    path: path_str,
                    name: entry
                        .path
                        .file_name()
                        .map(|n| n.to_string_lossy().to_string())
                        .unwrap_or_else(|| entry.path.to_string_lossy().to_string()),
                    is_dir: entry.is_dir,
                    is_expanded,
                    depth,
                });
            }
        }

        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(rgb(0x1e1e2e))
            .text_color(rgb(0xcdd6f4))
            .text_size(px(13.0))
            .children(items.into_iter().map(|item| {
                let path = item.path.clone();
                let worktree_id = item.worktree_id;
                let is_dir = item.is_dir;
                let indent = px((item.depth as f32) * 16.0 + 8.0);

                let icon: SharedString = if is_dir {
                    if item.is_expanded {
                        "▼ ".into()
                    } else {
                        "▶ ".into()
                    }
                } else {
                    "  ".into()
                };

                let name: SharedString = item.name.into();

                div()
                    .id(SharedString::from(format!("entry-{}-{}", worktree_id, item.path)))
                    .pl(indent)
                    .py(px(2.0))
                    .pr(px(8.0))
                    .flex()
                    .flex_row()
                    .cursor_pointer()
                    .hover(|style| style.bg(rgb(0x313244)))
                    .child(icon)
                    .child(name)
                    .on_click(cx.listener(move |this, _event, _window, cx| {
                        if is_dir {
                            this.toggle_dir(path.clone(), cx);
                        } else {
                            this.open_file(worktree_id, path.clone());
                        }
                    }))
            }))
    }
}
