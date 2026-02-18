mod dispatcher;
mod display;
mod platform;
mod window;

pub(crate) use display::*;
pub(crate) use platform::*;
pub(crate) use window::*;

pub(crate) type PlatformScreenCaptureFrame = ();
