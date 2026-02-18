//! Wire format types for streaming GPUI scenes to the web renderer.
//!
//! These types mirror the scene primitives in `crate::scene` but are
//! purpose-built for serialization. They exist as a separate layer because:
//!
//! - `Hsla` serializes as Rgba in the main codebase (for theme/settings JSON),
//!   but the browser shaders expect raw HSLA values.
//! - `ScaledPixels` and `ContentMask` don't derive `Serialize`.
//! - Atlas tile references need to be serialized with enough information for
//!   the browser to resolve them against its local atlas mirror.
//! - We want a stable wire format decoupled from internal GPUI type changes.
//!
//! Each type has a `From` conversion from its GPUI counterpart so the
//! serialization path is: `Scene` -> `SceneMessage` -> JSON/binary.

use serde::Serialize;

use crate::scene::{
    BorderStyle, DrawOrder, MonochromeSprite, Path, PolychromeSprite, Quad, Scene, Shadow,
    SubpixelSprite, TransformationMatrix, Underline,
};
use crate::{
    AtlasTextureId, AtlasTextureKind, AtlasTile, Bounds, ContentMask, Corners, DevicePixels, Edges,
    Hsla, Point, ScaledPixels, Size,
};

// ---------------------------------------------------------------------------
// Geometry
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, Serialize)]
pub struct PointMessage {
    pub x: f32,
    pub y: f32,
}

impl From<Point<ScaledPixels>> for PointMessage {
    fn from(p: Point<ScaledPixels>) -> Self {
        Self { x: p.x.0, y: p.y.0 }
    }
}

impl From<Point<f32>> for PointMessage {
    fn from(p: Point<f32>) -> Self {
        Self { x: p.x, y: p.y }
    }
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct SizeMessage {
    pub width: f32,
    pub height: f32,
}

impl From<Size<ScaledPixels>> for SizeMessage {
    fn from(s: Size<ScaledPixels>) -> Self {
        Self {
            width: s.width.0,
            height: s.height.0,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct BoundsMessage {
    pub origin: PointMessage,
    pub size: SizeMessage,
}

impl From<Bounds<ScaledPixels>> for BoundsMessage {
    fn from(b: Bounds<ScaledPixels>) -> Self {
        Self {
            origin: b.origin.into(),
            size: b.size.into(),
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct ContentMaskMessage {
    pub bounds: BoundsMessage,
}

impl From<ContentMask<ScaledPixels>> for ContentMaskMessage {
    fn from(m: ContentMask<ScaledPixels>) -> Self {
        Self {
            bounds: m.bounds.into(),
        }
    }
}

impl From<&ContentMask<ScaledPixels>> for ContentMaskMessage {
    fn from(m: &ContentMask<ScaledPixels>) -> Self {
        Self {
            bounds: m.bounds.into(),
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct CornersMessage {
    pub top_left: f32,
    pub top_right: f32,
    pub bottom_right: f32,
    pub bottom_left: f32,
}

impl From<Corners<ScaledPixels>> for CornersMessage {
    fn from(c: Corners<ScaledPixels>) -> Self {
        Self {
            top_left: c.top_left.0,
            top_right: c.top_right.0,
            bottom_right: c.bottom_right.0,
            bottom_left: c.bottom_left.0,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct EdgesMessage {
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub left: f32,
}

impl From<Edges<ScaledPixels>> for EdgesMessage {
    fn from(e: Edges<ScaledPixels>) -> Self {
        Self {
            top: e.top.0,
            right: e.right.0,
            bottom: e.bottom.0,
            left: e.left.0,
        }
    }
}

// ---------------------------------------------------------------------------
// Color
//
// We serialize Hsla as raw h/s/l/a floats rather than converting to Rgba,
// because the browser-side WGSL shaders perform the hsla_to_rgba conversion
// themselves (matching the native GPUI shaders).
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, Serialize)]
pub struct HslaMessage {
    pub h: f32,
    pub s: f32,
    pub l: f32,
    pub a: f32,
}

impl From<Hsla> for HslaMessage {
    fn from(c: Hsla) -> Self {
        Self {
            h: c.h,
            s: c.s,
            l: c.l,
            a: c.a,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct LinearColorStopMessage {
    pub color: HslaMessage,
    pub percentage: f32,
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct BackgroundMessage {
    pub tag: u32,
    pub color_space: u32,
    pub solid: HslaMessage,
    pub gradient_angle_or_pattern_height: f32,
    pub colors: [LinearColorStopMessage; 2],
}

impl From<crate::Background> for BackgroundMessage {
    fn from(bg: crate::Background) -> Self {
        // Access the fields through the Background's public API or
        // use the Serialize impl to extract values. Since Background
        // has pub(crate) fields and we're in the same crate, we can
        // access them directly.
        Self {
            tag: bg.tag as u32,
            color_space: bg.color_space as u32,
            solid: bg.solid.into(),
            gradient_angle_or_pattern_height: bg.gradient_angle_or_pattern_height,
            colors: [
                LinearColorStopMessage {
                    color: bg.colors[0].color.into(),
                    percentage: bg.colors[0].percentage,
                },
                LinearColorStopMessage {
                    color: bg.colors[1].color.into(),
                    percentage: bg.colors[1].percentage,
                },
            ],
        }
    }
}

// ---------------------------------------------------------------------------
// Atlas
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, Serialize)]
pub struct AtlasTextureIdMessage {
    pub index: u32,
    pub kind: u32,
}

impl From<AtlasTextureId> for AtlasTextureIdMessage {
    fn from(id: AtlasTextureId) -> Self {
        Self {
            index: id.index,
            kind: id.kind as u32,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct AtlasBoundsMessage {
    pub origin: AtlasPointMessage,
    pub size: AtlasSizeMessage,
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct AtlasPointMessage {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct AtlasSizeMessage {
    pub width: i32,
    pub height: i32,
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct AtlasTileMessage {
    pub texture_id: AtlasTextureIdMessage,
    pub tile_id: u32,
    pub padding: u32,
    pub bounds: AtlasBoundsMessage,
}

impl From<AtlasTile> for AtlasTileMessage {
    fn from(tile: AtlasTile) -> Self {
        Self {
            texture_id: tile.texture_id.into(),
            tile_id: tile.tile_id.0,
            padding: tile.padding,
            bounds: AtlasBoundsMessage {
                origin: AtlasPointMessage {
                    x: tile.bounds.origin.x.0,
                    y: tile.bounds.origin.y.0,
                },
                size: AtlasSizeMessage {
                    width: tile.bounds.size.width.0,
                    height: tile.bounds.size.height.0,
                },
            },
        }
    }
}

// ---------------------------------------------------------------------------
// Transformation
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, Serialize)]
pub struct TransformationMatrixMessage {
    pub rotation_scale: [[f32; 2]; 2],
    pub translation: [f32; 2],
}

impl From<TransformationMatrix> for TransformationMatrixMessage {
    fn from(t: TransformationMatrix) -> Self {
        Self {
            rotation_scale: t.rotation_scale,
            translation: t.translation,
        }
    }
}

// ---------------------------------------------------------------------------
// Scene Primitives
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize)]
pub struct QuadMessage {
    pub order: DrawOrder,
    pub border_style: u32,
    pub bounds: BoundsMessage,
    pub content_mask: ContentMaskMessage,
    pub background: BackgroundMessage,
    pub border_color: HslaMessage,
    pub corner_radii: CornersMessage,
    pub border_widths: EdgesMessage,
}

impl From<&Quad> for QuadMessage {
    fn from(q: &Quad) -> Self {
        Self {
            order: q.order,
            border_style: q.border_style as u32,
            bounds: q.bounds.into(),
            content_mask: (&q.content_mask).into(),
            background: q.background.into(),
            border_color: q.border_color.into(),
            corner_radii: q.corner_radii.into(),
            border_widths: q.border_widths.into(),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct ShadowMessage {
    pub order: DrawOrder,
    pub blur_radius: f32,
    pub bounds: BoundsMessage,
    pub corner_radii: CornersMessage,
    pub content_mask: ContentMaskMessage,
    pub color: HslaMessage,
}

impl From<&Shadow> for ShadowMessage {
    fn from(s: &Shadow) -> Self {
        Self {
            order: s.order,
            blur_radius: s.blur_radius.0,
            bounds: s.bounds.into(),
            corner_radii: s.corner_radii.into(),
            content_mask: (&s.content_mask).into(),
            color: s.color.into(),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct UnderlineMessage {
    pub order: DrawOrder,
    pub bounds: BoundsMessage,
    pub content_mask: ContentMaskMessage,
    pub color: HslaMessage,
    pub thickness: f32,
    pub wavy: u32,
}

impl From<&Underline> for UnderlineMessage {
    fn from(u: &Underline) -> Self {
        Self {
            order: u.order,
            bounds: u.bounds.into(),
            content_mask: (&u.content_mask).into(),
            color: u.color.into(),
            thickness: u.thickness.0,
            wavy: u.wavy,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct PathVertexMessage {
    pub xy_position: PointMessage,
    pub st_position: PointMessage,
    pub content_mask: ContentMaskMessage,
}

#[derive(Debug, Clone, Serialize)]
pub struct PathMessage {
    pub order: DrawOrder,
    pub bounds: BoundsMessage,
    pub content_mask: ContentMaskMessage,
    pub vertices: Vec<PathVertexMessage>,
    pub color: BackgroundMessage,
}

impl From<&Path<ScaledPixels>> for PathMessage {
    fn from(p: &Path<ScaledPixels>) -> Self {
        Self {
            order: p.order,
            bounds: p.bounds.into(),
            content_mask: (&p.content_mask).into(),
            vertices: p
                .vertices
                .iter()
                .map(|v| PathVertexMessage {
                    xy_position: v.xy_position.into(),
                    st_position: v.st_position.into(),
                    content_mask: (&v.content_mask).into(),
                })
                .collect(),
            color: p.color.into(),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct MonochromeSpriteMessage {
    pub order: DrawOrder,
    pub bounds: BoundsMessage,
    pub content_mask: ContentMaskMessage,
    pub color: HslaMessage,
    pub tile: AtlasTileMessage,
    pub transformation: TransformationMatrixMessage,
}

impl From<&MonochromeSprite> for MonochromeSpriteMessage {
    fn from(s: &MonochromeSprite) -> Self {
        Self {
            order: s.order,
            bounds: s.bounds.into(),
            content_mask: (&s.content_mask).into(),
            color: s.color.into(),
            tile: s.tile.clone().into(),
            transformation: s.transformation.into(),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct SubpixelSpriteMessage {
    pub order: DrawOrder,
    pub bounds: BoundsMessage,
    pub content_mask: ContentMaskMessage,
    pub color: HslaMessage,
    pub tile: AtlasTileMessage,
    pub transformation: TransformationMatrixMessage,
}

impl From<&SubpixelSprite> for SubpixelSpriteMessage {
    fn from(s: &SubpixelSprite) -> Self {
        Self {
            order: s.order,
            bounds: s.bounds.into(),
            content_mask: (&s.content_mask).into(),
            color: s.color.into(),
            tile: s.tile.clone().into(),
            transformation: s.transformation.into(),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct PolychromeSpriteMessage {
    pub order: DrawOrder,
    pub grayscale: bool,
    pub opacity: f32,
    pub bounds: BoundsMessage,
    pub content_mask: ContentMaskMessage,
    pub corner_radii: CornersMessage,
    pub tile: AtlasTileMessage,
}

impl From<&PolychromeSprite> for PolychromeSpriteMessage {
    fn from(s: &PolychromeSprite) -> Self {
        Self {
            order: s.order,
            grayscale: s.grayscale,
            opacity: s.opacity,
            bounds: s.bounds.into(),
            content_mask: (&s.content_mask).into(),
            corner_radii: s.corner_radii.into(),
            tile: s.tile.clone().into(),
        }
    }
}

// ---------------------------------------------------------------------------
// Atlas Delta
//
// Sent when new tiles are allocated in the atlas. The browser uses these to
// update its local atlas mirror.
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize)]
pub struct AtlasDeltaMessage {
    pub texture_id: AtlasTextureIdMessage,
    pub bounds: AtlasBoundsMessage,
    pub format: u32,
    pub bytes: Vec<u8>,
}

// ---------------------------------------------------------------------------
// Scene Message (top-level frame)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize)]
pub struct SceneBodyMessage {
    pub shadows: Vec<ShadowMessage>,
    pub quads: Vec<QuadMessage>,
    pub paths: Vec<PathMessage>,
    pub underlines: Vec<UnderlineMessage>,
    pub monochrome_sprites: Vec<MonochromeSpriteMessage>,
    pub subpixel_sprites: Vec<SubpixelSpriteMessage>,
    pub polychrome_sprites: Vec<PolychromeSpriteMessage>,
}

impl From<&Scene> for SceneBodyMessage {
    fn from(scene: &Scene) -> Self {
        Self {
            shadows: scene.shadows.iter().map(ShadowMessage::from).collect(),
            quads: scene.quads.iter().map(QuadMessage::from).collect(),
            paths: scene.paths.iter().map(PathMessage::from).collect(),
            underlines: scene
                .underlines
                .iter()
                .map(UnderlineMessage::from)
                .collect(),
            monochrome_sprites: scene
                .monochrome_sprites
                .iter()
                .map(MonochromeSpriteMessage::from)
                .collect(),
            subpixel_sprites: scene
                .subpixel_sprites
                .iter()
                .map(SubpixelSpriteMessage::from)
                .collect(),
            polychrome_sprites: scene
                .polychrome_sprites
                .iter()
                .map(PolychromeSpriteMessage::from)
                .collect(),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct FrameMessage {
    pub frame_id: u64,
    pub viewport_size: SizeMessage,
    pub scale_factor: f32,
    pub atlas_deltas: Vec<AtlasDeltaMessage>,
    pub scene: SceneBodyMessage,
}
