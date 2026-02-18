# TODO: Steps 4 & 5 — Browser-Side Proto Client + Project Hydration

## Current State Assessment

### What's Done (Steps 1-3)
- [x] GPUI compiles to `wasm32-unknown-unknown` with `--features web --no-default-features`
- [x] WebSocket transport in `remote_server` (activated via `ZED_WEB_SERVER_PORT`)
- [x] `gpui-web` crate: WASM module that connects to WebSocket, decodes `Envelope`, logs to console
- [x] Web inspector CLI tool (`web_inspector`)
- [x] `proto` crate compiles to WASM (minimal deps: `anyhow`, `prost`, `serde`)
- [x] `sum_tree` compiles to WASM
- [x] `collections` compiles to WASM

### What Doesn't Compile to WASM
- [ ] `rpc` crate — blocked by `async-tungstenite`, `rsa`, `zstd`, `rand` (all pull `getrandom`/`errno`)
- [ ] `worktree` crate — blocked by `fs` → `smol` → `async-io` → `errno`
- [ ] `language` crate — blocked by `fs`, `lsp`, `tree-sitter`, `smol`
- [ ] `text` crate — blocked by `util` → `smol`
- [ ] `project` crate — blocked by everything above plus `terminal`, `dap`, `extension`, etc.
- [ ] `remote` crate — blocked by `smol`, `fs`, `askpass`, `which`
- [ ] `client` crate — blocked by `async-tungstenite`, `tokio`, `fs`
- [ ] `settings` crate — blocked by `fs` → `smol`
- [ ] `util` crate — blocked by `smol` (used for archive operations)
- [ ] `rope` crate — blocked by `util` → `smol`

**Root cause:** `smol` (async runtime) is deeply embedded via `util` and `fs` crates. Almost every crate transitively depends on it.

---

## Step 4: Browser-Side Proto Client (WASM)

### Goal
A `WebProtoClient` that implements `ProtoClient` so the browser can use `AnyProtoClient` to register handlers, send requests, and receive messages — identical to how native Zed uses `ChannelClient`.

### TODO

#### 4.1: Make `rpc::proto_client` module available in WASM
- [ ] **Option A (Recommended): Feature-gate WASM-incompatible deps in `rpc` crate**
  - Add `cfg(not(target_arch = "wasm32"))` gates on `async-tungstenite`, `rsa`, `zstd`, `rand` in `rpc/Cargo.toml`
  - The `proto_client.rs` module only depends on `gpui`, `proto`, `parking_lot`, `futures`, `collections` — all WASM-compatible
  - The `peer.rs`, `conn.rs`, `message_stream.rs` modules use `async-tungstenite` and can be gated
  - This gives WASM access to `ProtoClient`, `AnyProtoClient`, `ProtoMessageHandlerSet`
  - **Risk:** Moderate refactoring of `rpc/src/rpc.rs` module structure
  
- [ ] **Option B: Extract `rpc-core` crate**
  - Move `proto_client.rs` into a new `rpc-core` crate with minimal deps
  - Both `rpc` and `gpui-web` depend on `rpc-core`
  - **Risk:** More workspace churn, but cleaner separation

- [ ] **Option C: Duplicate proto_client.rs into gpui-web**
  - Copy the ~550 lines of `proto_client.rs` into `gpui-web`
  - **Risk:** Code duplication, divergence over time

#### 4.2: Implement `WebProtoClient`
- [ ] Create `WebProtoClient` struct in `gpui-web` implementing `ProtoClient`:
  ```rust
  pub struct WebProtoClient {
      next_message_id: AtomicU32,
      ws: web_sys::WebSocket,
      response_channels: Mutex<HashMap<MessageId, oneshot::Sender<Envelope>>>,
      message_handlers: Mutex<ProtoMessageHandlerSet>,
  }
  ```
- [ ] Implement `ProtoClient::request()`:
  - Assign message ID, register response channel
  - Encode `Envelope` to bytes via `prost::Message::encode()`
  - Send via `WebSocket.send_with_u8_array()`
  - Return future that awaits the response channel
- [ ] Implement `ProtoClient::send()` and `send_response()`:
  - Assign message ID, encode, send via WebSocket
- [ ] Implement `ProtoClient::message_handler_set()`:
  - Return reference to the `Mutex<ProtoMessageHandlerSet>`

#### 4.3: Implement message receive loop
- [ ] In the WebSocket `onmessage` callback:
  1. Decode `Envelope` from binary data
  2. If `envelope.responding_to` is set → route to response channel
  3. Otherwise → call `build_typed_envelope()` and dispatch via `ProtoMessageHandlerSet::handle_message()`
- [ ] **Handle `Instant::now()` on WASM**: `build_typed_envelope()` uses `Instant::now()` for `received_at`. Options:
  - Use `web_time` crate (drop-in replacement for `std::time::Instant` on WASM)
  - Or make `received_at` use a WASM-compatible timestamp
  - Or cfg-gate the `Instant` usage in `proto/src/typed_envelope.rs`

#### 4.4: Integrate with GPUI App context
- [ ] The `WebProtoClient` needs access to `AsyncApp` for handler dispatch
- [ ] Create a GPUI `App` instance in the WASM entry point
- [ ] The message handlers run on the foreground executor (which in WASM runs synchronously or via `spawn_local`)
- [ ] Ensure `WebDispatcher` properly queues and runs tasks (current impl runs immediately, which may cause reentrancy issues)

#### 4.5: Handle the `RemoteStarted` handshake
- [ ] `ChannelClient` sends `RemoteStarted` on connection and waits for the server's `RemoteStarted`
- [ ] `WebProtoClient` needs to replicate this handshake
- [ ] Send `RemoteStarted` envelope after WebSocket connects
- [ ] Wait for server's `RemoteStarted` before dispatching other messages

### Verification
- [ ] Browser console shows decoded proto messages (already works)
- [ ] `WebProtoClient` can be wrapped in `AnyProtoClient`
- [ ] Handlers registered via `add_entity_message_handler` are called when messages arrive
- [ ] `request()` sends a message and receives the response

---

## Step 5: Browser-Side Project Hydration + File Tree

### Goal
Create browser-side entities that process proto messages and render a file tree.

### TODO

#### 5.1: Decide on compilation strategy

**The fundamental question:** Can we compile the existing `worktree`, `project`, `buffer_store` code to WASM, or do we need lightweight equivalents?

**Analysis:**
- The existing crates have deep dependency chains that don't compile to WASM
- The core blocker is `smol` (async runtime) embedded in `util` and `fs`
- Feature-gating `smol` out of `util` would require significant refactoring
- The `Worktree::remote()` constructor depends on `fs`, `language`, `settings`, `rpc` — all WASM-incompatible

**Recommended approach: Lightweight equivalents**

- [ ] **Create a `web-project` crate** (or extend `gpui-web`) with:
  - `WebWorktreeStore` — processes `UpdateWorktree` messages, maintains a tree of entries
  - `WebBufferStore` — processes `CreateBufferForPeer` messages, stores buffer content
  - `WebProject` — orchestrates the stores and proto client
  - These are ~500-1000 lines total, much simpler than the full crates

**Alternative approach: Make existing crates WASM-compatible**
- [ ] This would require:
  - Feature-gating `smol` in `util` (affects ~50 crates)
  - Feature-gating `fs` dependency in `worktree`, `language`, `settings`
  - Providing WASM stubs for `Fs` trait, `Settings`, etc.
  - **Estimated effort:** Very high, touches core infrastructure
  - **Benefit:** Code reuse, guaranteed protocol compatibility

#### 5.2: Implement `WebWorktreeStore` (lightweight equivalent)
- [ ] Define `WebWorktreeEntry` struct (subset of `worktree::Entry`):
  ```rust
  struct WebWorktreeEntry {
      id: u64,
      kind: EntryKind, // Dir, File
      path: Arc<Path>,
      is_ignored: bool,
  }
  ```
- [ ] Implement `UpdateWorktree` handler:
  - Parse `proto::UpdateWorktree` message
  - Add/remove entries from a `BTreeMap<Arc<Path>, WebWorktreeEntry>`
  - Emit events when entries change
- [ ] Register handler with `AnyProtoClient`:
  ```rust
  client.add_entity_message_handler(WebProject::handle_update_worktree);
  ```
- [ ] Subscribe entity:
  ```rust
  client.subscribe_to_entity(REMOTE_SERVER_PROJECT_ID, &project_entity);
  ```

#### 5.3: Implement `WebBufferStore` (lightweight equivalent)
- [ ] Handle `CreateBufferForPeer` messages:
  - Extract buffer ID, file path, initial content
  - Store in a `HashMap<BufferId, WebBuffer>`
- [ ] Handle `UpdateBuffer` messages:
  - Apply text operations to stored buffer content
  - (For initial milestone, just store the latest state)
- [ ] Implement `OpenBufferByPath` request:
  - Send request via `AnyProtoClient::request()`
  - Wait for `CreateBufferForPeer` response

#### 5.4: Implement `FileTreeView`
- [ ] Create a GPUI view that renders the worktree entries as a tree
- [ ] Use GPUI's element system (available in WASM via `WebPlatform`)
- [ ] Support expand/collapse directories (local state, no server round-trip)
- [ ] On file click: send `OpenBufferByPath` to server
- [ ] **Note:** The full `ProjectPanel` is ~7000 lines with many dependencies. The `FileTreeView` should be ~200-500 lines.

#### 5.5: Wire everything together
- [ ] In the WASM entry point (`gpui-web/src/gpui_web.rs`):
  1. Create GPUI `App` with `WebPlatform`
  2. Create `WebProtoClient`, wrap in `AnyProtoClient`
  3. Create `WebProject` entity with `WebWorktreeStore` and `WebBufferStore`
  4. Subscribe entities to proto client
  5. Register message handlers
  6. Open a GPUI window with `FileTreeView`
  7. Connect WebSocket → messages flow → entities update → view re-renders

### Verification
- [ ] Open Zed with a project, set `ZED_WEB_SERVER_PORT=8080`
- [ ] Open browser, load WASM client
- [ ] File tree appears matching the server's project
- [ ] Expand/collapse directories works locally
- [ ] Click a file → `OpenBufferByPath` sent (visible in inspector)
- [ ] Buffer state arrives → logged by inspector

---

## Key Technical Decisions Needed

### 1. How to get `ProtoClient`/`AnyProtoClient` in WASM?
**Recommendation:** Feature-gate the `rpc` crate (Option A from 4.1). The `proto_client.rs` module has clean dependencies (`gpui`, `proto`, `parking_lot`, `futures`, `collections`) that all compile to WASM. Only the networking modules (`peer.rs`, `conn.rs`, `message_stream.rs`) need gating.

### 2. Lightweight equivalents vs. compiling existing crates?
**Recommendation:** Lightweight equivalents for the initial milestone. The dependency chain is too deep to untangle quickly. The lightweight `WebWorktreeStore` only needs to:
- Parse `proto::UpdateWorktree` messages (proto crate compiles to WASM ✓)
- Store entries in a tree structure (sum_tree compiles to WASM ✓, or use BTreeMap)
- Emit GPUI events (gpui compiles to WASM ✓)

### 3. How to handle `Instant::now()` in WASM?
**Recommendation:** Add `web-time` crate as a dependency for `wasm32` targets. It's a drop-in replacement:
```toml
[target.'cfg(target_arch = "wasm32")'.dependencies]
web-time = "1"
```
Then alias: `#[cfg(target_arch = "wasm32")] use web_time::Instant;`

### 4. How to handle the async runtime in WASM?
**Recommendation:** Use `wasm-bindgen-futures::spawn_local()` for all async work. The `WebDispatcher` already runs tasks synchronously. For the message receive loop, use the WebSocket `onmessage` callback (event-driven, no polling needed).

---

## Dependency Graph for WASM Browser Client

```
gpui-web (WASM entry point)
├── proto ✅ (compiles to WASM)
├── gpui ✅ (compiles to WASM with --features web)
├── rpc ❌→✅ (needs feature-gating for WASM)
│   └── proto_client.rs ✅ (clean deps)
│   └── peer.rs ❌ (gate behind cfg)
│   └── conn.rs ❌ (gate behind cfg)
├── collections ✅
├── sum_tree ✅
├── web-sys ✅
├── wasm-bindgen ✅
└── prost ✅
```

## Estimated Effort

| Task | Effort | Risk |
|------|--------|------|
| 4.1: Feature-gate `rpc` for WASM | 1-2 days | Medium (module restructuring) |
| 4.2: Implement `WebProtoClient` | 2-3 days | Low (clear pattern from ChannelClient) |
| 4.3: Message receive loop | 1 day | Low |
| 4.4: GPUI App integration | 1-2 days | Medium (WASM executor quirks) |
| 4.5: RemoteStarted handshake | 0.5 day | Low |
| 5.1: Design lightweight stores | 0.5 day | Low |
| 5.2: WebWorktreeStore | 2-3 days | Medium (proto message parsing) |
| 5.3: WebBufferStore | 1-2 days | Low (simpler than worktree) |
| 5.4: FileTreeView | 2-3 days | Medium (GPUI rendering in WASM) |
| 5.5: Integration | 1-2 days | Medium |
| **Total** | **~12-18 days** | |
