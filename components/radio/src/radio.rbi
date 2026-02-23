pub type A11yDirection = ui_headless::A11yDirection;

pub enum RadioGroupOrientation {
    Vertical,
    Horizontal,
}

pub struct RadioMotion {
    pub spring: ui_motion::spring::SpringConfig,
    pub hover_scale: f64,
    pub tap_scale: f64,
}

pub fn sanitize_motion(motion: RadioMotion) -> RadioMotion;

pub fn attach_motion(
    node_ref: leptos::prelude::NodeRef<leptos::html::Button>,
    is_hovered: leptos::prelude::ReadSignal<bool>,
    is_pressed: leptos::prelude::ReadSignal<bool>,
    is_disabled: bool,
    motion: RadioMotion,
);

pub fn RadioGroup(
    id_base: String,
    options: Vec<String>,
    selected_index: leptos::prelude::ReadSignal<Option<usize>>,
    set_selected_index: leptos::prelude::WriteSignal<Option<usize>>,
    is_disabled: Option<bool>,
    disabled: bool,
    disabled_indices: Vec<usize>,
    orientation: RadioGroupOrientation,
    label: Option<String>,
    aria_label: Option<String>,
    aria_labelledby: Option<String>,
    lang: Option<String>,
    dir: Option<A11yDirection>,
    motion: RadioMotion,
    class_name: Option<String>,
) -> impl leptos::prelude::IntoView;

pub fn Radio(
    id: String,
    label: String,
    is_checked: Option<leptos::prelude::Signal<bool>>,
    checked: Option<leptos::prelude::Signal<bool>>,
    default_checked: Option<bool>,
    is_disabled: Option<bool>,
    disabled: bool,
    motion: RadioMotion,
    class_name: Option<String>,
    on_checked_change: Option<leptos::prelude::Callback<bool>>,
    on_change: Option<leptos::prelude::Callback<bool>>,
) -> impl leptos::prelude::IntoView;
