#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ListBoxAccessibleName {
    pub aria_label: Option<String>,
    pub aria_labelledby: Option<String>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ListBoxState {
    pub is_empty: bool,
    pub has_items: bool,
    pub has_selection: bool,
    pub has_disabled_options: bool,
}

fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.to_string())
    })
}

pub fn resolve_accessible_name(
    aria_label: Option<String>,
    aria_labelledby: Option<String>,
) -> ListBoxAccessibleName {
    let aria_label = normalize_optional_text(aria_label);
    let aria_labelledby = normalize_optional_text(aria_labelledby);

    if aria_label.is_some() {
        return ListBoxAccessibleName {
            aria_label,
            aria_labelledby: None,
        };
    }

    if aria_labelledby.is_some() {
        return ListBoxAccessibleName {
            aria_label: None,
            aria_labelledby,
        };
    }

    ListBoxAccessibleName {
        aria_label: Some("Listbox".to_string()),
        aria_labelledby: None,
    }
}

pub fn resolve_state(
    item_count: usize,
    selected_index: Option<usize>,
    has_disabled_options: bool,
) -> ListBoxState {
    let has_items = item_count > 0;
    let has_selection = selected_index.filter(|index| *index < item_count).is_some();

    ListBoxState {
        is_empty: !has_items,
        has_items,
        has_selection,
        has_disabled_options,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolve_accessible_name_prefers_explicit_aria_label() {
        assert_eq!(
            resolve_accessible_name(
                Some("  Fruit options  ".to_string()),
                Some("trigger-id".to_string())
            ),
            ListBoxAccessibleName {
                aria_label: Some("Fruit options".to_string()),
                aria_labelledby: None,
            }
        );
    }

    #[test]
    fn resolve_accessible_name_uses_labelledby_when_label_missing() {
        assert_eq!(
            resolve_accessible_name(None, Some("  trigger-id  ".to_string())),
            ListBoxAccessibleName {
                aria_label: None,
                aria_labelledby: Some("trigger-id".to_string()),
            }
        );
    }

    #[test]
    fn resolve_accessible_name_defaults_when_none_provided() {
        assert_eq!(
            resolve_accessible_name(None, None),
            ListBoxAccessibleName {
                aria_label: Some("Listbox".to_string()),
                aria_labelledby: None,
            }
        );
    }

    #[test]
    fn resolve_accessible_name_ignores_blank_inputs() {
        assert_eq!(
            resolve_accessible_name(Some("  ".to_string()), Some("".to_string())),
            ListBoxAccessibleName {
                aria_label: Some("Listbox".to_string()),
                aria_labelledby: None,
            }
        );
    }

    #[test]
    fn resolve_state_tracks_item_and_selection_flags() {
        let state = resolve_state(4, Some(2), true);
        assert!(!state.is_empty);
        assert!(state.has_items);
        assert!(state.has_selection);
        assert!(state.has_disabled_options);
    }

    #[test]
    fn resolve_state_treats_out_of_range_selection_as_empty_selection() {
        let state = resolve_state(2, Some(9), false);
        assert!(!state.is_empty);
        assert!(state.has_items);
        assert!(!state.has_selection);
        assert!(!state.has_disabled_options);
    }

    #[test]
    fn resolve_state_handles_empty_listbox() {
        let state = resolve_state(0, None, false);
        assert!(state.is_empty);
        assert!(!state.has_items);
        assert!(!state.has_selection);
        assert!(!state.has_disabled_options);
    }
}
