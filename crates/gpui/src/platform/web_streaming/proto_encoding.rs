//! Protobuf encoding for the web streaming wire protocol.
//!
//! This module bridges GPUI's scene types to the protobuf messages defined in
//! `proto/scene.proto`. It provides a `ProtoFrameEncoder` that converts a
//! `Scene` plus atlas tile data into a protobuf-encoded byte vector suitable
//! for sending as a binary WebSocket message.
//!
//! The generated protobuf types live in the `scene_proto` module, compiled by
//! `prost-build` in build.rs. All conversion logic is contained here so the
//! rest of the codebase only interacts with native GPUI types.

#[cfg(feature = "web-streaming")]
pub mod scene_proto {
    include!(concat!(env!("OUT_DIR"), "/zed.scene.rs"));
}

#[cfg(feature = "web-streaming")]
mod encoding {
    use prost::Message;

    use crate::scene::{
        MonochromeSprite, Path, PolychromeSprite, Quad, Scene, Shadow, SubpixelSprite, Underline,
    };
    use crate::{AtlasTile, Hsla, ScaledPixels};

    use super::super::mirroring_atlas::CachedTileData;
    use super::scene_proto;

    /// Encodes a complete frame into a protobuf byte vector.
    pub fn encode_frame(
        frame_id: u64,
        viewport_width: f32,
        viewport_height: f32,
        scale_factor: f32,
        atlas_tiles: &[CachedTileData],
        scene: &Scene,
    ) -> Vec<u8> {
        // Log the first few quad colors for debugging color pipeline issues.
        // Compare these values against the browser console output.
        if frame_id % 120 == 0 {
            if let Some(q) = scene.quads.first() {
                let s = &q.background.solid;
                log::info!(
                    "[web-streaming] frame {} bg quad[0] HSLA: h={:.4} s={:.4} l={:.4} a={:.4} | order={} tag={} bounds=({:.0},{:.0} {:.0}x{:.0})",
                    frame_id,
                    s.h,
                    s.s,
                    s.l,
                    s.a,
                    q.order,
                    q.background.tag as u32,
                    q.bounds.origin.x.0,
                    q.bounds.origin.y.0,
                    q.bounds.size.width.0,
                    q.bounds.size.height.0,
                );
            }
            for (i, q) in scene.quads.iter().take(5).enumerate().skip(1) {
                let s = &q.background.solid;
                log::debug!(
                    "[web-streaming] frame {} quad[{}] HSLA: h={:.4} s={:.4} l={:.4} a={:.4} | order={}",
                    frame_id,
                    i,
                    s.h,
                    s.s,
                    s.l,
                    s.a,
                    q.order,
                );
            }
            log::info!(
                "[web-streaming] frame {} scene: {} quads, {} shadows, {} mono_sprites, {} sub_sprites, {} atlas_tiles",
                frame_id,
                scene.quads.len(),
                scene.shadows.len(),
                scene.monochrome_sprites.len(),
                scene.subpixel_sprites.len(),
                atlas_tiles.len(),
            );
        }

        // Extract the background color from the largest quad in the scene.
        // The full-viewport background fill is not necessarily the first quad
        // in draw order -- it might be behind the title bar or other chrome.
        // We find the quad with the largest area, which is reliably the
        // full-window background.
        let bg_hsla = scene
            .quads
            .iter()
            .max_by(|a, b| {
                let area_a = a.bounds.size.width.0 * a.bounds.size.height.0;
                let area_b = b.bounds.size.width.0 * b.bounds.size.height.0;
                area_a
                    .partial_cmp(&area_b)
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .map(|q| q.background.solid);

        let background_color = bg_hsla.map(|c| convert_hsla(&c));

        // Build pre-converted theme hints so the browser can use CSS colors
        // directly without depending on the shader HSLA-to-RGBA pipeline.
        let theme_hints = bg_hsla.map(|hsla| {
            let rgba: crate::Rgba = hsla.into();
            let r8 = (rgba.r.clamp(0.0, 1.0) * 255.0).round() as u32;
            let g8 = (rgba.g.clamp(0.0, 1.0) * 255.0).round() as u32;
            let b8 = (rgba.b.clamp(0.0, 1.0) * 255.0).round() as u32;
            let rgb_int = (r8 << 16) | (g8 << 8) | b8;
            let css = format!("#{:06x}", rgb_int);

            let appearance = if hsla.l > 0.5 { "light" } else { "dark" };

            scene_proto::ThemeHints {
                appearance: appearance.to_string(),
                background_rgb: rgb_int,
                background_css: css,
                background_appearance: String::new(),
            }
        });

        let frame = scene_proto::FrameMessage {
            frame_id,
            viewport_width,
            viewport_height,
            scale_factor,
            atlas_entries: atlas_tiles.iter().map(convert_atlas_entry).collect(),
            scene: Some(convert_scene(scene)),
            background_color,
            theme_hints,
        };

        frame.encode_to_vec()
    }

    // -----------------------------------------------------------------------
    // Scene conversion
    // -----------------------------------------------------------------------

    fn convert_scene(scene: &Scene) -> scene_proto::SceneBody {
        scene_proto::SceneBody {
            shadows: scene.shadows.iter().map(convert_shadow).collect(),
            quads: scene.quads.iter().map(convert_quad).collect(),
            underlines: scene.underlines.iter().map(convert_underline).collect(),
            monochrome_sprites: scene
                .monochrome_sprites
                .iter()
                .map(convert_mono_sprite)
                .collect(),
            subpixel_sprites: scene
                .subpixel_sprites
                .iter()
                .map(convert_subpixel_sprite)
                .collect(),
            polychrome_sprites: scene
                .polychrome_sprites
                .iter()
                .map(convert_poly_sprite)
                .collect(),
            paths: scene.paths.iter().map(convert_path).collect(),
        }
    }

    // -----------------------------------------------------------------------
    // Primitive conversions
    // -----------------------------------------------------------------------

    fn convert_shadow(s: &Shadow) -> scene_proto::Shadow {
        scene_proto::Shadow {
            order: s.order,
            blur_radius: s.blur_radius.0,
            bounds: Some(convert_bounds_sp(&s.bounds)),
            corner_radii: Some(convert_corners_sp(&s.corner_radii)),
            content_mask: Some(convert_content_mask_sp(&s.content_mask)),
            color: Some(convert_hsla(&s.color)),
        }
    }

    fn convert_quad(q: &Quad) -> scene_proto::Quad {
        scene_proto::Quad {
            order: q.order,
            border_style: q.border_style as u32,
            bounds: Some(convert_bounds_sp(&q.bounds)),
            content_mask: Some(convert_content_mask_sp(&q.content_mask)),
            background: Some(convert_background(&q.background)),
            border_color: Some(convert_hsla(&q.border_color)),
            corner_radii: Some(convert_corners_sp(&q.corner_radii)),
            border_widths: Some(convert_edges_sp(&q.border_widths)),
        }
    }

    fn convert_underline(u: &Underline) -> scene_proto::Underline {
        scene_proto::Underline {
            order: u.order,
            bounds: Some(convert_bounds_sp(&u.bounds)),
            content_mask: Some(convert_content_mask_sp(&u.content_mask)),
            color: Some(convert_hsla(&u.color)),
            thickness: u.thickness.0,
            wavy: u.wavy != 0,
        }
    }

    fn convert_mono_sprite(s: &MonochromeSprite) -> scene_proto::MonochromeSprite {
        scene_proto::MonochromeSprite {
            order: s.order,
            bounds: Some(convert_bounds_sp(&s.bounds)),
            content_mask: Some(convert_content_mask_sp(&s.content_mask)),
            color: Some(convert_hsla(&s.color)),
            tile: Some(convert_atlas_tile(&s.tile)),
            transformation: Some(convert_transformation(&s.transformation)),
        }
    }

    fn convert_subpixel_sprite(s: &SubpixelSprite) -> scene_proto::SubpixelSprite {
        scene_proto::SubpixelSprite {
            order: s.order,
            bounds: Some(convert_bounds_sp(&s.bounds)),
            content_mask: Some(convert_content_mask_sp(&s.content_mask)),
            color: Some(convert_hsla(&s.color)),
            tile: Some(convert_atlas_tile(&s.tile)),
            transformation: Some(convert_transformation(&s.transformation)),
        }
    }

    fn convert_poly_sprite(s: &PolychromeSprite) -> scene_proto::PolychromeSprite {
        scene_proto::PolychromeSprite {
            order: s.order,
            grayscale: s.grayscale,
            opacity: s.opacity,
            bounds: Some(convert_bounds_sp(&s.bounds)),
            content_mask: Some(convert_content_mask_sp(&s.content_mask)),
            corner_radii: Some(convert_corners_sp(&s.corner_radii)),
            tile: Some(convert_atlas_tile(&s.tile)),
        }
    }

    fn convert_path(p: &Path<ScaledPixels>) -> scene_proto::Path {
        scene_proto::Path {
            order: p.order,
            bounds: Some(convert_bounds_sp(&p.bounds)),
            content_mask: Some(convert_content_mask_sp(&p.content_mask)),
            color: Some(convert_background(&p.color)),
            vertices: p.vertices.iter().map(|v| convert_path_vertex(v)).collect(),
        }
    }

    fn convert_path_vertex(v: &crate::scene::PathVertex<ScaledPixels>) -> scene_proto::PathVertex {
        scene_proto::PathVertex {
            xy_position: Some(scene_proto::Point {
                x: v.xy_position.x.0,
                y: v.xy_position.y.0,
            }),
            st_position: Some(scene_proto::Point {
                x: v.st_position.x,
                y: v.st_position.y,
            }),
            content_mask: Some(convert_content_mask_sp(&v.content_mask)),
        }
    }

    // -----------------------------------------------------------------------
    // Geometry conversions
    // -----------------------------------------------------------------------

    fn convert_bounds_sp(b: &crate::Bounds<ScaledPixels>) -> scene_proto::Bounds {
        scene_proto::Bounds {
            origin: Some(scene_proto::Point {
                x: b.origin.x.0,
                y: b.origin.y.0,
            }),
            size: Some(scene_proto::Size {
                width: b.size.width.0,
                height: b.size.height.0,
            }),
        }
    }

    fn convert_content_mask_sp(m: &crate::ContentMask<ScaledPixels>) -> scene_proto::ContentMask {
        scene_proto::ContentMask {
            bounds: Some(convert_bounds_sp(&m.bounds)),
        }
    }

    fn convert_corners_sp(c: &crate::Corners<ScaledPixels>) -> scene_proto::Corners {
        scene_proto::Corners {
            top_left: c.top_left.0,
            top_right: c.top_right.0,
            bottom_right: c.bottom_right.0,
            bottom_left: c.bottom_left.0,
        }
    }

    fn convert_edges_sp(e: &crate::Edges<ScaledPixels>) -> scene_proto::Edges {
        scene_proto::Edges {
            top: e.top.0,
            right: e.right.0,
            bottom: e.bottom.0,
            left: e.left.0,
        }
    }

    // -----------------------------------------------------------------------
    // Color conversions
    // -----------------------------------------------------------------------

    fn convert_hsla(c: &Hsla) -> scene_proto::Hsla {
        scene_proto::Hsla {
            h: c.h,
            s: c.s,
            l: c.l,
            a: c.a,
        }
    }

    fn convert_background(bg: &crate::Background) -> scene_proto::Background {
        scene_proto::Background {
            tag: bg.tag as u32,
            color_space: bg.color_space as u32,
            solid: Some(convert_hsla(&bg.solid)),
            gradient_angle_or_pattern_height: bg.gradient_angle_or_pattern_height,
            colors: vec![
                scene_proto::LinearColorStop {
                    color: Some(convert_hsla(&bg.colors[0].color)),
                    percentage: bg.colors[0].percentage,
                },
                scene_proto::LinearColorStop {
                    color: Some(convert_hsla(&bg.colors[1].color)),
                    percentage: bg.colors[1].percentage,
                },
            ],
        }
    }

    // -----------------------------------------------------------------------
    // Atlas conversions
    // -----------------------------------------------------------------------

    fn convert_atlas_tile(tile: &AtlasTile) -> scene_proto::AtlasTile {
        scene_proto::AtlasTile {
            texture_id: Some(scene_proto::AtlasTextureId {
                index: tile.texture_id.index,
                kind: tile.texture_id.kind as u32,
            }),
            tile_id: tile.tile_id.0,
            padding: tile.padding,
            bounds: Some(scene_proto::AtlasBounds {
                origin_x: tile.bounds.origin.x.0,
                origin_y: tile.bounds.origin.y.0,
                width: tile.bounds.size.width.0,
                height: tile.bounds.size.height.0,
            }),
        }
    }

    fn convert_atlas_entry(tile: &CachedTileData) -> scene_proto::AtlasEntry {
        scene_proto::AtlasEntry {
            texture_id: Some(scene_proto::AtlasTextureId {
                index: tile.texture_id.index,
                kind: tile.texture_id.kind as u32,
            }),
            bounds: Some(scene_proto::AtlasBounds {
                origin_x: tile.bounds.origin.x.0,
                origin_y: tile.bounds.origin.y.0,
                width: tile.bounds.size.width.0,
                height: tile.bounds.size.height.0,
            }),
            format: tile.format as u32,
            pixel_data: tile.bytes.clone(),
        }
    }

    // -----------------------------------------------------------------------
    // Transformation
    // -----------------------------------------------------------------------

    fn convert_transformation(
        t: &crate::scene::TransformationMatrix,
    ) -> scene_proto::TransformationMatrix {
        scene_proto::TransformationMatrix {
            r00: t.rotation_scale[0][0],
            r01: t.rotation_scale[0][1],
            r10: t.rotation_scale[1][0],
            r11: t.rotation_scale[1][1],
            tx: t.translation[0],
            ty: t.translation[1],
        }
    }
}

#[cfg(feature = "web-streaming")]
pub use encoding::encode_frame;

// Stub when the feature is not enabled.
#[cfg(not(feature = "web-streaming"))]
pub fn encode_frame(
    _frame_id: u64,
    _viewport_width: f32,
    _viewport_height: f32,
    _scale_factor: f32,
    _atlas_tiles: &[super::mirroring_atlas::CachedTileData],
    _scene: &crate::Scene,
) -> Vec<u8> {
    Vec::new()
}
