# Zed Web — Element Tree Sync Architecture

## Problem Statement

Zed Web puts the full Zed editor experience in a browser without rewriting views or compiling the entire native crate graph to WASM. The architecture splits responsibilities: the server runs full native Zed (real views, real entities, real LSP/git/filesystem) and the browser handles layout, paint, and local interactions.

The key insight is the **Element Tree Sync** pattern: during GPUI's render pipeline, the server produces a serializable `DisplayTree` — a parallel representation of the element tree capturing styles, text content, tree structure, and interaction capabilities. This tree is shipped to the browser over WebSocket, where the browser's GPUI instance hydrates it into real elements, runs Taffy layout locally, and paints locally. Scroll, resize, and hover are zero-latency. Content changes (open file, search, save) round-trip to the server, which re-renders and sends a delta update.

### Why Element Tree Sync?

We evaluated four approaches:

1. **Entity sync + WASM views** (Phase 2): Browser has lightweight entity stores + custom views compiled to WASM. Rejected because every UI view would need a web-specific rewrite — FileTreeView is 180 lines but the real editor is thousands, plus tabs, panels, search, etc.

2. **Scene streaming** (Phase 1 refined): Server renders full Zed, streams raw Scene data (positioned quads, glyphs) to browser. Works (hit 73fps), but every interaction round-trips — typing, scrolling, hovering all need server response. Browser needs a WebGPU/Canvas2D Scene renderer.

3. **DOM projection**: Server maps GPUI element tree to HTML/CSS DOM. Gets native browser features for free (text rendering, scrolling, accessibility) but loses GPUI's pixel-perfect rendering.

4. **Element Tree Sync** (chosen): Server produces a serializable element tree during its normal render passes. Browser hydrates it into real GPUI elements and handles layout/paint/local-input. Best balance of fidelity, latency, and implementation cost. No per-view rewrites. All present and future views work automatically because capture happens at the GPUI framework level.

## Architecture

```
┌────────────────────────────────────┐        ┌─────────────────────────────────┐
│         Server (native Zed)        │        │       Browser (GPUI WASM)       │
│                                    │        │                                 │
│  ┌──────────┐  ┌───────────────┐   │  WS    │  ┌──────────┐  ┌────────────┐  │
│  │ Real     │──│ GPUI render   │   │  proto  │  │ Hydrate  │──│ GPUI       │  │
│  │ Entities │  │ pipeline      │───│───────>│──│ Display  │  │ Taffy      │  │
│  │ (native) │  │               │   │  tree   │  │ Tree     │  │ Layout +   │  │
│  │          │  │ + DisplayTree │   │        │  │          │  │ Paint      │  │
│  │ Project, │  │   capture     │   │<───────│──│ Forward  │  │            │  │
│  │ Buffers, │  │   (headless-  │   │ action  │  │ Actions  │  │ Local:     │  │
│  │ LSP, Git │  │    web feat)  │   │ events  │  │          │  │ scroll,    │  │
│  └──────────┘  └───────────────┘   │        │  └──────────┘  │ hover,     │  │
│                                    │        │                │ resize     │  │
└────────────────────────────────────┘        └─────────────────┴────────────┘
```

### How It Works

1. **Server render pass**: GPUI calls `render()` on all views (Workspace, ProjectPanel, Editor, etc.) as normal. The real element tree goes through the standard request_layout → prepaint → paint pipeline.

2. **DisplayTree capture**: Gated behind `#[cfg(feature = "headless-web")]`, capture hooks in the render pipeline build a `DisplayTree` alongside the real element tree. This captures element types, resolved styles (already serializable), text content + style runs, interaction flags, and tree structure. No extra traversal — hooks piggyback on the three existing walks.

3. **Serialization (off-thread)**: After paint completes, the finished DisplayTree is handed to a background thread that diffs it against the previous frame, serializes the delta, and sends it over WebSocket. Render thread budget is unaffected.

4. **Browser hydration**: The browser receives the DisplayTree, constructs real GPUI elements from it (div with these styles, text with these runs, etc.), and runs Taffy layout + paint locally. Scroll, resize, hover are handled browser-side with zero latency.

5. **Action forwarding**: Interactive elements carry `InteractionFlags` indicating what events they handle. When the user clicks/types/scrolls, the browser sends `(node_id, element_id, event_type, event_data)` to the server. The server maps the element ID back to the real closure registered during render, dispatches the event, re-renders, and sends the updated DisplayTree delta.

### Performance

- **When headless-web is off**: Zero impact. `#[cfg(feature = "headless-web")]` means capture code is dead-code-eliminated. Binary is identical to today's desktop Zed.
- **When active**: Capture piggybacks on existing render walks. Per-element cost is a small struct copy (style + text content). SharedString clones are refcount bumps. ~0.1-0.5ms overhead on the render thread for a typical frame. Serialization and diff run on background thread.
- **Delta encoding**: Most frames only change a small part of the tree (cursor blink, hover state, scroll offset). Diffs are small.
- **Target**: 120fps on the render thread.

## Scope

### In Scope

- `headless-web` compile-time feature on GPUI
- `DisplayTree` data structure: types, builder, delta encoding
- Capture hooks in GPUI's render pipeline (request_layout, prepaint, paint)
- DisplayTree serialization wire format (serde + binary encoding)
- Action forwarding protocol (browser → server → dispatch → re-render → delta)
- `zed-web` server binary: full Zed with StreamingWindow, DisplayTree capture, WebSocket transport
- `zed_web` browser crate: WASM, DisplayTree hydration, GPUI layout/paint, action forwarding
- Functional GPUI web platform: WebDispatcher (rAF), WebTextSystem (Canvas2D), WebWindow (canvas)

### Out of Scope (deferred)

- Canvas element server-side rasterization (fallback: skip in first pass)
- Drag-and-drop across browser/OS boundary
- WebGPU-accelerated rendering on browser side (Canvas2D first)
- Offline/cached mode
- Multi-window support in browser
- Authentication/encryption on the WebSocket connection

## Constraints

- **Minimal disruption to GPUI**: All additions are feature-gated behind `headless-web`. Default builds are unaffected. No modifications to existing render behavior when the feature is off.
- **No per-view changes**: DisplayTree capture happens in the framework's render pipeline, not in individual view implementations. All views work automatically.
- **Reuse existing protocol infrastructure**: WebSocket transport from Phase 1, protobuf Envelope framing from rpc crate.
- **Server needs no GPU/display**: The entire render pipeline up to Scene is CPU computation. DisplayTree capture intercepts before draw(). A bare Linux box or container works.

## Key Files

### New
- `crates/gpui/src/display_tree.rs` — DisplayTree types, builder, diff, action forwarding types
- `crates/zed_web/` — Browser WASM crate (hydration, action forwarding, GPUI web platform)
- `crates/zed-web/` — Server binary (full Zed + StreamingWindow + WebSocket)

### Modified
- `crates/gpui/Cargo.toml` — `headless-web` feature flag
- `crates/gpui/src/gpui.rs` — `display_tree` module (cfg-gated)
- `crates/gpui/src/elements/div.rs` — Capture hooks in Element impl (cfg-gated)
- `crates/gpui/src/elements/text.rs` — Capture hooks for text elements
- `crates/gpui/src/elements/uniform_list.rs` — Capture hooks for virtualized lists
- `crates/gpui/src/window.rs` — DisplayTreeBuilder held on Window, handed off after paint
- `crates/gpui/src/platform/web/` — Functional web platform implementations

### Unchanged
- All existing views (they render normally; capture is transparent)
- All existing platform backends (mac, linux, windows)
- All proto definitions (reused as-is for transport)
- All existing tests

## Acceptance Criteria

1. **DisplayTree compiles**: `cargo check -p gpui --no-default-features --features headless-web` and `--target wasm32-unknown-unknown --features "web,headless-web"` both succeed
2. **Capture produces valid tree**: A test creates a GPUI window with known elements, renders, and verifies the DisplayTree matches expected structure
3. **Delta encoding works**: Two trees are diffed, patches applied, and result matches the target tree
4. **Browser renders file tree**: Connect browser to zed-web server, see the file tree rendered with correct layout and styling
5. **Interactions work**: Click a file in the browser → server receives action → buffer opens → DisplayTree updates with editor content
6. **Zero impact when off**: Default `cargo build` produces identical binary. No performance regression in benchmarks.
7. **120fps on server**: DisplayTree capture adds <1ms to the render thread per frame
