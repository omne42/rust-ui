pub use ui_state_primitives::native_select::{
    NativeSelectOption, NativeSelectOptionResolved, NativeSelectStateInput,
};

pub const DEFAULT_ARIA_LABEL: &str;

pub enum NativeSelectSize {
    Sm,
    Md,
    Lg,
}

pub struct NativeSelectState {
    pub size_class: &'static str,
    pub size_attr: &'static str,
    pub is_disabled: bool,
    pub control_disabled: bool,
    pub is_invalid: bool,
    pub is_required: bool,
    pub has_placeholder: bool,
    pub is_empty: bool,
    pub has_options: bool,
    pub option_count: usize,
    pub selected_index: Option<usize>,
    pub selected_value: Option<String>,
    pub has_selection: bool,
    pub has_disabled_options: bool,
    pub has_enabled_options: bool,
    pub disabled_option_count: usize,
    pub data_state_attr: &'static str,
    pub aria_source_attr: &'static str,
    pub class_source_attr: &'static str,
    pub has_custom_class_name: bool,
}

pub fn NativeSelect(
    id_base: String,
    options: Vec<NativeSelectOption>,
    selected_index: Option<leptos::prelude::Signal<Option<usize>>>,
    default_selected_index: Option<usize>,
    on_selected_index_change: Option<leptos::prelude::Callback<Option<usize>>>,
    is_disabled: bool,
    is_required: bool,
    is_invalid: bool,
    size: NativeSelectSize,
    name: Option<String>,
    aria_label: Option<String>,
    placeholder: Option<String>,
    class_name: Option<String>,
    lang: Option<String>,
    dir: Option<ui_headless::A11yDirection>,
) -> impl leptos::prelude::IntoView;
