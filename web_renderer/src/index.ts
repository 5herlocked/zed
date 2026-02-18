// Entry point for the Zed web streaming renderer.
//
// This module initializes WebGPU, creates the renderer and input handler, and
// connects to the server's WebSocket for scene streaming. It also supports
// loading static JSON fixture files for development and testing without a
// running server.
//
// Usage:
//   - With a live server: the page connects to ws://host:port/scene and
//     receives FrameMessage objects as binary WebSocket messages.
//   - With fixtures: add ?fixture=name to the URL and place a JSON file
//     at /fixtures/name.json. The renderer will load and draw it as a
//     single frame, useful for validating rendering without the server.

import { Atlas } from "./atlas";
import { Renderer } from "./renderer";
import { InputHandler } from "./input";
import { FrameMessage, InputMessage, deserializeFrame } from "./protocol";
import { decodeProtoFrame } from "./proto_decoder";

// ---------------------------------------------------------------------------
// Configuration
// ---------------------------------------------------------------------------

// WebSocket endpoint for scene streaming. Override with ?ws=url.
const DEFAULT_WS_URL = `ws://${window.location.hostname}:3101`;

// How long to wait before reconnecting after a WebSocket disconnect (ms).
const RECONNECT_DELAY = 2000;

// ---------------------------------------------------------------------------
// State
// ---------------------------------------------------------------------------

let renderer: Renderer | null = null;
let inputHandler: InputHandler | null = null;
let websocket: WebSocket | null = null;
let reconnectTimer: ReturnType<typeof setTimeout> | null = null;
let destroyed = false;

// Stats overlay
let frameCount = 0;
let framesReceived = 0;
let lastStatsTime = performance.now();
let statsElement: HTMLElement | null = null;
let lastFrame: FrameMessage | null = null;
let fixtureAnimationFrameId: number | null = null;
let wsAnimationFrameId: number | null = null;
let pendingFrame: FrameMessage | null = null;

// ---------------------------------------------------------------------------
// Initialization
// ---------------------------------------------------------------------------

async function main(): Promise<void> {
  const canvas = document.getElementById("canvas") as HTMLCanvasElement | null;
  const errorDiv = document.getElementById("error") as HTMLElement | null;

  if (!canvas) {
    console.error("Canvas element not found");
    return;
  }

  // Check WebGPU availability
  if (!navigator.gpu) {
    if (errorDiv) errorDiv.style.display = "flex";
    console.error("WebGPU is not supported in this browser");
    return;
  }

  const adapter = await navigator.gpu.requestAdapter({
    powerPreference: "high-performance",
  });

  if (!adapter) {
    if (errorDiv) errorDiv.style.display = "flex";
    console.error("Failed to get WebGPU adapter");
    return;
  }

  // Request device with storage buffer support in vertex shaders if available
  const requiredFeatures: GPUFeatureName[] = [];

  const device = await adapter.requestDevice({
    requiredFeatures,
    requiredLimits: {
      maxStorageBufferBindingSize: adapter.limits.maxStorageBufferBindingSize,
      maxBufferSize: adapter.limits.maxBufferSize,
    },
  });

  device.lost.then((info) => {
    console.error(`WebGPU device lost: ${info.message}`);
    if (info.reason !== "destroyed") {
      // Attempt to reinitialize
      main();
    }
  });

  // Create atlas and renderer
  const atlas = new Atlas(device);

  renderer = new Renderer({
    canvas,
    device,
    atlas,
  });

  // Initial sizing
  const scaleFactor = window.devicePixelRatio || 1;
  const displayWidth = canvas.clientWidth;
  const displayHeight = canvas.clientHeight;
  renderer.resize(displayWidth, displayHeight, scaleFactor);

  // Stats overlay
  statsElement = createStatsOverlay();

  // Input handler
  inputHandler = new InputHandler(canvas, onInput, scaleFactor);

  // Handle window resize
  const resizeObserver = new ResizeObserver((entries) => {
    for (const entry of entries) {
      if (entry.target === canvas && renderer) {
        const sf = window.devicePixelRatio || 1;
        const width = entry.contentRect.width;
        const height = entry.contentRect.height;
        renderer.resize(width, height, sf);
        inputHandler?.setScaleFactor(sf);
        inputHandler?.sendResize(width, height, sf);
      }
    }
  });
  resizeObserver.observe(canvas);

  // Check URL params for mode
  const params = new URLSearchParams(window.location.search);
  const fixtureName = params.get("fixture");
  const wsUrl = params.get("ws") || DEFAULT_WS_URL;

  if (fixtureName) {
    await loadFixture(fixtureName);
  } else {
    connectWebSocket(wsUrl);
  }
}

// ---------------------------------------------------------------------------
// Fixture loading (Phase 1 development mode)
// ---------------------------------------------------------------------------

async function loadFixture(name: string): Promise<void> {
  console.log(`Loading fixture: ${name}`);

  try {
    const response = await fetch(`/fixtures/${name}.json`);
    if (!response.ok) {
      throw new Error(`HTTP ${response.status}: ${response.statusText}`);
    }

    const json = await response.json();
    const frame = deserializeFrame(json);
    console.log(
      `Fixture loaded: ${frame.scene.quads.length} quads, ` +
        `${frame.scene.shadows.length} shadows, ` +
        `${frame.scene.underlines.length} underlines, ` +
        `${frame.scene.paths.length} paths, ` +
        `${frame.scene.monochrome_sprites.length} mono sprites, ` +
        `${frame.scene.subpixel_sprites.length} subpixel sprites, ` +
        `${frame.scene.polychrome_sprites.length} poly sprites`,
    );

    renderer?.drawFrame(frame);
    lastFrame = frame;
    frameCount++;
    updateStats(true);

    // Start continuous render loop to measure sustained fps for fixtures
    startFixtureRenderLoop();
  } catch (error) {
    console.error(`Failed to load fixture "${name}":`, error);
  }
}

// ---------------------------------------------------------------------------
// Fixture render loop (continuous redraw for fps measurement)
// ---------------------------------------------------------------------------

function startFixtureRenderLoop(): void {
  if (fixtureAnimationFrameId !== null) return;

  const renderTick = (): void => {
    if (!renderer || !lastFrame) return;

    renderer.drawFrame(lastFrame);
    frameCount++;
    updateStats();

    fixtureAnimationFrameId = requestAnimationFrame(renderTick);
  };

  fixtureAnimationFrameId = requestAnimationFrame(renderTick);
}

// ---------------------------------------------------------------------------
// WebSocket connection
// ---------------------------------------------------------------------------

function connectWebSocket(url: string): void {
  if (destroyed) return;

  console.log(`Connecting to ${url}`);
  websocket = new WebSocket(url);
  websocket.binaryType = "arraybuffer";

  websocket.onopen = () => {
    console.log("WebSocket connected");

    // Send initial resize so the server knows our viewport
    if (inputHandler) {
      const canvas = document.getElementById("canvas") as HTMLCanvasElement;
      if (canvas) {
        const scaleFactor = window.devicePixelRatio || 1;
        inputHandler.sendResize(canvas.clientWidth, canvas.clientHeight, scaleFactor);
      }
    }
  };

  websocket.onmessage = (event: MessageEvent) => {
    handleServerMessage(event.data);
  };

  websocket.onclose = (event: CloseEvent) => {
    console.log(`WebSocket closed: code=${event.code} reason="${event.reason}"`);
    scheduleReconnect(url);
  };

  websocket.onerror = (event: Event) => {
    console.error("WebSocket error:", event);
  };
}

function scheduleReconnect(url: string): void {
  if (destroyed) return;
  if (reconnectTimer !== null) return;

  console.log(`Reconnecting in ${RECONNECT_DELAY}ms...`);
  reconnectTimer = setTimeout(() => {
    reconnectTimer = null;
    connectWebSocket(url);
  }, RECONNECT_DELAY);
}

function handleServerMessage(data: unknown): void {
  if (!renderer) return;

  let frame: FrameMessage | null;

  if (data instanceof ArrayBuffer) {
    // Protobuf wire format from the Rust proto_encoding::encode_frame.
    frame = decodeProtoFrame(data);
    if (!frame) {
      console.error("Failed to decode protobuf frame");
      return;
    }
  } else if (data instanceof Blob) {
    // Some WebSocket implementations deliver binary as Blob.
    // Convert to ArrayBuffer and decode on next tick.
    data.arrayBuffer().then((buf) => handleServerMessage(buf));
    return;
  } else if (typeof data === "string") {
    // JSON fallback (for test server or debugging).
    try {
      frame = deserializeFrame(JSON.parse(data));
    } catch (error) {
      console.error("Failed to parse text message:", error);
      return;
    }
  } else {
    console.error("Unexpected message type:", typeof data);
    return;
  }

  framesReceived++;

  // Log atlas delta stats on first few frames for debugging
  if (framesReceived <= 5 && frame.atlas_deltas.length > 0) {
    const totalBytes = frame.atlas_deltas.reduce((sum, d) => sum + d.bytes.length, 0);
    console.log(
      `Frame ${frame.frame_id}: ${frame.atlas_deltas.length} atlas deltas, ` +
        `${(totalBytes / 1024).toFixed(1)} KB tile data, ` +
        `${frame.scene.quads.length} quads, ` +
        `${frame.scene.monochrome_sprites.length + frame.scene.subpixel_sprites.length} text sprites`,
    );
  }

  // Store the latest frame and schedule a render on the next animation frame.
  // This coalesces multiple server frames if they arrive faster than the
  // display refresh rate, always rendering the most recent one.
  pendingFrame = frame;

  if (wsAnimationFrameId === null) {
    wsAnimationFrameId = requestAnimationFrame(renderPendingFrame);
  }
}

function renderPendingFrame(): void {
  wsAnimationFrameId = null;

  if (!renderer || !pendingFrame) return;

  renderer.drawFrame(pendingFrame);
  lastFrame = pendingFrame;
  pendingFrame = null;
  frameCount++;
  updateStats();
}

// ---------------------------------------------------------------------------
// Input dispatch
// ---------------------------------------------------------------------------

function onInput(message: InputMessage): void {
  if (!websocket || websocket.readyState !== WebSocket.OPEN) return;

  try {
    websocket.send(JSON.stringify(message));
  } catch (error) {
    console.error("Failed to send input message:", error);
  }
}

// ---------------------------------------------------------------------------
// Stats overlay
// ---------------------------------------------------------------------------

function createStatsOverlay(): HTMLElement {
  const element = document.createElement("div");
  element.style.cssText = [
    "position: fixed",
    "top: 8px",
    "right: 8px",
    "padding: 4px 8px",
    "background: rgba(0, 0, 0, 0.7)",
    "color: #a6e3a1",
    "font-family: monospace",
    "font-size: 12px",
    "border-radius: 4px",
    "pointer-events: none",
    "z-index: 1000",
  ].join(";");
  element.textContent = "connecting...";
  document.body.appendChild(element);
  return element;
}

function updateStats(force = false): void {
  const now = performance.now();
  const elapsed = now - lastStatsTime;

  if (statsElement && (force || elapsed >= 1000)) {
    const renderFps = elapsed > 0 ? Math.round((frameCount / elapsed) * 1000) : 0;
    const serverFps = elapsed > 0 ? Math.round((framesReceived / elapsed) * 1000) : 0;
    statsElement.textContent = `${renderFps} render / ${serverFps} server fps`;
    frameCount = 0;
    framesReceived = 0;
    lastStatsTime = now;
  }
}

// ---------------------------------------------------------------------------
// Cleanup
// ---------------------------------------------------------------------------

function destroy(): void {
  destroyed = true;
  pendingFrame = null;

  if (fixtureAnimationFrameId !== null) {
    cancelAnimationFrame(fixtureAnimationFrameId);
    fixtureAnimationFrameId = null;
  }

  if (wsAnimationFrameId !== null) {
    cancelAnimationFrame(wsAnimationFrameId);
    wsAnimationFrameId = null;
  }

  if (reconnectTimer !== null) {
    clearTimeout(reconnectTimer);
    reconnectTimer = null;
  }

  websocket?.close();
  websocket = null;

  inputHandler?.destroy();
  inputHandler = null;

  renderer?.destroy();
  renderer = null;

  statsElement?.remove();
  statsElement = null;
}

// Clean up on page unload
window.addEventListener("beforeunload", destroy);

// ---------------------------------------------------------------------------
// Boot
// ---------------------------------------------------------------------------

main().catch((error) => {
  console.error("Failed to initialize renderer:", error);
  const errorDiv = document.getElementById("error");
  if (errorDiv) {
    errorDiv.style.display = "flex";
    const paragraph = errorDiv.querySelector("p");
    if (paragraph) {
      paragraph.textContent = `Initialization failed: ${error}`;
    }
  }
});
