mod dispatcher;
mod display;
mod display_link;
mod events;
mod screen_capture;

use media::core_video::CVImageBuffer;

#[cfg(feature = "macos-blade")]
use crate::platform::blade as renderer;

mod attributed_string;

#[cfg(feature = "font-kit")]
mod open_type;

#[cfg(feature = "font-kit")]
mod text_system;

mod platform;
mod window;
mod window_appearance;