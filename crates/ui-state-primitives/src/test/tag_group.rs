use super::{
    Tag, TagGroupItemStateInput, merge_describedby_ids, normalize_optional_text,
    resolve_item_state, resolve_state,
};

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
    let tags = vec![Tag::new("tag-rust", "Rust"), Tag::disabled("tag-ui", "UI")];

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
    let tags = vec![Tag::new("tag-baseline", "Baseline")];

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

#[test]
fn resolve_item_state_tracks_disabled_and_removable_flags() {
    let enabled = resolve_item_state(TagGroupItemStateInput {
        group_disabled: false,
        supports_removal: true,
        tag_disabled: false,
    });
    assert!(!enabled.is_disabled);
    assert!(enabled.is_removable);
    assert_eq!(enabled.disabled_source_attr, "none");
    assert_eq!(enabled.removable_source_attr, "removable");

    let group_disabled = resolve_item_state(TagGroupItemStateInput {
        group_disabled: true,
        supports_removal: true,
        tag_disabled: false,
    });
    assert!(group_disabled.is_disabled);
    assert!(!group_disabled.is_removable);
    assert_eq!(group_disabled.disabled_source_attr, "group");
    assert_eq!(group_disabled.removable_source_attr, "disabled");

    let item_disabled = resolve_item_state(TagGroupItemStateInput {
        group_disabled: false,
        supports_removal: true,
        tag_disabled: true,
    });
    assert!(item_disabled.is_disabled);
    assert!(!item_disabled.is_removable);
    assert_eq!(item_disabled.disabled_source_attr, "item");
    assert_eq!(item_disabled.removable_source_attr, "disabled");

    let unsupported = resolve_item_state(TagGroupItemStateInput {
        group_disabled: false,
        supports_removal: false,
        tag_disabled: false,
    });
    assert!(!unsupported.is_disabled);
    assert!(!unsupported.is_removable);
    assert_eq!(unsupported.disabled_source_attr, "none");
    assert_eq!(unsupported.removable_source_attr, "unsupported");
}
