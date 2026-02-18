# Web Streaming Renderer (Phase 1 -- Proof of Concept)

> **Status**: Phase 1 is complete. It validated the transport layer, protobuf encoding,
> WebGPU rendering, and atlas mirroring. The approach has fundamental limitations
> around latency, input handling, and UI fidelity that make it unsuitable as a
> production architecture. See [Web View State Architecture](web-view-state-architecture.md)
> for the Phase 2 design that replaces scene streaming with view state synchronization.

This document describes the Phase 1 architecture for making Zed accessible through a web browser. Zed runs natively on an isolated Linux sandbox, and the browser acts as a thin rendering client that receives scene data over a WebSocket and draws it using WebGPU.

The approach streams GPUI's scene primitives rather than pixels. A typical editor frame produces 20-50KB of primitive data (coordinates, colors, atlas tile references), compared to megabytes for pixel streaming. With protobuf encoding, the pipeline achieves 73fps sustained throughput.

## How GPUI Rendering Works Today

GPUI's rendering pipeline produces a `Scene` -- a flat collection of eight primitive types sorted by draw order. During each frame, the element tree paints into the scene by calling methods like `window.paint_quad()`, `window.paint_glyph()`, and `window.paint_svg()`. Each method constructs a primitive struct and inserts it into the scene. Once painting completes, `Window::present()` hands the scene to a platform renderer (Metal on macOS, wgpu on Linux/Windows) which iterates over batched primitives and issues GPU draw calls.

The eight primitive types are:

- **Quad**: Rectangles with background (solid, gradient, or pattern), border, and corner radii.
- **Shadow**: Box shadows with blur radius, corner radii, and color.
- **Underline**: Text underlines and strikethroughs with thickness and optional wavy rendering.
- **Path**: Arbitrary vector shapes, triangulated into vertices with xy/st positions.
- **MonochromeSprite**: Grayscale text glyphs and SVG icons, tinted with a color.
- **SubpixelSprite**: Subpixel-rendered text glyphs for LCD displays.
- **PolychromeSprite**: Emoji and raster images with optional corner radii and grayscale.
- **PaintSurface**: macOS-only video surfaces (not relevant for this work).

Every primitive carries a `DrawOrder` (u32) for z-sorting and a `ContentMask` (clipping rectangle). Sprites reference tiles in a texture atlas rather than carrying pixel data inline.

### The Atlas

GPUI packs rasterized glyphs, SVGs, and images into texture atlases using the `etagere` bin-packing allocator. Atlas textures are 1024x1024 by default and come in three kinds: Monochrome (R8, for text and SVGs), Subpixel (BGRA8, for subpixel-rendered text), and Polychrome (BGRA8, for images and emoji). Each sprite primitive contains an `AtlasTile` with a `texture_id` and `bounds` that point into one of these atlas textures.

Atlas content is cached aggressively. When `paint_glyph` is called, the atlas does a `get_or_insert_with` lookup -- if the glyph already exists, it returns the cached tile coordinates without any rasterization. New tiles only appear when genuinely new content enters the viewport (an unseen glyph, a new image, etc.).

### The wgpu Shaders

The Linux/Windows renderer uses wgpu with WGSL shaders (`crates/gpui/src/platform/wgpu/shaders.wgsl`). WGSL is also the shading language for WebGPU, which means the existing shaders can be adapted for the browser-side renderer with minimal modification. The shaders handle quad rendering with SDF-based corner radii and borders, shadow blur, underline waves, path rasterization via an intermediate texture, and sprite sampling from atlas textures with gamma/contrast correction.

## Architecture

The system has three components.

### 1. Server-Side Scene Serializer

A new platform backend (or a layer on top of the existing wgpu Linux backend) that intercepts the scene at `Window::present()` and serializes it for transmission. This lives in GPUI's platform layer.

The serializer's responsibilities:

- Serialize scene primitives into a compact binary format after each frame.
- Track atlas mutations and emit atlas tile deltas when new content is rasterized.
- Maintain a WebSocket server that sends frame messages to connected browser clients.
- Receive input events (mouse, keyboard, scroll) from the browser and inject them into GPUI's event dispatch.

The interception point is `platform_window.draw(&scene)` in `Window::present()`. The streaming backend would serialize the scene before (or instead of) passing it to the GPU renderer. For development, it can run alongside the normal renderer so the sandbox still has a visible window for debugging.

### 2. Browser-Side Renderer (build first)

A standalone web application that connects to the scene stream, maintains a local atlas mirror, and renders primitives using WebGPU. This is the component to build first because it can be developed and tested independently using recorded scene data.

The browser renderer's responsibilities:

- Connect to the WebSocket and deserialize incoming frame messages.
- Maintain atlas textures (WebGPU textures or canvas bitmaps) mirrored from the server.
- Render the eight primitive types using WebGPU pipelines ported from the WGSL shaders.
- Capture mouse, keyboard, and scroll events and send them back over the WebSocket.
- Handle resize events and report viewport size changes to the server.

### 3. Pre-Rasterized Atlas Bundle

A build-time step that pre-rasterizes the predictable atlas content and embeds it in the browser client. For a known Zed configuration, the atlas content is highly predictable: the default editor font at common sizes, the full SVG icon set from `assets/`, and standard UI glyphs. Pre-rasterizing eliminates the cold-start problem where the first frame would otherwise require streaming hundreds of glyph tiles.

The bundle would contain:

- Pre-rasterized glyph atlases for the default font (e.g., Zed Plex Mono) at common sizes and scale factors.
- Pre-rasterized SVG icon atlases from the `assets/icons/` directory.
- A manifest mapping `AtlasKey` values to tile positions so the browser can resolve sprite references without waiting for the server.

At runtime, the server's atlas serializer compares its atlas state against the pre-rasterized manifest and only streams tiles that aren't already in the bundle.

## Wire Protocol

The WebSocket carries binary messages in both directions.

### Server to Browser

Each frame message contains:

```text
FrameMessage {
    frame_id: u64,
    viewport_size: [f32; 2],
    scale_factor: f32,
    atlas_deltas: Vec<AtlasDelta>,
    primitives: Vec<PrimitiveMessage>,
}
```

`AtlasDelta` carries new tile data when the atlas changes:

```text
AtlasDelta {
    texture_id: AtlasTextureId,   // which atlas texture
    bounds: Bounds<DevicePixels>, // region within the texture
    format: AtlasTextureKind,     // Monochrome, Subpixel, or Polychrome
    bytes: Vec<u8>,               // raw pixel data for this tile
}
```

`PrimitiveMessage` is a tagged union of the eight primitive types. Each variant carries the same fields as its Rust counterpart, with `AtlasTile` references left as-is (the browser resolves them against its local atlas mirror).

For delta compression between frames, primitives can be sent as a diff against the previous frame. Since most primitives between keystrokes only change in a few fields (e.g., cursor position shifts, one line of text reflows), this reduces payload size substantially. The simplest delta scheme is: send the full primitive list but omit unchanged primitives by referencing their index in the previous frame. A more aggressive scheme would diff at the field level, but that's an optimization for later.

### Browser to Server

Input messages carry user events:

```text
InputMessage {
    kind: InputKind,
}

enum InputKind {
    MouseMove { position: [f32; 2], modifiers: Modifiers },
    MouseDown { button: u8, position: [f32; 2], click_count: u32, modifiers: Modifiers },
    MouseUp { button: u8, position: [f32; 2], modifiers: Modifiers },
    Scroll { position: [f32; 2], delta: [f32; 2], modifiers: Modifiers },
    KeyDown { key: String, modifiers: Modifiers },
    KeyUp { key: String, modifiers: Modifiers },
    Resize { size: [f32; 2], scale_factor: f32 },
}
```

## Build Order

### Phase 1: Browser-Side Renderer (standalone, testable independently)

Build the browser renderer first so it can be validated against static scene data before the server serializer exists.

**Step 1: Record scene fixtures.** Add a debug hook in `Window::present()` that serializes the scene to a JSON file. This gives real test data without needing the full streaming pipeline. A few frames from different UI states (empty editor, file with syntax highlighting, file tree open, command palette open) provide good coverage.

**Step 2: Build the WebGPU renderer.** Start with quads -- they're the most common primitive and the simplest to render. Then shadows, underlines, paths, and finally sprites. The WGSL shaders from `crates/gpui/src/platform/wgpu/shaders.wgsl` can be adapted directly since WebGPU uses WGSL natively.

**Step 3: Validate rendering.** Load the recorded scene fixtures, deserialize them, and render. Compare the output against screenshots from Zed's existing `render_to_image` test infrastructure (available on macOS via `VisualTestAppContext::capture_screenshot`).

**Step 4: Input and WebSocket.** Add input event capture and a mock WebSocket connection so the full client loop can be tested locally.

The browser renderer is a standalone web app with no Rust/wasm dependency. It's pure TypeScript talking to WebGPU.

### Phase 2: Server-Side Scene Serializer

Once the browser renderer can draw recorded scenes correctly:

**Step 1: Serialization format.** Implement the binary serialization format for `Scene` and its primitives. Use a compact binary encoding (MessagePack, FlatBuffers, or a custom format) rather than JSON for production. JSON is fine for Phase 1 debugging.

**Step 2: WebSocket server.** Add a WebSocket server to the GPUI platform layer. This can be a new `platform/web_streaming` module or a composable layer that wraps an existing platform backend.

**Step 3: Scene broadcast.** Hook into `Window::present()` to serialize and broadcast the scene after each frame.

**Step 4: Atlas delta tracking.** Intercept `PlatformAtlas::get_or_insert_with` to detect new tile allocations and queue them for the next frame message.

**Step 5: Input injection.** Receive `InputMessage` from the WebSocket and dispatch them through GPUI's `Window::dispatch_event`.

### Phase 3: Pre-Rasterized Atlas Bundle

Once streaming works end-to-end:

**Step 1: Atlas capture.** Write a build script that launches Zed headlessly, renders a representative set of content (empty editor, file tree, settings panel, etc.), and captures the atlas state.

**Step 2: Export and manifest.** Export the atlas textures as PNGs and generate a manifest mapping atlas keys to tile positions.

**Step 3: Bundle into browser client.** Include the atlas PNGs and manifest in the browser client's static assets.

**Step 4: Delta optimization.** Modify the server's atlas delta logic to skip tiles that exist in the pre-rasterized manifest.

### Phase 4: Optimization

- Delta compression for primitives between frames.
- Batched atlas updates (coalesce multiple small tiles into a single texture upload message).
- Adaptive frame rate (skip scene serialization if nothing changed, or throttle to match network conditions).
- Input prediction on the browser side (optimistic cursor movement, scroll) to mask latency.

## File Structure

```text
crates/gpui/src/platform/web_streaming/       # Server-side scene serializer
    web_streaming.rs                           # Module root
    scene_serializer.rs                        # Scene -> binary format
    atlas_tracker.rs                           # Tracks atlas deltas
    websocket_server.rs                        # WebSocket transport
    input_injector.rs                          # Browser input -> GPUI events

web_renderer/                                  # Browser-side renderer (standalone)
    src/
        index.ts                               # Entry point, WebSocket client
        renderer.ts                            # WebGPU rendering pipeline
        shaders.wgsl                           # Adapted from gpui's shaders
        atlas.ts                               # Client-side atlas mirror
        input.ts                               # Input capture and serialization
        protocol.ts                            # Wire format deserialization
    public/
        index.html
    package.json
    tsconfig.json
```

## Phase 1 Results

**What worked:**
- Protobuf-encoded scene frames at 73fps over WebSocket (up from 4fps with JSON).
- GPUI WGSL shaders ported to browser WebGPU with minor modifications.
- `MirroringAtlas` intercepting Metal atlas tile rasterization and mirroring pixel data to browser WebGPU textures.
- Pre-converted theme colors (`ThemeHints`) solving the color space mismatch.
- `scene_observer` hook on `Window` enabling any platform to stream scenes via `ZED_WEB_STREAMING=1`.
- Text glyph rendering with correct syntax highlighting colors and positions.
- Background colors, borders, rounded corners, gradients, shadows, and underlines rendering correctly.

**What broke down:**
- Missing glyphs on file switches due to atlas cache key mismatches.
- No input forwarding from browser to server.
- UI chrome (SVG icons, toolbar buttons, scrollbars, breadcrumbs) either missing or rendered as anonymous primitives.
- Every interaction requires a full server round-trip, making hover states and scrolling feel sluggish.
- Semi-transparent popup backgrounds (vibrancy/blur) cannot be replicated in the browser.

**Conclusion:** Scene streaming works as a proof of concept but is fundamentally a "dumb terminal" approach. Phase 2 replaces it with view state synchronization. See [web-view-state-architecture.md](web-view-state-architecture.md).

## Key References in the Codebase

| What | Where |
|---|---|
| Phase 2 design doc | `docs/src/development/web-view-state-architecture.md` |
| Scene struct and all primitives | `crates/gpui/src/scene.rs` |
| Platform traits (Platform, PlatformWindow) | `crates/gpui/src/platform.rs` |
| Window draw cycle | `crates/gpui/src/window.rs` (`draw`, `present`) |
| Web streaming Rust module | `crates/gpui/src/platform/web_streaming/` |
| Protobuf schema | `crates/gpui/proto/scene.proto` |
| Browser renderer | `web_renderer/` |
| wgpu renderer and batch iteration | `crates/gpui/src/platform/wgpu/wgpu_renderer.rs` |
| WGSL shaders | `crates/gpui/src/platform/wgpu/shaders.wgsl` |
| wgpu atlas implementation | `crates/gpui/src/platform/wgpu/wgpu_atlas.rs` |
| Atlas traits and tile types | `crates/gpui/src/platform.rs` (`PlatformAtlas`, `AtlasTile`) |
| Headless Linux client | `crates/gpui/src/platform/linux/headless/client.rs` |
| Remote server (existing headless project) | `crates/remote_server/src/` |