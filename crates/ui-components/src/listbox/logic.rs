#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ListBoxAccessibleName {
    pub aria_label: Option<String>,
    pub aria_labelledby: Option<String>,
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
}
