//! Binary frame serializer for the web streaming wire protocol.
//!
//! This module serializes GPUI scenes into a compact binary format that can
//! be efficiently decoded on the browser side using `DataView`. The format
//! avoids JSON entirely -- atlas tile pixel data is stored as raw bytes
//! inline, and primitive structs are written as flat little-endian fields.
//!
//! Wire format (all values little-endian):
//!
//! ```text
//! FRAME HEADER
//!   u32   magic (0x5A454446 = "ZEDF")
//!   u32   version (1)
//!   u64   frame_id
//!   f32   viewport_width   (in scaled pixels)
//!   f32   viewport_height  (in scaled pixels)
//!   f32   scale_factor
//!
//! ATLAS SECTION
//!   u32   num_atlas_entries
//!   For each atlas entry:
//!     u32   texture_index
//!     u32   texture_kind    (0=Mono, 1=Poly, 2=Subpixel)
//!     i32   origin_x
//!     i32   origin_y
//!     i32   width
//!     i32   height
//!     u32   format          (same as texture_kind)
//!     u32   byte_length
//!     [u8]  pixel data      (byte_length bytes, NO padding)
//!
//! PRIMITIVE SECTIONS (one per type, in draw order)
//!   Each section:
//!     u32   count
//!     For each primitive:
//!       fields written as raw little-endian values
//!
//!   Section order:
//!     1. Shadows
//!     2. Quads
//!     3. Underlines
//!     4. Monochrome sprites
//!     5. Subpixel sprites
//!     6. Polychrome sprites
//!     7. Paths
//! ```
//!
//! The browser decodes this with a single pass through a `DataView`, reading
//! fields at known offsets. No allocations or string parsing needed for the
//! primitive data.

use crate::scene::{
    MonochromeSprite, Path, PolychromeSprite, Quad, Scene, Shadow, SubpixelSprite, Underline,
};
use crate::{Hsla, ScaledPixels};

use super::mirroring_atlas::CachedTileData;

/// Magic number identifying a binary frame: "ZEDF" in ASCII.
pub const FRAME_MAGIC: u32 = 0x5A454446;

/// Current wire format version.
pub const FRAME_VERSION: u32 = 1;

/// Pre-allocated capacity for the frame buffer (256 KB). Grows as needed.
const INITIAL_CAPACITY: usize = 256 * 1024;

/// A reusable buffer for serializing binary frames.
pub struct BinaryFrameEncoder {
    buf: Vec<u8>,
}

impl BinaryFrameEncoder {
    pub fn new() -> Self {
        Self {
            buf: Vec::with_capacity(INITIAL_CAPACITY),
        }
    }

    /// Encode a complete frame into the internal buffer and return the bytes.
    ///
    /// The returned slice is valid until the next call to `encode`.
    pub fn encode(
        &mut self,
        frame_id: u64,
        viewport_width: f32,
        viewport_height: f32,
        scale_factor: f32,
        atlas_tiles: &[CachedTileData],
        scene: &Scene,
    ) -> &[u8] {
        self.buf.clear();

        // Frame header
        self.write_u32(FRAME_MAGIC);
        self.write_u32(FRAME_VERSION);
        self.write_u64(frame_id);
        self.write_f32(viewport_width);
        self.write_f32(viewport_height);
        self.write_f32(scale_factor);

        // Atlas section
        self.write_u32(atlas_tiles.len() as u32);
        for tile in atlas_tiles {
            self.write_u32(tile.texture_id.index);
            self.write_u32(tile.texture_id.kind as u32);
            self.write_i32(tile.bounds.origin.x.0);
            self.write_i32(tile.bounds.origin.y.0);
            self.write_i32(tile.bounds.size.width.0);
            self.write_i32(tile.bounds.size.height.0);
            self.write_u32(tile.format as u32);
            self.write_u32(tile.bytes.len() as u32);
            self.buf.extend_from_slice(&tile.bytes);
        }

        // Shadows
        self.write_u32(scene.shadows.len() as u32);
        for shadow in &scene.shadows {
            self.write_shadow(shadow);
        }

        // Quads
        self.write_u32(scene.quads.len() as u32);
        for quad in &scene.quads {
            self.write_quad(quad);
        }

        // Underlines
        self.write_u32(scene.underlines.len() as u32);
        for underline in &scene.underlines {
            self.write_underline(underline);
        }

        // Monochrome sprites
        self.write_u32(scene.monochrome_sprites.len() as u32);
        for sprite in &scene.monochrome_sprites {
            self.write_mono_sprite(sprite);
        }

        // Subpixel sprites
        self.write_u32(scene.subpixel_sprites.len() as u32);
        for sprite in &scene.subpixel_sprites {
            self.write_subpixel_sprite(sprite);
        }

        // Polychrome sprites
        self.write_u32(scene.polychrome_sprites.len() as u32);
        for sprite in &scene.polychrome_sprites {
            self.write_poly_sprite(sprite);
        }

        // Paths
        self.write_u32(scene.paths.len() as u32);
        for path in &scene.paths {
            self.write_path(path);
        }

        &self.buf
    }

    // -----------------------------------------------------------------------
    // Primitive writers
    // -----------------------------------------------------------------------

    fn write_shadow(&mut self, s: &Shadow) {
        self.write_u32(s.order);
        self.write_f32(s.blur_radius.0);
        self.write_bounds_sp(&s.bounds);
        self.write_corners_sp(&s.corner_radii);
        self.write_content_mask_sp(&s.content_mask);
        self.write_hsla(&s.color);
    }

    fn write_quad(&mut self, q: &Quad) {
        self.write_u32(q.order);
        self.write_u32(q.border_style as u32);
        self.write_bounds_sp(&q.bounds);
        self.write_content_mask_sp(&q.content_mask);
        self.write_background(&q.background);
        self.write_hsla(&q.border_color);
        self.write_corners_sp(&q.corner_radii);
        self.write_edges_sp(&q.border_widths);
    }

    fn write_underline(&mut self, u: &Underline) {
        self.write_u32(u.order);
        self.write_bounds_sp(&u.bounds);
        self.write_content_mask_sp(&u.content_mask);
        self.write_hsla(&u.color);
        self.write_f32(u.thickness.0);
        self.write_u32(u.wavy);
    }

    fn write_mono_sprite(&mut self, s: &MonochromeSprite) {
        self.write_u32(s.order);
        self.write_bounds_sp(&s.bounds);
        self.write_content_mask_sp(&s.content_mask);
        self.write_hsla(&s.color);
        self.write_atlas_tile(&s.tile);
        self.write_transformation(&s.transformation);
    }

    fn write_subpixel_sprite(&mut self, s: &SubpixelSprite) {
        self.write_u32(s.order);
        self.write_bounds_sp(&s.bounds);
        self.write_content_mask_sp(&s.content_mask);
        self.write_hsla(&s.color);
        self.write_atlas_tile(&s.tile);
        self.write_transformation(&s.transformation);
    }

    fn write_poly_sprite(&mut self, s: &PolychromeSprite) {
        self.write_u32(s.order);
        self.write_u32(if s.grayscale { 1 } else { 0 });
        self.write_f32(s.opacity);
        self.write_bounds_sp(&s.bounds);
        self.write_content_mask_sp(&s.content_mask);
        self.write_corners_sp(&s.corner_radii);
        self.write_atlas_tile(&s.tile);
    }

    fn write_path(&mut self, p: &Path<ScaledPixels>) {
        self.write_u32(p.order);
        self.write_bounds_sp(&p.bounds);
        self.write_content_mask_sp(&p.content_mask);
        self.write_background(&p.color);
        // Vertices: length-prefixed array
        self.write_u32(p.vertices.len() as u32);
        for v in &p.vertices {
            self.write_f32(v.xy_position.x.0);
            self.write_f32(v.xy_position.y.0);
            self.write_f32(v.st_position.x);
            self.write_f32(v.st_position.y);
            self.write_content_mask_sp(&v.content_mask);
        }
    }

    // -----------------------------------------------------------------------
    // Composite type writers
    // -----------------------------------------------------------------------

    fn write_bounds_sp(&mut self, b: &crate::Bounds<ScaledPixels>) {
        self.write_f32(b.origin.x.0);
        self.write_f32(b.origin.y.0);
        self.write_f32(b.size.width.0);
        self.write_f32(b.size.height.0);
    }

    fn write_content_mask_sp(&mut self, m: &crate::ContentMask<ScaledPixels>) {
        self.write_bounds_sp(&m.bounds);
    }

    fn write_corners_sp(&mut self, c: &crate::Corners<ScaledPixels>) {
        self.write_f32(c.top_left.0);
        self.write_f32(c.top_right.0);
        self.write_f32(c.bottom_right.0);
        self.write_f32(c.bottom_left.0);
    }

    fn write_edges_sp(&mut self, e: &crate::Edges<ScaledPixels>) {
        self.write_f32(e.top.0);
        self.write_f32(e.right.0);
        self.write_f32(e.bottom.0);
        self.write_f32(e.left.0);
    }

    fn write_hsla(&mut self, c: &Hsla) {
        self.write_f32(c.h);
        self.write_f32(c.s);
        self.write_f32(c.l);
        self.write_f32(c.a);
    }

    fn write_background(&mut self, bg: &crate::Background) {
        self.write_u32(bg.tag as u32);
        self.write_u32(bg.color_space as u32);
        self.write_hsla(&bg.solid);
        self.write_f32(bg.gradient_angle_or_pattern_height);
        // colors[0]
        self.write_hsla(&bg.colors[0].color);
        self.write_f32(bg.colors[0].percentage);
        // colors[1]
        self.write_hsla(&bg.colors[1].color);
        self.write_f32(bg.colors[1].percentage);
    }

    fn write_atlas_tile(&mut self, tile: &crate::AtlasTile) {
        self.write_u32(tile.texture_id.index);
        self.write_u32(tile.texture_id.kind as u32);
        self.write_u32(tile.tile_id.0);
        self.write_u32(tile.padding);
        self.write_i32(tile.bounds.origin.x.0);
        self.write_i32(tile.bounds.origin.y.0);
        self.write_i32(tile.bounds.size.width.0);
        self.write_i32(tile.bounds.size.height.0);
    }

    fn write_transformation(&mut self, t: &crate::scene::TransformationMatrix) {
        self.write_f32(t.rotation_scale[0][0]);
        self.write_f32(t.rotation_scale[0][1]);
        self.write_f32(t.rotation_scale[1][0]);
        self.write_f32(t.rotation_scale[1][1]);
        self.write_f32(t.translation[0]);
        self.write_f32(t.translation[1]);
    }

    // -----------------------------------------------------------------------
    // Scalar writers
    // -----------------------------------------------------------------------

    #[inline(always)]
    fn write_u32(&mut self, v: u32) {
        self.buf.extend_from_slice(&v.to_le_bytes());
    }

    #[inline(always)]
    fn write_i32(&mut self, v: i32) {
        self.buf.extend_from_slice(&v.to_le_bytes());
    }

    #[inline(always)]
    fn write_u64(&mut self, v: u64) {
        self.buf.extend_from_slice(&v.to_le_bytes());
    }

    #[inline(always)]
    fn write_f32(&mut self, v: f32) {
        self.buf.extend_from_slice(&v.to_le_bytes());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_magic_and_version() {
        let mut encoder = BinaryFrameEncoder::new();
        let scene = Scene::default();
        let bytes = encoder.encode(42, 1920.0, 1080.0, 2.0, &[], &scene);

        // Read back header
        assert!(bytes.len() >= 24);
        let magic = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
        let version = u32::from_le_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]);
        let frame_id = u64::from_le_bytes([
            bytes[8], bytes[9], bytes[10], bytes[11], bytes[12], bytes[13], bytes[14], bytes[15],
        ]);
        let vp_w = f32::from_le_bytes([bytes[16], bytes[17], bytes[18], bytes[19]]);
        let vp_h = f32::from_le_bytes([bytes[20], bytes[21], bytes[22], bytes[23]]);
        let sf = f32::from_le_bytes([bytes[24], bytes[25], bytes[26], bytes[27]]);

        assert_eq!(magic, FRAME_MAGIC);
        assert_eq!(version, FRAME_VERSION);
        assert_eq!(frame_id, 42);
        assert_eq!(vp_w, 1920.0);
        assert_eq!(vp_h, 1080.0);
        assert_eq!(sf, 2.0);
    }

    #[test]
    fn test_empty_scene_sections() {
        let mut encoder = BinaryFrameEncoder::new();
        let scene = Scene::default();
        let bytes = encoder.encode(0, 100.0, 100.0, 1.0, &[], &scene);

        // After the 28-byte header: atlas count (u32) + 7 section counts (u32 each)
        // = 28 + 4 + 7*4 = 60 bytes
        assert_eq!(bytes.len(), 60);

        // All counts should be zero
        let mut offset = 28;
        for _ in 0..8 {
            let count = u32::from_le_bytes([
                bytes[offset],
                bytes[offset + 1],
                bytes[offset + 2],
                bytes[offset + 3],
            ]);
            assert_eq!(count, 0, "expected zero count at offset {}", offset);
            offset += 4;
        }
    }
}
