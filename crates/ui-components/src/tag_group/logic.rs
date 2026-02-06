#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Tag {
    pub id: String,
    pub label: String,
    pub disabled: bool,
}

impl Tag {
    pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            disabled: false,
        }
    }

    pub fn disabled(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            disabled: true,
        }
    }
}

pub(crate) fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.to_string())
    })
}

pub(crate) fn merge_describedby_ids(
    external: Option<String>,
    description_id: Option<&str>,
    error_id: Option<&str>,
) -> Option<String> {
    let mut ids = Vec::new();

    if let Some(external) = normalize_optional_text(external) {
        ids.push(external);
    }

    if let Some(description_id) = description_id {
        ids.push(description_id.to_string());
    }

    if let Some(error_id) = error_id {
        ids.push(error_id.to_string());
    }

    (!ids.is_empty()).then(|| ids.join(" "))
}

#[cfg(test)]
mod tests {
    use super::{merge_describedby_ids, normalize_optional_text};

    #[test]
    fn normalize_optional_text_trims_and_filters_empty_values() {
        assert_eq!(normalize_optional_text(None), None);
        assert_eq!(normalize_optional_text(Some("".to_string())), None);
        assert_eq!(normalize_optional_text(Some("   \n\t ".to_string())), None);
        assert_eq!(
            normalize_optional_text(Some("  Tags  ".to_string())),
            Some("Tags".to_string())
        );
    }

    #[test]
    fn merge_describedby_ids_merges_in_stable_order() {
        assert_eq!(
            merge_describedby_ids(
                Some("hint-id".to_string()),
                Some("group-description"),
                Some("group-error")
            ),
            Some("hint-id group-description group-error".to_string())
        );
    }

    #[test]
    fn merge_describedby_ids_omits_missing_parts() {
        assert_eq!(merge_describedby_ids(None, None, None), None);
        assert_eq!(
            merge_describedby_ids(Some("  ".to_string()), Some("group-description"), None),
            Some("group-description".to_string())
        );
        assert_eq!(
            merge_describedby_ids(None, None, Some("group-error")),
            Some("group-error".to_string())
        );
    }
}
