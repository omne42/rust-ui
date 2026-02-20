use super::*;

#[test]
fn normalize_value_axis_tracks_on_value_change_source() {
    let (preferred_value, set_preferred_value) = signal(String::new());
    let on_value_change = Callback::new(move |next: String| set_preferred_value.set(next));

    let state = normalize_value_axis(ValueAxisInput {
        value: None,
        default_value: Some("seed".to_string()),
        on_value_change: Some(on_value_change),
    });

    let callback = state
        .on_value_change
        .expect("value axis should keep normalized callback");
    callback.run("next".to_string());

    assert_eq!(state.control_mode_attr, "uncontrolled");
    assert_eq!(state.default_value_source_attr, "custom");
    assert_eq!(state.value_change_source_attr, "on_value_change");
    assert_eq!(preferred_value.get_untracked(), "next");
}

#[test]
fn normalize_value_axis_allows_absent_change_handler() {
    let state = normalize_value_axis(ValueAxisInput {
        value: None,
        default_value: None,
        on_value_change: None,
    });

    assert!(state.on_value_change.is_none());
    assert_eq!(state.value_change_source_attr, "none");
}

#[test]
fn normalize_accessibility_state_prefers_is_prefixed_inputs() {
    let (preferred_required, _set_preferred_required) = signal(true);
    let (preferred_invalid, _set_preferred_invalid) = signal(true);

    let state = normalize_accessibility_state(AccessibilityStateInput {
        is_disabled: Some(true),
        is_read_only: Some(true),
        is_required: Some(preferred_required.into()),
        is_invalid: Some(preferred_invalid.into()),
    });

    assert!(state.is_disabled);
    assert!(state.is_read_only);
    assert!(state.is_required.get_untracked());
    assert!(state.is_invalid.get_untracked());
}

#[test]
fn resolve_root_class_normalizes_optional_class_name() {
    let base = resolve_root_class(None);
    assert_eq!(base.class, "ui-search-field");
    assert!(!base.has_custom_class_name);
    assert_eq!(base.class_source_attr, "default");

    let custom = resolve_root_class(Some("  docs-search  ".to_string()));
    assert_eq!(custom.class, "ui-search-field docs-search");
    assert!(custom.has_custom_class_name);
    assert_eq!(custom.class_source_attr, "custom");
}

#[test]
fn resolve_clear_button_label_prefers_prop_then_i18n_then_default() {
    let prop = resolve_clear_button_label(ClearButtonLabelInput {
        aria_label: Some("  Clear search box  ".to_string()),
        i18n_clear_aria_label: Some("Effacer".to_string()),
    });
    assert_eq!(prop.aria_label, "Clear search box");
    assert_eq!(prop.source_attr, "prop");

    let i18n = resolve_clear_button_label(ClearButtonLabelInput {
        aria_label: None,
        i18n_clear_aria_label: Some("  Effacer  ".to_string()),
    });
    assert_eq!(i18n.aria_label, "Effacer");
    assert_eq!(i18n.source_attr, "i18n");

    let fallback = resolve_clear_button_label(ClearButtonLabelInput {
        aria_label: Some("   ".to_string()),
        i18n_clear_aria_label: None,
    });
    assert_eq!(fallback.aria_label, DEFAULT_CLEAR_BUTTON_ARIA_LABEL);
    assert_eq!(fallback.source_attr, "default");
}

#[test]
fn search_field_agent_contract_exposes_closed_schema_markers() {
    let contract = search_field_agent_contract();
    assert_eq!(contract.schema_attr, "ui.search-field");
    assert_eq!(
        contract.schema_version_attr,
        SearchFieldAgentSchemaVersion::V1.as_attr()
    );
    assert_eq!(
        contract.intent_attr,
        SearchFieldAgentIntent::FormSearchInput.as_attr()
    );
    assert_eq!(
        contract.action_model_attr,
        SearchFieldAgentActionModel::InputSubmitClear.as_attr()
    );
    assert_eq!(
        contract.state_axis_attr,
        "state|value|requirement|disabled|readonly|focus-visible|empty"
    );
    assert_eq!(
        contract.source_axis_attr,
        "class|clear-label|value-axis|locale"
    );
}
