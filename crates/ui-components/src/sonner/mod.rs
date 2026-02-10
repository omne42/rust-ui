mod logic;
pub mod styles;
mod view;

pub use logic::{DEFAULT_ARIA_LABEL, DEFAULT_MAX_TOASTS, DEFAULT_PORTAL};
pub use view::Sonner;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum SonnerPosition {
    TopLeft,
    TopCenter,
    TopRight,
    BottomLeft,
    BottomCenter,
    #[default]
    BottomRight,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SonnerSlot {
    Root,
    Viewport,
}

impl SonnerSlot {
    pub fn as_attr(self) -> &'static str {
        match self {
            SonnerSlot::Root => "sonner",
            SonnerSlot::Viewport => "sonner-viewport",
        }
    }

    pub fn base_class(self) -> &'static str {
        match self {
            SonnerSlot::Root => "ui-sonner",
            SonnerSlot::Viewport => "ui-sonner__viewport",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SonnerStoreSource {
    Provided,
    Context,
    Local,
}

impl SonnerStoreSource {
    pub fn as_attr(self) -> &'static str {
        match self {
            SonnerStoreSource::Provided => "provided",
            SonnerStoreSource::Context => "context",
            SonnerStoreSource::Local => "local",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SonnerPartStateInput {
    pub slot: SonnerSlot,
    pub position: SonnerPosition,
    pub portal: bool,
    pub max_toasts: usize,
    pub has_custom_position: bool,
    pub has_custom_portal: bool,
    pub has_custom_max_toasts: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
    pub has_custom_motion: bool,
    pub store_source: SonnerStoreSource,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SonnerPartState {
    pub slot: SonnerSlot,
    pub slot_attr: &'static str,
    pub base_class: &'static str,
    pub state_attr: &'static str,
    pub queue_attr: &'static str,
    pub position: SonnerPosition,
    pub position_attr: &'static str,
    pub portal: bool,
    pub portal_attr: &'static str,
    pub max_toasts: usize,
    pub has_custom_position: bool,
    pub has_custom_portal: bool,
    pub has_custom_max_toasts: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
    pub has_custom_motion: bool,
    pub position_source_attr: &'static str,
    pub portal_source_attr: &'static str,
    pub max_toasts_source_attr: &'static str,
    pub aria_source_attr: &'static str,
    pub class_source_attr: &'static str,
    pub motion_source_attr: &'static str,
    pub store_source: SonnerStoreSource,
    pub store_source_attr: &'static str,
}
