use crate::a11y::{A11yDirection, locale_attrs};
use ui_state_primitives::field::FieldState;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct FieldHandlers;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FieldAttrs {
    pub aria_label: String,
    pub aria_disabled: Option<&'static str>,
    pub aria_invalid: Option<&'static str>,
    pub lang: Option<String>,
    pub dir: Option<&'static str>,
    pub data_orientation: &'static str,
    pub data_tone: &'static str,
    pub data_state: &'static str,
    pub data_message_kind: &'static str,
    pub data_required: Option<&'static str>,
    pub data_disabled: Option<&'static str>,
    pub data_invalid: Option<&'static str>,
    pub data_has_label: Option<&'static str>,
    pub data_has_description: Option<&'static str>,
    pub data_has_error: Option<&'static str>,
    pub data_aria_source: &'static str,
    pub data_error_source: &'static str,
    pub data_custom_class: Option<&'static str>,
    pub data_class_source: &'static str,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FieldSemanticState {
    pub orientation: &'static str,
    pub tone: &'static str,
    pub state: &'static str,
    pub message_kind: &'static str,
    pub is_required: bool,
    pub is_disabled: bool,
    pub is_invalid: bool,
    pub has_label: bool,
    pub has_description: bool,
    pub has_error_message: bool,
    pub aria_source: &'static str,
    pub error_source: &'static str,
    pub has_custom_class_name: bool,
    pub class_source: &'static str,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FieldContract {
    pub attrs: FieldAttrs,
    pub handlers: FieldHandlers,
    pub state: FieldSemanticState,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FieldOptions {
    pub state: FieldState,
    pub aria_label: String,
    pub lang: Option<String>,
    pub dir: Option<A11yDirection>,
}

pub fn use_field(options: FieldOptions) -> FieldContract {
    let locale = locale_attrs(options.lang, options.dir);
    let state = options.state;

    FieldContract {
        attrs: FieldAttrs {
            aria_label: options.aria_label,
            aria_disabled: state.is_disabled.then_some("true"),
            aria_invalid: state.is_invalid.then_some("true"),
            lang: locale.lang,
            dir: locale.dir,
            data_orientation: state.orientation_attr,
            data_tone: state.tone_attr,
            data_state: state.data_state_attr,
            data_message_kind: state.message_kind_attr,
            data_required: state.is_required.then_some("true"),
            data_disabled: state.is_disabled.then_some("true"),
            data_invalid: state.is_invalid.then_some("true"),
            data_has_label: state.has_label.then_some("true"),
            data_has_description: state.has_description.then_some("true"),
            data_has_error: state.has_error_message.then_some("true"),
            data_aria_source: state.aria_source_attr,
            data_error_source: state.error_source_attr,
            data_custom_class: state.has_custom_class_name.then_some("true"),
            data_class_source: state.class_source_attr,
        },
        handlers: FieldHandlers,
        state: FieldSemanticState {
            orientation: state.orientation_attr,
            tone: state.tone_attr,
            state: state.data_state_attr,
            message_kind: state.message_kind_attr,
            is_required: state.is_required,
            is_disabled: state.is_disabled,
            is_invalid: state.is_invalid,
            has_label: state.has_label,
            has_description: state.has_description,
            has_error_message: state.has_error_message,
            aria_source: state.aria_source_attr,
            error_source: state.error_source_attr,
            has_custom_class_name: state.has_custom_class_name,
            class_source: state.class_source_attr,
        },
    }
}

#[cfg(test)]
#[path = "test/field.rs"]
mod tests;
