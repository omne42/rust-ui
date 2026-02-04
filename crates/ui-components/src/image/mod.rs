pub mod logic;
pub mod motion;
pub mod styles;
mod view;

pub use logic::{ImageStatus, ImageViewState, resolve_view_state};
pub use motion::{ImageMotion, ImageMotionState, attach_zoom_motion, use_image_motion};
pub use view::Image;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ImageRadius {
    Sm,
    Md,
    #[default]
    Lg,
    Full,
}

impl ImageRadius {
    pub fn class_name(self) -> &'static str {
        match self {
            ImageRadius::Sm => "ui-image--radius-sm",
            ImageRadius::Md => "ui-image--radius-md",
            ImageRadius::Lg => "ui-image--radius-lg",
            ImageRadius::Full => "ui-image--radius-full",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ImageShadow {
    None,
    #[default]
    Sm,
    Md,
}

impl ImageShadow {
    pub fn class_name(self) -> &'static str {
        match self {
            ImageShadow::None => "ui-image--shadow-none",
            ImageShadow::Sm => "ui-image--shadow-sm",
            ImageShadow::Md => "ui-image--shadow-md",
        }
    }
}
