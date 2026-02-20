use super::*;

#[test]
fn variant_class_names_and_attrs_are_stable() {
    assert_eq!(CodeVariant::Inline.class_name(), "ui-code--variant-inline");
    assert_eq!(CodeVariant::Block.class_name(), "ui-code--variant-block");

    assert_eq!(CodeVariant::Inline.as_attr(), "inline");
    assert_eq!(CodeVariant::Block.as_attr(), "block");
}

#[test]
fn normalize_optional_text_filters_blank_values() {
    assert_eq!(normalize_optional_text(None), None);
    assert_eq!(normalize_optional_text(Some("  \n\t".to_string())), None);
    assert_eq!(
        normalize_optional_text(Some("  docs-code  ".to_string())),
        Some("docs-code".to_string())
    );
}

#[test]
fn resolve_state_tracks_variant_and_class_source() {
    let state = resolve_state(CodeStateInput {
        variant: CodeVariant::Block,
        has_custom_class_name: true,
    });

    assert_eq!(state.variant, CodeVariant::Block);
    assert_eq!(state.variant_class, "ui-code--variant-block");
    assert_eq!(state.variant_attr, "block");
    assert_eq!(state.state_class, "ui-code--state-block");
    assert_eq!(state.state_attr, "block");
    assert!(!state.is_inline);
    assert!(state.is_block);
    assert!(state.has_custom_class_name);
}

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
