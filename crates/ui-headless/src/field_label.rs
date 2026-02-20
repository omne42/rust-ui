use crate::a11y::{A11yDirection, locale_attrs};
use ui_state_primitives::field_label::FieldLabelState;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct FieldLabelHandlers;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FieldLabelAttrs {
    pub aria_label: String,
    pub aria_disabled: Option<&'static str>,
    pub lang: Option<String>,
    pub dir: Option<&'static str>,
    pub data_tone: &'static str,
    pub data_state: &'static str,
    pub data_required: Option<&'static str>,
    pub data_disabled: Option<&'static str>,
    pub data_has_for: Option<&'static str>,
    pub data_text_source: &'static str,
    pub data_indicator_source: &'static str,
    pub data_aria_source: &'static str,
    pub data_custom_class: Option<&'static str>,
    pub data_class_source: &'static str,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FieldLabelSemanticState {
    pub tone: &'static str,
    pub state: &'static str,
    pub is_required: bool,
    pub is_disabled: bool,
    pub has_for_id: bool,
    pub has_custom_class_name: bool,
    pub text_source: &'static str,
    pub indicator_source: &'static str,
    pub aria_source: &'static str,
    pub class_source: &'static str,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FieldLabelContract {
    pub attrs: FieldLabelAttrs,
    pub handlers: FieldLabelHandlers,
    pub state: FieldLabelSemanticState,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FieldLabelOptions {
    pub state: FieldLabelState,
    pub aria_label: String,
    pub lang: Option<String>,
    pub dir: Option<A11yDirection>,
}

pub fn use_field_label(options: FieldLabelOptions) -> FieldLabelContract {
    let locale = locale_attrs(options.lang, options.dir);
    let data_state = if options.state.is_required {
        "required"
    } else {
        "optional"
    };

    FieldLabelContract {
        attrs: FieldLabelAttrs {
            aria_label: options.aria_label,
            aria_disabled: options.state.is_disabled.then_some("true"),
            lang: locale.lang,
            dir: locale.dir,
            data_tone: options.state.tone_attr,
            data_state,
            data_required: options.state.is_required.then_some("true"),
            data_disabled: options.state.is_disabled.then_some("true"),
            data_has_for: options.state.has_for_id.then_some("true"),
            data_text_source: options.state.text_source_attr,
            data_indicator_source: options.state.indicator_source_attr,
            data_aria_source: options.state.aria_source_attr,
            data_custom_class: options.state.has_custom_class_name.then_some("true"),
            data_class_source: options.state.class_source_attr,
        },
        handlers: FieldLabelHandlers,
        state: FieldLabelSemanticState {
            tone: options.state.tone_attr,
            state: data_state,
            is_required: options.state.is_required,
            is_disabled: options.state.is_disabled,
            has_for_id: options.state.has_for_id,
            has_custom_class_name: options.state.has_custom_class_name,
            text_source: options.state.text_source_attr,
            indicator_source: options.state.indicator_source_attr,
            aria_source: options.state.aria_source_attr,
            class_source: options.state.class_source_attr,
        },
    }
}

#[cfg(test)]
#[path = "test/field_label.rs"]
mod tests;
