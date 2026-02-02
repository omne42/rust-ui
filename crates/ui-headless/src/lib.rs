//! `ui-headless` — interaction & accessibility primitives (React Aria analogue).

#[cfg(all(feature = "web", feature = "ssr"))]
compile_error!("features `web` and `ssr` are mutually exclusive; enable exactly one");

pub mod button;
pub mod focus_visible;
pub mod modality;
pub mod press;

pub use button::{use_button, ButtonAria, ButtonHandlers, ButtonOptions};
pub use focus_visible::{provide_focus_visible, use_focus_visible, FocusVisibleState};
pub use modality::Modality;
pub use press::{use_press, OnPress, PressHandlers, PressOptions, PressState};
