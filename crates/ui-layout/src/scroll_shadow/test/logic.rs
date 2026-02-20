use super::*;

#[test]
fn normalize_helpers_filter_empty_inputs() {
    assert_eq!(normalize_optional_text(None), None);
    assert_eq!(normalize_optional_text(Some("  \n\t".to_string())), None);
    assert_eq!(
        normalize_optional_text(Some("  docs-scroll-shadow  ".to_string())),
        Some("docs-scroll-shadow".to_string())
    );

    assert_eq!(
        ui_state_primitives::scroll_shadow::normalize_max_height(None),
        None
    );
    assert_eq!(
        ui_state_primitives::scroll_shadow::normalize_max_height(Some(0)),
        None
    );
    assert_eq!(
        ui_state_primitives::scroll_shadow::normalize_max_height(Some(160)),
        Some(160)
    );
}

#[test]
fn compose_class_name_includes_state_markers() {
    let class_name = compose_class_name(
        Some("custom".to_string()),
        resolve_state(ScrollShadowStateInput {
            max_height_px: Some(200),
            has_custom_class_name: true,
        }),
    );

    for token in [
        "ui-scroll-shadow",
        "ui-scroll-shadow--max-height-custom",
        "ui-scroll-shadow--custom-class",
        "custom",
    ] {
        assert!(
            class_name.contains(token),
            "composed class name should include `{token}`"
        );
    }
}

#[test]
fn compose_inline_style_outputs_css_var_only() {
    assert_eq!(
        compose_inline_style(240),
        "--ui-scroll-shadow-max-h: 240px;"
    );
}
