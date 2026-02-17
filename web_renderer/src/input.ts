// Input capture for the Zed web streaming renderer.
//
// This module listens for mouse, keyboard, and scroll events on the canvas
// element and translates them into InputMessage objects matching the wire
// protocol defined in protocol.ts. Messages are dispatched through a callback
// so the caller can send them over WebSocket or buffer them as needed.
//
// Design notes:
//   - We capture at the canvas level rather than window level so that input
//     only fires when the Zed viewport is focused.
//   - Keyboard events use event.key for the key value and track modifier
//     state from the event itself rather than maintaining our own state.
//   - Click counting (double-click, triple-click) is handled here with a
//     timeout and distance threshold, matching typical OS behavior.
//   - Scroll events normalize deltaY/deltaX into pixel deltas regardless
//     of the browser's deltaMode.

import {
  InputMessage,
  InputKind,
  Modifiers,
  Point,
} from "./protocol";

// Maximum time between clicks to count as a multi-click (ms).
const MULTI_CLICK_TIMEOUT = 500;
// Maximum distance between clicks to count as a multi-click (px).
const MULTI_CLICK_RADIUS = 4;
// Multiplier for deltaMode === DOM_DELTA_LINE
const LINE_HEIGHT = 20;
// Multiplier for deltaMode === DOM_DELTA_PAGE
const PAGE_HEIGHT = 800;

export type InputCallback = (message: InputMessage) => void;

export class InputHandler {
  private canvas: HTMLCanvasElement;
  private callback: InputCallback;
  private scaleFactor: number;

  // Click counting state
  private lastClickTime = 0;
  private lastClickPosition: Point = { x: 0, y: 0 };
  private clickCount = 0;

  // Bound event handlers (stored so we can remove them on destroy)
  private onMouseMove: (e: MouseEvent) => void;
  private onMouseDown: (e: MouseEvent) => void;
  private onMouseUp: (e: MouseEvent) => void;
  private onWheel: (e: WheelEvent) => void;
  private onKeyDown: (e: KeyboardEvent) => void;
  private onKeyUp: (e: KeyboardEvent) => void;
  private onContextMenu: (e: Event) => void;

  constructor(canvas: HTMLCanvasElement, callback: InputCallback, scaleFactor = 1) {
    this.canvas = canvas;
    this.callback = callback;
    this.scaleFactor = scaleFactor;

    // Make the canvas focusable so it receives keyboard events
    if (!canvas.hasAttribute("tabindex")) {
      canvas.setAttribute("tabindex", "0");
    }

    this.onMouseMove = this.handleMouseMove.bind(this);
    this.onMouseDown = this.handleMouseDown.bind(this);
    this.onMouseUp = this.handleMouseUp.bind(this);
    this.onWheel = this.handleWheel.bind(this);
    this.onKeyDown = this.handleKeyDown.bind(this);
    this.onKeyUp = this.handleKeyUp.bind(this);
    this.onContextMenu = (e: Event) => e.preventDefault();

    canvas.addEventListener("mousemove", this.onMouseMove);
    canvas.addEventListener("mousedown", this.onMouseDown);
    canvas.addEventListener("mouseup", this.onMouseUp);
    canvas.addEventListener("wheel", this.onWheel, { passive: false });
    canvas.addEventListener("keydown", this.onKeyDown);
    canvas.addEventListener("keyup", this.onKeyUp);
    canvas.addEventListener("contextmenu", this.onContextMenu);
  }

  setScaleFactor(scaleFactor: number): void {
    this.scaleFactor = scaleFactor;
  }

  // Send a resize input message. Called externally when the viewport changes.
  sendResize(width: number, height: number, scaleFactor: number): void {
    this.scaleFactor = scaleFactor;
    this.callback({
      kind: InputKind.Resize,
      size: { width, height },
      scale_factor: scaleFactor,
    });
  }

  destroy(): void {
    this.canvas.removeEventListener("mousemove", this.onMouseMove);
    this.canvas.removeEventListener("mousedown", this.onMouseDown);
    this.canvas.removeEventListener("mouseup", this.onMouseUp);
    this.canvas.removeEventListener("wheel", this.onWheel);
    this.canvas.removeEventListener("keydown", this.onKeyDown);
    this.canvas.removeEventListener("keyup", this.onKeyUp);
    this.canvas.removeEventListener("contextmenu", this.onContextMenu);
  }

  // -----------------------------------------------------------------------
  // Mouse events
  // -----------------------------------------------------------------------

  private canvasPosition(e: MouseEvent): Point {
    const rect = this.canvas.getBoundingClientRect();
    return {
      x: (e.clientX - rect.left) * this.scaleFactor,
      y: (e.clientY - rect.top) * this.scaleFactor,
    };
  }

  private handleMouseMove(e: MouseEvent): void {
    this.callback({
      kind: InputKind.MouseMove,
      position: this.canvasPosition(e),
      modifiers: modifiersFromEvent(e),
    });
  }

  private handleMouseDown(e: MouseEvent): void {
    // Focus the canvas on click so keyboard events route here
    this.canvas.focus();

    const position = this.canvasPosition(e);
    const now = performance.now();

    // Click counting: reset if too much time passed or the mouse moved too far
    const dx = position.x - this.lastClickPosition.x;
    const dy = position.y - this.lastClickPosition.y;
    const distance = Math.sqrt(dx * dx + dy * dy);

    if (
      now - this.lastClickTime > MULTI_CLICK_TIMEOUT ||
      distance > MULTI_CLICK_RADIUS * this.scaleFactor
    ) {
      this.clickCount = 0;
    }

    this.clickCount++;
    this.lastClickTime = now;
    this.lastClickPosition = position;

    this.callback({
      kind: InputKind.MouseDown,
      button: e.button,
      position,
      click_count: this.clickCount,
      modifiers: modifiersFromEvent(e),
    });

    e.preventDefault();
  }

  private handleMouseUp(e: MouseEvent): void {
    this.callback({
      kind: InputKind.MouseUp,
      button: e.button,
      position: this.canvasPosition(e),
      modifiers: modifiersFromEvent(e),
    });
  }

  // -----------------------------------------------------------------------
  // Scroll events
  // -----------------------------------------------------------------------

  private handleWheel(e: WheelEvent): void {
    e.preventDefault();

    let deltaX = e.deltaX;
    let deltaY = e.deltaY;

    // Normalize to pixel deltas
    switch (e.deltaMode) {
      case WheelEvent.DOM_DELTA_LINE:
        deltaX *= LINE_HEIGHT;
        deltaY *= LINE_HEIGHT;
        break;
      case WheelEvent.DOM_DELTA_PAGE:
        deltaX *= PAGE_HEIGHT;
        deltaY *= PAGE_HEIGHT;
        break;
      // DOM_DELTA_PIXEL: already in pixels
    }

    this.callback({
      kind: InputKind.Scroll,
      position: this.canvasPosition(e),
      delta: { x: deltaX, y: deltaY },
      modifiers: modifiersFromEvent(e),
    });
  }

  // -----------------------------------------------------------------------
  // Keyboard events
  // -----------------------------------------------------------------------

  private handleKeyDown(e: KeyboardEvent): void {
    // Prevent default for most keys so the browser doesn't handle them
    // (scrolling with arrow keys, opening find with Cmd+F, etc.).
    // Allow Cmd+C/V/X for clipboard and F11/F12 for dev tools.
    const allowDefault =
      (e.metaKey || e.ctrlKey) &&
      (e.key === "c" || e.key === "v" || e.key === "x");
    const isDevTools = e.key === "F11" || e.key === "F12";

    if (!allowDefault && !isDevTools) {
      e.preventDefault();
    }

    this.callback({
      kind: InputKind.KeyDown,
      key: e.key,
      modifiers: modifiersFromEvent(e),
    });
  }

  private handleKeyUp(e: KeyboardEvent): void {
    this.callback({
      kind: InputKind.KeyUp,
      key: e.key,
      modifiers: modifiersFromEvent(e),
    });
  }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

function modifiersFromEvent(e: MouseEvent | KeyboardEvent): Modifiers {
  return {
    control: e.ctrlKey,
    alt: e.altKey,
    shift: e.shiftKey,
    meta: e.metaKey,
  };
}
