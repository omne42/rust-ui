use std::collections::BTreeSet;

pub const DEFAULT_ID_BASE: &str = "action-menu";
pub const DEFAULT_TRIGGER_ARIA_LABEL: &str = "More actions";
pub const DEFAULT_DISABLED: bool = false;
pub const DEFAULT_CLOSE_ON_ACTION: bool = true;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ActionMenuBooleanProps {
    pub is_disabled: bool,
    pub is_close_on_action: bool,
    pub has_custom_disabled: bool,
    pub has_custom_close_on_action: bool,
}

pub fn state_attr(is_open: bool, trigger_disabled: bool, item_count: usize) -> &'static str {
    if is_open {
        "open"
    } else if trigger_disabled {
        "disabled"
    } else if item_count == 0 {
        "empty"
    } else {
        "closed"
    }
}

pub fn item_attr(item_count: usize) -> &'static str {
    if item_count == 0 {
        "empty"
    } else {
        "populated"
    }
}

pub fn action_attr(close_on_action: bool) -> &'static str {
    if close_on_action {
        "close"
    } else {
        "keep-open"
    }
}

pub fn open_mode_attr(is_controlled: bool) -> &'static str {
    if is_controlled {
        "controlled"
    } else {
        "uncontrolled"
    }
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.into())
    })
}

pub fn normalize_id_base(id_base: String) -> String {
    normalize_optional_text(Some(id_base)).unwrap_or_else(|| DEFAULT_ID_BASE.into())
}

pub fn resolve_id_pair(id_base: &str) -> (String, String) {
    (format!("{id_base}-trigger"), format!("{id_base}-menu"))
}

pub fn normalize_disabled_indices(disabled_indices: Vec<usize>, item_count: usize) -> Vec<usize> {
    let mut unique = BTreeSet::new();
    for index in disabled_indices {
        if index < item_count {
            unique.insert(index);
        }
    }
    unique.into_iter().collect()
}

pub fn resolve_trigger_disabled(disabled: bool, item_count: usize) -> bool {
    disabled || item_count == 0
}

pub fn resolve_trigger_aria_label(value: Option<String>) -> (String, bool) {
    resolve_trigger_aria_label_with_fallback(value, DEFAULT_TRIGGER_ARIA_LABEL)
}

pub fn resolve_trigger_aria_label_with_fallback(
    value: Option<String>,
    fallback_aria_label: &str,
) -> (String, bool) {
    if let Some(label) = normalize_optional_text(value) {
        return (label, true);
    }

    let fallback = fallback_aria_label.trim();
    if fallback.is_empty() {
        (DEFAULT_TRIGGER_ARIA_LABEL.into(), false)
    } else {
        (fallback.into(), false)
    }
}

pub fn normalize_boolean_props(
    is_disabled: Option<bool>,
    disabled: Option<bool>,
    is_close_on_action: Option<bool>,
    close_on_action: Option<bool>,
) -> ActionMenuBooleanProps {
    let is_disabled = is_disabled.or(disabled).unwrap_or(DEFAULT_DISABLED);
    let is_close_on_action = is_close_on_action
        .or(close_on_action)
        .unwrap_or(DEFAULT_CLOSE_ON_ACTION);

    ActionMenuBooleanProps {
        is_disabled,
        is_close_on_action,
        has_custom_disabled: is_disabled != DEFAULT_DISABLED,
        has_custom_close_on_action: is_close_on_action != DEFAULT_CLOSE_ON_ACTION,
    }
}

#[cfg(test)]
#[path = "test/action_menu.rs"]
mod tests;
