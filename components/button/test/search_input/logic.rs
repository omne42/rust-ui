use super::*;

#[test]
fn normalize_optional_text_trims_and_drops_blank_values() {
    assert_eq!(
        normalize_optional_text(Some("  Search docs  ".to_string())),
        Some("Search docs".to_string())
    );
    assert_eq!(normalize_optional_text(Some("   ".to_string())), None);
    assert_eq!(normalize_optional_text(None), None);
}

#[test]
fn resolve_button_type_defaults_to_button() {
    assert_eq!(resolve_button_type(None), ButtonType::Button);
    assert_eq!(
        resolve_button_type(Some(ButtonType::Button)),
        ButtonType::Button
    );
    assert_eq!(
        resolve_button_type(Some(ButtonType::Submit)),
        ButtonType::Submit
    );
    assert_eq!(
        resolve_button_type(Some(ButtonType::Reset)),
        ButtonType::Reset
    );
}

#[test]
fn resolve_effective_aria_label_prefers_explicit_and_falls_back_to_placeholder() {
    let custom = resolve_effective_aria_label(Some("  Open search  ".to_string()), "Search");
    assert_eq!(custom.aria_label, "Open search");
    assert!(custom.has_custom_aria_label);

    let fallback = resolve_effective_aria_label(Some("   ".to_string()), "Search docs");
    assert_eq!(fallback.aria_label, "Search docs");
    assert!(!fallback.has_custom_aria_label);
}

#[test]
fn resolve_state_tracks_enablement_and_metadata_flags() {
    let enabled_state = resolve_state(SearchInputButtonStateInput {
        is_disabled: false,
        has_shortcut: true,
        has_custom_placeholder: true,
        has_custom_compact_placeholder: true,
        has_custom_aria_label: true,
        has_custom_class_name: true,
    });
    assert!(enabled_state.is_enabled);
    assert!(!enabled_state.is_disabled);
    assert_eq!(enabled_state.state_attr, "enabled");
    assert_eq!(enabled_state.shortcut_attr, "visible");
    assert_eq!(enabled_state.placeholder_source_attr, "custom");
    assert_eq!(enabled_state.compact_placeholder_source_attr, "custom");
    assert_eq!(enabled_state.aria_label_source_attr, "custom");
    assert!(enabled_state.has_custom_class_name);

    let disabled_state = resolve_state(SearchInputButtonStateInput {
        is_disabled: true,
        has_shortcut: false,
        has_custom_placeholder: false,
        has_custom_compact_placeholder: false,
        has_custom_aria_label: false,
        has_custom_class_name: false,
    });
    assert!(!disabled_state.is_enabled);
    assert!(disabled_state.is_disabled);
    assert_eq!(disabled_state.state_attr, "disabled");
    assert_eq!(disabled_state.shortcut_attr, "hidden");
    assert_eq!(disabled_state.placeholder_source_attr, "default");
    assert_eq!(disabled_state.compact_placeholder_source_attr, "default");
    assert_eq!(disabled_state.aria_label_source_attr, "placeholder");
}

#[test]
fn view_state_defaults_and_trims_with_blank_fallbacks() {
    let state = resolve_view_state(Some("  Search docs... "), None, None, None, "Search");
    assert_eq!(state.placeholder, "Search docs...");
    assert_eq!(state.compact_placeholder, "Search docs...");
    assert!(!state.show_shortcut);

    let state = resolve_view_state(
        Some("   "),
        Some("  Go "),
        Some(" ⌘ "),
        Some(" K "),
        "Search",
    );
    assert_eq!(state.placeholder, "Search");
    assert_eq!(state.compact_placeholder, "Go");
    assert_eq!(state.meta_key_label.as_deref(), Some("⌘"));
    assert_eq!(state.key_label.as_deref(), Some("K"));
    assert!(state.show_shortcut);
}

#[test]
fn shortcut_requires_both_keys() {
    let state = resolve_view_state(Some("Search"), None, Some("⌘"), None, "Search");
    assert!(!state.show_shortcut);

    let state = resolve_view_state(Some("Search"), None, None, Some("K"), "Search");
    assert!(!state.show_shortcut);
}

#[test]
fn compose_class_name_includes_state_markers() {
    let class_name = compose_class_name(
        Some("custom".to_string()),
        resolve_state(SearchInputButtonStateInput {
            is_disabled: false,
            has_shortcut: true,
            has_custom_placeholder: true,
            has_custom_compact_placeholder: true,
            has_custom_aria_label: false,
            has_custom_class_name: true,
        }),
    );

    for token in [
        "ui-search-input-button",
        "ui-search-input-button--enabled",
        "ui-search-input-button--with-shortcut",
        "ui-search-input-button--custom-placeholder",
        "ui-search-input-button--custom-compact-placeholder",
        "ui-search-input-button--placeholder-aria-label",
        "ui-search-input-button--custom-class",
        "custom",
    ] {
        assert!(
            class_name.contains(token),
            "composed class name should include `{token}`"
        );
    }
}
