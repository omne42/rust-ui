pub use crate::motion::ErrorViewMotion;
pub use ui_state_primitives::error_view::{ErrorViewState, ErrorViewStateInput, ErrorViewTone};

pub const DEFAULT_ARIA_LABEL: &str;
pub const DEFAULT_MESSAGE: &str;

pub fn sanitize_motion(
    motion: crate::motion::ErrorViewMotion,
) -> crate::motion::ErrorViewMotion;

pub fn attach_motion(
    node_ref: leptos::prelude::NodeRef<leptos::html::Div>,
    visible: leptos::prelude::Signal<bool>,
    motion: crate::motion::ErrorViewMotion,
);

pub fn ErrorView(
    is_invalid: bool,
    tone: Option<crate::ErrorViewTone>,
    is_compact: Option<bool>,
    is_bordered: Option<bool>,
    motion: crate::ErrorViewMotion,
    message: Option<String>,
    aria_label: Option<String>,
    class_name: Option<String>,
    icon: Option<leptos::children::ViewFn>,
    actions: Option<leptos::children::ViewFn>,
    lang: Option<String>,
    dir: Option<ui_headless::A11yDirection>,
    children: Option<leptos::children::Children>,
) -> impl leptos::prelude::IntoView;
