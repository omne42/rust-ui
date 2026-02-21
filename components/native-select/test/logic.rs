use super::*;

#[test]
fn size_class_names_and_attrs_are_stable() {
    assert_eq!(
        NativeSelectSize::Sm.class_name(),
        "ui-native-select--size-sm"
    );
    assert_eq!(
        NativeSelectSize::Md.class_name(),
        "ui-native-select--size-md"
    );
    assert_eq!(
        NativeSelectSize::Lg.class_name(),
        "ui-native-select--size-lg"
    );

    assert_eq!(NativeSelectSize::Sm.data_size(), "sm");
    assert_eq!(NativeSelectSize::Md.data_size(), "md");
    assert_eq!(NativeSelectSize::Lg.data_size(), "lg");
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
fn default_selected_index_normalization_is_explicit() {
    assert_eq!(normalize_default_selected_index(Some(2)), Some(Some(2)));
    assert_eq!(normalize_default_selected_index(None), None);
}

#[test]
fn control_value_falls_back_to_empty_string() {
    assert_eq!(resolve_control_value(Some("manual")), "manual");
    assert_eq!(resolve_control_value(None), "");
}

#[test]
fn selected_index_correction_emits_only_when_value_changes() {
    let options = resolve_options(
        "x",
        &normalize_options(vec![
            NativeSelectOption::new("system", "System"),
            NativeSelectOption::new("manual", "Manual").disabled(true),
        ]),
    );

    assert_eq!(
        resolve_selected_index_correction(Some(0), &options),
        None,
        "already valid index should not trigger correction"
    );
    assert_eq!(
        resolve_selected_index_correction(Some(1), &options),
        Some(None),
        "disabled index should be corrected to None"
    );
}

#[test]
fn resolve_states_for_render_centralizes_primitive_and_component_derivation() {
    let options = resolve_options(
        "docs",
        &normalize_options(vec![
            NativeSelectOption::new("system", "System"),
            NativeSelectOption::new("manual", "Manual").disabled(true),
        ]),
    );

    let states = resolve_states_for_render(NativeSelectStateParams {
        size: NativeSelectSize::Lg,
        is_disabled: false,
        is_invalid: true,
        is_required: true,
        has_placeholder: true,
        selected_index: Some(0),
        options: &options,
        has_custom_aria_label: false,
        has_custom_class_name: true,
    });

    assert_eq!(states.component.selected_index, Some(0));
    assert_eq!(states.primitive.selected_index, Some(0));
    assert_eq!(
        states.component.data_state_attr,
        states.primitive.data_state_attr
    );
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

    let state = resolve_state(
        NativeSelectStateInput {
            disabled: false,
            invalid: true,
            required: true,
            has_placeholder: true,
            selected_index: Some(0),
            options: &options,
            has_custom_aria_label: false,
            has_custom_class_name: true,
        },
        NativeSelectSize::Lg,
    );

    assert_eq!(state.size_class, "ui-native-select--size-lg");
    assert_eq!(state.size_attr, "lg");
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

#[test]
fn compose_class_name_adds_state_and_custom_markers() {
    let state = NativeSelectState {
        size_class: "ui-native-select--size-md",
        size_attr: "md",
        is_disabled: false,
        control_disabled: true,
        is_invalid: true,
        is_required: false,
        has_placeholder: true,
        is_empty: true,
        has_options: false,
        option_count: 0,
        selected_index: None,
        selected_value: None,
        has_selection: false,
        has_disabled_options: false,
        has_enabled_options: false,
        disabled_option_count: 0,
        data_state_attr: "disabled",
        aria_source_attr: "custom",
        class_source_attr: "custom",
        has_custom_class_name: true,
    };

    let class = compose_class_name(Some("docs-native-select".to_string()), &state);
    assert!(class.contains("ui-native-select"));
    assert!(class.contains("ui-native-select--size-md"));
    assert!(class.contains("ui-native-select--disabled"));
    assert!(class.contains("ui-native-select--invalid"));
    assert!(class.contains("ui-native-select--empty"));
    assert!(class.contains("ui-native-select--has-placeholder"));
    assert!(class.contains("ui-native-select--custom-class"));
    assert!(class.contains("docs-native-select"));
}
