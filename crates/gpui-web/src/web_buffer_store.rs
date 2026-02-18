use anyhow::Result;
use collections::HashMap;
use gpui::{AsyncApp, Context, Entity, EventEmitter};
use proto::TypedEnvelope;
use rpc::AnyProtoClient;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct BufferInfo {
    pub id: u64,
    pub file_path: Option<PathBuf>,
    pub base_text: String,
    pub is_complete: bool,
}

#[derive(Debug, Clone)]
pub enum WebBufferStoreEvent {
    BufferCreated { buffer_id: u64 },
    BufferUpdated { buffer_id: u64 },
}

pub struct WebBufferStore {
    buffers: HashMap<u64, BufferInfo>,
    /// Buffers being received in chunks (CreateBufferForPeer with BufferChunk variant)
    pending_chunks: HashMap<u64, PendingBuffer>,
    project_id: u64,
}

struct PendingBuffer {
    file_path: Option<PathBuf>,
    base_text: String,
}

impl EventEmitter<WebBufferStoreEvent> for WebBufferStore {}

impl WebBufferStore {
    pub fn new(project_id: u64) -> Self {
        Self {
            buffers: HashMap::default(),
            pending_chunks: HashMap::default(),
            project_id,
        }
    }

    pub fn buffers(&self) -> &HashMap<u64, BufferInfo> {
        &self.buffers
    }

    pub fn buffer(&self, id: u64) -> Option<&BufferInfo> {
        self.buffers.get(&id)
    }

    pub fn register_message_handlers(
        client: &AnyProtoClient,
        store: &Entity<Self>,
    ) {
        client.add_entity_message_handler::<proto::CreateBufferForPeer, Self, _, _>(
            move |store: Entity<Self>,
                  envelope: TypedEnvelope<proto::CreateBufferForPeer>,
                  mut cx: AsyncApp| async move {
                store.update(&mut cx, |store, cx| {
                    store.handle_create_buffer_for_peer(envelope.payload, cx);
                });
                Ok(())
            },
        );

        client.add_entity_message_handler::<proto::UpdateBuffer, Self, _, _>(
            move |store: Entity<Self>,
                  envelope: TypedEnvelope<proto::UpdateBuffer>,
                  mut cx: AsyncApp| async move {
                store.update(&mut cx, |store, cx| {
                    store.handle_update_buffer(envelope.payload, cx);
                });
                Ok(())
            },
        );
    }

    fn handle_create_buffer_for_peer(
        &mut self,
        message: proto::CreateBufferForPeer,
        cx: &mut Context<Self>,
    ) {
        use proto::create_buffer_for_peer::Variant;

        match message.variant {
            Some(Variant::State(state)) => {
                let file_path = state
                    .file
                    .as_ref()
                    .map(|f| PathBuf::from(&f.path));

                let buffer_id = state.id;
                self.buffers.insert(
                    buffer_id,
                    BufferInfo {
                        id: buffer_id,
                        file_path,
                        base_text: state.base_text,
                        is_complete: true,
                    },
                );
                cx.emit(WebBufferStoreEvent::BufferCreated { buffer_id });
                cx.notify();
            }
            Some(Variant::Chunk(chunk)) => {
                let buffer_id = chunk.buffer_id;
                if chunk.is_last {
                    if let Some(pending) = self.pending_chunks.remove(&buffer_id) {
                        self.buffers.insert(
                            buffer_id,
                            BufferInfo {
                                id: buffer_id,
                                file_path: pending.file_path,
                                base_text: pending.base_text,
                                is_complete: true,
                            },
                        );
                        cx.emit(WebBufferStoreEvent::BufferCreated { buffer_id });
                        cx.notify();
                    }
                } else {
                    self.pending_chunks
                        .entry(buffer_id)
                        .or_insert_with(|| PendingBuffer {
                            file_path: None,
                            base_text: String::new(),
                        });
                }
            }
            None => {}
        }
    }

    fn handle_update_buffer(
        &mut self,
        message: proto::UpdateBuffer,
        cx: &mut Context<Self>,
    ) {
        let buffer_id = message.buffer_id;
        if self.buffers.contains_key(&buffer_id) {
            cx.emit(WebBufferStoreEvent::BufferUpdated { buffer_id });
            cx.notify();
        }
    }
}
