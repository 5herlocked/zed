# Web View State Architecture

This document describes the next-phase architecture for making Zed accessible through a web browser. It supersedes the scene-streaming approach documented in `web-streaming-renderer.md`, which served as a proof of concept and validated the transport layer, protobuf encoding, and WebGPU rendering pipeline.

## What We Proved (Phase 1: Scene Streaming)

The Phase 1 proof of concept streamed GPUI's rendered scene primitives (quads, shadows, underlines, sprites) over WebSocket to a browser-side WebGPU renderer. Key results:

- **Transport**: Protobuf-encoded scene frames at 73fps over WebSocket with full atlas tile data. Binary encoding eliminated the JSON bottleneck that capped throughput at 4fps.
- **Rendering**: The GPUI WGSL shaders port to browser WebGPU with minimal changes (strip `dual_source_blending`, grayscale fallback for subpixel sprites, remove `f` float literal suffixes, add `@interpolate(flat)` on integer inter-stage variables).
- **Atlas mirroring**: A `MirroringAtlas` wrapping the native platform atlas (Metal/wgpu) intercepts glyph rasterization on cache miss and captures CPU-side copies of tile pixel data. The browser mirrors these tiles into WebGPU textures.
- **Theme**: Pre-converted theme colors (CSS hex strings) from the Rust side via `ThemeHints` in the frame message solve the color space mismatch between native and browser rendering.
- **Integration**: A `set_scene_observer_factory` hook on `App` and a `scene_observer` on `Window` allow any platform (macOS, Linux, Windows) to stream scenes without modifying the native renderer. Enabled via `ZED_WEB_STREAMING=1`.

## Why Scene Streaming Breaks Down

Scene streaming treats the browser as a dumb terminal. Every pixel of the UI is decided server-side and shipped to the browser as flat rendering primitives. This has fundamental limitations:

**Round-trip latency on every interaction.** Every mouse move, scroll, hover, and keystroke must travel to the server, trigger a GPUI re-render, and stream the new scene back. At 20ms network latency, hover states feel sluggish. At 100ms (cross-region), the editor is unusable.

**No local responsiveness.** The browser cannot animate cursor blinks, smooth-scroll, show hover tooltips, or highlight selections without waiting for the server. Native Zed handles these locally on the render thread.

**Missing UI fidelity.** GPUI's scene is the output of rendering -- it discards the semantic structure. SVG icons become monochrome atlas tiles. Toolbar buttons become quads. Scroll indicators, breadcrumbs, file tree icons, inline code block backgrounds, avatar indicators, diff markers -- all the rich UI chrome that makes Zed feel like Zed -- are flattened into anonymous primitives that the browser cannot distinguish or interact with.

**No input path.** The Phase 1 prototype captures browser input events but has no mechanism to inject them into GPUI's dispatch pipeline on the server. Building this for scene streaming means reimplementing GPUI's hit testing, focus management, and action dispatch on the server based on browser coordinates -- duplicating the work GPUI already does.

**Atlas complexity.** Mirroring the native GPU atlas to the browser is fragile. Tile eviction, reuse, subpixel variants, and scale factor changes all create synchronization edge cases. The Phase 1 prototype showed missing glyphs on file switches due to cache key mismatches.

## Phase 2 Strategy: WebPlatform + Entity Sync

The architecture has two layers, built in sequence:

1. **WebPlatform**: A `Platform` trait implementation targeting `wasm32-unknown-unknown` that makes GPUI run natively in the browser. This is the foundation -- once GPUI runs in the browser, the entire view/element/layout/paint/input stack works locally.

2. **Entity Sync**: A state synchronization layer that mirrors server-side entity state (buffers, editor state, workspace layout, diagnostics, etc.) into the browser-side GPUI instance. The browser GPUI renders from this mirrored state with full local responsiveness.

The key insight: GPUI's `Platform` trait is the abstraction boundary. Everything above it (entities, views, Taffy layout, element painting, input dispatch, action system) is platform-agnostic Rust. Everything below it (window management, text rendering, GPU context, clipboard) is platform-specific. By implementing `Platform` for the web, the entire GPUI stack runs in the browser unchanged.

## Layer 1: WebPlatform (GPUI in the Browser)

### The Platform Trait

GPUI already has four Platform implementations:

| Platform | Renderer | Window System | Text System |
|---|---|---|---|
| `MacPlatform` | Metal | Cocoa/AppKit | CoreText |
| `LinuxPlatform` | wgpu (Vulkan/GL) | Wayland/X11 | cosmic-text/swash |
| `WindowsPlatform` | DirectX | Win32 | DirectWrite |
| `TestPlatform` | None (headless) | Simulated | NoopTextSystem |

A `WebPlatform` would be the fifth:

| Platform | Renderer | Window System | Text System |
|---|---|---|---|
| `WebPlatform` | WebGPU | DOM/Canvas | Canvas2D + HarfBuzz (WASM) |

### What the Platform Trait Requires

The `Platform` trait (`crates/gpui/src/platform.rs`) has roughly 50 methods. Not all are relevant for the web. Here's the breakdown:

**Core (must implement):**

| Method | Web Implementation |
|---|---|
| `background_executor` | Web Workers via `wasm-bindgen-futures` |
| `foreground_executor` | `requestAnimationFrame` + microtask queue |
| `text_system` | `PlatformTextSystem` backed by Canvas2D for shaping, HarfBuzz-WASM for metrics |
| `run` | Start the rAF render loop |
| `quit` | Close the tab / no-op |
| `activate` | `window.focus()` |
| `displays` | Single virtual display from `window.screen` |
| `open_window` | Create a `<canvas>` element, return `WebWindow` |
| `window_appearance` | `prefers-color-scheme` media query |
| `set_cursor_style` | `document.body.style.cursor` |
| `read_from_clipboard` / `write_to_clipboard` | `navigator.clipboard` API |
| `keyboard_layout` | `navigator.keyboard` API or static US layout |

**Delegated to server (via entity sync):**

| Method | Why |
|---|---|
| `open_url` | Server opens the URL or sends it back to the browser via a different channel |
| `prompt_for_paths` | Server shows a file picker or the browser uses `<input type="file">` |
| `reveal_path` | Server-side filesystem operation |
| `write_credentials` / `read_credentials` | Server-side keychain |
| `register_url_scheme` | Server-side OS integration |
| `app_path` / `path_for_auxiliary_executable` | Server-side paths |

**Not applicable (stub or no-op):**

| Method | Why |
|---|---|
| `hide` / `hide_other_apps` / `unhide_other_apps` | No concept in browser |
| `set_menus` / `set_dock_menu` | No native menu bar |
| `add_recent_document` | No OS-level recent docs |
| `screen_capture_sources` | Not supported |
| `thermal_state` | Not available |

### PlatformWindow for the Web

The `PlatformWindow` trait is implemented by `WebWindow`, backed by a `<canvas>` element with a WebGPU context. This is largely proven by Phase 1 -- the WebGPU rendering pipeline, WGSL shaders, atlas management, and batch drawing all carry forward.

Key differences from the Phase 1 `StreamingWindow`:

- **Real rendering**: `WebWindow::draw()` renders the scene to the canvas using the WebGPU pipeline (Phase 1's `renderer.ts` logic, but compiled from Rust via wasm-bindgen instead of TypeScript).
- **Real input**: `WebWindow::on_input()` receives DOM events translated to GPUI's `PlatformInput` types. Input dispatch goes through GPUI's standard action/keymap system locally.
- **Real display**: `WebWindow::display()` returns a `WebDisplay` with actual screen dimensions from `window.screen`.

### PlatformTextSystem for the Web

Text rendering is the hardest part. The `PlatformTextSystem` trait requires:

| Method | What it does |
|---|---|
| `add_fonts` | Load font data (TTF/OTF bytes) |
| `all_font_names` | List available fonts |
| `font_id` | Look up a font by name/style |
| `font_metrics` | Return ascent, descent, line height, etc. |
| `typographic_bounds` | Bounding box for a glyph |
| `advance` | Horizontal advance for a glyph |
| `glyph_for_char` | Map character to glyph ID |
| `glyph_raster_bounds` | Pixel bounds for rasterized glyph |
| `rasterize_glyph` | Produce pixel data for a glyph |
| `layout_line` | Shape and lay out a line of text |

**Recommended approach: HarfBuzz (WASM) + Canvas2D rasterization.**

HarfBuzz compiles to WASM and handles text shaping (the hard part -- ligatures, kerning, complex scripts). It produces glyph IDs and positions. For rasterization, we use an offscreen Canvas2D to draw each glyph and read back the pixel data into the atlas. Font files are shipped from the server at connection time (or pre-bundled for common fonts like Zed Plex Mono).

This gives us accurate shaping that matches the server's text layout (HarfBuzz is the same shaping engine used by cosmic-text on Linux) and efficient rasterization using the browser's native font renderer.

### Compiling GPUI to WASM

GPUI's core is mostly platform-agnostic Rust. The main obstacles for `wasm32-unknown-unknown`:

**Threading.** GPUI uses `BackgroundExecutor` and `ForegroundExecutor` backed by a thread pool. In WASM, true threads require SharedArrayBuffer + Web Workers. The `wasm-bindgen-futures` crate provides async executors that run on the main thread. For CPU-intensive work (text shaping, layout), Web Workers can be used.

**Async runtime.** GPUI uses `smol` as its async runtime. In WASM, we need a browser-compatible executor. `wasm-bindgen-futures::spawn_local` handles single-threaded async. For the background executor, tasks can be posted to a Web Worker pool.

**Platform dependencies.** Any code that touches `std::fs`, `std::net`, `std::process`, or platform-specific APIs needs to be gated behind `cfg(not(target_arch = "wasm32"))` or abstracted behind the Platform trait.

**Crate compatibility.** Most of GPUI's dependencies are pure Rust and compile to WASM: `taffy`, `serde`, `smallvec`, `parking_lot` (with WASM feature), `etagere`. The GPU-specific crates (`metal`, `wgpu`, `cocoa`) are already behind `cfg` gates. The text system crates (`cosmic-text`, `font-kit`) need to be swapped for the WASM text system.

### Build Plan for WebPlatform

**Step 1: Compile GPUI core to WASM.** Gate all platform-specific code behind `cfg`. Add a `web` feature flag. Get `cargo build --target wasm32-unknown-unknown --features web` to succeed with stub implementations.

**Step 2: Implement `WebPlatform` and `WebWindow`.** Port the Phase 1 TypeScript renderer to Rust using `wgpu` (which has a `wasm32` backend targeting WebGPU). The shaders are already WGSL. The atlas, batch iterator, and draw pipeline translate directly.

**Step 3: Implement `WebTextSystem`.** Compile HarfBuzz to WASM. Implement `PlatformTextSystem` using HarfBuzz for shaping and Canvas2D for rasterization. Ship font files from the server.

**Step 4: Run a standalone GPUI app in the browser.** Build a minimal GPUI application (hello world with text, buttons, scrolling) that runs entirely in the browser via the WebPlatform. No server connection needed. This validates the Platform implementation.

**Step 5: Run Zed's UI components.** Incrementally bring up Zed's UI crates (`ui`, `editor`, `workspace`, `terminal_view`, etc.) in the browser. Each crate that compiles to WASM and renders correctly is a milestone.

## Layer 2: Entity Sync (Server State in the Browser)

Once GPUI runs in the browser, we need to feed it state from the server. The browser-side GPUI instance has its own entity map, but the entities are mirrors of server-side state rather than locally-owned.

### Architecture

```text
┌─────────────────────────────────┐         ┌─────────────────────────────────┐
│           Server (Linux)        │         │       Browser (WASM + WebGPU)   │
│                                 │         │                                 │
│  ┌───────────┐  ┌────────────┐  │  state  │  ┌────────────┐  ┌───────────┐ │
│  │   Zed     │──│  GPUI      │──│────────>│──│  State     │──│  GPUI     │ │
│  │   Backend │  │  Entities  │  │  deltas │  │  Mirror    │  │  (WASM)   │ │
│  │           │  │            │  │         │  │            │  │           │ │
│  │  LSP,Git, │  │  Buffers,  │  │<────────│──│  Input     │  │  Layout,  │ │
│  │  FS,DAP,  │  │  Editors,  │  │  events │  │  + Actions │  │  Paint,   │ │
│  │  Terminal  │  │  Workspace │  │         │  │            │  │  Render   │ │
│  └───────────┘  └────────────┘  │         │  └────────────┘  └───────────┘ │
└─────────────────────────────────┘         └─────────────────────────────────┘
```

The server runs Zed's full backend: LSP, Git, filesystem, DAP, terminal, AI agents, extensions. It maintains the canonical entity state. The browser runs GPUI natively (via WebPlatform) and renders from mirrored state. Input is handled locally with immediate feedback. Server-dependent operations (file I/O, LSP requests, git operations) are forwarded to the server.

### What Gets Synchronized

| Entity | What the browser needs | Sync frequency |
|---|---|---|
| Buffer content | Rope text, selections, diagnostics, inlays | On edit, via OT/CRDT |
| Editor state | Scroll position, fold state, cursor positions | On change |
| Workspace layout | Pane arrangement, panel visibility, sidebar state | On layout change |
| File tree | Directory structure, open/closed state, file status | On FS change |
| Theme | Full color palette, font settings, UI density | On theme change |
| Settings | Editor settings (tab size, word wrap, etc.) | On settings change |
| Terminal | Screen buffer, cursor, scrollback | On terminal output |
| Search | Query, results, match positions | On search update |
| Git status | File statuses, diff hunks | On Git change |
| Diagnostics | Error/warning locations, messages | On diagnostic update |

### What Stays Server-Side Only

- LSP communication (completions, hover, go-to-definition requests)
- Filesystem operations (file read/write, directory listing)
- Git operations (commit, push, pull)
- Extension host (WASM execution)
- AI agent tool execution
- Build/run/debug processes

### Reusing the Remote Server Infrastructure

Zed already solves this problem. The remote development feature (`crates/remote_server/`) runs a `HeadlessProject` on a remote machine that serializes project state -- buffers, worktrees, LSP, DAP, git, tasks, context servers, extensions -- over a protocol client. The local Zed instance connects via SSH, receives this state, hydrates it into a local `Project`, and renders normally. The user edits as if the files were local.

The browser client is architecturally identical to a local Zed instance connecting to a remote server. The difference is transport (WebSocket instead of SSH) and platform (WebPlatform/WASM instead of macOS/Linux). From the server's perspective, the browser is just another Zed client.

**What `HeadlessProject` already serializes:**

| Component | Protocol | Crate |
|---|---|---|
| Worktrees (file tree, file contents) | `proto::AddWorktree`, `proto::UpdateWorktree` | `crates/worktree/` |
| Buffers (rope text, edit operations) | `proto::OpenBufferByPath`, `proto::UpdateBuffer` | `crates/text/`, `crates/language/` |
| LSP state (diagnostics, completions, hover) | `proto::UpdateDiagnostics`, forwarded LSP requests | `crates/lsp/`, `crates/project/` |
| DAP (debugger state) | `proto::` DAP messages | `crates/dap/` |
| Tasks (run configurations) | `proto::` task messages | `crates/task/` |
| Git (status, diff hunks) | `proto::` git messages | `crates/git/` |
| Settings (project settings) | Settings file sync | `crates/settings/` |
| Extensions | Extension host proxy | `crates/extension_host/` |
| Agent/Context servers | Server management | `crates/agent_servers/`, `crates/context_server/` |

**What we add for the browser:**

| Component | What's new | Why |
|---|---|---|
| Transport | WebSocket adapter for `AnyProtoClient` | SSH transport doesn't work in browsers |
| Initial state | Full workspace layout, theme, keybindings | Remote server sends project state but not UI chrome |
| View state | Editor scroll positions, panel visibility, focus | Remote server syncs project data, not editor UI state |
| Fonts | Font file delivery at connection time | Remote server doesn't need to send fonts |

The implementation path: make `HeadlessProject` connectable via WebSocket (it currently uses Unix sockets via SSH), add workspace/UI state to the sync protocol, and connect the browser-side GPUI instance as the rendering frontend. The browser's `Project` entity hydrates from the same protocol messages that a native Zed instance uses.

This means the browser client gets collaboration support for free -- the same OT/CRDT buffer sync that powers multi-user editing works between the server and browser, with the same conflict resolution guarantees.

The browser sends semantic events to the server:

| Browser event | Server action |
|---|---|
| "Insert text 'hello' at cursor" | Buffer edit via OT/CRDT |
| "Open file src/main.rs" | FS read, buffer creation, state push |
| "Request completions at line 42 col 10" | LSP completion request, results pushed back |
| "Toggle fold at line 100" | Editor state update, delta pushed |
| "Run action ToggleTerminal" | Action dispatch, state delta pushed |

### Sync Protocol

Protobuf-encoded bidirectional messages over WebSocket (reusing the Phase 1 transport):

```protobuf
// Server -> Browser
message StateUpdate {
  uint64 sequence = 1;
  repeated EntityDelta deltas = 2;
}

message EntityDelta {
  uint64 entity_id = 1;
  string entity_type = 2;
  oneof change {
    bytes full_state = 3;
    bytes incremental_patch = 4;
    bool removed = 5;
  }
}

// Browser -> Server
message ClientEvent {
  uint64 sequence = 1;
  oneof event {
    ActionDispatch action = 2;
    BufferEdit edit = 3;
    ViewportChange viewport = 4;
  }
}
```

### Build Plan for Entity Sync

**Step 1: WebSocket transport for HeadlessProject.** Adapt the existing `HeadlessProject` protocol to accept WebSocket connections alongside Unix socket / SSH connections. The `RemoteClient::proto_client_from_channels` pattern in `crates/remote_server/src/server.rs` already abstracts over the transport -- add a WebSocket channel implementation. This is a small, testable change with no browser dependency.

**Step 2: Browser-side ProtoClient.** Implement `AnyProtoClient` in WASM that communicates over the WebSocket. This is the browser equivalent of the SSH-based proto client that native Zed uses for remote development. It receives `Envelope` messages and dispatches them to request/response handlers.

**Step 3: Hydrate Project in the browser.** Connect the browser GPUI instance to the server via the WebSocket proto client. The server sends worktree, buffer, and diagnostic updates using the existing protocol. The browser hydrates a `Project` entity from these messages, identical to how native Zed hydrates a remote project. Verify by checking that the file tree renders and buffers open.

**Step 4: Add workspace/UI state sync.** The existing remote protocol syncs project data but not editor UI state (scroll positions, panel visibility, focus, theme). Extend the protocol with workspace layout messages. The server observes its own workspace state and pushes changes. The browser applies them.

**Step 5: Local input handling.** Wire GPUI's input dispatch in the browser to handle editor actions locally (cursor movement, selection, scrolling) and forward server-dependent actions (file open, LSP requests, git operations) over the proto client. Buffer edits use the existing OT/CRDT machinery from `crates/text/`.

**Step 6: Optimistic editing.** Text edits are applied locally for immediate feedback and sent to the server for authoritative processing. Conflicts are resolved via the same CRDT sync that powers Zed's multi-user collaboration. From the server's perspective, the browser is just another collaborator.

## What We Keep From Phase 1

- **WebGPU rendering pipeline**: Shaders, atlas management, batch drawing, all rewritten in Rust (for WASM) instead of TypeScript.
- **Protobuf transport**: WebSocket + protobuf encoding. The `scene.proto` schema is retired but the transport infrastructure (`WebStreamingServer`, async-tungstenite) carries forward.
- **`web-streaming` feature flag**: Extended to gate WebPlatform compilation.
- **`ThemeHints`**: Pre-converted theme data.
- **`web_renderer/` project structure**: Evolves from a standalone TypeScript app into a thin HTML/JS shell that loads the WASM GPUI module.

## What We Retire From Phase 1

- **`MirroringAtlas`**: Replaced by browser-side font rasterization.
- **`BinaryFrameEncoder` / `scene_message.rs`**: Replaced by entity sync protocol.
- **`scene_observer` on `Window`**: Replaced by entity-level change tracking.
- **TypeScript renderer** (`renderer.ts`, `shaders.wgsl`, `atlas.ts`, `protocol.ts`, `binary_decoder.ts`, `proto_decoder.ts`): Replaced by Rust GPUI compiled to WASM.
- **Full-scene-per-frame streaming**: Replaced by incremental entity state deltas.

## Open Design Questions

**GPUI in WASM: how much compiles?** The answer determines the scope of Step 1. A spike that attempts `cargo build --target wasm32-unknown-unknown -p gpui` and catalogs the failures would be the fastest way to size this.

**wgpu WASM backend maturity.** `wgpu` supports `wasm32` with the WebGPU backend, but maturity varies. The alternative is to keep the TypeScript WebGPU renderer from Phase 1 and call it from Rust via `wasm-bindgen`, which is less elegant but more proven.

**Font shipping.** Zed Plex Mono and other bundled fonts need to be available in the browser. Options: embed in the WASM binary, lazy-load from a CDN, or ship at WebSocket connection time. The font files total ~2MB which is acceptable for initial load.

**Operational transform reuse.** Zed's collaboration OT/CRDT lives in `crates/text/` and `crates/rpc/`. How much of this can be compiled to WASM and reused on the browser side for local buffer editing? If it compiles, we get conflict-free collaborative editing for free. This is the highest-leverage WASM compilation target after GPUI core.

**Action registry sync.** The browser needs to know available actions and keybindings to handle input locally. This is a relatively small, infrequently-changing dataset that can be sent at connection time and updated on keymap changes.

**Multiple browser clients.** Since the server is just a `HeadlessProject`, multiple browsers can connect simultaneously. This is collaboration for free -- each browser is a participant in the same CRDT session. Whether this is a feature or a footgun depends on the use case.

**Incremental adoption of HeadlessProject protocol.** Not every message in the existing remote protocol needs to work on day one. The browser can start with worktree + buffer sync (enough to render the editor), then incrementally add LSP, git, terminal, DAP, and extensions. Each addition is a self-contained milestone.

## File Structure (Proposed)

```text
crates/gpui/src/
  platform/
    web/                            # New: WebPlatform implementation
      mod.rs                        # Platform trait impl
      window.rs                     # PlatformWindow backed by <canvas> + WebGPU
      display.rs                    # PlatformDisplay from window.screen
      text_system.rs                # HarfBuzz WASM + Canvas2D rasterization
      dispatcher.rs                 # wasm-bindgen-futures executor
      clipboard.rs                  # navigator.clipboard API
      events.rs                     # DOM event -> PlatformInput translation
    web_streaming/                  # Phase 1 infrastructure (retained for transport)
      server.rs                     # WebSocket server (reused)
      proto_encoding.rs             # Extended for entity sync

crates/gpui-wasm/                   # WASM entry point and JS bindings
  src/
    lib.rs                          # wasm-bindgen entry point
  Cargo.toml                        # wasm32 target, gpui with web feature

crates/remote_server/src/
  server.rs                         # Extended: WebSocket transport alongside SSH
  headless_project.rs               # Reused as-is for browser clients

crates/remote_connection/           # Extended: WebSocket channel for proto client
  src/
    websocket_channel.rs            # New: WebSocket-based proto channel

web_renderer/                       # Thin HTML/JS shell (replaces Phase 1 TS app)
  index.html                        # Loads WASM module
  bootstrap.js                      # WASM initialization, WebSocket setup
  package.json
```

## Key References

| What | Where |
|---|---|
| Phase 1 design doc | `docs/src/development/web-streaming-renderer.md` |
| Phase 1 Rust module | `crates/gpui/src/platform/web_streaming/` |
| Phase 1 browser renderer | `web_renderer/` |
| GPUI Platform trait | `crates/gpui/src/platform.rs` |
| GPUI entity system | `crates/gpui/src/app.rs` (App, Entity, EntityMap) |
| GPUI view/render system | `crates/gpui/src/view.rs`, `crates/gpui/src/element.rs` |
| GPUI layout engine | `crates/gpui/src/taffy.rs` |
| GPUI scene primitives | `crates/gpui/src/scene.rs` |
| GPUI input dispatch | `crates/gpui/src/key_dispatch.rs`, `crates/gpui/src/interactive.rs` |
| Remote server (HeadlessProject) | `crates/remote_server/src/headless_project.rs` |
| Remote server startup | `crates/remote_server/src/server.rs` |
| Remote connection (proto client) | `crates/remote_connection/` |
| Proto client channels | `crates/rpc/` |
| Existing protobuf schema | `crates/proto/proto/zed.proto` |
| Buffer CRDT / OT | `crates/text/` |
| Collaboration infrastructure | `crates/collab/`, `crates/client/` |
| Worktree sync | `crates/worktree/` |
| wgpu WASM support | https://github.com/gfx-rs/wgpu/wiki/Running-on-the-Web |
| HarfBuzz WASM | https://github.com/nicolo-ribaudo/harfbuzzjs |