# Zed Web Renderer

Browser-side rendering client for Zed's web streaming architecture. This standalone TypeScript application receives GPUI scene primitives over a WebSocket and renders them using WebGPU, making a native Zed instance running on a Linux sandbox accessible through a web browser.

See [the design document](../docs/src/development/web-streaming-renderer.md) for full architectural context.

## How It Works

GPUI's rendering pipeline produces a `Scene` containing eight primitive types: quads, shadows, underlines, paths, monochrome sprites, subpixel sprites, polychrome sprites, and surfaces. Normally these get drawn by a Metal (macOS) or wgpu (Linux/Windows) renderer. This project implements the same rendering in the browser using WebGPU.

The WGSL shaders in `src/shaders.wgsl` are ported directly from `crates/gpui/src/platform/wgpu/shaders.wgsl`, with one notable change: subpixel sprite rendering falls back to grayscale alpha blending because browser WebGPU does not broadly support the `dual_source_blending` extension needed for per-channel subpixel alpha.

Sprite primitives (text glyphs, SVGs, images) reference tiles in a texture atlas rather than carrying pixel data inline. The `Atlas` class mirrors the server-side atlas textures as WebGPU textures, receiving tile uploads via `AtlasDelta` messages in the frame stream.

## Project Structure

```text
src/
  index.ts       Entry point. Initializes WebGPU, connects WebSocket, drives render loop.
  renderer.ts    WebGPU pipeline setup and per-frame draw. One pipeline per primitive type.
  shaders.wgsl   WGSL shaders ported from GPUI's wgpu backend.
  atlas.ts       Client-side atlas texture mirror. Manages WebGPU textures for sprite tiles.
  input.ts       Mouse, keyboard, and scroll capture. Serializes events for the server.
  protocol.ts    TypeScript types mirroring GPUI's scene primitives and wire format.
  wgsl.d.ts      TypeScript declaration for Vite's .wgsl file imports.
fixtures/
  sample.json    Hand-crafted scene fixture for testing without a running server.
```

## Development

```sh
npm install
npm run dev
```

This starts a Vite dev server on port 3100. Open `http://localhost:3100?fixture=sample` to render the sample fixture, which draws a mock editor layout using only quad and underline primitives (no atlas/sprites needed).

To connect to a live Zed instance streaming scenes, open `http://localhost:3100` (no query params). The client will attempt to connect to `ws://localhost:3101/scene` and reconnect automatically on disconnect. Override the WebSocket URL with `?ws=ws://host:port/path`.

## Requirements

A browser with WebGPU support: Chrome 113+, Edge 113+, or Firefox Nightly with `dom.webgpu.enabled`. Safari 18+ has experimental support behind a flag.

## Current Status

This is Phase 1 of the web streaming renderer. The browser-side rendering pipeline is implemented for all primitive types except two-pass path rendering (paths are relatively rare in typical editor UIs and will be added in a follow-up). The server-side scene serializer and WebSocket transport (Phase 2) do not exist yet -- the renderer currently loads static JSON fixture files.

What works:
- WebGPU initialization and pipeline creation for quads, shadows, underlines, monochrome sprites, subpixel sprites (grayscale fallback), and polychrome sprites
- Atlas texture management with tile upload support
- Batch iteration matching the native GPUI renderer's draw-order merge logic
- JSON fixture loading for development and visual validation
- Input capture (mouse, keyboard, scroll) with click counting
- WebSocket connection with auto-reconnect
- Stats overlay showing FPS

What's next:
- Phase 2: Server-side scene serializer with WebSocket transport
- Phase 3: Pre-rasterized atlas bundle for instant startup
- Phase 4: Delta compression and adaptive frame rate