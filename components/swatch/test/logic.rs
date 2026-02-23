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
