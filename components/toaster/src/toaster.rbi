pub type ToasterPartState = ui_state_primitives::toaster::ToasterPartState;
pub type ToasterPartStateInput = ui_state_primitives::toaster::ToasterPartStateInput;
pub type ToasterPosition = ui_state_primitives::toaster::ToasterPosition;
pub type ToasterSlot = ui_state_primitives::toaster::ToasterSlot;
pub type ToasterStoreSource = ui_state_primitives::toaster::ToasterStoreSource;
pub type A11yDirection = ui_headless::A11yDirection;
pub type ToastMotion = crate::toast::ToastMotion;
pub type ToastStore = crate::toast::ToastStore;

pub const DEFAULT_ARIA_LABEL: &str;
pub const DEFAULT_MAX_TOASTS: usize;
pub const DEFAULT_PORTAL: bool;

pub fn Toaster(
    position: ToasterPosition,
    portal: bool,
    max_toasts: usize,
    aria_label: Option<String>,
    class_name: Option<String>,
    lang: Option<String>,
    dir: Option<A11yDirection>,
    motion: ToastMotion,
    store: Option<ToastStore>,
) -> impl leptos::prelude::IntoView;
