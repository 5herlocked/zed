# Zed Web -- Element Tree Sync

**Branch:** `feature/web-view`
**Last updated:** 2026-02-18

## Architecture

Zed Web runs full native Zed on a server and streams a serialized element tree to a browser client over WebSocket. The browser runs real GPUI (compiled to WASM) with Taffy layout and canvas paint -- not DOM projection, not scene streaming.

```
Server (native)                          Browser (WASM + GPUI)
+----------------------------------+     +----------------------------------+
| Full Zed: views, LSP, git, fs   |     | GPUI (WASM) + Taffy layout       |
| GPUI render pipeline             |     | Canvas2D/WebGL paint             |
|   request_layout -> prepaint --->|---->| Hydrate DisplayTree -> Elements  |
|   capture DisplayTree            |     | Local: scroll, hover, resize     |
| StreamingWindow (no GPU)         |     | Forward: click, key, command     |
| postcard serialize on bg thread  |     |                                  |
+-------------|--------------------+     +-------------|--------------------+
              |  WebSocket (binary)                    |
              +----------------------------------------+
              WireFrame: Snapshot | Delta | Action
```

**Why Element Tree Sync over other approaches:**
- Scene streaming (Phase 1) hit 73fps but required porting all WGSL shaders to WebGPU and couldn't do local interactions
- Entity sync + WASM views required rewriting every UI view for the web (all tightly coupled to native entity types)
- DOM projection loses GPUI's layout system and styling
- Element Tree Sync captures at the right abstraction level: after style resolution but before rasterization, so the browser can run real GPUI layout/paint locally with zero-latency scroll and resize

## Implementation Status

### Done

| Commit | What |
|--------|------|
| `268da08bc8` | DisplayTree data structure + headless-web feature gate |
| `07f4a2627e` | Style conversion: GPUI Style -> DisplayStyle (layout, visual, text) |
| `bf5ddaf436` | Container (div) capture hooks in Interactivity::prepaint() |
| `91a6d21283` | Text (StyledText) + UniformList capture hooks |
| `768c435d19` | Wire protocol (WireFrame enum) + postcard serialization |
| `a9d6104699` | StreamingWindow: PlatformWindow impl, frame channel, input injection |

### Remaining

- [ ] **Window::draw() frame send** -- finalize DisplayTree from builder, ship through StreamingWindow's frame channel. Currently builder creates/finishes but doesn't transmit.
- [ ] **zed_web server binary** -- external crate that boots GPUI with StreamingPlatform, opens full Zed workspace on StreamingWindow, runs WebSocket server consuming the frame channel. Needs a StreamingPlatform (Platform trait impl) in addition to the StreamingWindow we already have.
- [ ] **Browser client (WASM + GPUI)** -- receives WireFrames, deserializes DisplayTree, hydrates into real GPUI elements, runs Taffy layout + paint locally. Builds on GPUI's existing `web` platform module (`cfg(target_arch = "wasm32")`). This is the heaviest piece -- needs a hydration layer that converts DisplayNode tree into GPUI element calls.
- [ ] **Additional capture hooks** -- Image, SVG, Anchored, Canvas, List (variable-height). Current hooks cover Container, Text, UniformList which handles the bulk of the UI.
- [ ] **Background serialization** -- move postcard serialization off the render thread. Current WireFrame::serialize() is synchronous; needs to happen on background executor to preserve frame budget.
- [ ] **Delta optimization** -- diff_display_trees() exists but isn't wired into the transport. First pass sends full snapshots; delta encoding reduces bandwidth by ~90% for typical keystrokes.
- [ ] **Action replay** -- server receives DisplayAction, maps node_id back to the original element's closure via a retained ID->closure map, replays the interaction into GPUI's event system.
- [ ] **Font metrics sync** -- browser needs server's font metrics for accurate Taffy layout. Either ship font data or use a metrics table synced at connection time.
- [ ] **Text run extraction** -- StyledText hook currently captures text content but not style runs (color, weight, underline per character). Needs access to the resolved TextRuns from TextLayout.

## Key Files

| File | Role |
|------|------|
| `crates/gpui/src/display_tree.rs` | All types: DisplayNode, DisplayNodeKind, DisplayStyle, WireFrame, builder, diff, actions (~1140 lines) |
| `crates/gpui/src/platform/streaming/window.rs` | StreamingWindow PlatformWindow impl + StreamingAtlas |
| `crates/gpui/src/platform/streaming/mod.rs` | Module declaration |
| `crates/gpui/src/platform.rs` | Conditional module inclusion + re-export |
| `crates/gpui/src/gpui.rs` | `#[cfg(feature = "headless-web")] pub mod display_tree` |
| `crates/gpui/src/window.rs` | DisplayTreeBuilder field on Window, create/finalize in draw() |
| `crates/gpui/src/elements/div.rs` | Container capture hooks in Interactivity::prepaint() |
| `crates/gpui/src/elements/text.rs` | Text leaf capture hook in StyledText::prepaint() |
| `crates/gpui/src/elements/uniform_list.rs` | UniformList kind override in prepaint closure |
| `crates/gpui/Cargo.toml` | `headless-web = ["postcard"]` feature definition |

## Wire Protocol

`WireFrame` is the top-level message type, serialized with postcard (compact binary, serde-based):

- **Snapshot** -- full DisplayTree. Sent on first connection and after desync.
- **Delta** -- incremental patches (replace node, update style/text/bounds/scroll, insert/remove child, update list range).
- **Action** -- browser-to-server interaction forwarding (click, key, scroll, hover, resize).
- **ActionAck** -- server confirms receipt, tells browser which frame will reflect the change.
- **SetViewport / ViewportChanged** -- bidirectional viewport size negotiation.
- **Ping / Pong** -- keepalive.

## Design Decisions

**Compile-time gating:** All capture code is `#[cfg(feature = "headless-web")]`. Zero overhead when disabled. No runtime checks on the hot path.

**Capture piggybacking:** Hooks live inside existing prepaint() calls. No fourth traversal of the element tree. The builder's push/pop mirrors the natural recursion.

**set_current_kind pattern:** UniformList wraps Interactivity which pushes a generic Container node. Rather than duplicating push logic, UniformList calls `set_current_kind()` inside the closure to specialize the node after Interactivity pushes it.

**Custom DisplayStyle:** Compact wire format decoupled from GPUI's internal StyleRefinement. Resolved values (not Option-wrapped refinement deltas) because the server has already cascaded.

**InteractionFlags bitfield:** 11 flags extracted from Interactivity's listener fields. Browser only captures and forwards event types the element actually handles.

**StreamingWindow lives in gpui:** PlatformWindow is `pub(crate)`, so the implementation must be inside the gpui crate. The external zed_web binary uses it through a public constructor re-exported from `gpui::StreamingWindow`.

**Browser is WASM + GPUI:** The browser client compiles GPUI to WASM and runs real Taffy layout + element painting locally. DisplayTree nodes hydrate into actual GPUI elements. Local interactions (scroll, resize, hover) are zero-latency. Content-changing interactions round-trip to the server as DisplayActions. This reuses GPUI's existing `web` platform module.

## Previous Approaches (Rejected)

| Approach | Why rejected |
|----------|--------------|
| Phase 1: Scene streaming | Hit 73fps but required WGSL->WebGPU shader porting, no local interactions, full atlas bandwidth on every frame |
| Phase 2: Entity sync + WASM views | Required rewriting every UI view; all views tightly coupled to native entity types (smol, fs, git) |
| DOM projection | Loses GPUI's layout system; would need to replicate Taffy in CSS which is lossy |
| HeadlessProject wrapping | HeadlessProject is a different struct from Project; can't be directly wrapped into Workspace |
