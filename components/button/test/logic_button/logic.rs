use super::*;

#[test]
fn variant_contract_is_stable() {
    assert_eq!(
        LogicButtonVariant::And.class_name(),
        "ui-logic-button--variant-and"
    );
    assert_eq!(
        LogicButtonVariant::Or.class_name(),
        "ui-logic-button--variant-or"
    );

    assert_eq!(LogicButtonVariant::And.as_attr(), "and");
    assert_eq!(LogicButtonVariant::Or.as_attr(), "or");
}

#[test]
fn normalize_helpers_trim_and_fallback() {
    assert_eq!(normalize_optional_text(None), None);
    assert_eq!(normalize_optional_text(Some(" \n\t ".to_string())), None);
    assert_eq!(
        normalize_optional_text(Some("  docs-logic-button  ".to_string())),
        Some("docs-logic-button".to_string())
    );

    let (aria_label, custom) = normalize_aria_label(Some("  Logical operator  ".to_string()));
    assert_eq!(aria_label, "Logical operator");
    assert!(custom);

    let (aria_label, custom) = normalize_aria_label(None);
    assert_eq!(aria_label, DEFAULT_ARIA_LABEL);
    assert!(!custom);
}

#[test]
fn resolve_state_tracks_variant_and_sources() {
    let state = resolve_state(LogicButtonStateInput {
        variant: LogicButtonVariant::Or,
        disabled: false,
        has_custom_aria_label: true,
        has_custom_class_name: false,
        has_custom_press_handler: true,
    });

    assert_eq!(state.variant_attr, "or");
    assert!(!state.is_disabled);
    assert_eq!(state.data_state_attr, "ready");
    assert_eq!(state.aria_source_attr, "custom");
    assert_eq!(state.class_source_attr, "default");
}

#[test]
fn compose_class_name_includes_state_markers() {
    let class_name = compose_class_name(
        Some("docs-logic-button-custom".to_string()),
        resolve_state(LogicButtonStateInput {
            variant: LogicButtonVariant::And,
            disabled: true,
            has_custom_aria_label: false,
            has_custom_class_name: true,
            has_custom_press_handler: true,
        }),
    );

    for token in [
        "ui-logic-button",
        "ui-logic-button--variant-and",
        "ui-logic-button--disabled",
        "ui-logic-button--custom-handler",
        "ui-logic-button--custom-class",
        "docs-logic-button-custom",
    ] {
        assert!(
            class_name.contains(token),
            "composed class should include `{token}`"
        );
    }
}
