pub(crate) mod logic;
pub mod styles;
mod view;

pub use logic::{
    DEFAULT_ARIA_LABEL, DEFAULT_ERROR_MESSAGE, DEFAULT_LABEL, FormFieldIndicatorPlacement,
    FormFieldIndicatorVariant, FormFieldTone,
};
pub use view::FormField;

#[cfg(all(test, not(feature = "component-form_field")))]
#[path = "../test/semantics.rs"]
mod semantics_tests;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct FormFieldStateInput {
    pub is_selected: bool,
    pub is_disabled: bool,
    pub is_invalid: bool,
    pub tone: FormFieldTone,
    pub indicator_variant: FormFieldIndicatorVariant,
    pub indicator_placement: FormFieldIndicatorPlacement,
    pub has_description: bool,
    pub has_error_message: bool,
    pub has_custom_label: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_error_message: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct FormFieldState {
    pub is_selected: bool,
    pub is_unselected: bool,
    pub is_disabled: bool,
    pub is_invalid: bool,
    pub tone: FormFieldTone,
    pub tone_class: &'static str,
    pub tone_attr: &'static str,
    pub indicator_variant: FormFieldIndicatorVariant,
    pub indicator_variant_class: &'static str,
    pub indicator_variant_attr: &'static str,
    pub indicator_placement: FormFieldIndicatorPlacement,
    pub indicator_placement_class: &'static str,
    pub indicator_placement_attr: &'static str,
    pub has_description: bool,
    pub has_error_message: bool,
    pub shows_error: bool,
    pub message_kind_attr: &'static str,
    pub state_attr: &'static str,
    pub label_source_attr: &'static str,
    pub aria_source_attr: &'static str,
    pub error_source_attr: &'static str,
    pub class_source_attr: &'static str,
    pub has_custom_class_name: bool,
}
