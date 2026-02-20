use super::*;

#[test]
fn state_primitives_are_reexported_from_ui_state_primitives() {
    assert_eq!(
        normalize_id_base("   ".to_string()),
        ui_state_primitives::dropdown::DEFAULT_ID_BASE
    );
    assert_eq!(
        normalize_aria_label(None),
        (
            ui_state_primitives::dropdown::DEFAULT_ARIA_LABEL.into(),
            false
        )
    );
    assert_eq!(
        focus_strategy_for_open_key("ArrowDown"),
        Some(DropdownOpenFocusStrategy::First)
    );
}

#[test]
fn normalize_disabled_state_prefers_is_prefix() {
    assert!(normalize_disabled_state(DisabledStateInput {
        is_disabled: Some(true),
        disabled: false,
    }));
    assert!(!normalize_disabled_state(DisabledStateInput {
        is_disabled: None,
        disabled: false,
    }));
}

#[test]
fn normalize_open_state_prefers_is_open_and_preserves_triplet() {
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
            .expect("normalized open should exist")
            .get_untracked()
    );
    assert_eq!(open_state.default_open, Some(false));
    assert!(open_state.on_open_change.is_some());
}

#[test]
fn compose_class_name_includes_state_markers() {
    let state = resolve_state(DropdownStateInput {
        item_count: 0,
        disabled: true,
        close_on_action: true,
        has_custom_aria_label: false,
        has_custom_class_name: true,
        is_controlled: false,
        has_disabled_items: false,
        has_item_kinds: false,
    });

    let class_name = compose_class_name(Some("docs-dropdown-custom".to_string()), state);

    assert!(class_name.contains("ui-dropdown"));
    assert!(class_name.contains("ui-dropdown--disabled"));
    assert!(class_name.contains("ui-dropdown--empty"));
    assert!(class_name.contains("ui-dropdown--custom-class"));
    assert!(class_name.contains("docs-dropdown-custom"));
}
