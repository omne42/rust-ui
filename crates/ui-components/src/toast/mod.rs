mod logic;
pub mod motion;
pub mod styles;
mod view;

pub use logic::{
    DEFAULT_TITLE, DEFAULT_VIEWPORT_MAX_TOASTS, DEFAULT_VIEWPORT_PORTAL, ToastInstance,
    ToastOptions, ToastStore, ToastStoreOptions, ToastVariant, provide_toast_store,
    use_toast_store,
};
pub use motion::ToastMotion;
pub use view::{Toast, ToastViewport};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ToastSlot {
    Root,
    Content,
    Title,
    Description,
    Close,
}

impl ToastSlot {
    pub fn as_attr(self) -> &'static str {
        match self {
            ToastSlot::Root => "toast",
            ToastSlot::Content => "toast-content",
            ToastSlot::Title => "toast-title",
            ToastSlot::Description => "toast-description",
            ToastSlot::Close => "toast-close",
        }
    }

    pub fn base_class(self) -> &'static str {
        match self {
            ToastSlot::Root => "ui-toast",
            ToastSlot::Content => "ui-toast__content",
            ToastSlot::Title => "ui-toast__title",
            ToastSlot::Description => "ui-toast__description",
            ToastSlot::Close => "ui-toast__close",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ToastPartStateInput {
    pub slot: ToastSlot,
    pub variant: ToastVariant,
    pub is_open: bool,
    pub has_description: bool,
    pub has_custom_id: bool,
    pub has_custom_description: bool,
    pub has_custom_class_name: bool,
    pub has_custom_motion: bool,
    pub has_custom_on_close: bool,
    pub has_custom_on_exit_complete: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ToastPartState {
    pub slot: ToastSlot,
    pub slot_attr: &'static str,
    pub base_class: &'static str,
    pub state_attr: &'static str,
    pub variant: ToastVariant,
    pub variant_attr: &'static str,
    pub description_attr: &'static str,
    pub close_mode_attr: &'static str,
    pub open_attr: Option<&'static str>,
    pub is_open: bool,
    pub has_description: bool,
    pub has_custom_id: bool,
    pub has_custom_description: bool,
    pub has_custom_class_name: bool,
    pub has_custom_motion: bool,
    pub has_custom_on_close: bool,
    pub has_custom_on_exit_complete: bool,
    pub id_source_attr: &'static str,
    pub description_source_attr: &'static str,
    pub class_source_attr: &'static str,
    pub motion_source_attr: &'static str,
    pub close_source_attr: &'static str,
    pub exit_source_attr: &'static str,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ToastViewportSlot {
    Root,
}

impl ToastViewportSlot {
    pub fn as_attr(self) -> &'static str {
        match self {
            ToastViewportSlot::Root => "toast-viewport",
        }
    }

    pub fn base_class(self) -> &'static str {
        match self {
            ToastViewportSlot::Root => "ui-toast-viewport",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ToastStoreSource {
    Provided,
    Context,
    Local,
}

impl ToastStoreSource {
    pub fn as_attr(self) -> &'static str {
        match self {
            ToastStoreSource::Provided => "provided",
            ToastStoreSource::Context => "context",
            ToastStoreSource::Local => "local",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ToastViewportStateInput {
    pub slot: ToastViewportSlot,
    pub portal: bool,
    pub max_toasts: usize,
    pub has_custom_portal: bool,
    pub has_custom_max_toasts: bool,
    pub has_custom_class_name: bool,
    pub has_custom_motion: bool,
    pub store_source: ToastStoreSource,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ToastViewportState {
    pub slot: ToastViewportSlot,
    pub slot_attr: &'static str,
    pub base_class: &'static str,
    pub state_attr: &'static str,
    pub queue_attr: &'static str,
    pub portal_attr: &'static str,
    pub max_toasts: usize,
    pub portal: bool,
    pub has_custom_portal: bool,
    pub has_custom_max_toasts: bool,
    pub has_custom_class_name: bool,
    pub has_custom_motion: bool,
    pub portal_source_attr: &'static str,
    pub max_toasts_source_attr: &'static str,
    pub class_source_attr: &'static str,
    pub motion_source_attr: &'static str,
    pub store_source: ToastStoreSource,
    pub store_source_attr: &'static str,
}
