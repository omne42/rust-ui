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
        (!trimmed.is_empty()).then(|| trimmed.to_string())
    })
}

pub fn normalize_id_base(id_base: String) -> String {
    normalize_optional_text(Some(id_base)).unwrap_or_else(|| DEFAULT_ID_BASE.to_string())
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
        (DEFAULT_TRIGGER_ARIA_LABEL.to_string(), false)
    } else {
        (fallback.to_string(), false)
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
mod tests {
    use super::*;

    #[test]
    fn state_helpers_remain_stable() {
        assert_eq!(state_attr(true, false, 3), "open");
        assert_eq!(state_attr(false, true, 3), "disabled");
        assert_eq!(state_attr(false, false, 0), "empty");
        assert_eq!(state_attr(false, false, 3), "closed");
        assert_eq!(item_attr(0), "empty");
        assert_eq!(item_attr(2), "populated");
        assert_eq!(action_attr(true), "close");
        assert_eq!(action_attr(false), "keep-open");
        assert_eq!(open_mode_attr(true), "controlled");
        assert_eq!(open_mode_attr(false), "uncontrolled");
    }

    #[test]
    fn optional_text_trims_and_drops_blank_values() {
        assert_eq!(normalize_optional_text(None), None);
        assert_eq!(normalize_optional_text(Some("  \n\t".to_string())), None);
        assert_eq!(
            normalize_optional_text(Some("  demo  ".to_string())),
            Some("demo".to_string())
        );
    }

    #[test]
    fn id_base_defaults_for_blank_values() {
        assert_eq!(normalize_id_base(" demo ".to_string()), "demo");
        assert_eq!(normalize_id_base("   ".to_string()), DEFAULT_ID_BASE);
    }

    #[test]
    fn resolve_id_pair_uses_stable_suffixes() {
        assert_eq!(
            resolve_id_pair("workspace"),
            (
                "workspace-trigger".to_string(),
                "workspace-menu".to_string()
            )
        );
    }

    #[test]
    fn disabled_indices_are_deduped_and_clamped() {
        assert_eq!(normalize_disabled_indices(vec![2, 1, 1, 9], 3), vec![1, 2]);
        assert_eq!(normalize_disabled_indices(vec![4], 0), Vec::<usize>::new());
    }

    #[test]
    fn trigger_disabled_when_explicit_or_empty() {
        assert!(resolve_trigger_disabled(true, 3));
        assert!(resolve_trigger_disabled(false, 0));
        assert!(!resolve_trigger_disabled(false, 2));
    }

    #[test]
    fn aria_label_uses_default_when_empty() {
        assert_eq!(
            resolve_trigger_aria_label(None),
            (DEFAULT_TRIGGER_ARIA_LABEL.to_string(), false)
        );
        assert_eq!(
            resolve_trigger_aria_label(Some("  More  ".to_string())),
            ("More".to_string(), true)
        );
    }

    #[test]
    fn aria_label_can_use_external_fallback() {
        assert_eq!(
            resolve_trigger_aria_label_with_fallback(None, "  Workspace actions  "),
            ("Workspace actions".to_string(), false)
        );
        assert_eq!(
            resolve_trigger_aria_label_with_fallback(None, "   "),
            (DEFAULT_TRIGGER_ARIA_LABEL.to_string(), false)
        );
    }

    #[test]
    fn boolean_props_defaults_and_alias_priority_are_stable() {
        assert_eq!(
            normalize_boolean_props(None, None, None, None),
            ActionMenuBooleanProps {
                is_disabled: DEFAULT_DISABLED,
                is_close_on_action: DEFAULT_CLOSE_ON_ACTION,
                has_custom_disabled: false,
                has_custom_close_on_action: false,
            }
        );

        assert_eq!(
            normalize_boolean_props(None, Some(true), None, Some(false)),
            ActionMenuBooleanProps {
                is_disabled: true,
                is_close_on_action: false,
                has_custom_disabled: true,
                has_custom_close_on_action: true,
            }
        );

        assert_eq!(
            normalize_boolean_props(Some(false), Some(true), Some(true), Some(false)),
            ActionMenuBooleanProps {
                is_disabled: false,
                is_close_on_action: true,
                has_custom_disabled: false,
                has_custom_close_on_action: false,
            }
        );
    }
}
