use super::*;

#[test]
fn state_primitives_are_reexported_from_ui_state_primitives() {
    assert_eq!(normalize_label("  Language  ".to_string()), "Language");
    assert_eq!(
        normalize_id_base("   ".to_string()),
        ui_state_primitives::combo_box::DEFAULT_ID_BASE
    );
    assert_eq!(
        resolve_placeholder(None),
        ui_state_primitives::combo_box::DEFAULT_PLACEHOLDER
    );
    assert_eq!(
        resolve_empty_message(None),
        ui_state_primitives::combo_box::DEFAULT_EMPTY_MESSAGE
    );
    assert_eq!(
        resolve_toggle_aria_label(None),
        ui_state_primitives::combo_box::DEFAULT_TOGGLE_ARIA_LABEL
    );
}

#[test]
fn normalize_accessibility_state_applies_explicit_priority_and_defaults() {
    let (preferred_required, _set_preferred_required) = signal(true);
    let (legacy_required, _set_legacy_required) = signal(false);
    let (preferred_invalid, _set_preferred_invalid) = signal(true);
    let (legacy_invalid, _set_legacy_invalid) = signal(false);

    let state = normalize_accessibility_state(AccessibilityStateInput {
        is_disabled: Some(true),
        disabled: false,
        is_required: Some(preferred_required.into()),
        required: Some(legacy_required.into()),
        is_invalid: Some(preferred_invalid.into()),
        invalid: Some(legacy_invalid.into()),
    });

    assert!(state.is_disabled);
    assert!(state.required.get_untracked());
    assert!(state.invalid.get_untracked());

    let fallback = normalize_accessibility_state(AccessibilityStateInput {
        is_disabled: None,
        disabled: false,
        is_required: None,
        required: None,
        is_invalid: None,
        invalid: None,
    });
    assert!(!fallback.required.get_untracked());
    assert!(!fallback.invalid.get_untracked());
}

#[test]
fn normalize_open_state_applies_explicit_priority_and_triplet_passthrough() {
    let (is_open_signal, _set_is_open_signal) = signal(true);
    let (legacy_open_signal, _set_legacy_open_signal) = signal(false);
    let on_open_change = Callback::new(|_: bool| {});

    let open_state = normalize_open_state(OpenStateInput {
        is_open: Some(is_open_signal.into()),
        open: Some(legacy_open_signal.into()),
        default_open: Some(false),
        on_open_change: Some(on_open_change),
    });

    assert!(open_state.is_controlled);
    assert!(
        open_state
            .open
            .expect("normalized open signal should exist")
            .get_untracked()
    );
    assert_eq!(open_state.default_open, Some(false));
    assert!(open_state.on_open_change.is_some());
}

#[test]
fn normalize_root_state_centralizes_normalization_and_state_derivation() {
    let root = normalize_root_state(RootStateInput {
        id_base: "  ".to_string(),
        label: "  ".to_string(),
        placeholder: Some("  ".to_string()),
        empty_message: Some("  nothing  ".to_string()),
        toggle_button_aria_label: Some("  expand  ".to_string()),
        description: Some("  desc  ".to_string()),
        error: Some("  err  ".to_string()),
        class_name: Some("  custom  ".to_string()),
        item_count: 3,
        disabled_indices: vec![2, 2, 9],
        is_disabled: true,
        is_controlled: true,
        has_custom_motion: true,
    });

    assert_eq!(
        root.id_base,
        ui_state_primitives::combo_box::DEFAULT_ID_BASE
    );
    assert_eq!(root.label, ui_state_primitives::combo_box::DEFAULT_LABEL);
    assert_eq!(
        root.placeholder,
        ui_state_primitives::combo_box::DEFAULT_PLACEHOLDER
    );
    assert_eq!(root.empty_message, "nothing");
    assert_eq!(root.toggle_button_aria_label, "expand");
    assert_eq!(root.description.as_deref(), Some("desc"));
    assert_eq!(root.error.as_deref(), Some("err"));
    assert_eq!(root.disabled_indices, vec![2]);
    assert!(root.state.is_disabled);
    assert!(root.state.is_controlled);
    assert!(root.class_name.contains("ui-combo-box"));
}

#[test]
fn resolve_root_data_state_uses_type_safe_exclusive_enum() {
    assert_eq!(resolve_root_data_state(true, true), RootDataState::Open);
    assert_eq!(
        resolve_root_data_state(false, true),
        RootDataState::Disabled
    );
    assert_eq!(resolve_root_data_state(false, false), RootDataState::Closed);
    assert_eq!(RootDataState::Open.as_attr(), "open");
    assert_eq!(RootDataState::Disabled.as_attr(), "disabled");
    assert_eq!(RootDataState::Closed.as_attr(), "closed");
}

#[test]
fn compose_class_name_includes_state_markers() {
    let class_name = compose_class_name(
        Some("custom".to_string()),
        resolve_state(ComboBoxStateInput {
            item_count: 0,
            disabled_option_count: 1,
            is_disabled: true,
            has_custom_label: true,
            has_custom_description: true,
            has_custom_error: true,
            has_custom_placeholder: true,
            has_custom_id_base: true,
            has_custom_class_name: true,
            has_custom_motion: true,
            is_controlled: true,
        }),
    );

    for token in [
        "ui-combo-box",
        "ui-combo-box--disabled",
        "ui-combo-box--empty",
        "ui-combo-box--has-description",
        "ui-combo-box--has-error",
        "ui-combo-box--has-disabled-options",
        "ui-combo-box--controlled",
        "ui-combo-box--custom-label",
        "ui-combo-box--custom-description",
        "ui-combo-box--custom-error",
        "ui-combo-box--custom-placeholder",
        "ui-combo-box--custom-id",
        "ui-combo-box--custom-motion",
        "ui-combo-box--custom-class",
        "custom",
    ] {
        assert!(
            class_name.contains(token),
            "composed class name should include `{token}`"
        );
    }
}
