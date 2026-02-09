mod logic;
pub mod styles;
mod view;

pub use logic::{DEFAULT_ARIA_LABEL, NativeSelectSize};
pub use view::NativeSelect;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NativeSelectOption {
    pub value: String,
    pub label: String,
    pub disabled: bool,
}

impl NativeSelectOption {
    pub fn new(value: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
            disabled: false,
        }
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NativeSelectOptionResolved {
    pub id: String,
    pub index: usize,
    pub value: String,
    pub label: String,
    pub disabled: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NativeSelectStateInput<'a> {
    pub disabled: bool,
    pub invalid: bool,
    pub required: bool,
    pub has_placeholder: bool,
    pub selected_index: Option<usize>,
    pub options: &'a [NativeSelectOptionResolved],
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
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
