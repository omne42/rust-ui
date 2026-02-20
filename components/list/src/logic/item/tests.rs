use super::*;

#[test]
fn normalize_helpers_trim_and_fallback() {
    assert_eq!(normalize_class_name(None), None);
    assert_eq!(normalize_class_name(Some("  \n\t  ".to_string())), None);
    assert_eq!(
        normalize_class_name(Some("  San Jose  ".to_string())),
        Some("San Jose".to_string())
    );

    assert_eq!(
        normalize_aria_label(Some("  Favorite city  ".to_string())),
        ("Favorite city".to_string(), true)
    );
    assert_eq!(
        normalize_aria_label(Some("".to_string())),
        (DEFAULT_ARIA_LABEL.into(), false)
    );
}

#[test]
fn selection_indicator_contract_is_stable() {
    assert_eq!(
        resolve_selection_indicator(false),
        ListItemSelectionIndicator::Hidden
    );
    assert_eq!(
        resolve_selection_indicator(true),
        ListItemSelectionIndicator::Checkmark
    );
    assert_eq!(ListItemSelectionIndicator::Hidden.as_attr(), "hidden");
    assert_eq!(ListItemSelectionIndicator::Checkmark.as_attr(), "checkmark");
}

#[test]
fn resolve_state_tracks_selection_focus_and_sources() {
    let state = resolve_state(ListItemStateInput {
        selected: true,
        focused: true,
        disabled: false,
        show_selection_indicator: true,
        has_divider: true,
        has_custom_aria_label: true,
        has_custom_class_name: true,
    });

    assert!(state.is_selected);
    assert!(state.is_focused);
    assert!(!state.is_disabled);
    assert!(state.show_selection_indicator);
    assert!(state.has_divider);
    assert_eq!(state.data_state_attr, "focused-selected");
    assert_eq!(state.selection_indicator_attr, "checkmark");
    assert_eq!(state.aria_source_attr, "custom");
    assert_eq!(state.class_source_attr, "custom");
}

#[test]
fn compose_class_name_includes_state_markers() {
    let state = resolve_state(ListItemStateInput {
        selected: true,
        focused: false,
        disabled: false,
        show_selection_indicator: true,
        has_divider: true,
        has_custom_aria_label: false,
        has_custom_class_name: true,
    });

    let class_name = compose_class_name(Some("docs-listbox-item-custom".to_string()), state);

    for needle in [
        "ui-listbox-item",
        "ui-listbox-item--selected",
        "ui-listbox-item--selection-indicator",
        "ui-listbox-item--divider",
        "ui-listbox-item--custom-class",
        "docs-listbox-item-custom",
    ] {
        assert!(
            class_name.contains(needle),
            "ListItem class list should include `{needle}`"
        );
    }
}
