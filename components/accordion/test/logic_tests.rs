use super::*;

#[test]
fn resolve_id_base_prefers_user_value_and_falls_back_to_generated() {
    assert_eq!(
        resolve_id_base(
            Some(" docs-accordion ".to_string()),
            "ui-accordion-1".to_string()
        ),
        "docs-accordion"
    );
    assert_eq!(
        resolve_id_base(Some("  ".to_string()), "ui-accordion-2".to_string()),
        "ui-accordion-2"
    );
    assert_eq!(
        resolve_id_base(None, "ui-accordion-3".to_string()),
        "ui-accordion-3"
    );
}

#[test]
fn resolve_item_label_trims_and_falls_back_when_empty() {
    assert_eq!(resolve_item_label(" Overview ".to_string(), 0), "Overview");
    assert_eq!(resolve_item_label("   ".to_string(), 0), "Section 1");
}

#[test]
fn resolve_item_key_prefers_explicit_value() {
    assert_eq!(resolve_item_key(Some(7), 0), 7);
    assert_eq!(resolve_item_key(None, 3), 3);
}

#[test]
fn derive_runtime_init_collects_requested_open_and_markers() {
    let runtime = derive_runtime_init(&[
        AccordionItemStateInput {
            key: 1,
            open: Some(true),
            default_open: false,
            is_disabled: false,
        },
        AccordionItemStateInput {
            key: 3,
            open: None,
            default_open: true,
            is_disabled: true,
        },
        AccordionItemStateInput {
            key: 5,
            open: Some(false),
            default_open: true,
            is_disabled: false,
        },
    ]);

    assert!(runtime.has_controlled_open);
    assert!(runtime.has_default_open);
    assert_eq!(runtime.item_keys, vec![1, 3, 5]);
    assert_eq!(runtime.requested_open, BTreeSet::from([1, 3]));
    assert!(runtime.has_per_item_disabled);
}

#[test]
fn apply_external_item_sync_toggles_target_key() {
    let current = BTreeSet::from([2, 4]);
    let next = apply_external_item_sync(&current, 6, true);
    assert_eq!(next, BTreeSet::from([2, 4, 6]));

    let next = apply_external_item_sync(&next, 4, false);
    assert_eq!(next, BTreeSet::from([2, 6]));
}

#[test]
fn plan_open_commit_normalizes_and_emits_changed_callback_states() {
    let before = BTreeSet::from([1, 2]);
    let requested_next = BTreeSet::from([2, 3]);
    let plan = plan_open_commit(
        AccordionSelectionMode::Multiple,
        &before,
        &requested_next,
        &[1, 2, 3],
        &[1, 2, 3],
        false,
    )
    .expect("changed plan expected");

    assert_eq!(plan.next, BTreeSet::from([2, 3]));
    assert_eq!(plan.changed_by_key, BTreeMap::from([(1, false), (3, true)]));
}

#[test]
fn plan_open_commit_returns_none_when_normalized_state_is_unchanged() {
    let before = BTreeSet::from([2]);
    let requested_next = BTreeSet::from([2]);
    let plan = plan_open_commit(
        AccordionSelectionMode::Single,
        &before,
        &requested_next,
        &[1, 2, 3],
        &[1, 2, 3],
        false,
    );
    assert!(plan.is_none());
}

#[test]
fn assign_item_keys_enforces_unique_numeric_keys() {
    let keys = assign_item_keys(&[Some(3), None, Some(3), None, Some(0)]);
    assert_eq!(keys, vec![3, 0, 1, 2, 4]);
}

#[test]
fn normalize_default_open_uses_empty_when_none() {
    let normalized =
        normalize_default_open_for_items(AccordionSelectionMode::Single, None, &[0, 1, 2], false);
    assert!(normalized.is_empty());
}

#[test]
fn normalize_open_for_single_mode_keeps_first_visible_key() {
    let normalized = normalize_open_for_items(
        AccordionSelectionMode::Single,
        &BTreeSet::from([2, 0]),
        &[1, 0, 2],
        false,
    );
    assert_eq!(normalized, BTreeSet::from([0]));
}

#[test]
fn toggle_open_for_items_respects_mode_and_valid_keys() {
    let next = toggle_open_for_items(
        AccordionSelectionMode::Multiple,
        &BTreeSet::from([1]),
        2,
        &[1, 2, 3],
        false,
    );
    assert_eq!(next, BTreeSet::from([1, 2]));

    let next = toggle_open_for_items(
        AccordionSelectionMode::Single,
        &BTreeSet::from([1]),
        2,
        &[1, 2, 3],
        false,
    );
    assert_eq!(next, BTreeSet::from([2]));
}

#[test]
fn disallow_empty_selection_keeps_at_least_one_item_open() {
    let normalized = normalize_open_for_items(
        AccordionSelectionMode::Single,
        &BTreeSet::new(),
        &[5, 6],
        true,
    );
    assert_eq!(normalized, BTreeSet::from([5]));

    let next = toggle_open_for_items(
        AccordionSelectionMode::Single,
        &BTreeSet::from([6]),
        6,
        &[5, 6],
        true,
    );
    assert_eq!(next, BTreeSet::from([6]));
}

#[test]
fn resolve_open_sources_return_closed_set_values() {
    assert_eq!(
        resolve_open_state_source(true).as_str(),
        AccordionOpenStateSource::Controlled.as_str()
    );
    assert_eq!(
        resolve_open_state_source(false).as_str(),
        AccordionOpenStateSource::Uncontrolled.as_str()
    );

    assert_eq!(
        resolve_open_init_source(true, true).as_str(),
        AccordionOpenInitSource::External.as_str()
    );
    assert_eq!(
        resolve_open_init_source(false, true).as_str(),
        AccordionOpenInitSource::Default.as_str()
    );
    assert_eq!(
        resolve_open_init_source(false, false).as_str(),
        AccordionOpenInitSource::Empty.as_str()
    );

    for source in [
        AccordionOpenChangeSource::Init,
        AccordionOpenChangeSource::ExternalSync,
        AccordionOpenChangeSource::Keyboard,
        AccordionOpenChangeSource::Pointer,
        AccordionOpenChangeSource::Programmatic,
    ] {
        assert!(!source.as_str().is_empty());
    }
}

#[test]
fn agent_contract_is_schema_typed_and_stage_mapped() {
    let contract = resolve_agent_contract(AccordionOpenChangeSource::ExternalSync, 3, 1);
    assert_eq!(contract.schema_name, "ui.accordion.agent-contract");
    assert_eq!(contract.schema_version.as_str(), "1");
    assert_eq!(contract.intent.as_str(), "disclosure");
    assert_eq!(contract.action.as_str(), "external-sync");
    assert_eq!(contract.state.as_str(), "has-open");
    assert_eq!(contract.output_status, AccordionAgentOutputStatus::Verified);
    assert_eq!(contract.stream_support.as_str(), "unsupported");
    assert_eq!(contract.stream_fallback.as_str(), "full-snapshot");
    assert!(contract.capabilities.can_toggle);
    assert!(contract.capabilities.can_focus_move);
    assert!(contract.capabilities.can_external_sync);
    assert!(contract.capabilities.can_programmatic_replay);
}
