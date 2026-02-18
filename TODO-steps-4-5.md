# TODO: Zed Web — Element Tree Sync Implementation

## Status

Architecture: **Element Tree Sync** (decided after evaluating entity sync, scene streaming, and DOM projection).
Branch: `feature/web-view`

## Completed

- [x] DisplayTree data structure designed and implemented (`crates/gpui/src/display_tree.rs`)
  - DisplayNode, DisplayNodeKind (Container, Text, Image, Svg, UniformList, List, Anchored, Canvas)
  - DisplayStyle (layout + visual + text), DisplayColor, DisplayBoxShadow, DisplayLength
  - InteractionFlags (bitfield: clickable, hoverable, scrollable, focusable, etc.)
  - DisplayTreeBuilder (stack-based builder for incremental construction during render walks)
  - DisplayTreeDelta + DisplayTreePatch (diff/patch for delta encoding between frames)
  - DisplayAction + DisplayActionKind (action forwarding from browser to server)
  - diff_display_trees() diffing algorithm
- [x] `headless-web` feature flag added to GPUI Cargo.toml
- [x] Module wired into GPUI (`#[cfg(feature = "headless-web")] pub mod display_tree`)
- [x] Compiles on native: `cargo check -p gpui --no-default-features --features headless-web`
- [x] Compiles on WASM: `cargo check --target wasm32-unknown-unknown -p gpui --no-default-features --features "web,headless-web"`

## In Progress

### Capture Hooks in GPUI Render Pipeline
- [ ] Add `DisplayTreeBuilder` to Window struct (cfg-gated)
- [ ] Hook into `Drawable::request_layout()` — capture element type, styles, children
- [ ] Hook into `Drawable::prepaint()` — capture bounds, hitbox IDs, interaction flags
- [ ] Hook into `Window::draw()` — hand finished tree to background thread
- [ ] Style conversion: GPUI StyleRefinement → DisplayStyle
- [ ] Interactivity → InteractionFlags extraction

### Proto Schema for Wire Transport
- [ ] Define DisplayTree message in scene.proto or new display_tree.proto
- [ ] Define DisplayTreeDelta message
- [ ] Define DisplayAction message (browser → server)
- [ ] Binary serialization (bincode or postcard over serde)

## Planned

### Delta Encoding (background thread)
- [ ] Thread-safe handoff from render thread to serialization thread
- [ ] Previous-frame caching for diff
- [ ] DisplayTreeDelta patch application (for browser-side)

### Server Binary (zed-web)
- [ ] StreamingWindow: virtual PlatformWindow that captures DisplayTree instead of GPU rendering
- [ ] Full Zed Workspace creation with real views over StreamingWindow
- [ ] WebSocket server integration (reuse existing from remote_server)
- [ ] Action dispatch: browser events → GPUI input dispatch → re-render → delta
- [ ] Text atlas: server rasterizes glyphs to CPU bitmaps, sends as atlas deltas

### Browser Crate (zed_web)
- [ ] Rename from gpui_web to zed_web (update Cargo.toml, root workspace)
- [ ] DisplayTree hydration: deserialize → construct GPUI elements → Taffy layout → paint
- [ ] Action forwarding: DOM events → DisplayAction → WebSocket → server
- [ ] WebDispatcher: rAF scheduling, setTimeout for delays
- [ ] WebTextSystem: Canvas2D measureText for text metrics
- [ ] WebWindow: store frame callbacks, Canvas2D or WebGPU Scene renderer
- [ ] DOM input capture: mouse, keyboard, scroll → GPUI InputEvent mapping

### GPUI Web Platform (make browser-side GPUI functional)
- [ ] WebDispatcher: requestAnimationFrame for frame scheduling
- [ ] WebTextSystem: Canvas2D text measurement for Taffy layout
- [ ] WebWindow: frame callbacks, input dispatch, canvas-based rendering
- [ ] WebDisplay: viewport info from browser window

## Architecture Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Overall approach | Element Tree Sync | No per-view rewrites, local scroll/hover/resize, all views work automatically |
| Feature gate | Compile-time (`headless-web`) | Zero overhead when off, dead-code eliminated |
| Style serialization | Custom DisplayStyle (not raw StyleRefinement) | Compact wire format, decoupled from GPUI internals |
| Interaction handling | InteractionFlags + element ID forwarding | Closures can't serialize; flags tell browser what to capture, IDs map back to server closures |
| Delta encoding | Tree diff with targeted patches | Most frames change very little; reduces wire bandwidth |
| Server rendering | CPU-only (no GPU/display needed) | Layout and Scene construction are pure computation |
| Browser layout | GPUI WASM + Taffy (local) | Zero-latency scroll/resize, pixel-perfect match with server layout |

## Dependency Graph

```
crates/zed-web (server binary)
├── gpui [headless-web]           — DisplayTree capture
├── workspace, editor, etc.       — Real Zed views (native)
├── remote_server                 — WebSocket transport (reuse)
└── StreamingWindow               — Virtual platform window

crates/zed_web (browser WASM)
├── gpui [web, headless-web]      — GPUI WASM + DisplayTree types
├── proto                         — Wire format
├── web-sys, wasm-bindgen         — Browser APIs
└── serde, bincode/postcard       — Serialization
```
