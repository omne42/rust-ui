pub use crate::motion::ComboBoxMotion;

pub struct ComboBoxMotion {
    pub popover: crate::motion::PopoverMotion,
    pub highlight: ui_visual_primitive::active_highlight::ActiveHighlightMotion,
}

pub fn sanitize_motion(
    motion: crate::motion::ComboBoxMotion,
) -> crate::motion::ComboBoxMotion;

pub fn sanitize_popover_motion(
    motion: crate::motion::PopoverMotion,
) -> crate::motion::PopoverMotion;

pub fn ComboBox(
    id_base: String,
    label: String,
    items: Vec<String>,
    selected_index: leptos::prelude::ReadSignal<Option<usize>>,
    set_selected_index: leptos::prelude::WriteSignal<Option<usize>>,
    is_disabled: Option<bool>,
    disabled_indices: Vec<usize>,
    is_required: Option<leptos::prelude::Signal<bool>>,
    is_invalid: Option<leptos::prelude::Signal<bool>>,
    aria_describedby: leptos::prelude::Signal<Option<String>>,
    description: Option<String>,
    error: Option<String>,
    placeholder: Option<String>,
    empty_message: Option<String>,
    toggle_button_aria_label: Option<String>,
    is_open: Option<leptos::prelude::Signal<bool>>,
    default_open: Option<bool>,
    on_open_change: Option<leptos::prelude::Callback<bool>>,
    lang: Option<String>,
    dir: Option<ui_headless::A11yDirection>,
    motion: crate::motion::ComboBoxMotion,
    class_name: Option<String>,
) -> impl leptos::prelude::IntoView;
