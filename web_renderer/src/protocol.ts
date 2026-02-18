// Wire protocol types for the Zed web streaming renderer.
//
// These types mirror the GPUI scene primitives defined in crates/gpui/src/scene.rs
// and the platform types in crates/gpui/src/platform.rs. Field layouts match the
// Rust repr(C) structs so that binary deserialization is straightforward.

// ---------------------------------------------------------------------------
// Geometry
// ---------------------------------------------------------------------------

export interface Point {
  x: number;
  y: number;
}

export interface Size {
  width: number;
  height: number;
}

export interface Bounds {
  origin: Point;
  size: Size;
}

export interface Corners {
  top_left: number;
  top_right: number;
  bottom_right: number;
  bottom_left: number;
}

export interface Edges {
  top: number;
  right: number;
  bottom: number;
  left: number;
}

export interface ContentMask {
  bounds: Bounds;
}

// ---------------------------------------------------------------------------
// Color
// ---------------------------------------------------------------------------

export interface Hsla {
  h: number;
  s: number;
  l: number;
  a: number;
}

export interface LinearColorStop {
  color: Hsla;
  percentage: number;
}

export const enum BackgroundTag {
  Solid = 0,
  LinearGradient = 1,
  PatternSlash = 2,
  Checkerboard = 3,
}

export const enum ColorSpace {
  Srgb = 0,
  Oklab = 1,
}

export interface Background {
  tag: BackgroundTag;
  color_space: ColorSpace;
  solid: Hsla;
  gradient_angle_or_pattern_height: number;
  colors: [LinearColorStop, LinearColorStop];
}

// ---------------------------------------------------------------------------
// Atlas
// ---------------------------------------------------------------------------

export const enum AtlasTextureKind {
  Monochrome = 0,
  Polychrome = 1,
  Subpixel = 2,
}

export interface AtlasTextureId {
  index: number;
  kind: AtlasTextureKind;
}

export interface AtlasBounds {
  origin: Point;
  size: Size;
}

export interface AtlasTile {
  texture_id: AtlasTextureId;
  tile_id: number;
  padding: number;
  bounds: AtlasBounds;
}

// ---------------------------------------------------------------------------
// Transformation
// ---------------------------------------------------------------------------

export interface TransformationMatrix {
  rotation_scale: [[number, number], [number, number]];
  translation: [number, number];
}

export const IDENTITY_TRANSFORM: TransformationMatrix = {
  rotation_scale: [
    [1, 0],
    [0, 1],
  ],
  translation: [0, 0],
};

// ---------------------------------------------------------------------------
// Scene Primitives
// ---------------------------------------------------------------------------

export const enum BorderStyle {
  Solid = 0,
  Dashed = 1,
}

export interface Quad {
  order: number;
  border_style: BorderStyle;
  bounds: Bounds;
  content_mask: ContentMask;
  background: Background;
  border_color: Hsla;
  corner_radii: Corners;
  border_widths: Edges;
}

export interface Shadow {
  order: number;
  blur_radius: number;
  bounds: Bounds;
  corner_radii: Corners;
  content_mask: ContentMask;
  color: Hsla;
}

export interface Underline {
  order: number;
  bounds: Bounds;
  content_mask: ContentMask;
  color: Hsla;
  thickness: number;
  wavy: boolean;
}

export interface PathVertex {
  xy_position: Point;
  st_position: Point;
  content_mask: ContentMask;
}

export interface ScenePath {
  order: number;
  bounds: Bounds;
  content_mask: ContentMask;
  vertices: PathVertex[];
  color: Background;
}

export interface MonochromeSprite {
  order: number;
  bounds: Bounds;
  content_mask: ContentMask;
  color: Hsla;
  tile: AtlasTile;
  transformation: TransformationMatrix;
}

export interface SubpixelSprite {
  order: number;
  bounds: Bounds;
  content_mask: ContentMask;
  color: Hsla;
  tile: AtlasTile;
  transformation: TransformationMatrix;
}

export interface PolychromeSprite {
  order: number;
  grayscale: boolean;
  opacity: number;
  bounds: Bounds;
  content_mask: ContentMask;
  corner_radii: Corners;
  tile: AtlasTile;
}

// ---------------------------------------------------------------------------
// Primitive batch (mirrors crates/gpui/src/scene.rs PrimitiveBatch)
// ---------------------------------------------------------------------------

export const enum PrimitiveKind {
  Shadow = 0,
  Quad = 1,
  Path = 2,
  Underline = 3,
  MonochromeSprite = 4,
  SubpixelSprite = 5,
  PolychromeSprite = 6,
  Surface = 7,
}

// ---------------------------------------------------------------------------
// Scene
// ---------------------------------------------------------------------------

export interface Scene {
  shadows: Shadow[];
  quads: Quad[];
  paths: ScenePath[];
  underlines: Underline[];
  monochrome_sprites: MonochromeSprite[];
  subpixel_sprites: SubpixelSprite[];
  polychrome_sprites: PolychromeSprite[];
}

export function emptyScene(): Scene {
  return {
    shadows: [],
    quads: [],
    paths: [],
    underlines: [],
    monochrome_sprites: [],
    subpixel_sprites: [],
    polychrome_sprites: [],
  };
}

// ---------------------------------------------------------------------------
// Wire Protocol Messages
// ---------------------------------------------------------------------------

export interface AtlasDelta {
  texture_id: AtlasTextureId;
  bounds: AtlasBounds;
  format: AtlasTextureKind;
  bytes: Uint8Array;
}

export interface ThemeHints {
  appearance: string;
  background_rgb: number;
  background_css: string;
  background_appearance: string;
}

export interface FrameMessage {
  frame_id: number;
  viewport_size: Size;
  scale_factor: number;
  atlas_deltas: AtlasDelta[];
  scene: Scene;
  background_color?: Hsla;
  theme_hints?: ThemeHints;
}

// ---------------------------------------------------------------------------
// Input (browser -> server)
// ---------------------------------------------------------------------------

export interface Modifiers {
  control: boolean;
  alt: boolean;
  shift: boolean;
  meta: boolean;
}

export const enum InputKind {
  MouseMove = "mouse_move",
  MouseDown = "mouse_down",
  MouseUp = "mouse_up",
  Scroll = "scroll",
  KeyDown = "key_down",
  KeyUp = "key_up",
  Resize = "resize",
}

export interface MouseMoveInput {
  kind: InputKind.MouseMove;
  position: Point;
  modifiers: Modifiers;
}

export interface MouseDownInput {
  kind: InputKind.MouseDown;
  button: number;
  position: Point;
  click_count: number;
  modifiers: Modifiers;
}

export interface MouseUpInput {
  kind: InputKind.MouseUp;
  button: number;
  position: Point;
  modifiers: Modifiers;
}

export interface ScrollInput {
  kind: InputKind.Scroll;
  position: Point;
  delta: Point;
  modifiers: Modifiers;
}

export interface KeyDownInput {
  kind: InputKind.KeyDown;
  key: string;
  modifiers: Modifiers;
}

export interface KeyUpInput {
  kind: InputKind.KeyUp;
  key: string;
  modifiers: Modifiers;
}

export interface ResizeInput {
  kind: InputKind.Resize;
  size: Size;
  scale_factor: number;
}

export type InputMessage =
  | MouseMoveInput
  | MouseDownInput
  | MouseUpInput
  | ScrollInput
  | KeyDownInput
  | KeyUpInput
  | ResizeInput;

// ---------------------------------------------------------------------------
// JSON deserialization
//
// During Phase 1 the wire format is JSON (recorded fixtures). These helpers
// decode a parsed JSON object into the typed protocol structures, handling
// minor representation differences (e.g. boolean-as-integer for wavy).
// ---------------------------------------------------------------------------

export function deserializeFrame(json: unknown): FrameMessage {
  const obj = json as Record<string, unknown>;
  const scene = obj["scene"] as Record<string, unknown>;
  const viewportSize = obj["viewport_size"] as Record<string, number>;
  const atlasDeltas = (obj["atlas_deltas"] ?? []) as Record<string, unknown>[];

  return {
    frame_id: (obj["frame_id"] as number) ?? 0,
    viewport_size: {
      width: viewportSize?.["width"] ?? 0,
      height: viewportSize?.["height"] ?? 0,
    },
    scale_factor: (obj["scale_factor"] as number) ?? 1,
    atlas_deltas: atlasDeltas.map(deserializeAtlasDelta),
    scene: deserializeScene(scene),
  };
}

function deserializeScene(json: Record<string, unknown>): Scene {
  return {
    shadows: asArray(json["shadows"]) as Shadow[],
    quads: asArray(json["quads"]) as Quad[],
    paths: asArray(json["paths"]).map(deserializePath),
    underlines: asArray(json["underlines"]).map(deserializeUnderline),
    monochrome_sprites: asArray(json["monochrome_sprites"]) as MonochromeSprite[],
    subpixel_sprites: asArray(json["subpixel_sprites"]) as SubpixelSprite[],
    polychrome_sprites: asArray(json["polychrome_sprites"]) as PolychromeSprite[],
  };
}

function deserializePath(json: unknown): ScenePath {
  const obj = json as Record<string, unknown>;
  return {
    order: obj["order"] as number,
    bounds: obj["bounds"] as Bounds,
    content_mask: obj["content_mask"] as ContentMask,
    vertices: asArray(obj["vertices"]) as PathVertex[],
    color: obj["color"] as Background,
  };
}

function deserializeUnderline(json: unknown): Underline {
  const obj = json as Record<string, unknown>;
  return {
    order: obj["order"] as number,
    bounds: obj["bounds"] as Bounds,
    content_mask: obj["content_mask"] as ContentMask,
    color: obj["color"] as Hsla,
    thickness: obj["thickness"] as number,
    wavy: !!(obj["wavy"] as number),
  };
}

function deserializeAtlasDelta(json: Record<string, unknown>): AtlasDelta {
  const bytesRaw = json["bytes"];
  let bytes: Uint8Array;
  if (bytesRaw instanceof Uint8Array) {
    bytes = bytesRaw;
  } else if (Array.isArray(bytesRaw)) {
    bytes = new Uint8Array(bytesRaw as number[]);
  } else {
    bytes = new Uint8Array(0);
  }

  return {
    texture_id: json["texture_id"] as AtlasTextureId,
    bounds: json["bounds"] as AtlasBounds,
    format: json["format"] as AtlasTextureKind,
    bytes,
  };
}

function asArray(value: unknown): unknown[] {
  return Array.isArray(value) ? value : [];
}
