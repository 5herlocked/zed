// Protobuf decoder for the Zed web streaming wire protocol.
//
// This module decodes binary protobuf frames produced by the Rust
// `proto_encoding::encode_frame` function. It bridges the generated
// protobuf types (from scene.proto via protobufjs) to the renderer's
// internal protocol types defined in protocol.ts.
//
// Usage:
//   import { decodeProtoFrame } from "./proto_decoder";
//   const frame = decodeProtoFrame(arrayBuffer);

import { zed } from "./proto/scene.js";

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
  Scene,
  Shadow,
  SubpixelSprite,
  ThemeHints,
  TransformationMatrix,
  Underline,
  AtlasTile,
  Quad,
  IDENTITY_TRANSFORM,
} from "./protocol";

const FrameMessageProto = zed.scene.FrameMessage;

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

// Decode a binary protobuf frame from an ArrayBuffer into the renderer's
// FrameMessage type. Returns null on decode failure.
export function decodeProtoFrame(buffer: ArrayBuffer): FrameMessage | null {
  try {
    const bytes = new Uint8Array(buffer);
    const proto = FrameMessageProto.decode(bytes);
    return convertFrameMessage(proto);
  } catch (error) {
    console.error("Failed to decode protobuf frame:", error);
    return null;
  }
}

// ---------------------------------------------------------------------------
// Frame conversion
// ---------------------------------------------------------------------------

function convertFrameMessage(proto: zed.scene.IFrameMessage): FrameMessage {
  const scene = proto.scene ? convertScene(proto.scene) : emptyScene();

  const atlas_deltas: AtlasDelta[] = (proto.atlasEntries ?? []).map(convertAtlasEntry);

  const background_color = proto.backgroundColor ? convertHsla(proto.backgroundColor) : undefined;

  const theme_hints: ThemeHints | undefined = proto.themeHints
    ? {
        appearance: proto.themeHints.appearance ?? "dark",
        background_rgb: proto.themeHints.backgroundRgb ?? 0,
        background_css: proto.themeHints.backgroundCss ?? "#000000",
        background_appearance: proto.themeHints.backgroundAppearance ?? "",
      }
    : undefined;

  return {
    frame_id: typeof proto.frameId === "number" ? proto.frameId : Number(proto.frameId ?? 0),
    viewport_size: {
      width: proto.viewportWidth ?? 0,
      height: proto.viewportHeight ?? 0,
    },
    scale_factor: proto.scaleFactor ?? 1,
    atlas_deltas,
    scene,
    background_color,
    theme_hints,
  };
}

function emptyScene(): Scene {
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

function convertScene(proto: zed.scene.ISceneBody): Scene {
  return {
    shadows: (proto.shadows ?? []).map(convertShadow),
    quads: (proto.quads ?? []).map(convertQuad),
    underlines: (proto.underlines ?? []).map(convertUnderline),
    monochrome_sprites: (proto.monochromeSprites ?? []).map(convertMonoSprite),
    subpixel_sprites: (proto.subpixelSprites ?? []).map(convertSubpixelSprite),
    polychrome_sprites: (proto.polychromeSprites ?? []).map(convertPolySprite),
    paths: (proto.paths ?? []).map(convertPath),
  };
}

// ---------------------------------------------------------------------------
// Atlas conversion
// ---------------------------------------------------------------------------

function convertAtlasEntry(proto: zed.scene.IAtlasEntry): AtlasDelta {
  const texId = proto.textureId;
  const b = proto.bounds;

  return {
    texture_id: {
      index: texId?.index ?? 0,
      kind: (texId?.kind ?? 0) as AtlasTextureKind,
    },
    bounds: {
      origin: { x: b?.originX ?? 0, y: b?.originY ?? 0 },
      size: { width: b?.width ?? 0, height: b?.height ?? 0 },
    },
    format: (proto.format ?? 0) as AtlasTextureKind,
    bytes: proto.pixelData instanceof Uint8Array ? proto.pixelData : new Uint8Array(proto.pixelData ?? []),
  };
}

// ---------------------------------------------------------------------------
// Primitive conversions
// ---------------------------------------------------------------------------

function convertShadow(proto: zed.scene.IShadow): Shadow {
  return {
    order: proto.order ?? 0,
    blur_radius: proto.blurRadius ?? 0,
    bounds: convertBounds(proto.bounds),
    corner_radii: convertCorners(proto.cornerRadii),
    content_mask: convertContentMask(proto.contentMask),
    color: convertHsla(proto.color),
  };
}

function convertQuad(proto: zed.scene.IQuad): Quad {
  return {
    order: proto.order ?? 0,
    border_style: proto.borderStyle ?? 0,
    bounds: convertBounds(proto.bounds),
    content_mask: convertContentMask(proto.contentMask),
    background: convertBackground(proto.background),
    border_color: convertHsla(proto.borderColor),
    corner_radii: convertCorners(proto.cornerRadii),
    border_widths: convertEdges(proto.borderWidths),
  };
}

function convertUnderline(proto: zed.scene.IUnderline): Underline {
  return {
    order: proto.order ?? 0,
    bounds: convertBounds(proto.bounds),
    content_mask: convertContentMask(proto.contentMask),
    color: convertHsla(proto.color),
    thickness: proto.thickness ?? 0,
    wavy: proto.wavy ?? false,
  };
}

function convertMonoSprite(proto: zed.scene.IMonochromeSprite): MonochromeSprite {
  return {
    order: proto.order ?? 0,
    bounds: convertBounds(proto.bounds),
    content_mask: convertContentMask(proto.contentMask),
    color: convertHsla(proto.color),
    tile: convertAtlasTile(proto.tile),
    transformation: convertTransformation(proto.transformation),
  };
}

function convertSubpixelSprite(proto: zed.scene.ISubpixelSprite): SubpixelSprite {
  return {
    order: proto.order ?? 0,
    bounds: convertBounds(proto.bounds),
    content_mask: convertContentMask(proto.contentMask),
    color: convertHsla(proto.color),
    tile: convertAtlasTile(proto.tile),
    transformation: convertTransformation(proto.transformation),
  };
}

function convertPolySprite(proto: zed.scene.IPolychromeSprite): PolychromeSprite {
  return {
    order: proto.order ?? 0,
    grayscale: proto.grayscale ?? false,
    opacity: proto.opacity ?? 1,
    bounds: convertBounds(proto.bounds),
    content_mask: convertContentMask(proto.contentMask),
    corner_radii: convertCorners(proto.cornerRadii),
    tile: convertAtlasTile(proto.tile),
  };
}

function convertPath(proto: zed.scene.IPath): ScenePath {
  return {
    order: proto.order ?? 0,
    bounds: convertBounds(proto.bounds),
    content_mask: convertContentMask(proto.contentMask),
    color: convertBackground(proto.color),
    vertices: (proto.vertices ?? []).map(convertPathVertex),
  };
}

function convertPathVertex(proto: zed.scene.IPathVertex): PathVertex {
  return {
    xy_position: convertPoint(proto.xyPosition),
    st_position: convertPoint(proto.stPosition),
    content_mask: convertContentMask(proto.contentMask),
  };
}

// ---------------------------------------------------------------------------
// Geometry conversions
// ---------------------------------------------------------------------------

function convertPoint(proto: zed.scene.IPoint | null | undefined): { x: number; y: number } {
  return {
    x: proto?.x ?? 0,
    y: proto?.y ?? 0,
  };
}

function convertBounds(proto: zed.scene.IBounds | null | undefined): Bounds {
  return {
    origin: convertPoint(proto?.origin),
    size: {
      width: proto?.size?.width ?? 0,
      height: proto?.size?.height ?? 0,
    },
  };
}

function convertContentMask(proto: zed.scene.IContentMask | null | undefined): ContentMask {
  return {
    bounds: convertBounds(proto?.bounds),
  };
}

function convertCorners(proto: zed.scene.ICorners | null | undefined): Corners {
  return {
    top_left: proto?.topLeft ?? 0,
    top_right: proto?.topRight ?? 0,
    bottom_right: proto?.bottomRight ?? 0,
    bottom_left: proto?.bottomLeft ?? 0,
  };
}

function convertEdges(proto: zed.scene.IEdges | null | undefined): Edges {
  return {
    top: proto?.top ?? 0,
    right: proto?.right ?? 0,
    bottom: proto?.bottom ?? 0,
    left: proto?.left ?? 0,
  };
}

// ---------------------------------------------------------------------------
// Color conversions
// ---------------------------------------------------------------------------

function convertHsla(proto: zed.scene.IHsla | null | undefined): Hsla {
  return {
    h: proto?.h ?? 0,
    s: proto?.s ?? 0,
    l: proto?.l ?? 0,
    a: proto?.a ?? 0,
  };
}

function convertBackground(proto: zed.scene.IBackground | null | undefined): Background {
  const colors = proto?.colors ?? [];
  const color0 = colors[0];
  const color1 = colors[1];

  return {
    tag: (proto?.tag ?? 0) as BackgroundTag,
    color_space: (proto?.colorSpace ?? 0) as ColorSpace,
    solid: convertHsla(proto?.solid),
    gradient_angle_or_pattern_height: proto?.gradientAngleOrPatternHeight ?? 0,
    colors: [
      {
        color: convertHsla(color0?.color),
        percentage: color0?.percentage ?? 0,
      } as LinearColorStop,
      {
        color: convertHsla(color1?.color),
        percentage: color1?.percentage ?? 1,
      } as LinearColorStop,
    ],
  };
}

// ---------------------------------------------------------------------------
// Atlas conversions
// ---------------------------------------------------------------------------

function convertAtlasTile(proto: zed.scene.IAtlasTile | null | undefined): AtlasTile {
  const texId = proto?.textureId;
  const b = proto?.bounds;

  return {
    texture_id: {
      index: texId?.index ?? 0,
      kind: (texId?.kind ?? 0) as AtlasTextureKind,
    },
    tile_id: proto?.tileId ?? 0,
    padding: proto?.padding ?? 0,
    bounds: {
      origin: { x: b?.originX ?? 0, y: b?.originY ?? 0 },
      size: { width: b?.width ?? 0, height: b?.height ?? 0 },
    },
  };
}

// ---------------------------------------------------------------------------
// Transformation conversion
// ---------------------------------------------------------------------------

function convertTransformation(proto: zed.scene.ITransformationMatrix | null | undefined): TransformationMatrix {
  if (!proto) return IDENTITY_TRANSFORM;

  return {
    rotation_scale: [
      [proto.r00 ?? 1, proto.r01 ?? 0],
      [proto.r10 ?? 0, proto.r11 ?? 1],
    ],
    translation: [proto.tx ?? 0, proto.ty ?? 0],
  };
}
