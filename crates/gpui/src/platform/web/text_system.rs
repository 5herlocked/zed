use crate::{
    Bounds, DevicePixels, Font, FontId, FontMetrics, FontRun, GlyphId, LineLayout, Pixels,
    PlatformTextSystem, Point, RenderGlyphParams, ShapedGlyph, ShapedRun, Size, TextRenderingMode,
    point, px, size,
};
use anyhow::Result;
use collections::HashMap;
use parking_lot::Mutex;
use std::borrow::Cow;

struct WebTextSystemState {
    fonts: Vec<WebFont>,
    font_name_to_id: HashMap<String, FontId>,
    next_font_id: usize,
}

struct WebFont {
    family: String,
    weight: u16,
    italic: bool,
}

impl WebFont {
    fn css_font_string(&self, size_px: f32) -> String {
        let style = if self.italic { "italic" } else { "normal" };
        format!("{style} {} {size_px}px {}", self.weight, self.family)
    }
}

pub(crate) struct WebTextSystem {
    state: Mutex<WebTextSystemState>,
}

// SAFETY: WASM is single-threaded.
unsafe impl Send for WebTextSystem {}
unsafe impl Sync for WebTextSystem {}

impl WebTextSystem {
    pub fn new() -> Self {
        let mut state = WebTextSystemState {
            fonts: Vec::new(),
            font_name_to_id: HashMap::default(),
            next_font_id: 0,
        };

        // Register a default monospace font so that font_id lookups don't fail
        // before any fonts are explicitly added.
        let default_id = FontId(state.next_font_id);
        state.next_font_id += 1;
        state.fonts.push(WebFont {
            family: "monospace".to_string(),
            weight: 400,
            italic: false,
        });
        state
            .font_name_to_id
            .insert("monospace".to_string(), default_id);

        Self {
            state: Mutex::new(state),
        }
    }

    fn measure_text(font_css: &str, text: &str) -> f64 {
        let document = web_sys::window()
            .and_then(|w| w.document())
            .expect("no document");
        let canvas = document
            .create_element("canvas")
            .expect("failed to create canvas")
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .expect("not a canvas");
        let context = canvas
            .get_context("2d")
            .expect("failed to get 2d context")
            .expect("no 2d context")
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .expect("not a CanvasRenderingContext2d");
        context.set_font(font_css);
        context
            .measure_text(text)
            .map(|m| m.width())
            .unwrap_or(0.0)
    }
}

use wasm_bindgen::JsCast;

impl PlatformTextSystem for WebTextSystem {
    fn add_fonts(&self, _fonts: Vec<Cow<'static, [u8]>>) -> Result<()> {
        // In a full implementation, we'd parse font files and register them
        // via CSS @font-face. For now, accept and ignore.
        Ok(())
    }

    fn all_font_names(&self) -> Vec<String> {
        let state = self.state.lock();
        state.font_name_to_id.keys().cloned().collect()
    }

    fn font_id(&self, descriptor: &Font) -> Result<FontId> {
        let mut state = self.state.lock();
        let family = descriptor.family.to_string();

        if let Some(&id) = state.font_name_to_id.get(&family) {
            return Ok(id);
        }

        let id = FontId(state.next_font_id);
        state.next_font_id += 1;
        state.fonts.push(WebFont {
            family: family.clone(),
            weight: descriptor.weight.0 as u16,
            italic: descriptor.style == crate::FontStyle::Italic,
        });
        state.font_name_to_id.insert(family, id);
        Ok(id)
    }

    fn font_metrics(&self, _font_id: FontId) -> FontMetrics {
        // Approximate metrics for a typical sans-serif font at 1000 units-per-em.
        // Canvas2D doesn't expose full font metrics, so we use reasonable defaults.
        FontMetrics {
            units_per_em: 1000,
            ascent: 900.0,
            descent: -250.0,
            line_gap: 0.0,
            underline_position: -100.0,
            underline_thickness: 50.0,
            cap_height: 700.0,
            x_height: 500.0,
            bounding_box: Bounds {
                origin: Point {
                    x: -200.0,
                    y: -250.0,
                },
                size: Size {
                    width: 1200.0,
                    height: 1250.0,
                },
            },
        }
    }

    fn typographic_bounds(&self, _font_id: FontId, _glyph_id: GlyphId) -> Result<Bounds<f32>> {
        Ok(Bounds {
            origin: Point { x: 0.0, y: 0.0 },
            size: size(500.0, 700.0),
        })
    }

    fn advance(&self, _font_id: FontId, _glyph_id: GlyphId) -> Result<Size<f32>> {
        Ok(size(500.0, 0.0))
    }

    fn glyph_for_char(&self, _font_id: FontId, ch: char) -> Option<GlyphId> {
        Some(GlyphId(ch as u32))
    }

    fn glyph_raster_bounds(&self, _params: &RenderGlyphParams) -> Result<Bounds<DevicePixels>> {
        Ok(Bounds::default())
    }

    fn rasterize_glyph(
        &self,
        _params: &RenderGlyphParams,
        raster_bounds: Bounds<DevicePixels>,
    ) -> Result<(Size<DevicePixels>, Vec<u8>)> {
        // Glyph rasterization is not needed for the web platform since
        // the browser handles text rendering natively.
        Ok((raster_bounds.size, Vec::new()))
    }

    fn layout_line(&self, text: &str, font_size: Pixels, runs: &[FontRun]) -> LineLayout {
        let state = self.state.lock();

        let metrics = self.font_metrics(FontId(0));
        let scale = font_size.0 / metrics.units_per_em as f32;

        let mut shaped_runs = Vec::new();
        let mut glyphs = Vec::new();
        let mut position = px(0.0);
        let mut byte_offset = 0usize;

        for run in runs {
            let font = state.fonts.get(run.font_id.0).unwrap_or(&state.fonts[0]);
            let css_font = font.css_font_string(font_size.0);

            let run_end = (byte_offset + run.len).min(text.len());
            let run_text = &text[byte_offset..run_end];

            let run_width = Self::measure_text(&css_font, run_text) as f32;
            let char_count = run_text.chars().count().max(1);
            let char_width = run_width / char_count as f32;

            let mut run_glyphs = Vec::new();
            for (char_index, (index, ch)) in run_text.char_indices().enumerate() {
                let glyph_id = GlyphId(ch as u32);
                run_glyphs.push(ShapedGlyph {
                    id: glyph_id,
                    position: point(position + px(char_index as f32 * char_width), px(0.0)),
                    index: byte_offset + index,
                    is_emoji: false,
                });
            }

            if !run_glyphs.is_empty() {
                glyphs.extend(run_glyphs.iter().cloned());
                shaped_runs.push(ShapedRun {
                    font_id: run.font_id,
                    glyphs: run_glyphs,
                });
            }

            position += px(run_width);
            byte_offset = run_end;
        }

        // If no runs were provided, measure the whole text with the default font.
        if runs.is_empty() && !text.is_empty() {
            let font = &state.fonts[0];
            let css_font = font.css_font_string(font_size.0);
            let total_width = Self::measure_text(&css_font, text) as f32;
            let char_count = text.chars().count().max(1);
            let char_width = total_width / char_count as f32;

            for (index, ch) in text.char_indices() {
                let glyph_id = GlyphId(ch as u32);
                let char_pos = text[..index].chars().count();
                glyphs.push(ShapedGlyph {
                    id: glyph_id,
                    position: point(px(char_pos as f32 * char_width), px(0.0)),
                    index,
                    is_emoji: false,
                });
            }

            if !glyphs.is_empty() {
                shaped_runs.push(ShapedRun {
                    font_id: FontId(0),
                    glyphs: glyphs.clone(),
                });
            }

            position = px(total_width);
        }

        LineLayout {
            font_size,
            width: position,
            ascent: px(metrics.ascent * scale),
            descent: px(metrics.descent * scale),
            runs: shaped_runs,
            len: text.len(),
        }
    }

    fn recommended_rendering_mode(
        &self,
        _font_id: FontId,
        _font_size: Pixels,
    ) -> TextRenderingMode {
        TextRenderingMode::Grayscale
    }
}
