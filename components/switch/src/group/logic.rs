use super::SwitchGroupIds;
pub use ui_state_primitives::switch_group::{
    SwitchGroupOrientation, SwitchGroupState, SwitchGroupStateInput, SwitchGroupTone,
};

pub const DEFAULT_LABEL: &str = "Switches";
pub const DEFAULT_ARIA_LABEL: &str = "SwitchGroup";
pub const DEFAULT_ERROR_MESSAGE: &str = "Invalid selection";

pub fn resolve_ids(id_base: String) -> SwitchGroupIds {
    let normalized = id_base.trim();
    let root_id = if normalized.is_empty() {
        "switch-group".to_string()
    } else {
        normalized.to_string()
    };

    SwitchGroupIds {
        label_id: format!("{root_id}-label"),
        description_id: format!("{root_id}-description"),
        error_id: format!("{root_id}-error"),
        root_id,
    }
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.into())
    })
}

pub fn normalize_label(value: Option<String>) -> (String, bool) {
    if let Some(label) = normalize_optional_text(value) {
        return (label, true);
    }

    (DEFAULT_LABEL.into(), false)
}

pub fn normalize_description(value: Option<String>) -> Option<String> {
    normalize_optional_text(value)
}

pub fn normalize_aria_label(value: Option<String>) -> (String, bool) {
    if let Some(label) = normalize_optional_text(value) {
        return (label, true);
    }

    (DEFAULT_ARIA_LABEL.into(), false)
}

pub fn normalize_error_message(value: Option<String>, invalid: bool) -> (Option<String>, bool) {
    if !invalid {
        return (None, false);
    }

    if let Some(message) = normalize_optional_text(value) {
        return (Some(message), true);
    }

    (Some(DEFAULT_ERROR_MESSAGE.into()), false)
}

pub fn resolve_state(input: SwitchGroupStateInput) -> SwitchGroupState {
    ui_state_primitives::switch_group::resolve_state(input)
}

pub fn compose_describedby(state: SwitchGroupState, ids: &SwitchGroupIds) -> Option<String> {
    let mut ids_out = Vec::new();

    if state.has_description {
        ids_out.push(ids.description_id.clone());
    }

    if state.shows_error {
        ids_out.push(ids.error_id.clone());
    }

    if ids_out.is_empty() {
        None
    } else {
        Some(ids_out.join(" "))
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: SwitchGroupState) -> String {
    let mut classes = vec![
        "ui-switch-group".to_string(),
        state.orientation_class.into(),
        state.tone_class.into(),
    ];

    if state.is_required {
        classes.push("ui-switch-group--required".to_string());
    }

    if state.is_disabled {
        classes.push("ui-switch-group--disabled".to_string());
    }

    if state.is_invalid {
        classes.push("ui-switch-group--invalid".to_string());
    }

    if state.has_description {
        classes.push("ui-switch-group--has-description".to_string());
    }

    if state.shows_error {
        classes.push("ui-switch-group--has-error".to_string());
    }

    if state.label_source_attr == "custom" {
        classes.push("ui-switch-group--label-custom".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-switch-group--custom-class".to_string());
        if let Some(base_class_name) = base_class_name {
            classes.push(base_class_name);
        }
    }

    classes.join(" ")
}

#[cfg(test)]
#[path = "../../test/group/logic.rs"]
mod tests;
