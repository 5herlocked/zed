// Binary frame decoder for the Zed web streaming wire protocol.
//
// This module decodes binary frames produced by the Rust `BinaryFrameEncoder`
// in `crates/gpui/src/platform/web_streaming/binary_frame.rs`. The format is
// a flat sequence of little-endian values that can be decoded in a single pass
// through a DataView with zero allocations for the scalar fields.
//
// See binary_frame.rs for the complete wire format specification.

import {
  AtlasDelta,
  AtlasTextureKind,
  Background,
  BackgroundTag,
  Bounds,
  ColorSpace,
  ContentMask,
  Corners,
  Edges,
  FrameMessage,
  Hsla,
  LinearColorStop,
  MonochromeSprite,
  PathVertex,
  PolychromeSprite,
  ScenePath,
  Shadow,
  SubpixelSprite,
  TransformationMatrix,
  Underline,
  AtlasTile,
  AtlasTextureId,
  AtlasBounds,
  Scene,
  Quad,
} from "./protocol";

// Magic number identifying a binary frame: "ZEDF" in ASCII.
const FRAME_MAGIC = 0x5a454446;

// Expected wire format version.
const FRAME_VERSION = 1;

// A cursor that walks through a DataView, reading values sequentially.
// All reads are little-endian.
class BinaryReader {
  private view: DataView;
  private offset: number;
  private bytes: Uint8Array;

  constructor(buffer: ArrayBuffer) {
    this.view = new DataView(buffer);
    this.bytes = new Uint8Array(buffer);
    this.offset = 0;
  }

  get position(): number {
    return this.offset;
  }

  get remaining(): number {
    return this.view.byteLength - this.offset;
  }

  u32(): number {
    const v = this.view.getUint32(this.offset, true);
    this.offset += 4;
    return v;
  }

  i32(): number {
    const v = this.view.getInt32(this.offset, true);
    this.offset += 4;
    return v;
  }

  u64(): number {
    // Read as two u32s. JavaScript can't represent full u64 precisely,
    // but frame IDs won't exceed Number.MAX_SAFE_INTEGER in practice.
    const lo = this.view.getUint32(this.offset, true);
    const hi = this.view.getUint32(this.offset + 4, true);
    this.offset += 8;
    return hi * 0x100000000 + lo;
  }

  f32(): number {
    const v = this.view.getFloat32(this.offset, true);
    this.offset += 4;
    return v;
  }

  // Read raw bytes without copying (returns a view into the original buffer).
  rawBytes(length: number): Uint8Array {
    const slice = this.bytes.subarray(this.offset, this.offset + length);
    this.offset += length;
    return slice;
  }

  // --- Composite readers matching the Rust encoder's write order ---

  bounds(): Bounds {
    return {
      origin: { x: this.f32(), y: this.f32() },
      size: { width: this.f32(), height: this.f32() },
    };
  }

  contentMask(): ContentMask {
    return { bounds: this.bounds() };
  }

  corners(): Corners {
    return {
      top_left: this.f32(),
      top_right: this.f32(),
      bottom_right: this.f32(),
      bottom_left: this.f32(),
    };
  }

  edges(): Edges {
    return {
      top: this.f32(),
      right: this.f32(),
      bottom: this.f32(),
      left: this.f32(),
    };
  }

  hsla(): Hsla {
    return {
      h: this.f32(),
      s: this.f32(),
      l: this.f32(),
      a: this.f32(),
    };
  }

  background(): Background {
    const tag: BackgroundTag = this.u32();
    const color_space: ColorSpace = this.u32();
    const solid = this.hsla();
    const gradient_angle_or_pattern_height = this.f32();
    const color0: LinearColorStop = { color: this.hsla(), percentage: this.f32() };
    const color1: LinearColorStop = { color: this.hsla(), percentage: this.f32() };

    return {
      tag,
      color_space,
      solid,
      gradient_angle_or_pattern_height,
      colors: [color0, color1],
    };
  }

  atlasTile(): AtlasTile {
    const texture_id: AtlasTextureId = {
      index: this.u32(),
      kind: this.u32() as AtlasTextureKind,
    };
    const tile_id = this.u32();
    const padding = this.u32();
    const atlasBounds: AtlasBounds = {
      origin: { x: this.i32(), y: this.i32() },
      size: { width: this.i32(), height: this.i32() },
    };

    return { texture_id, tile_id, padding, bounds: atlasBounds };
  }

  transformation(): TransformationMatrix {
    return {
      rotation_scale: [
        [this.f32(), this.f32()],
        [this.f32(), this.f32()],
      ],
      translation: [this.f32(), this.f32()],
    };
  }

  // --- Primitive readers matching the Rust encoder's write order ---

  shadow(): Shadow {
    return {
      order: this.u32(),
      blur_radius: this.f32(),
      bounds: this.bounds(),
      corner_radii: this.corners(),
      content_mask: this.contentMask(),
      color: this.hsla(),
    };
  }

  quad(): Quad {
    return {
      order: this.u32(),
      border_style: this.u32(),
      bounds: this.bounds(),
      content_mask: this.contentMask(),
      background: this.background(),
      border_color: this.hsla(),
      corner_radii: this.corners(),
      border_widths: this.edges(),
    };
  }

  underline(): Underline {
    return {
      order: this.u32(),
      bounds: this.bounds(),
      content_mask: this.contentMask(),
      color: this.hsla(),
      thickness: this.f32(),
      wavy: this.u32() !== 0,
    };
  }

  monoSprite(): MonochromeSprite {
    return {
      order: this.u32(),
      bounds: this.bounds(),
      content_mask: this.contentMask(),
      color: this.hsla(),
      tile: this.atlasTile(),
      transformation: this.transformation(),
    };
  }

  subpixelSprite(): SubpixelSprite {
    return {
      order: this.u32(),
      bounds: this.bounds(),
      content_mask: this.contentMask(),
      color: this.hsla(),
      tile: this.atlasTile(),
      transformation: this.transformation(),
    };
  }

  polySprite(): PolychromeSprite {
    return {
      order: this.u32(),
      grayscale: this.u32() !== 0,
      opacity: this.f32(),
      bounds: this.bounds(),
      content_mask: this.contentMask(),
      corner_radii: this.corners(),
      tile: this.atlasTile(),
    };
  }

  path(): ScenePath {
    const order = this.u32();
    const bounds = this.bounds();
    const content_mask = this.contentMask();
    const color = this.background();

    const vertexCount = this.u32();
    const vertices: PathVertex[] = new Array(vertexCount);
    for (let i = 0; i < vertexCount; i++) {
      vertices[i] = {
        xy_position: { x: this.f32(), y: this.f32() },
        st_position: { x: this.f32(), y: this.f32() },
        content_mask: this.contentMask(),
      };
    }

    return { order, bounds, content_mask, vertices, color };
  }
}

// Decode a binary frame message from an ArrayBuffer.
//
// Returns null if the buffer is too small, has the wrong magic, or has
// an unsupported version.
export function decodeBinaryFrame(buffer: ArrayBuffer): FrameMessage | null {
  if (buffer.byteLength < 28) {
    console.error(`Binary frame too small: ${buffer.byteLength} bytes`);
    return null;
  }

  const r = new BinaryReader(buffer);

  // Frame header
  const magic = r.u32();
  if (magic !== FRAME_MAGIC) {
    console.error(`Invalid frame magic: 0x${magic.toString(16)} (expected 0x${FRAME_MAGIC.toString(16)})`);
    return null;
  }

  const version = r.u32();
  if (version !== FRAME_VERSION) {
    console.error(`Unsupported frame version: ${version} (expected ${FRAME_VERSION})`);
    return null;
  }

  const frame_id = r.u64();
  const viewport_width = r.f32();
  const viewport_height = r.f32();
  const scale_factor = r.f32();

  // Atlas section
  const numAtlasEntries = r.u32();
  const atlas_deltas: AtlasDelta[] = new Array(numAtlasEntries);
  for (let i = 0; i < numAtlasEntries; i++) {
    const texture_id: AtlasTextureId = {
      index: r.u32(),
      kind: r.u32() as AtlasTextureKind,
    };
    const origin_x = r.i32();
    const origin_y = r.i32();
    const width = r.i32();
    const height = r.i32();
    const format = r.u32() as AtlasTextureKind;
    const byteLength = r.u32();
    // Copy the bytes -- the subarray view would be invalidated if the buffer
    // is reused, and we need to pass owned data to the atlas uploader.
    const bytes = new Uint8Array(r.rawBytes(byteLength));

    atlas_deltas[i] = {
      texture_id,
      bounds: {
        origin: { x: origin_x, y: origin_y },
        size: { width, height },
      },
      format,
      bytes,
    };
  }

  // Shadows
  const numShadows = r.u32();
  const shadows: Shadow[] = new Array(numShadows);
  for (let i = 0; i < numShadows; i++) {
    shadows[i] = r.shadow();
  }

  // Quads
  const numQuads = r.u32();
  const quads: Quad[] = new Array(numQuads);
  for (let i = 0; i < numQuads; i++) {
    quads[i] = r.quad();
  }

  // Underlines
  const numUnderlines = r.u32();
  const underlines: Underline[] = new Array(numUnderlines);
  for (let i = 0; i < numUnderlines; i++) {
    underlines[i] = r.underline();
  }

  // Monochrome sprites
  const numMonoSprites = r.u32();
  const monochrome_sprites: MonochromeSprite[] = new Array(numMonoSprites);
  for (let i = 0; i < numMonoSprites; i++) {
    monochrome_sprites[i] = r.monoSprite();
  }

  // Subpixel sprites
  const numSubpixelSprites = r.u32();
  const subpixel_sprites: SubpixelSprite[] = new Array(numSubpixelSprites);
  for (let i = 0; i < numSubpixelSprites; i++) {
    subpixel_sprites[i] = r.subpixelSprite();
  }

  // Polychrome sprites
  const numPolySprites = r.u32();
  const polychrome_sprites: PolychromeSprite[] = new Array(numPolySprites);
  for (let i = 0; i < numPolySprites; i++) {
    polychrome_sprites[i] = r.polySprite();
  }

  // Paths
  const numPaths = r.u32();
  const paths: ScenePath[] = new Array(numPaths);
  for (let i = 0; i < numPaths; i++) {
    paths[i] = r.path();
  }

  const scene: Scene = {
    shadows,
    quads,
    paths,
    underlines,
    monochrome_sprites,
    subpixel_sprites,
    polychrome_sprites,
  };

  return {
    frame_id,
    viewport_size: { width: viewport_width, height: viewport_height },
    scale_factor,
    atlas_deltas,
    scene,
  };
}
