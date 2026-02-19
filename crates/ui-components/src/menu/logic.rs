#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MenuAccessibleName {
    pub aria_label: Option<String>,
    pub aria_labelledby: Option<String>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MenuState {
    pub is_empty: bool,
    pub has_items: bool,
    pub has_checked_items: bool,
    pub has_disabled_items: bool,
}

fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.into())
    })
}

pub fn resolve_accessible_name(
    aria_label: Option<String>,
    aria_labelledby: Option<String>,
) -> MenuAccessibleName {
    let aria_label = normalize_optional_text(aria_label);
    let aria_labelledby = normalize_optional_text(aria_labelledby);

    if aria_label.is_some() {
        return MenuAccessibleName {
            aria_label,
            aria_labelledby: None,
        };
    }

    if aria_labelledby.is_some() {
        return MenuAccessibleName {
            aria_label: None,
            aria_labelledby,
        };
    }

    MenuAccessibleName {
        aria_label: Some("Menu".to_string()),
        aria_labelledby: None,
    }
}

pub fn resolve_state(
    item_count: usize,
    has_checked_items: bool,
    has_disabled_items: bool,
) -> MenuState {
    let has_items = item_count > 0;

    MenuState {
        is_empty: !has_items,
        has_items,
        has_checked_items,
        has_disabled_items,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolve_accessible_name_prefers_explicit_aria_label() {
        assert_eq!(
            resolve_accessible_name(
                Some("  File actions  ".to_string()),
                Some("trigger-id".to_string())
            ),
            MenuAccessibleName {
                aria_label: Some("File actions".to_string()),
                aria_labelledby: None,
            }
        );
    }

    #[test]
    fn resolve_accessible_name_uses_labelledby_when_label_missing() {
        assert_eq!(
            resolve_accessible_name(None, Some("  trigger-id  ".to_string())),
            MenuAccessibleName {
                aria_label: None,
                aria_labelledby: Some("trigger-id".to_string()),
            }
        );
    }

    #[test]
    fn resolve_accessible_name_defaults_when_none_provided() {
        assert_eq!(
            resolve_accessible_name(None, None),
            MenuAccessibleName {
                aria_label: Some("Menu".to_string()),
                aria_labelledby: None,
            }
        );
    }

    #[test]
    fn resolve_accessible_name_ignores_blank_inputs() {
        assert_eq!(
            resolve_accessible_name(Some("  ".to_string()), Some("".to_string())),
            MenuAccessibleName {
                aria_label: Some("Menu".to_string()),
                aria_labelledby: None,
            }
        );
    }

    #[test]
    fn resolve_state_tracks_item_checked_and_disabled_flags() {
        let state = resolve_state(3, true, true);
        assert!(!state.is_empty);
        assert!(state.has_items);
        assert!(state.has_checked_items);
        assert!(state.has_disabled_items);
    }

    #[test]
    fn resolve_state_handles_empty_menu() {
        let state = resolve_state(0, false, false);
        assert!(state.is_empty);
        assert!(!state.has_items);
        assert!(!state.has_checked_items);
        assert!(!state.has_disabled_items);
    }
}
