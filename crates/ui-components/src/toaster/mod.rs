mod logic;
pub mod styles;
mod view;

pub use logic::{DEFAULT_ARIA_LABEL, DEFAULT_MAX_TOASTS, DEFAULT_PORTAL};
pub use view::Toaster;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ToasterPosition {
    TopLeft,
    TopCenter,
    TopRight,
    BottomLeft,
    BottomCenter,
    #[default]
    BottomRight,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ToasterSlot {
    Root,
    Sonner,
}

impl ToasterSlot {
    pub fn as_attr(self) -> &'static str {
        match self {
            ToasterSlot::Root => "toaster",
            ToasterSlot::Sonner => "toaster-sonner",
        }
    }

    pub fn base_class(self) -> &'static str {
        match self {
            ToasterSlot::Root => "ui-toaster",
            ToasterSlot::Sonner => "ui-toaster__sonner",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ToasterStoreSource {
    Provided,
    Context,
    Local,
}

impl ToasterStoreSource {
    pub fn as_attr(self) -> &'static str {
        match self {
            ToasterStoreSource::Provided => "provided",
            ToasterStoreSource::Context => "context",
            ToasterStoreSource::Local => "local",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ToasterPartStateInput {
    pub slot: ToasterSlot,
    pub position: ToasterPosition,
    pub portal: bool,
    pub max_toasts: usize,
    pub has_custom_position: bool,
    pub has_custom_portal: bool,
    pub has_custom_max_toasts: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
    pub has_custom_motion: bool,
    pub store_source: ToasterStoreSource,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ToasterPartState {
    pub slot: ToasterSlot,
    pub slot_attr: &'static str,
    pub base_class: &'static str,
    pub state_attr: &'static str,
    pub queue_attr: &'static str,
    pub position: ToasterPosition,
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
    pub store_source: ToasterStoreSource,
    pub store_source_attr: &'static str,
}
