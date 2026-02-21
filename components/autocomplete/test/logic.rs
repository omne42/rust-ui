use super::*;

#[test]
fn state_primitives_are_reexported_from_ui_state_primitives() {
    assert_eq!(normalize_label("  City  ".to_string()), "City");
    assert_eq!(
        normalize_id_base("   ".to_string()),
        ui_state_primitives::autocomplete::DEFAULT_ID_BASE
    );
    assert_eq!(
        resolve_placeholder(None),
        ui_state_primitives::autocomplete::DEFAULT_PLACEHOLDER
    );
    assert_eq!(
        resolve_empty_message(None),
        ui_state_primitives::autocomplete::DEFAULT_EMPTY_MESSAGE
    );
}

#[test]
fn input_state_reducer_is_consumed_from_ui_state_primitives() {
    let next = reduce_input_state(
        AutocompleteInputState {
            query: "Sh".to_string(),
            has_typed: true,
        },
        AutocompleteInputEvent::SyncFromSelection {
            selected_label: Some("Shenzhen".to_string()),
        },
    );
    assert_eq!(
        next,
        AutocompleteInputState {
            query: "Shenzhen".to_string(),
            has_typed: false
        }
    );
}

#[test]
fn input_state_event_reduction_is_centralized_via_logic_helpers() {
    let sync_from_selection = reduce_sync_from_selection(
        InputStateSource {
            query: "Sh".to_string(),
            has_typed: true,
        },
        Some("Shenzhen".to_string()),
    );
    assert_eq!(
        sync_from_selection,
        AutocompleteInputState {
            query: "Shenzhen".to_string(),
            has_typed: false,
        }
    );

    let option_committed = reduce_after_option_commit(
        InputStateSource {
            query: "Sh".to_string(),
            has_typed: true,
        },
        "Shanghai".to_string(),
    );
    assert_eq!(
        option_committed,
        AutocompleteInputState {
            query: "Shanghai".to_string(),
            has_typed: false,
        }
    );

    let blurred = reduce_after_input_blur(InputStateSource {
        query: "Shang".to_string(),
        has_typed: true,
    });
    assert_eq!(
        blurred,
        AutocompleteInputState {
            query: "Shang".to_string(),
            has_typed: false,
        }
    );

    let changed = reduce_after_input_change(
        InputStateSource {
            query: String::new(),
            has_typed: false,
        },
        "Sh".to_string(),
    );
    assert_eq!(
        changed,
        AutocompleteInputState {
            query: "Sh".to_string(),
            has_typed: true,
        }
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
fn normalize_selection_change_prefers_on_selected_index_change_and_keeps_legacy_alias_path() {
    let (legacy_selected, set_legacy_selected) = signal(None::<usize>);
    let (preferred_selected, set_preferred_selected) = signal(None::<usize>);
    let (controlled_selected, _set_controlled_selected) = signal(Some(3_usize));
    let preferred_callback = Callback::new(move |next: Option<usize>| {
        set_preferred_selected.set(next);
    });

    let preferred = normalize_selection_change(SelectionChangeInput {
        selected_index: Some(controlled_selected.into()),
        default_selected_index: Some(1),
        on_selected_index_change: Some(preferred_callback),
        set_selected_index: Some(set_legacy_selected),
        item_count: 5,
    });
    preferred
        .on_selected_index_change
        .expect("preferred callback should exist")
        .run(Some(2));
    assert_eq!(preferred_selected.get_untracked(), Some(2));
    assert_eq!(legacy_selected.get_untracked(), None);
    assert_eq!(
        preferred.change_source,
        SelectedChangeSource::OnSelectedIndexChange
    );
    assert!(preferred.is_controlled);
    assert_eq!(preferred.selected_source, SelectedSource::SelectedIndex);
    assert_eq!(
        preferred.change_source.as_attr(),
        "on_selected_index_change"
    );
    assert_eq!(preferred.selected_source.as_attr(), "selected_index");
    assert_eq!(preferred.default_selected_index, Some(1));

    let legacy_only = normalize_selection_change(SelectionChangeInput {
        selected_index: None,
        default_selected_index: Some(9),
        on_selected_index_change: None,
        set_selected_index: Some(set_legacy_selected),
        item_count: 4,
    });
    legacy_only
        .on_selected_index_change
        .expect("legacy callback adapter should exist")
        .run(Some(1));
    assert_eq!(legacy_selected.get_untracked(), Some(1));
    assert_eq!(
        legacy_only.change_source,
        SelectedChangeSource::SetSelectedIndex
    );
    assert!(!legacy_only.is_controlled);
    assert_eq!(
        legacy_only.selected_source,
        SelectedSource::DefaultSelectedIndex
    );
    assert_eq!(legacy_only.change_source.as_attr(), "set_selected_index");
    assert_eq!(
        legacy_only.selected_source.as_attr(),
        "default_selected_index"
    );
    assert_eq!(legacy_only.default_selected_index, None);

    let none = normalize_selection_change(SelectionChangeInput {
        selected_index: None,
        default_selected_index: Some(2),
        on_selected_index_change: None,
        set_selected_index: None,
        item_count: 3,
    });
    assert!(none.on_selected_index_change.is_none());
    assert_eq!(none.change_source, SelectedChangeSource::None);
    assert_eq!(none.change_source.as_attr(), "none");
    assert_eq!(none.default_selected_index, Some(2));
}

#[test]
fn normalize_root_state_centralizes_normalization_and_state_derivation() {
    let root = normalize_root_state(RootStateInput {
        id_base: "   ".to_string(),
        has_custom_id_base: false,
        label: "  ".to_string(),
        placeholder: Some("  ".to_string()),
        empty_message: Some("  Nothing  ".to_string()),
        i18n_empty_message: Some("  From i18n  ".to_string()),
        description: Some("  desc  ".to_string()),
        error: Some("  err  ".to_string()),
        class_name: Some("  custom  ".to_string()),
        item_count: 3,
        disabled_indices: vec![1, 1, 9],
        is_disabled: true,
        is_controlled: true,
        has_custom_motion: true,
    });

    assert_eq!(
        root.id_base,
        ui_state_primitives::autocomplete::DEFAULT_ID_BASE
    );
    assert_eq!(root.label, ui_state_primitives::autocomplete::DEFAULT_LABEL);
    assert_eq!(
        root.placeholder,
        ui_state_primitives::autocomplete::DEFAULT_PLACEHOLDER
    );
    assert_eq!(root.empty_message, "Nothing");
    assert_eq!(root.description.as_deref(), Some("desc"));
    assert_eq!(root.error.as_deref(), Some("err"));
    assert_eq!(root.disabled_indices, vec![1]);
    assert!(root.state.is_disabled);
    assert!(root.state.is_controlled);
    assert!(root.class_name.contains("ui-autocomplete"));
}

#[test]
fn normalize_root_state_keeps_empty_message_default_priority_in_logic() {
    let from_i18n = normalize_root_state(RootStateInput {
        id_base: "city".to_string(),
        has_custom_id_base: true,
        label: "City".to_string(),
        placeholder: None,
        empty_message: None,
        i18n_empty_message: Some("  I18n Empty  ".to_string()),
        description: None,
        error: None,
        class_name: None,
        item_count: 2,
        disabled_indices: vec![],
        is_disabled: false,
        is_controlled: false,
        has_custom_motion: false,
    });
    assert_eq!(from_i18n.empty_message, "I18n Empty");

    let from_primitive_default = normalize_root_state(RootStateInput {
        id_base: "city".to_string(),
        has_custom_id_base: true,
        label: "City".to_string(),
        placeholder: None,
        empty_message: None,
        i18n_empty_message: None,
        description: None,
        error: None,
        class_name: None,
        item_count: 2,
        disabled_indices: vec![],
        is_disabled: false,
        is_controlled: false,
        has_custom_motion: false,
    });
    assert_eq!(
        from_primitive_default.empty_message,
        ui_state_primitives::autocomplete::DEFAULT_EMPTY_MESSAGE
    );
}

#[test]
fn resolve_id_base_prefers_explicit_input_and_falls_back_to_generated_seeded_value() {
    assert_eq!(
        resolve_id_base(
            " docs-autocomplete ".to_string(),
            "autocomplete-7".to_string()
        ),
        "docs-autocomplete"
    );
    assert_eq!(
        resolve_id_base("   ".to_string(), "autocomplete-7".to_string()),
        "autocomplete-7"
    );
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
fn resolve_agent_contract_is_schema_typed_and_traceable() {
    let state = resolve_state(AutocompleteStateInput {
        item_count: 3,
        disabled_option_count: 1,
        is_disabled: false,
        has_custom_label: false,
        has_custom_description: true,
        has_custom_error: false,
        has_custom_placeholder: true,
        has_custom_id_base: true,
        has_custom_class_name: true,
        has_custom_motion: true,
        is_controlled: true,
    });

    let open_query = resolve_agent_contract(AutocompleteAgentContractInput {
        is_open: true,
        is_disabled: false,
        has_typed: true,
        has_selection: false,
        is_open_controlled: true,
        selected_source: SelectedSource::SelectedIndex,
        selected_change_source: SelectedChangeSource::OnSelectedIndexChange,
        render_state: state,
    });
    assert_eq!(open_query.schema_name, AUTOCOMPLETE_AGENT_SCHEMA);
    assert_eq!(
        open_query.schema_version,
        AutocompleteAgentSchemaVersion::V1
    );
    assert_eq!(open_query.intent, AutocompleteAgentIntent::SuggestAndSelect);
    assert_eq!(open_query.action, AutocompleteAgentAction::Query);
    assert_eq!(open_query.state, AutocompleteAgentState::Open);
    assert_eq!(open_query.source, AutocompleteAgentSource::StatePrimitives);
    assert_eq!(
        open_query.output_status,
        AutocompleteAgentOutputStatus::Verified
    );
    assert_eq!(
        open_query.stream_support,
        AutocompleteAgentStreamSupport::Unsupported
    );
    assert_eq!(
        open_query.stream_fallback,
        AutocompleteAgentStreamFallback::Snapshot
    );
    assert_eq!(
        open_query.stream_mode,
        AutocompleteAgentStreamMode::Snapshot
    );
    assert_eq!(AutocompleteAgentStreamMode::Streaming.as_str(), "streaming");
    assert_eq!(AutocompleteAgentStreamMode::Snapshot.as_str(), "snapshot");
    assert_eq!(open_query.state_source, "controlled");
    assert_eq!(open_query.open_value_source, "controlled");
    assert_eq!(open_query.selected_source, "selected_index");
    assert_eq!(
        open_query.selected_change_source,
        "on_selected_index_change"
    );
    assert_eq!(open_query.config_policy, "whitelist");

    let closed_selected = resolve_agent_contract(AutocompleteAgentContractInput {
        is_open: false,
        is_disabled: false,
        has_typed: false,
        has_selection: true,
        is_open_controlled: false,
        selected_source: SelectedSource::DefaultSelectedIndex,
        selected_change_source: SelectedChangeSource::SetSelectedIndex,
        render_state: state,
    });
    assert_eq!(
        closed_selected.action,
        AutocompleteAgentAction::CommitSelection
    );
    assert_eq!(closed_selected.state, AutocompleteAgentState::Closed);
    assert_eq!(closed_selected.state_source, "uncontrolled");
    assert_eq!(closed_selected.open_value_source, "uncontrolled");
    assert_eq!(closed_selected.selected_source, "default_selected_index");
    assert_eq!(closed_selected.selected_change_source, "set_selected_index");

    let disabled_idle = resolve_agent_contract(AutocompleteAgentContractInput {
        is_open: false,
        is_disabled: true,
        has_typed: false,
        has_selection: false,
        is_open_controlled: false,
        selected_source: SelectedSource::DefaultSelectedIndex,
        selected_change_source: SelectedChangeSource::None,
        render_state: state,
    });
    assert_eq!(disabled_idle.action, AutocompleteAgentAction::Idle);
    assert_eq!(disabled_idle.state, AutocompleteAgentState::Disabled);
    assert_eq!(disabled_idle.selected_change_source, "none");
}

#[test]
fn compose_class_name_includes_state_markers() {
    let class_name = compose_class_name(
        Some("custom".to_string()),
        resolve_state(AutocompleteStateInput {
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
        "ui-autocomplete",
        "ui-autocomplete--disabled",
        "ui-autocomplete--empty",
        "ui-autocomplete--has-description",
        "ui-autocomplete--has-error",
        "ui-autocomplete--has-disabled-options",
        "ui-autocomplete--controlled",
        "ui-autocomplete--custom-label",
        "ui-autocomplete--custom-description",
        "ui-autocomplete--custom-error",
        "ui-autocomplete--custom-placeholder",
        "ui-autocomplete--custom-id",
        "ui-autocomplete--custom-motion",
        "ui-autocomplete--custom-class",
        "custom",
    ] {
        assert!(
            class_name.contains(token),
            "composed class name should include `{token}`"
        );
    }
}
