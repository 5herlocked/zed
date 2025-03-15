use crate::{px, size, Bounds, DisplayId, Pixels, PlatformDisplay};
use anyhow::Result;
use std::sync::atomic::{AtomicU32, Ordering};
use core_graphics::base::CGFloat;
use core_graphics::geometry::CGRect;
use objc2::{class, msg_send};
use objc2::ffi::id;
use objc2::rc::Retained;
use objc2_ui_kit::{UIScreen, UIView, UIWindow};
use uuid::Uuid;

#[derive(Debug)]
pub(crate) struct IosDisplay(pub(crate) UIWindow);

static DISPLAY_ID_COUNTER: AtomicU32 = AtomicU32::new(0);

unsafe impl Send for IosDisplay {}

impl IosDisplay {
    pub fn find_by_id(id: DisplayId) -> Option<Self> {
        if id.0 == 0 {
            Some(Self::primary())
        } else {
            None
        }
    }

    pub fn primary() -> Self {
        unsafe {
            let screens = UI
        }
    }

    pub fn all() -> impl Iterator<Item = Self> {
        // iOS only has one display, the device screen
        // TODO: MacCatalyst things to get it to support multiple screens
        std::iter::once(Self::primary())
    }
}

impl PlatformDisplay for IosDisplay {
    fn id(&self) -> DisplayId {
        DisplayId(self.0.)
    }

    fn uuid(&self) -> Result<Uuid> {
        unsafe {
            // Generate a stable UUID for the display
            let id_str = format!("ios-display-{}", self.0);
            let bytes = id_str.as_bytes();
            let mut uuid_bytes = [0u8; 16];

            // Copy bytes from the id_str, or use zeros if not enough
            for (i, &b) in bytes.iter().take(16).enumerate() {
                uuid_bytes[i] = b;
            }

            Ok(Uuid::from_bytes(uuid_bytes))
        }
    }

    fn bounds(&self) -> Bounds<Pixels> {
        unsafe {
            let bounds = UIScreen::bounds();
            Bounds {
                origin: Default::default(),
                size: size(px(width as f32), px(height as f32)),
            }
        }
    }
}
