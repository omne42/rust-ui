use super::*;

#[test]
fn compose_class_name_includes_state_markers() {
    let class_name = compose_class_name(
        Some("docs-code".to_string()),
        resolve_state(CodeStateInput {
            variant: CodeVariant::Inline,
            has_custom_class_name: true,
        }),
    );

    for token in [
        "ui-code",
        "ui-code--variant-inline",
        "ui-code--state-inline",
        "ui-code--custom-class",
        "docs-code",
    ] {
        assert!(
            class_name.contains(token),
            "composed class name should include `{token}`"
        );
    }
}

#[test]
fn compose_class_name_skips_external_class_without_custom_flag() {
    let class_name = compose_class_name(
        Some("docs-code".to_string()),
        resolve_state(CodeStateInput {
            variant: CodeVariant::Inline,
            has_custom_class_name: false,
        }),
    );

    assert!(!class_name.contains("docs-code"));
    assert!(!class_name.contains("ui-code--custom-class"));
}

#[test]
fn resolve_view_state_centralizes_default_variant_in_logic() {
    let resolved = resolve_view_state(CodeViewInput {
        variant: None,
        class_name: Some("docs-code".to_string()),
    });

    assert_eq!(resolved.state.variant, CodeVariant::Inline);
    assert_eq!(resolved.state.variant_attr, "inline");
    assert!(resolved.class.contains("ui-code--variant-inline"));
    assert!(resolved.class.contains("docs-code"));
}
