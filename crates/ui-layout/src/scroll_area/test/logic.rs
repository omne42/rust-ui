use super::*;

#[test]
fn normalize_disable_state_prefers_is_prefixed_prop() {
    let state = normalize_disable_state(ScrollAreaDisableInput {
        is_disabled: Some(true),
    });

    assert!(state.is_disabled);
    assert_eq!(
        state.disabled_source_attr,
        ScrollAreaDisabledSourceAttr::IsProp
    );
}

#[test]
fn normalize_disable_state_uses_default_when_prop_absent() {
    let state = normalize_disable_state(ScrollAreaDisableInput { is_disabled: None });

    assert!(!state.is_disabled);
    assert_eq!(
        state.disabled_source_attr,
        ScrollAreaDisabledSourceAttr::Default
    );
}

#[test]
fn normalize_root_state_centralizes_defaults_and_sources() {
    let root = normalize_root_state(ScrollAreaRootInput {
        class_name: Some("  docs-scroll-area-custom ".to_string()),
        aria_label: None,
        fallback_aria_label: DEFAULT_ARIA_LABEL.into(),
        orientation: ScrollAreaOrientation::Vertical,
        max_height_px: Some(180),
        disabled: ScrollAreaDisableInput {
            is_disabled: Some(true),
        },
    });

    assert_eq!(root.class_name, Some("docs-scroll-area-custom".to_string()));
    assert_eq!(root.aria_label, DEFAULT_ARIA_LABEL);
    assert_eq!(root.state.orientation_attr, "vertical");
    assert!(root.state.disabled);
    assert_eq!(
        root.state.max_height_attr,
        ui_state_primitives::scroll_area::ScrollAreaMaxHeightAttr::Custom
    );
    assert_eq!(
        root.state.aria_source_attr,
        ui_state_primitives::scroll_area::ScrollAreaSourceAttr::Default
    );
    assert_eq!(
        root.state.class_source_attr,
        ui_state_primitives::scroll_area::ScrollAreaSourceAttr::Custom
    );
    assert_eq!(
        root.disabled_source_attr,
        ScrollAreaDisabledSourceAttr::IsProp
    );
}

#[test]
fn normalize_aria_label_with_fallback_prefers_prop_then_fallback_then_default() {
    assert_eq!(
        normalize_aria_label_with_fallback(
            Some("  Custom label  ".to_string()),
            "Localized fallback",
        ),
        ("Custom label".to_string(), true)
    );
    assert_eq!(
        normalize_aria_label_with_fallback(None, "  Localized fallback  "),
        ("Localized fallback".to_string(), false)
    );
    assert_eq!(
        normalize_aria_label_with_fallback(None, "   "),
        (DEFAULT_ARIA_LABEL.into(), false)
    );
}

#[test]
fn compose_class_name_contains_state_markers() {
    let class_name = compose_class_name(
        Some("custom".to_string()),
        resolve_state(ScrollAreaStateInput {
            orientation: ScrollAreaOrientation::Both,
            disabled: true,
            max_height_px: Some(160),
            has_custom_aria_label: false,
            has_custom_class_name: true,
        }),
    );

    for expected in [
        "ui-scroll-area",
        "ui-scroll-area--both",
        "ui-scroll-area--disabled",
        "ui-scroll-area--max-height-custom",
        "ui-scroll-area--custom-class",
        "custom",
    ] {
        assert!(
            class_name.contains(expected),
            "expected class list to contain `{expected}`, got `{class_name}`"
        );
    }
}

#[test]
fn resolve_agent_contract_uses_closed_set_markers() {
    let enabled = resolve_agent_contract(
        resolve_state(ScrollAreaStateInput {
            orientation: ScrollAreaOrientation::Vertical,
            disabled: false,
            max_height_px: None,
            has_custom_aria_label: false,
            has_custom_class_name: false,
        }),
        ScrollAreaDisabledSourceAttr::Default,
    );
    assert_eq!(enabled.schema_attr, "ui.scroll-area.agent-contract.v1");
    assert_eq!(enabled.stream_support_attr, "unsupported");
    assert_eq!(enabled.stream_fallback_attr, "snapshot");
    assert_eq!(enabled.stream_mode_attr, "snapshot");
    assert_eq!(enabled.output_status_attr, "verified");
    assert_eq!(enabled.intent_attr, "inspect-region");
    assert_eq!(enabled.action_attr, "observe");
    assert_eq!(enabled.state_attr, "enabled");
    assert_eq!(enabled.source_attr, "default");

    let disabled = resolve_agent_contract(
        resolve_state(ScrollAreaStateInput {
            orientation: ScrollAreaOrientation::Vertical,
            disabled: true,
            max_height_px: None,
            has_custom_aria_label: false,
            has_custom_class_name: false,
        }),
        ScrollAreaDisabledSourceAttr::IsProp,
    );
    assert_eq!(disabled.action_attr, "disabled");
    assert_eq!(disabled.state_attr, "disabled");
    assert_eq!(disabled.source_attr, "is-prop");
}
