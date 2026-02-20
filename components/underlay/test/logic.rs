use super::*;

#[test]
fn normalize_helpers_trim_text_and_filter_blanks() {
    assert_eq!(normalize_optional_text(None), None);
    assert_eq!(normalize_optional_text(Some(" ".to_string())), None);
    assert_eq!(
        normalize_optional_text(Some(" docs-underlay ".to_string())),
        Some("docs-underlay".to_string())
    );
}

#[test]
fn normalize_open_state_supports_is_open_open_and_default_open() {
    let controlled = normalize_open_state(UnderlayOpenStateInput {
        is_open: Some(Signal::derive(|| true)),
        open: None,
        default_open: Some(false),
        on_open_change: None,
    });
    assert!(matches!(controlled.mode, UnderlayOpenMode::Controlled));
    assert!(controlled.open.is_some());
    assert_eq!(controlled.open_prop_source_attr, "is_open");

    let legacy_controlled = normalize_open_state(UnderlayOpenStateInput {
        is_open: None,
        open: Some(Signal::derive(|| false)),
        default_open: Some(true),
        on_open_change: None,
    });
    assert!(matches!(
        legacy_controlled.mode,
        UnderlayOpenMode::Controlled
    ));
    assert_eq!(legacy_controlled.open_prop_source_attr, "open");

    let uncontrolled = normalize_open_state(UnderlayOpenStateInput {
        is_open: None,
        open: None,
        default_open: Some(true),
        on_open_change: Some(Callback::new(|_: bool| {})),
    });
    assert!(matches!(uncontrolled.mode, UnderlayOpenMode::Uncontrolled));
    assert!(uncontrolled.default_open);
    assert!(uncontrolled.has_default_open);
    assert!(uncontrolled.has_open_change_handler);
}

#[test]
fn normalize_flags_prefers_is_prefixed_inputs() {
    let flags = normalize_flags(UnderlayFlagsInput {
        is_transparent: Some(true),
        transparent: Some(false),
        is_disabled: Some(true),
        disabled: Some(false),
    });

    assert!(flags.transparent);
    assert!(flags.disabled);
    assert_eq!(flags.transparent_prop_source_attr, "is_transparent");
    assert_eq!(flags.disabled_prop_source_attr, "is_disabled");
}

#[test]
fn resolve_view_state_annotates_open_and_prop_sources() {
    let open_state = normalize_open_state(UnderlayOpenStateInput {
        is_open: None,
        open: None,
        default_open: Some(true),
        on_open_change: Some(Callback::new(|_: bool| {})),
    });
    let flags = normalize_flags(UnderlayFlagsInput {
        is_transparent: None,
        transparent: Some(true),
        is_disabled: None,
        disabled: Some(false),
    });

    let state = resolve_view_state(UnderlayViewStateInput {
        slot: UnderlaySlot::Root,
        open: true,
        transparent: flags.transparent,
        disabled: flags.disabled,
        has_on_close: true,
        has_custom_class_name: true,
        open_state,
        flags,
    });

    assert_eq!(state.part.state_attr, "open");
    assert_eq!(state.part.close_mode_attr, "interactive");
    assert_eq!(state.open_mode_attr, "uncontrolled");
    assert_eq!(state.open_source_attr, "default");
    assert_eq!(state.open_change_source_attr, "provided");
    assert_eq!(state.transparent_prop_source_attr, "transparent");
    assert_eq!(state.disabled_prop_source_attr, "disabled");
}

#[test]
fn resolve_agent_contract_is_schema_typed_and_traceable() {
    let controlled_state = resolve_view_state(UnderlayViewStateInput {
        slot: UnderlaySlot::Root,
        open: true,
        transparent: false,
        disabled: false,
        has_on_close: true,
        has_custom_class_name: false,
        open_state: normalize_open_state(UnderlayOpenStateInput {
            is_open: Some(Signal::derive(|| true)),
            open: None,
            default_open: None,
            on_open_change: Some(Callback::new(|_: bool| {})),
        }),
        flags: normalize_flags(UnderlayFlagsInput {
            is_transparent: None,
            transparent: None,
            is_disabled: None,
            disabled: None,
        }),
    });
    let controlled_contract = resolve_agent_contract(controlled_state);
    assert_eq!(
        controlled_contract.schema_name,
        "ui.underlay.agent-contract"
    );
    assert_eq!(controlled_contract.schema_version.as_str(), "1");
    assert_eq!(controlled_contract.intent.as_str(), "overlay-dismiss");
    assert_eq!(controlled_contract.action.as_str(), "request-close");
    assert_eq!(controlled_contract.state.as_str(), "open");
    assert_eq!(controlled_contract.source.as_str(), "controlled-external");
    assert_eq!(controlled_contract.stream_support.as_str(), "optional");
    assert_eq!(controlled_contract.stream_fallback.as_str(), "snapshot");
    assert!(controlled_contract.capabilities.can_dismiss);
    assert!(controlled_contract.capabilities.can_external_sync);

    let disabled_state = resolve_view_state(UnderlayViewStateInput {
        slot: UnderlaySlot::Root,
        open: false,
        transparent: false,
        disabled: true,
        has_on_close: false,
        has_custom_class_name: false,
        open_state: normalize_open_state(UnderlayOpenStateInput {
            is_open: None,
            open: None,
            default_open: None,
            on_open_change: None,
        }),
        flags: normalize_flags(UnderlayFlagsInput {
            is_transparent: None,
            transparent: None,
            is_disabled: Some(true),
            disabled: None,
        }),
    });
    let disabled_contract = resolve_agent_contract(disabled_state);
    assert_eq!(disabled_contract.action.as_str(), "static-barrier");
    assert_eq!(disabled_contract.state.as_str(), "disabled");
    assert_eq!(
        disabled_contract.source.as_str(),
        "uncontrolled-implicit-default"
    );
    assert_eq!(disabled_contract.stream_support.as_str(), "optional");
    assert_eq!(disabled_contract.stream_fallback.as_str(), "snapshot");
    assert!(!disabled_contract.capabilities.can_dismiss);
    assert!(!disabled_contract.capabilities.can_external_sync);
}

#[test]
fn compose_class_name_includes_state_and_custom_markers() {
    let part = resolve_state(UnderlayPartStateInput {
        slot: UnderlaySlot::Root,
        open: true,
        transparent: false,
        disabled: true,
        has_on_close: true,
        has_custom_transparent: false,
        has_custom_disabled: true,
        has_custom_close_handler: true,
        has_custom_class_name: true,
    });

    let class_name = compose_class_name(Some("docs-underlay".to_string()), part);
    assert!(class_name.contains("ui-underlay"));
    assert!(class_name.contains("ui-underlay--disabled"));
    assert!(class_name.contains("ui-underlay--custom-disabled"));
    assert!(class_name.contains("ui-underlay--custom-close"));
    assert!(class_name.contains("ui-underlay--custom-class"));
    assert!(class_name.contains("docs-underlay"));
}
