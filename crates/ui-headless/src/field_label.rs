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
mod tests {
    use super::*;
    use ui_state_primitives::field_label::{FieldLabelStateInput, FieldLabelTone, resolve_state};

    #[test]
    fn use_field_label_maps_locale_and_semantic_attrs() {
        let state = resolve_state(FieldLabelStateInput {
            tone: FieldLabelTone::Strong,
            required: true,
            disabled: true,
            has_for_id: true,
            has_custom_text: true,
            has_custom_indicator: false,
            has_custom_aria_label: true,
            has_custom_class_name: true,
        });

        let contract = use_field_label(FieldLabelOptions {
            state,
            aria_label: " Assignee field label ".to_string(),
            lang: Some("  zh-CN ".to_string()),
            dir: Some(A11yDirection::Rtl),
        });

        assert_eq!(contract.attrs.aria_label, " Assignee field label ");
        assert_eq!(contract.attrs.aria_disabled, Some("true"));
        assert_eq!(contract.attrs.lang.as_deref(), Some("zh-CN"));
        assert_eq!(contract.attrs.dir, Some("rtl"));
        assert_eq!(contract.attrs.data_tone, "strong");
        assert_eq!(contract.attrs.data_state, "required");
        assert_eq!(contract.attrs.data_required, Some("true"));
        assert_eq!(contract.attrs.data_disabled, Some("true"));
        assert_eq!(contract.attrs.data_has_for, Some("true"));
        assert_eq!(contract.attrs.data_text_source, "custom");
        assert_eq!(contract.attrs.data_indicator_source, "default");
        assert_eq!(contract.attrs.data_aria_source, "custom");
        assert_eq!(contract.attrs.data_custom_class, Some("true"));
        assert_eq!(contract.attrs.data_class_source, "custom");
    }

    #[test]
    fn use_field_label_omits_optional_markers_for_default_optional_case() {
        let state = resolve_state(FieldLabelStateInput {
            tone: FieldLabelTone::Default,
            required: false,
            disabled: false,
            has_for_id: false,
            has_custom_text: false,
            has_custom_indicator: false,
            has_custom_aria_label: false,
            has_custom_class_name: false,
        });

        let contract = use_field_label(FieldLabelOptions {
            state,
            aria_label: "Field label".to_string(),
            lang: None,
            dir: None,
        });

        assert_eq!(contract.attrs.data_tone, "default");
        assert_eq!(contract.attrs.data_state, "optional");
        assert_eq!(contract.attrs.data_required, None);
        assert_eq!(contract.attrs.data_disabled, None);
        assert_eq!(contract.attrs.data_has_for, None);
        assert_eq!(contract.attrs.data_text_source, "default");
        assert_eq!(contract.attrs.data_indicator_source, "default");
        assert_eq!(contract.attrs.data_aria_source, "default");
        assert_eq!(contract.attrs.data_custom_class, None);
        assert_eq!(contract.attrs.data_class_source, "default");
        assert_eq!(contract.attrs.lang, None);
        assert_eq!(contract.attrs.dir, None);
    }
}
