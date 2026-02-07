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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TagGroupState {
    pub item_count: usize,
    pub is_empty: bool,
    pub has_items: bool,
    pub is_disabled: bool,
    pub has_disabled_tags: bool,
    pub has_removable_tags: bool,
    pub is_invalid: bool,
    pub is_required: bool,
}

pub fn resolve_state(
    tags: &[Tag],
    is_disabled: bool,
    supports_removal: bool,
    is_invalid: bool,
    is_required: bool,
) -> TagGroupState {
    let item_count = tags.len();
    let has_items = item_count > 0;
    let has_disabled_tags = has_items && (is_disabled || tags.iter().any(|tag| tag.disabled));
    let has_removable_tags =
        supports_removal && has_items && tags.iter().any(|tag| !is_disabled && !tag.disabled);

    TagGroupState {
        item_count,
        is_empty: !has_items,
        has_items,
        is_disabled,
        has_disabled_tags,
        has_removable_tags,
        is_invalid,
        is_required,
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
    use super::{Tag, merge_describedby_ids, normalize_optional_text, resolve_state};

    #[test]
    fn resolve_state_tracks_empty_defaults() {
        let state = resolve_state(&[], false, false, false, false);

        assert_eq!(state.item_count, 0);
        assert!(state.is_empty);
        assert!(!state.has_items);
        assert!(!state.is_disabled);
        assert!(!state.has_disabled_tags);
        assert!(!state.has_removable_tags);
        assert!(!state.is_invalid);
        assert!(!state.is_required);
    }

    #[test]
    fn resolve_state_tracks_disabled_and_removable_tags() {
        let tags = vec![
            Tag::new("tag-rust", "Rust"),
            Tag::disabled("tag-leptos", "Leptos"),
        ];

        let state = resolve_state(&tags, false, true, true, true);
        assert_eq!(state.item_count, 2);
        assert!(!state.is_empty);
        assert!(state.has_items);
        assert!(!state.is_disabled);
        assert!(state.has_disabled_tags);
        assert!(state.has_removable_tags);
        assert!(state.is_invalid);
        assert!(state.is_required);
    }

    #[test]
    fn resolve_state_disables_removal_when_group_disabled() {
        let tags = vec![Tag::new("tag-spectrum", "Spectrum")];

        let state = resolve_state(&tags, true, true, false, false);
        assert_eq!(state.item_count, 1);
        assert!(!state.is_empty);
        assert!(state.has_items);
        assert!(state.is_disabled);
        assert!(state.has_disabled_tags);
        assert!(!state.has_removable_tags);
    }

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
