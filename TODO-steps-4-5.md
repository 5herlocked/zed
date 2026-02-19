# Zed Web -- Element Tree Sync

Branch: `feature/web-view` (20 commits ahead of `personal/feature/web-view`)

This document is the single source of truth for any agent picking up this work. Phase 1 is complete and verified. Phases 2-6 are TODO. Read the whole file before starting -- the "Previous Attempts" and "Known Gotchas" sections exist to prevent you from repeating mistakes we already made.

## How to Run

```bash
# Build (once, or after code changes)
cargo build -p zed_server

# Terminal 1: start the server
RUST_LOG=info ./target/debug/zed_server --bind 127.0.0.1:8080

# Terminal 2: serve the Canvas2D PoC viewer
cd crates/zed_server && python3 -m http.server 8888

# Open http://127.0.0.1:8888/viewer.html
```

## Architecture

The server runs full native Zed (tree-sitter, LSP, git, filesystem) inside a `StreamingPlatform` that captures a serializable `DisplayTree` during GPUI's render pipeline. Frames stream over WebSocket to a browser client. The browser is a thin rendering client -- all state lives on the server.

```
Zed (native) -> GPUI render -> DisplayTree capture -> JSON/WebSocket -> Browser renderer
```

The browser will ultimately run GPUI compiled to WASM, hydrating `DisplayTree` frames into real DOM elements (positioned divs, text spans, CSS styling). DOM projection was chosen over Canvas/WebGPU so that text is natively selectable, Cmd+F works, copy/paste works, and accessibility comes free.

## Phase 1: DisplayTree Capture -- DONE

Everything needed to capture a complete, serializable snapshot of GPUI's rendered output.

### DisplayTree Data Structure (`crates/gpui/src/display_tree.rs`, ~1154 lines)

DisplayNode with kinds: Container, Text, Image, Svg, UniformList, List, Anchored, Canvas. DisplayStyle split into layout (Taffy properties), visual (backgrounds, borders, shadows, corners, opacity, overflow, visibility), and text (font family, size, weight, style, line height, alignment). InteractionFlags bitfield (clickable, hoverable, scrollable, focusable, draggable, focusable). DisplayTreeBuilder with stack-based construction during render walks. DisplayTreeDelta + DisplayTreePatch for diff/patch delta encoding. DisplayAction + DisplayActionKind for browser-to-server action forwarding. Compiled behind `#[cfg(feature = "headless-web")]`.

### Capture Hooks

All gated behind `#[cfg(feature = "headless-web")]`:

- **Container capture** (`crates/gpui/src/elements/div.rs`): Interactivity::prepaint pushes a container node with full style conversion (layout, visual, interaction flags, bounds). Every div, panel, toolbar, and layout element is captured here.

- **Text capture** (`crates/gpui/src/text_system/line.rs`): ShapedLine::paint() and WrappedLine::paint() push styled text nodes with decoration runs (foreground color, background highlight, underline, strikethrough). This is the single capture point for ALL painted text -- editor buffer lines, terminal output, labels, markdown preview, everything. Earlier text.rs element-level hooks were removed because ShapedLine subsumes them completely.

- **SVG capture** (`crates/gpui/src/elements/svg.rs`): prepaint hook upgrades the node kind from Container to Svg with the SVG path identifier. Icons in the file tree, toolbar, and status bar are all captured.

- **Image capture** (`crates/gpui/src/elements/img.rs`): prepaint hook upgrades to Image kind with Resource source (Uri, Path, or Embedded asset name) and ObjectFit mode.

- **Canvas elements** (~25 call sites: git graph, circular progress, dividers): These are opaque paint closures that draw directly to the Scene. They get a Container node but the paint content is not captured. All are decorative, not content-bearing.

### Streaming Infrastructure

- **StreamingPlatform** (`crates/gpui/src/platform/streaming/`): CFRunLoopTimer at 120fps drives frame requests on the main thread. Resize channel (smol bounded(4)) bridges tokio WebSocket recv to main thread for thread safety. StreamingWindow implements PlatformWindow with simulate_resize() and simulate_input().

- **Wire protocol**: WireFrame enum with Full/Delta/ViewportChanged variants. postcard serialize/deserialize. JSON used in the PoC viewer; postcard binary for production.

- **Frame pipeline** (`crates/zed_server/src/main.rs`, ~280 lines): smol channel (render thread) -> frame_bridge (serde_json conversion, tokio task) -> tokio watch channel -> WebSocket broadcast. Watch channel means late-connecting clients get the latest frame immediately.

- **zed_server binary**: Full Zed workspace initialization (AppState, client, fs, languages, node_runtime, theme, editor, workspace). Opens a real workspace via workspace::open_new() showing the welcome screen.

- **Canvas2D PoC viewer** (`crates/zed_server/viewer.html`, ~343 lines): Renders DisplayTree with text runs, backgrounds, borders, shadows, rounded corners. Reports viewport size on connect and resize (debounced 150ms). Logs node kind stats every 5 seconds.

### Verified Output (Welcome Screen)

| Kind | Count | What |
|------|-------|------|
| Container | 79 | All layout: panels, toolbars, divs, status bar |
| Text | 13 | Welcome screen labels via ShapedLine hooks |
| Svg | 10 | Icons: file tree, toolbar, status bar |
| Image | 0 | None on welcome screen (expected) |

Frame size: ~90KB JSON, 102 nodes total.

## Phase 2: WebDispatcher -- TODO

Implement `crates/gpui/src/platform/web/dispatcher.rs`. Currently stubbed with unimplemented!(). This is the foundation for GPUI running in WASM -- without it, nothing can schedule work.

- `dispatch()`: use `wasm_bindgen_futures::spawn_local()` to run async closures
- `dispatch_after()`: use `web_sys::window().set_timeout_with_callback_and_timeout_and_arguments_0()` for delayed dispatch
- `dispatch_on_main_thread()`: in WASM everything is already on the main thread, so just invoke directly (or spawn_local for async)
- Frame scheduling: `web_sys::window().request_animation_frame()` for frame callbacks

Reference: the existing web platform stubs are in `crates/gpui/src/platform/web/` (web_platform.rs, web_window.rs, web_dispatcher.rs, web_display.rs). The Linux dispatcher at `crates/gpui/src/platform/linux/dispatcher.rs` is a good structural reference.

## Phase 3: Web Text System -- TODO

Wire cosmic-text as the text system for wasm32, replacing the current NoopTextSystem stub. cosmic-text is pure Rust, already used in the Linux platform at `crates/gpui/src/platform/linux/text_system.rs`. It compiles to wasm32 without modification.

This gives the browser-side GPUI accurate text measurement for Taffy layout, so elements are sized correctly before DOM projection.

## Phase 4: DOM Renderer -- TODO

Implement `WebWindow::draw()` as DOM mutations instead of Scene-to-canvas. This is where the DisplayTree becomes real browser elements:

- Scene quads -> positioned `<div>` elements with CSS backgrounds, borders, border-radius, box-shadow
- Text runs -> `<span>` elements with CSS color, font-family, font-size, text-decoration
- SVG nodes -> inline `<svg>` or `<img>` referencing SVG assets
- Image nodes -> `<img>` elements with object-fit CSS

CSS handles all visual styling (backgrounds, borders, shadows, rounded corners, opacity, overflow, z-index). The DOM tree mirrors the DisplayTree hierarchy.

Fallback: if pure DOM doesn't perform well for the editor buffer (thousands of styled spans), a hybrid approach is available -- DOM for chrome (panels, toolbars, file tree) and Canvas2D for the editor surface (a la VS Code). But try DOM-only first.

## Phase 5: WebPlatform Event Wiring -- TODO

Wire the browser event loop and input capture:

- `requestAnimationFrame` drives the GPUI frame loop
- DOM event listeners capture mouse (click, move, scroll), keyboard, and resize events
- Map DOM events to GPUI `PlatformInput` variants (MouseDown, MouseUp, MouseMove, ScrollWheel, KeyDown, KeyUp, ModifiersChanged)
- Viewport resize -> server gets ViewportChanged message (already implemented in the PoC)

## Phase 6: Build Pipeline -- TODO

Compile `crates/zed_web` to WASM and serve alongside `zed_server`:

- wasm-pack or trunk to compile the zed_web crate targeting wasm32-unknown-unknown
- Serve the WASM bundle, JS glue, and HTML shell from zed_server (or a static file server)
- WebSocket connection from WASM client to zed_server for frame streaming and action dispatch
- Integration test: full round-trip from server render -> WebSocket -> WASM client -> DOM -> user interaction -> action -> server re-render

## Architecture Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Overall approach | Element Tree Sync | No per-view rewrites, local scroll/hover/resize, all views work automatically |
| Browser rendering | DOM projection | Native text selection, Cmd+F, copy/paste, accessibility. Canvas makes all of these impossible without full reimplementation |
| Feature gate | Compile-time (`headless-web`) | Zero overhead when off, dead-code eliminated |
| Streaming gate | `#[cfg(all(feature = "headless-web", not(target_arch = "wasm32")))]` | Streaming infra only on the server, not in browser WASM |
| Style serialization | Custom DisplayStyle | Compact wire format, decoupled from GPUI internals |
| Interaction model | InteractionFlags + element ID forwarding | Closures can't serialize; flags tell browser what to capture, IDs map back to server closures |
| Delta encoding | Tree diff with targeted patches | Most frames change very little; reduces wire bandwidth |
| Frame delivery | tokio::sync::watch channel | Late-connecting clients get latest frame immediately, no missed-frame bugs |
| Server rendering | CPU-only (no GPU/display needed) | Layout and Scene construction are pure computation |
| Text capture | ShapedLine::paint() hooks | Single capture point for ALL painted text regardless of source (editor, terminal, labels, markdown) |
| Web text system | cosmic-text (pure Rust) | Already used on Linux, compiles to wasm32, accurate metrics for Taffy layout |

## Key Files

```
crates/gpui/src/display_tree.rs          -- DisplayTree, all node types, builder, diff, wire protocol (~1154 lines)
crates/gpui/src/elements/div.rs          -- Container capture hook in Interactivity::prepaint
crates/gpui/src/text_system/line.rs      -- ShapedLine/WrappedLine paint hooks for all text capture
crates/gpui/src/elements/svg.rs          -- SVG capture hook
crates/gpui/src/elements/img.rs          -- Image capture hook
crates/gpui/src/platform/streaming/      -- StreamingPlatform (CFRunLoopTimer) + StreamingWindow
crates/gpui/src/platform/web/            -- WebPlatform, WebWindow, WebDispatcher, WebDisplay (STUBS -- Phase 2+)
crates/zed_server/src/main.rs            -- Server binary: Zed init, frame bridge, WebSocket server (~280 lines)
crates/zed_server/viewer.html            -- Canvas2D PoC renderer with viewport reporting (~343 lines)
crates/zed_web/src/                      -- WASM client skeleton: lib.rs, connection.rs, remote_view.rs
```

## Known Gotchas

- `PlatformWindow` and `Platform` traits are `pub(crate)` -- implementations MUST live inside the gpui crate
- `Pixels.0` is `pub(crate)` -- external crates must use `f32::from(pixels)`
- `smol::Timer::after` is disallowed by clippy.toml -- use `gpui::BackgroundExecutor::timer()`
- `headless-web` feature needs font-kit on macOS and pulls in screen-capture via workspace->call dependency chain
- Session crate uses test-support feature to avoid SQLite DB -- `Session::test()`
- `drive_frame()` inside `cx.update()` causes RefCell double-borrow panic -- fixed with CFRunLoopTimer running outside cx
- Canvas elements (~25 call sites) are opaque paint closures -- content not captured, all decorative
- WrappedLine bounds: use `self.layout.unwrapped_layout.width` not `self.layout.layout.width`
- Image Resource doesn't impl Display, ObjectFit doesn't impl Debug -- use match-based string conversion
- GPUI has 94 existing `cfg(target_arch = "wasm32")` blocks and `wasm_shims.rs` (392 lines) for reference
- wgpu v28.0 supports `Backends::BROWSER_WEBGPU` but currently hardcoded to VULKAN|GL in wgpu_context.rs

## Previous Attempts (Dead Ends -- Do Not Retry)

1. **Scene streaming over WebSocket (commit 8cba93cb9a):** Serialized GPUI Scene primitives (quads, shadows, underlines, glyphs) and rendered them on a Canvas2D in the browser. Hit 73fps but was a dead end -- Scene is a flat GPU draw list, not a semantic tree. Rendering it in the browser required porting the WGSL shader pipeline, and there was no way to get text selection, Cmd+F, copy/paste, or accessibility. Rejected in favor of Element Tree Sync.

2. **Entity sync via gpui_web crate:** Attempted to synchronize GPUI entities/views to a separate WASM client that would re-render them. Compiled to WASM but required rewriting every UI view's rendering logic for the web. Fundamentally unscalable -- Zed has hundreds of views. Rejected and deleted.

3. **drive_frame() inside cx.update():** Calling `drive_frame()` (which triggers GPUI rendering) while already inside a `cx.update()` closure causes a RefCell double-borrow panic. The fix was CFRunLoopTimer running on the main thread *outside* the GPUI context, which requests frames at 120fps independently.

4. **Broadcast channel for frame delivery:** tokio::sync::broadcast lost frames sent before a client connected. Replaced with tokio::sync::watch, which holds the latest frame so late-connecting clients immediately get the current state.

5. **Canvas/WebGPU for browser rendering:** Seriously considered but rejected. Canvas makes text a bitmap -- you lose native text selection, Cmd+F, right-click context menus, screen reader accessibility, and mobile text interactions. All would need full reimplementation. DOM projection gives all of these for free.

6. **Text.rs prepaint hooks for text capture:** Initially added headless-web hooks in `crates/gpui/src/elements/text.rs` at the element level. These created duplicate nodes when combined with ShapedLine paint hooks, because the same text goes through both paths. Removed text.rs hooks entirely -- ShapedLine::paint() is the single capture point for ALL painted text (editor, terminal, labels, markdown, everything).

## Critical Implementation Details

These are internals the next agent needs to know that aren't obvious from reading the code:

- **Entry point:** `Application::streaming()` in gpui returns `(App, Receiver<DisplayTree>, Sender<(f32, f32, f32)>)` -- a frame receiver and a resize sender. The zed_server binary calls this instead of `Application::new()`.

- **Feature gates:** All DisplayTree capture code is behind `#[cfg(feature = "headless-web")]`. Streaming infrastructure (StreamingPlatform, frame channels) is behind `#[cfg(all(feature = "headless-web", not(target_arch = "wasm32")))]` so it compiles on the server but not in browser WASM. The `web` feature flag exists but is currently empty -- Phase 2+ will populate it.

- **zed_web crate** (`crates/zed_web/`): Already exists as a skeleton. `Cargo.toml` has `crate-type = ["cdylib"]`, depends on gpui with features `['web', 'headless-web']`, plus wasm-bindgen, web-sys, js-sys, postcard. Has `lib.rs` (wasm entry), `connection.rs` (WebSocket client), `remote_view.rs` (DisplayTree hydration into GPUI divs). This is where the WASM client lives.

- **Web platform stubs** (`crates/gpui/src/platform/web/`): WebPlatform, WebWindow, WebDispatcher, WebDisplay all exist as files with stub implementations (mostly `unimplemented!()`). Phase 2 starts by making WebDispatcher functional.

- **cosmic-text location:** The Linux platform's text system at `crates/gpui/src/platform/linux/text_system.rs` uses cosmic-text for text shaping and layout. This is pure Rust with no system dependencies, so it compiles to wasm32 as-is. Phase 3 wires the same approach for the web platform.

- **Resize flow end-to-end:** Browser sends ViewportChanged JSON over WebSocket -> tokio task receives it -> sends `(width, height, scale)` through a smol bounded(4) channel -> CFRunLoopTimer on main thread drains the channel -> calls `StreamingWindow::simulate_resize()` -> GPUI re-layouts at new size -> next frame captures the new layout. The bounded channel bridges the tokio runtime to the main thread safely.

- **Editor buffer paint path:** EditorElement::paint() -> paint_lines() -> ShapedLine::paint() -> paint_line() -> window.paint_glyph(). The ShapedLine hooks intercept at paint() before glyphs go to the Scene, capturing all text with syntax highlighting colors in the decoration runs.

- **Compilation command:** `cargo build -p zed_server` builds the server. gpui gets compiled with features: headless-web, font-kit (macOS), and screen-capture (pulled in transitively via workspace->call dependency chain). The headless-web feature adds postcard as a dependency.

- **UniformList / file tree / symbol outline:** These use normal GPUI element rendering internally, so their children go through the standard div prepaint path and get captured as Container nodes automatically. No special handling needed.

## Build Verification

After any code change, verify the build compiles and the frame pipeline works:

```bash
cargo build -p zed_server 2>&1 | tail -5    # should end with "Finished"
RUST_LOG=info ./target/debug/zed_server --bind 127.0.0.1:8080 &
sleep 3 && curl -s -o /dev/null -w "%{http_code}" --no-buffer \
  -H "Connection: Upgrade" -H "Upgrade: websocket" \
  -H "Sec-WebSocket-Version: 13" -H "Sec-WebSocket-Key: dGVzdA==" \
  http://127.0.0.1:8080  # should get 101 (switching protocols)
kill %1
```
