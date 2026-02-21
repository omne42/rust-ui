pub type ColorFieldStateInput = ui_state_primitives::color_field::ColorFieldStateInput;
pub type ColorFieldState = ui_state_primitives::color_field::ColorFieldState;
pub type ColorFieldVisualState = ui_state_primitives::color_field::ColorFieldVisualState;

pub fn ColorField(
    id_base: String,
    label: Option<String>,
    placeholder: Option<String>,
    is_disabled: Option<bool>,
    disabled: Option<bool>,
    value: Option<leptos::prelude::Signal<Option<String>>>,
    default_value: Option<String>,
    on_value_change: Option<leptos::prelude::Callback<Option<String>>>,
    is_preview_visible: Option<bool>,
    show_preview: Option<bool>,
    aria_label: Option<String>,
    class_name: Option<String>,
    lang: Option<String>,
    dir: Option<ui_headless::A11yDirection>,
) -> impl leptos::prelude::IntoView;
