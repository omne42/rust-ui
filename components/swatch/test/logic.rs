use super::*;

#[test]
fn compose_class_name_includes_expected_tokens() {
    let state = resolve_state(SwatchStateInput {
        size: SwatchSize::L,
        border: SwatchBorder::Light,
        rounding: SwatchRounding::Full,
        shape: SwatchShape::Rectangle,
        has_color: false,
        nothing: true,
        mixed_value: false,
        disabled: true,
        decorative: true,
        has_custom_class_name: true,
    });

    let class_name = compose_class_name(Some("docs-swatch".to_string()), state);
    for token in [
        "ui-swatch",
        "ui-swatch--size-l",
        "ui-swatch--border-light",
        "ui-swatch--rounding-full",
        "ui-swatch--shape-rectangle",
        "ui-swatch--nothing",
        "ui-swatch--disabled",
        "ui-swatch--static",
        "ui-swatch--custom-class",
        "docs-swatch",
    ] {
        assert!(
            class_name.contains(token),
            "composed class name should include `{token}`"
        );
    }
}

#[test]
fn compose_inline_style_includes_css_variable() {
    assert_eq!(
        compose_inline_style(Some("#ff0000")),
        Some("--ui-swatch-color: #ff0000;".to_string())
    );
}

#[test]
fn normalize_default_selected_falls_back_to_false() {
    assert!(!normalize_default_selected(None));
    assert!(normalize_default_selected(Some(true)));
    assert!(!normalize_default_selected(Some(false)));
}

#[test]
fn resolve_selection_control_state_maps_control_and_source_markers() {
    let controlled = resolve_selection_control_state(SwatchSelectionControlInput {
        has_controlled_selected: true,
        default_selected: Some(true),
        has_on_selected_change: true,
    });
    assert!(controlled.default_selected);
    assert!(controlled.is_controlled_selected);
    assert!(!controlled.is_uncontrolled_selected);
    assert_eq!(controlled.control_mode_attr, "controlled");
    assert_eq!(controlled.default_selected_source_attr, "custom");
    assert_eq!(controlled.selected_change_source_attr, "custom");

    let uncontrolled = resolve_selection_control_state(SwatchSelectionControlInput {
        has_controlled_selected: false,
        default_selected: None,
        has_on_selected_change: false,
    });
    assert!(!uncontrolled.default_selected);
    assert!(!uncontrolled.is_controlled_selected);
    assert!(uncontrolled.is_uncontrolled_selected);
    assert_eq!(uncontrolled.control_mode_attr, "uncontrolled");
    assert_eq!(uncontrolled.default_selected_source_attr, "default");
    assert_eq!(uncontrolled.selected_change_source_attr, "none");
}

#[test]
fn resolve_agent_contract_is_schema_typed_and_snapshot_fallback_explicit() {
    let state = resolve_state(SwatchStateInput {
        size: SwatchSize::M,
        border: SwatchBorder::Default,
        rounding: SwatchRounding::Default,
        shape: SwatchShape::Square,
        has_color: true,
        nothing: false,
        mixed_value: false,
        disabled: false,
        decorative: false,
        has_custom_class_name: false,
    });
    let contract = resolve_agent_contract(state, true, SwatchAgentSource::TogglePress);

    assert_eq!(contract.schema_name, "ui.swatch.agent-contract");
    assert_eq!(contract.schema_version.as_str(), "1");
    assert_eq!(contract.intent.as_str(), "color-selection");
    assert_eq!(contract.action.as_str(), "toggle-press");
    assert_eq!(contract.state.as_str(), "selected");
    assert_eq!(contract.source.as_str(), "toggle-press");
    assert_eq!(contract.output_status.as_str(), "submittable");
    assert_eq!(contract.stream_support.as_str(), "unsupported");
    assert_eq!(contract.stream_fallback.as_str(), "full-snapshot");
    assert!(contract.capabilities.can_toggle);
    assert!(contract.capabilities.can_disable);
}
