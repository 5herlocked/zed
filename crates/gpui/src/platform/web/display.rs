use crate::{Bounds, DisplayId, Pixels, PlatformDisplay, Point, Size, px};
use anyhow::Result;

#[derive(Debug)]
pub(crate) struct WebDisplay {
    id: DisplayId,
    uuid: uuid::Uuid,
    bounds: Bounds<Pixels>,
}

impl Default for WebDisplay {
    fn default() -> Self {
        Self {
            id: DisplayId(1),
            uuid: uuid::Uuid::nil(),
            bounds: Bounds::from_corners(Point::default(), Point::new(px(1920.), px(1080.))),
        }
    }
}

impl WebDisplay {
    /// Read the display size from the browser window, falling back to defaults.
    pub fn from_browser() -> Self {
        let (width, height) = web_sys::window()
            .map(|w| {
                let width = w.inner_width().ok().and_then(|v| v.as_f64()).unwrap_or(1920.0);
                let height = w.inner_height().ok().and_then(|v| v.as_f64()).unwrap_or(1080.0);
                (width as f32, height as f32)
            })
            .unwrap_or((1920.0, 1080.0));

        Self {
            id: DisplayId(1),
            uuid: uuid::Uuid::nil(),
            bounds: Bounds {
                origin: Point::default(),
                size: Size {
                    width: px(width),
                    height: px(height),
                },
            },
        }
    }
}

impl PlatformDisplay for WebDisplay {
    fn id(&self) -> DisplayId {
        self.id
    }

    fn uuid(&self) -> Result<uuid::Uuid> {
        Ok(self.uuid)
    }

    fn bounds(&self) -> Bounds<Pixels> {
        self.bounds
    }
}
