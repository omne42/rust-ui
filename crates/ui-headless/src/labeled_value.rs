use crate::a11y::{A11yDirection, labeled_group_attrs};
use ui_state_primitives::labeled_value::LabeledValueState;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct LabeledValueHandlers;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LabeledValueAttrs {
    pub role: &'static str,
    pub aria_label: String,
    pub lang: Option<String>,
    pub dir: Option<&'static str>,
    pub data_orientation: &'static str,
    pub data_tone: &'static str,
    pub data_state: &'static str,
    pub data_has_description: Option<&'static str>,
    pub data_label_source: &'static str,
    pub data_value_source: &'static str,
    pub data_aria_source: &'static str,
    pub data_custom_class: Option<&'static str>,
    pub data_class_source: &'static str,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LabeledValueSemanticState {
    pub orientation: &'static str,
    pub tone: &'static str,
    pub state: &'static str,
    pub label_source: &'static str,
    pub value_source: &'static str,
    pub aria_source: &'static str,
    pub class_source: &'static str,
    pub has_description: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LabeledValueContract {
    pub attrs: LabeledValueAttrs,
    pub handlers: LabeledValueHandlers,
    pub state: LabeledValueSemanticState,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LabeledValueOptions {
    pub state: LabeledValueState,
    pub aria_label: String,
    pub lang: Option<String>,
    pub dir: Option<A11yDirection>,
}

pub fn use_labeled_value(options: LabeledValueOptions) -> LabeledValueContract {
    let group = labeled_group_attrs(options.aria_label, options.lang, options.dir);
    let data_state = if options.state.has_description {
        "with-description"
    } else {
        "default"
    };

    LabeledValueContract {
        attrs: LabeledValueAttrs {
            role: group.role,
            aria_label: group.aria_label,
            lang: group.lang,
            dir: group.dir,
            data_orientation: options.state.orientation_attr,
            data_tone: options.state.tone_attr,
            data_state,
            data_has_description: options.state.has_description.then_some("true"),
            data_label_source: options.state.label_source_attr,
            data_value_source: options.state.value_source_attr,
            data_aria_source: options.state.aria_source_attr,
            data_custom_class: options.state.has_custom_class_name.then_some("true"),
            data_class_source: options.state.class_source_attr,
        },
        handlers: LabeledValueHandlers,
        state: LabeledValueSemanticState {
            orientation: options.state.orientation_attr,
            tone: options.state.tone_attr,
            state: data_state,
            label_source: options.state.label_source_attr,
            value_source: options.state.value_source_attr,
            aria_source: options.state.aria_source_attr,
            class_source: options.state.class_source_attr,
            has_description: options.state.has_description,
            has_custom_class_name: options.state.has_custom_class_name,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ui_state_primitives::labeled_value::{
        LabeledValueOrientation, LabeledValueStateInput, LabeledValueTone, resolve_state,
    };

    #[test]
    fn use_labeled_value_maps_locale_and_state_markers() {
        let state = resolve_state(LabeledValueStateInput {
            orientation: LabeledValueOrientation::Inline,
            tone: LabeledValueTone::Strong,
            has_custom_label: true,
            has_custom_value: false,
            has_description: true,
            has_custom_aria_label: true,
            has_custom_class_name: true,
        });

        let contract = use_labeled_value(LabeledValueOptions {
            state,
            aria_label: " Build status ".to_string(),
            lang: Some(" zh-CN ".to_string()),
            dir: Some(A11yDirection::Rtl),
        });

        assert_eq!(contract.attrs.role, "group");
        assert_eq!(contract.attrs.lang.as_deref(), Some("zh-CN"));
        assert_eq!(contract.attrs.dir, Some("rtl"));
        assert_eq!(contract.attrs.data_orientation, "inline");
        assert_eq!(contract.attrs.data_tone, "strong");
        assert_eq!(contract.attrs.data_state, "with-description");
        assert_eq!(contract.attrs.data_has_description, Some("true"));
        assert_eq!(contract.attrs.data_label_source, "custom");
        assert_eq!(contract.attrs.data_value_source, "default");
        assert_eq!(contract.attrs.data_aria_source, "custom");
        assert_eq!(contract.attrs.data_custom_class, Some("true"));
        assert_eq!(contract.attrs.data_class_source, "custom");
    }

    #[test]
    fn use_labeled_value_omits_optional_markers_for_defaults() {
        let state = resolve_state(LabeledValueStateInput {
            orientation: LabeledValueOrientation::Stacked,
            tone: LabeledValueTone::Default,
            has_custom_label: false,
            has_custom_value: false,
            has_description: false,
            has_custom_aria_label: false,
            has_custom_class_name: false,
        });

        let contract = use_labeled_value(LabeledValueOptions {
            state,
            aria_label: "Status".to_string(),
            lang: None,
            dir: None,
        });

        assert_eq!(contract.attrs.data_state, "default");
        assert_eq!(contract.attrs.data_has_description, None);
        assert_eq!(contract.attrs.data_custom_class, None);
        assert_eq!(contract.attrs.data_class_source, "default");
    }
}
