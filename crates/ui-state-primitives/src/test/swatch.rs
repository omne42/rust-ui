use super::*;

#[test]
fn enum_contracts_are_stable() {
    assert_eq!(SwatchSize::Xs.class_name(), "ui-swatch--size-xs");
    assert_eq!(SwatchSize::L.as_attr(), "l");

    assert_eq!(SwatchBorder::Light.class_name(), "ui-swatch--border-light");
    assert_eq!(SwatchBorder::None.as_attr(), "none");

    assert_eq!(
        SwatchRounding::Default.class_name(),
        "ui-swatch--rounding-default"
    );
    assert_eq!(SwatchRounding::Full.as_attr(), "full");

    assert_eq!(
        SwatchShape::Rectangle.class_name(),
        "ui-swatch--shape-rectangle"
    );
    assert_eq!(SwatchShape::Square.as_attr(), "square");
}

#[test]
fn normalize_optional_text_filters_blank_values() {
    assert_eq!(normalize_optional_text(None), None);
    assert_eq!(normalize_optional_text(Some("\n\t ".to_string())), None);
    assert_eq!(
        normalize_optional_text(Some("  docs-swatch  ".to_string())),
        Some("docs-swatch".to_string())
    );
}

#[test]
fn sanitize_color_value_allows_safe_css_tokens_and_rejects_unsafe_values() {
    assert_eq!(
        sanitize_color_value(Some("  #ff0000  ".to_string())),
        Some("#ff0000".to_string())
    );
    assert_eq!(
        sanitize_color_value(Some("rgb(12, 24, 36)".to_string())),
        Some("rgb(12, 24, 36)".to_string())
    );
    assert_eq!(
        sanitize_color_value(Some("javascript:alert(1)".to_string())),
        None
    );
    assert_eq!(sanitize_color_value(Some("".to_string())), None);
}

#[test]
fn aria_label_prefers_custom_then_label_then_state_defaults() {
    let (label, source) = resolve_aria_label(
        Some("  Accent  ".to_string()),
        Some("Ignored".to_string()),
        Some("#ff0"),
        false,
        false,
    );
    assert_eq!(label, "Accent");
    assert_eq!(source, "custom");

    let (label, source) =
        resolve_aria_label(None, Some("  Brand  ".to_string()), None, false, false);
    assert_eq!(label, "Brand");
    assert_eq!(source, "label");

    let (label, source) = resolve_aria_label(None, None, None, false, true);
    assert_eq!(label, "Mixed");
    assert_eq!(source, "mixed");

    let (label, source) = resolve_aria_label(None, None, None, true, false);
    assert_eq!(label, "No fill");
    assert_eq!(source, "nothing");

    let (label, source) = resolve_aria_label(None, None, Some(""), false, false);
    assert_eq!(label, DEFAULT_ARIA_LABEL);
    assert_eq!(source, "default");
}

#[test]
fn aria_label_fallbacks_allow_i18n_override_but_reject_blank_values() {
    let (label, source) = resolve_aria_label_with_fallbacks(
        None,
        None,
        None,
        true,
        false,
        SwatchAriaLabelFallbacks {
            mixed: "Mixed locale",
            nothing: "Aucune couleur",
            default: "Nuancier",
        },
    );
    assert_eq!(label, "Aucune couleur");
    assert_eq!(source, "nothing");

    let (label, source) = resolve_aria_label_with_fallbacks(
        None,
        None,
        None,
        false,
        true,
        SwatchAriaLabelFallbacks {
            mixed: "Mischung",
            nothing: "Kein Fill",
            default: "Farbfeld",
        },
    );
    assert_eq!(label, "Mischung");
    assert_eq!(source, "mixed");

    let (label, source) = resolve_aria_label_with_fallbacks(
        None,
        None,
        None,
        false,
        false,
        SwatchAriaLabelFallbacks {
            mixed: "  ",
            nothing: " ",
            default: "",
        },
    );
    assert_eq!(label, DEFAULT_ARIA_LABEL);
    assert_eq!(source, "default");
}

#[test]
fn state_resolves_interactive_and_state_markers() {
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
        has_custom_class_name: true,
    });
    assert!(state.has_color);
    assert!(state.is_interactive);
    assert_eq!(state.data_state_attr, "color");

    let state = resolve_state(SwatchStateInput {
        mixed_value: true,
        ..SwatchStateInput {
            size: SwatchSize::M,
            border: SwatchBorder::Default,
            rounding: SwatchRounding::Default,
            shape: SwatchShape::Square,
            has_color: true,
            nothing: true,
            mixed_value: false,
            disabled: false,
            decorative: false,
            has_custom_class_name: false,
        }
    });
    assert!(state.show_mixed_value);
    assert!(!state.has_color);
    assert!(!state.show_nothing);
    assert!(!state.is_interactive);
    assert_eq!(state.data_state_attr, "mixed");
}

#[test]
fn normalize_default_selected_falls_back_to_false() {
    assert!(!normalize_default_selected(None));
    assert!(normalize_default_selected(Some(true)));
    assert!(!normalize_default_selected(Some(false)));
}

#[test]
fn selection_control_state_maps_control_and_source_markers() {
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
fn agent_contract_is_schema_typed_and_snapshot_fallback_explicit() {
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
