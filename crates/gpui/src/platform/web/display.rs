use crate::{Bounds, DisplayId, Pixels, PlatformDisplay, Point, px};
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
