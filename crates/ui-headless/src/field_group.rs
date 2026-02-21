use crate::a11y::{A11yDirection, locale_attrs};
use ui_state_primitives::field_group::FieldGroupState;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct FieldGroupHandlers;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FieldGroupAttrs {
    pub role: &'static str,
    pub aria_label: Option<String>,
    pub aria_labelledby: Option<String>,
    pub aria_describedby: Option<String>,
    pub aria_disabled: Option<&'static str>,
    pub aria_invalid: Option<&'static str>,
    pub lang: Option<String>,
    pub dir: Option<&'static str>,
    pub data_orientation: &'static str,
    pub data_density: &'static str,
    pub data_state: &'static str,
    pub data_disabled: Option<&'static str>,
    pub data_invalid: Option<&'static str>,
    pub data_label: &'static str,
    pub data_description: &'static str,
    pub data_aria_source: &'static str,
    pub data_custom_class: Option<&'static str>,
    pub data_class_source: &'static str,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FieldGroupSemanticState {
    pub orientation: &'static str,
    pub density: &'static str,
    pub state: &'static str,
    pub is_disabled: bool,
    pub is_invalid: bool,
    pub has_label: bool,
    pub has_description: bool,
    pub has_custom_aria_label: bool,
    pub aria_source: &'static str,
    pub has_custom_class_name: bool,
    pub class_source: &'static str,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FieldGroupContract {
    pub attrs: FieldGroupAttrs,
    pub handlers: FieldGroupHandlers,
    pub state: FieldGroupSemanticState,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FieldGroupOptions {
    pub state: FieldGroupState,
    pub aria_label: String,
    pub label_id: Option<String>,
    pub description_id: Option<String>,
    pub lang: Option<String>,
    pub dir: Option<A11yDirection>,
}

fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.into())
    })
}

pub fn use_field_group(options: FieldGroupOptions) -> FieldGroupContract {
    let locale = locale_attrs(options.lang, options.dir);
    let state = options.state;
    let label_id = normalize_optional_text(options.label_id);
    let description_id = normalize_optional_text(options.description_id);

    let aria_labelledby = (!state.has_custom_aria_label && state.has_label)
        .then_some(label_id.clone())
        .flatten();
    let aria_label = if !state.has_custom_aria_label && state.has_label {
        None
    } else {
        Some(options.aria_label)
    };
    let aria_describedby = state
        .has_description
        .then_some(description_id.clone())
        .flatten();

    FieldGroupContract {
        attrs: FieldGroupAttrs {
            role: "group",
            aria_label,
            aria_labelledby,
            aria_describedby,
            aria_disabled: state.is_disabled.then_some("true"),
            aria_invalid: state.is_invalid.then_some("true"),
            lang: locale.lang,
            dir: locale.dir,
            data_orientation: state.orientation_attr,
            data_density: state.density_attr,
            data_state: state.state_attr,
            data_disabled: state.is_disabled.then_some("true"),
            data_invalid: state.is_invalid.then_some("true"),
            data_label: state.label_attr,
            data_description: state.description_attr,
            data_aria_source: state.aria_source_attr,
            data_custom_class: state.has_custom_class_name.then_some("true"),
            data_class_source: state.class_source_attr,
        },
        handlers: FieldGroupHandlers,
        state: FieldGroupSemanticState {
            orientation: state.orientation_attr,
            density: state.density_attr,
            state: state.state_attr,
            is_disabled: state.is_disabled,
            is_invalid: state.is_invalid,
            has_label: state.has_label,
            has_description: state.has_description,
            has_custom_aria_label: state.has_custom_aria_label,
            aria_source: state.aria_source_attr,
            has_custom_class_name: state.has_custom_class_name,
            class_source: state.class_source_attr,
        },
    }
}

#[cfg(test)]
#[path = "test/field_group.rs"]
mod tests;
