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
#[path = "test/labeled_value.rs"]
mod tests;
