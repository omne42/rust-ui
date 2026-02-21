pub type FieldsetOrientation = ui_state_primitives::fieldset::FieldsetOrientation;
pub type FieldsetTone = ui_state_primitives::fieldset::FieldsetTone;
pub type FieldsetDataState = ui_state_primitives::fieldset::FieldsetDataState;

pub struct FieldsetMotion {
    pub duration_ms: f64,
    pub distance_px: f64,
    pub stiffness: f64,
    pub damping: f64,
}

pub fn sanitize_motion(
    motion: crate::motion::FieldsetMotion,
) -> crate::motion::FieldsetMotion;

pub fn attach_motion(motion: crate::motion::FieldsetMotion) -> String;

pub fn Fieldset(
    children: leptos::children::Children,
    orientation: crate::FieldsetOrientation,
    tone: crate::FieldsetTone,
    is_required: Option<bool>,
    default_is_required: Option<bool>,
    on_is_required_change: Option<leptos::prelude::Callback<bool>>,
    is_disabled: Option<bool>,
    default_is_disabled: Option<bool>,
    on_is_disabled_change: Option<leptos::prelude::Callback<bool>>,
    is_invalid: Option<bool>,
    default_is_invalid: Option<bool>,
    on_is_invalid_change: Option<leptos::prelude::Callback<bool>>,
    legend: Option<String>,
    description: Option<String>,
    error_message: Option<String>,
    actions: Option<leptos::children::ViewFn>,
    aria_label: Option<String>,
    class_name: Option<String>,
    lang: Option<String>,
    dir: Option<ui_headless::A11yDirection>,
    motion: crate::FieldsetMotion,
) -> impl leptos::prelude::IntoView;
