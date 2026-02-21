use super::*;

#[test]
fn normalize_optional_text_filters_blank_values() {
    assert_eq!(normalize_optional_text(None), None);
    assert_eq!(normalize_optional_text(Some("   ".to_string())), None);
    assert_eq!(
        normalize_optional_text(Some("  Frequency  ".to_string())),
        Some("Frequency".to_string())
    );
}

#[test]
fn normalize_aria_label_prefers_custom_and_has_fallback() {
    let (label, custom) = normalize_aria_label(Some("  Frequency  ".to_string()));
    assert_eq!(label, "Frequency");
    assert!(custom);

    let (label, custom) = normalize_aria_label(Some("   ".to_string()));
    assert_eq!(label, DEFAULT_ARIA_LABEL);
    assert!(!custom);

    let (label, custom) = normalize_aria_label(None);
    assert_eq!(label, DEFAULT_ARIA_LABEL);
    assert!(!custom);
}

#[test]
fn resolve_options_normalizes_ids_labels_and_values() {
    let options = vec![
        NativeSelectOption::new("", ""),
        NativeSelectOption::new("manual", " Manual ").disabled(true),
    ];

    let normalized = normalize_options(options);
    assert_eq!(normalized[0].value, "option-1");
    assert_eq!(normalized[0].label, "option-1");
    assert_eq!(normalized[1].value, "manual");
    assert_eq!(normalized[1].label, "Manual");

    let resolved = resolve_options("docs-native-select", &normalized);
    assert_eq!(resolved[0].id, "docs-native-select-option-0");
    assert_eq!(resolved[1].id, "docs-native-select-option-1");
    assert!(resolved[1].disabled);
}

#[test]
fn selected_index_and_lookup_skip_disabled_options() {
    let options = resolve_options(
        "x",
        &normalize_options(vec![
            NativeSelectOption::new("system", "System"),
            NativeSelectOption::new("manual", "Manual").disabled(true),
        ]),
    );

    assert_eq!(sanitize_selected_index(Some(0), &options), Some(0));
    assert_eq!(sanitize_selected_index(Some(1), &options), None);
    assert_eq!(sanitize_selected_index(Some(8), &options), None);

    assert_eq!(find_index_by_value("system", &options), Some(0));
    assert_eq!(find_index_by_value("manual", &options), None);
    assert_eq!(find_index_by_value("missing", &options), None);
}

#[test]
fn resolve_state_tracks_disabled_invalid_selection_and_counts() {
    let options = resolve_options(
        "docs",
        &normalize_options(vec![
            NativeSelectOption::new("system", "System"),
            NativeSelectOption::new("manual", "Manual").disabled(true),
        ]),
    );

    let state = resolve_state(NativeSelectStateInput {
        disabled: false,
        invalid: true,
        required: true,
        has_placeholder: true,
        selected_index: Some(0),
        options: &options,
        has_custom_aria_label: false,
        has_custom_class_name: true,
    });

    assert!(state.has_options);
    assert!(!state.is_empty);
    assert!(state.has_selection);
    assert_eq!(state.selected_index, Some(0));
    assert_eq!(state.selected_value.as_deref(), Some("system"));
    assert!(state.is_invalid);
    assert!(state.is_required);
    assert!(state.has_placeholder);
    assert!(state.has_disabled_options);
    assert_eq!(state.disabled_option_count, 1);
    assert!(state.has_enabled_options);
    assert!(!state.control_disabled);
    assert_eq!(state.data_state_attr, "invalid");
    assert_eq!(state.aria_source_attr, "default");
    assert_eq!(state.class_source_attr, "custom");
}
