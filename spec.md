# Web View State Architecture — Implementation Spec

## Problem Statement

Phase 1 (scene streaming) proved the transport layer works but is fundamentally a "dumb terminal" — every interaction requires a server round-trip, the browser has no semantic understanding of the UI, and there's no input path. Phase 2 replaces this with two layers:

1. **WebPlatform**: GPUI compiled to WASM so the full view/element/layout/paint/input stack runs natively in the browser.
2. **Entity Sync**: Server-side entity state (buffers, worktrees, workspace layout) mirrored to the browser-side GPUI instance via the existing `HeadlessProject` protocol over WebSocket.

The goal is a browser client that can connect to a running Zed server, receive worktree and buffer state, and render a file tree — with a diagnostic tool to observe the state sync wire protocol.

## Scope

Both layers, built incrementally. The minimum viable outcome is:

- GPUI core compiles to `wasm32-unknown-unknown` with stub platform implementations
- `HeadlessProject` accepts WebSocket connections alongside Unix socket connections
- A browser client connects, receives `UpdateWorktree` and `CreateBufferForPeer` messages, and renders a file tree
- A WebSocket inspector tool logs and visualizes state sync messages

## Constraints

- **Minimal disruption**: Extend existing crates (`remote_server`, `rpc`, `proto`, `gpui`) with feature-gated additions. Existing behavior must be unchanged when the new features are not enabled.
- **Additive pattern**: Follow the `scene_observer` pattern from Phase 1 — new hooks are feature-gated, new modules are added alongside existing ones, existing code paths are untouched.
- **Reuse existing protocol**: The browser client is architecturally identical to a native Zed instance connecting to a remote server. It uses the same `Envelope`-based protobuf protocol, the same `AnyProtoClient` abstraction, and the same message handlers.

## Architecture

```
┌─────────────────────────────────┐         ┌─────────────────────────────────┐
│           Server (Linux)        │         │       Browser (WASM + WebGPU)   │
│                                 │         │                                 │
│  ┌───────────┐  ┌────────────┐  │  proto  │  ┌────────────┐  ┌───────────┐ │
│  │   Zed     │──│ Headless   │──│────────>│──│  Proto     │──│  GPUI     │ │
│  │   Backend │  │ Project    │  │Envelope │  │  Client    │  │  (WASM)   │ │
│  │           │  │            │  │  over   │  │  (WASM)    │  │           │ │
│  │  LSP,Git, │  │  Buffers,  │  │  WS     │  │            │  │  Layout,  │ │
│  │  FS,DAP   │  │  Worktrees │  │<────────│──│  Actions   │  │  Paint,   │ │
│  │           │  │            │  │  proto  │  │  + Events  │  │  Render   │ │
│  └───────────┘  └────────────┘  │         │  └────────────┘  └───────────┘ │
└─────────────────────────────────┘         └─────────────────────────────────┘
                                    ▲
                                    │
                              ┌─────┴──────┐
                              │  Inspector │
                              │  (CLI/Web) │
                              └────────────┘
```

### Key Integration Points

1. **Server transport**: The server currently accepts connections via Unix sockets (`UnixListener` in `crates/remote_server/src/server.rs`). A WebSocket listener is added alongside it, producing the same `(incoming_tx, outgoing_rx)` channel pair that feeds into `RemoteClient::proto_client_from_channels`. The `HeadlessProject` doesn't know or care about the transport.

2. **Protocol reuse**: The browser receives the same `Envelope` messages that a native Zed client receives: `UpdateWorktree`, `CreateBufferForPeer`, `UpdateBuffer`, `UpdateDiagnosticSummary`, etc. No new proto messages are needed for the initial milestone.

3. **Browser-side hydration**: The browser creates a `Project` entity (or a lightweight equivalent) that processes incoming proto messages the same way `Project::remote()` does — creating `WorktreeStore`, `BufferStore`, etc. from the proto stream.

4. **WebPlatform**: A new `Platform` trait implementation for `wasm32-unknown-unknown` that provides the minimum needed to run GPUI: a foreground executor (rAF-based), a stub text system (upgraded later to HarfBuzz+Canvas2D), and a `WebWindow` backed by a `<canvas>` element with WebGPU.

## Detailed Requirements

### R1: GPUI WASM Compilation

Add a `web` feature flag to `crates/gpui/Cargo.toml`. When targeting `wasm32-unknown-unknown` with this feature:

- Platform-specific modules (`mac`, `linux`, `windows`) are excluded via `cfg`
- A new `platform/web/` module provides stub/minimal implementations of `Platform`, `PlatformWindow`, `PlatformDisplay`, `PlatformTextSystem`, `PlatformAtlas`, and `PlatformDispatcher`
- Dependencies that don't compile to WASM (`resvg`, `font-kit`, `cosmic-text`, `calloop`, `smol`, OS-specific crates) are gated behind `cfg(not(target_arch = "wasm32"))` or made optional
- `cargo build --target wasm32-unknown-unknown -p gpui --features web --no-default-features` succeeds

The WebPlatform implementations:

| Trait | Implementation |
|---|---|
| `Platform` | `WebPlatform` — rAF render loop, single virtual display, `navigator.clipboard` |
| `PlatformWindow` | `WebWindow` — `<canvas>` + WebGPU context, DOM event translation to `PlatformInput` |
| `PlatformTextSystem` | `StubWebTextSystem` initially (returns fixed metrics), upgraded to HarfBuzz+Canvas2D later |
| `PlatformAtlas` | Reuse `StreamingAtlas` (CPU-only, already exists) or a new WebGPU-backed atlas |
| `PlatformDispatcher` | `wasm-bindgen-futures::spawn_local` for foreground, Web Workers for background |

### R2: WebSocket Transport for HeadlessProject

Add a `web-server` feature to `crates/remote_server/Cargo.toml`. When enabled:

- A WebSocket listener (using `async-tungstenite`) binds alongside the existing Unix socket listeners
- Incoming WebSocket connections are upgraded and produce `(incoming_tx, outgoing_rx)` channel pairs
- These channels feed into `RemoteClient::proto_client_from_channels` — the same path Unix sockets use
- The `HeadlessProject` is created and wired up identically regardless of transport
- Configurable via `ZED_WEB_SERVER_PORT` environment variable (default: 8080)

The WebSocket framing wraps `Envelope` protobuf messages with the same length-prefix format used by `read_message`/`write_message` in `crates/remote/src/protocol.rs`, adapted for WebSocket binary messages (the length prefix is implicit in WebSocket framing, so each binary message is one `Envelope`).

### R3: Browser-Side Proto Client

A WASM module that:

- Connects to the server's WebSocket endpoint
- Deserializes incoming `Envelope` messages (protobuf)
- Dispatches them to registered handlers (mirroring `ChannelClient` behavior)
- Sends outgoing `Envelope` messages (requests, buffer edits)

This is the WASM equivalent of `ChannelClient` in `crates/remote/src/remote_client.rs`. It implements `AnyProtoClient` so the browser-side `Project` entity can use it identically to how native Zed uses the SSH-based proto client.

### R4: Browser-Side Project Hydration

The browser creates entities from the proto stream:

1. `WorktreeStore::remote()` — receives `UpdateWorktree` messages, builds the file tree
2. `BufferStore::remote()` — receives `CreateBufferForPeer`, `UpdateBuffer` messages
3. A minimal `Workspace` view that renders the file tree from `WorktreeStore`

For the initial milestone, only worktree and buffer sync are needed. LSP, git, DAP, terminal, and extensions are deferred.

### R5: WebSocket Inspector Tool

A diagnostic tool that connects to the WebSocket endpoint and logs state sync messages. Two modes:

**CLI mode**: A Rust binary (`web_inspector`) that connects to the WebSocket, deserializes `Envelope` messages, and prints them as structured JSON to stdout. Supports filtering by message type.

**Web mode**: A lightweight HTML page (served alongside the browser client) that displays a live feed of messages with:
- Message type, sequence number, timestamp
- Payload summary (e.g., "UpdateWorktree: 15 entries added, 2 removed")
- Expandable full payload view
- Message rate and bandwidth counters

The inspector reuses the existing `WebStreamingServer` WebSocket infrastructure from Phase 1, extended to also broadcast proto `Envelope` messages on a separate channel.

### R6: File Tree Rendering

The browser GPUI instance renders a file tree from the synced `WorktreeStore`. This validates the full pipeline:

1. Server sends `UpdateWorktree` with `Entry` list
2. Browser `WorktreeStore` processes the message
3. GPUI renders a tree view from the worktree entries
4. User can expand/collapse directories (local interaction, no server round-trip)
5. Clicking a file sends `OpenBufferByPath` to the server
6. Server responds with `CreateBufferForPeer` + `BufferState`
7. Browser `BufferStore` hydrates the buffer

For the initial milestone, the file tree can be a simplified view (not the full `ProjectPanel`). Rendering buffer content in an editor view is a stretch goal.

## Build Plan

### Step 1: GPUI WASM Compilation Spike

**Goal**: `cargo check --target wasm32-unknown-unknown -p gpui --features web --no-default-features` passes.

- Add `web` feature to `gpui/Cargo.toml`
- Gate platform modules behind `cfg(not(target_arch = "wasm32"))`
- Gate OS-specific dependencies behind `cfg(not(target_arch = "wasm32"))`
- Create `crates/gpui/src/platform/web/` with stub implementations
- Fix compilation errors iteratively (catalog and fix each one)

**Verification**: `cargo check` succeeds for the WASM target.

### Step 2: WebSocket Transport for HeadlessProject

**Goal**: `HeadlessProject` accepts WebSocket connections and serves proto messages.

- Add `web-server` feature to `remote_server/Cargo.toml`
- Add WebSocket accept loop in `server.rs` alongside Unix socket loop
- Bridge WebSocket binary frames to `(mpsc::UnboundedSender<Envelope>, mpsc::UnboundedReceiver<Envelope>)` channels
- Feed channels into `proto_client_from_channels`
- Wire up `HeadlessProject` identically to the Unix socket path

**Verification**: A test client (e.g., `websocat`) can connect and receive the initial `Hello` handshake message.

### Step 3: WebSocket Inspector Tool

**Goal**: A CLI tool that connects to the WebSocket and logs proto messages.

- Create `crates/web_inspector/` with a binary that:
  - Connects to `ws://localhost:8080`
  - Reads binary WebSocket messages
  - Decodes `Envelope` protobuf
  - Prints message type, id, and payload summary as JSON
- Add filtering flags (`--type UpdateWorktree`, `--verbose`)
- Add a simple HTML inspector page that does the same in the browser

**Verification**: Run Zed with `ZED_WEB_SERVER_PORT=8080`, connect the inspector, open a file — see `UpdateWorktree` and buffer messages logged.

### Step 4: Browser-Side Proto Client (WASM)

**Goal**: A WASM module that connects to the WebSocket and processes proto messages.

- Create `crates/gpui-web/` (WASM entry point)
- Implement `WebProtoClient` (WASM equivalent of `ChannelClient`)
- Connect to WebSocket via `web-sys`
- Deserialize `Envelope` messages using `prost`
- Dispatch to registered handlers

**Verification**: Browser console logs decoded proto messages from the server.

### Step 5: Browser-Side Project Hydration + File Tree

**Goal**: Browser renders a file tree from server state.

- Create browser-side `WorktreeStore::remote()` and `BufferStore::remote()` (compile existing Rust code to WASM, or create lightweight equivalents)
- Create a minimal `FileTreeView` that renders worktree entries
- Wire up the proto client to feed messages into the stores
- Render the file tree using GPUI's element system via `WebPlatform`

**Verification**: Open Zed with a project, connect the browser — see the file tree rendered. Expand/collapse directories. Click a file — see buffer state arrive (logged by inspector).

## Acceptance Criteria

1. **GPUI compiles to WASM**: `cargo check --target wasm32-unknown-unknown -p gpui --features web --no-default-features` succeeds with no errors
2. **WebSocket transport works**: A WebSocket client can connect to the running Zed server and receive proto `Envelope` messages
3. **Inspector shows messages**: The CLI inspector tool displays `UpdateWorktree`, `CreateBufferForPeer`, and `UpdateBuffer` messages with readable summaries
4. **File tree renders in browser**: The browser client displays a file tree matching the server's open project, with expand/collapse working locally
5. **Existing behavior unchanged**: All existing tests pass. The `web` and `web-server` features are opt-in. Default builds are unaffected.
6. **Buffer open works**: Clicking a file in the browser file tree triggers `OpenBufferByPath`, and the buffer state arrives and is logged by the inspector

## Completion Criteria (Ralph Loop)

The implementation is complete when:

- [ ] Acceptance criteria 1-6 all pass
- [ ] The inspector tool is functional and demonstrates the state sync pipeline
- [ ] Code is committed on the `feature/web-view` branch with clean history
- [ ] No regressions in existing functionality (verified by running relevant tests)

## Files to Create/Modify

### New files
- `crates/gpui/src/platform/web/` — WebPlatform module (platform.rs, window.rs, display.rs, text_system.rs, dispatcher.rs, atlas.rs)
- `crates/web_inspector/` — Inspector CLI tool
- `crates/web_inspector/src/main.rs` — Inspector entry point
- `crates/gpui-web/` — WASM entry point crate
- `crates/gpui-web/src/lib.rs` — wasm-bindgen entry, WebSocket client, proto dispatch

### Modified files
- `crates/gpui/Cargo.toml` — Add `web` feature, gate OS deps
- `crates/gpui/src/platform.rs` — Add `mod web` behind `cfg(target_arch = "wasm32")`
- `crates/gpui/src/gpui.rs` — Gate platform-specific re-exports
- `crates/remote_server/Cargo.toml` — Add `web-server` feature
- `crates/remote_server/src/server.rs` — Add WebSocket accept loop
- `Cargo.toml` (workspace) — Add new crates to workspace members

### Unchanged
- All existing proto definitions (reused as-is)
- `HeadlessProject` (receives the same `AnyProtoClient`, doesn't know about transport)
- All existing platform backends (mac, linux, windows)
- All existing tests
