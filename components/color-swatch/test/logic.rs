use super::*;

#[test]
fn style_contracts_are_stable() {
    assert_eq!(ColorSwatchSize::Xs.class_name(), "ui-color-swatch--size-xs");
    assert_eq!(ColorSwatchSize::Md.as_attr(), "md");
    assert_eq!(
        ColorSwatchRounding::Default.class_name(),
        "ui-color-swatch--rounding-default"
    );
    assert_eq!(ColorSwatchRounding::Full.as_attr(), "full");
    assert_eq!(
        ColorSwatchShape::Square.class_name(),
        "ui-color-swatch--shape-square"
    );
    assert_eq!(ColorSwatchShape::Wide.as_attr(), "wide");
    assert_eq!(
        ColorSwatchAlpha::Translucent.class_name(),
        "ui-color-swatch--alpha-translucent"
    );
    assert_eq!(ColorSwatchAlpha::Transparent.as_attr(), "transparent");
}

#[test]
fn normalize_and_sanitize_helpers_drop_invalid_values() {
    assert_eq!(normalize_optional_text(None), None);
    assert_eq!(normalize_optional_text(Some("  \n".to_string())), None);
    assert_eq!(
        normalize_optional_text(Some("  #ff0000  ".to_string())),
        Some("#ff0000".to_string())
    );

    assert_eq!(
        sanitize_color_value(Some("  #ff0000  ".to_string())),
        Some("#ff0000".to_string())
    );
    assert_eq!(
        sanitize_color_value(Some("javascript:alert(1)".to_string())),
        None
    );
}

#[test]
fn is_prefixed_boolean_props_use_canonical_inputs_only() {
    assert_eq!(
        normalize_is_bordered(Some(false)),
        (false, ColorSwatchBoolSource::IsProp)
    );
    assert_eq!(
        normalize_is_bordered(None),
        (true, ColorSwatchBoolSource::Default)
    );

    assert_eq!(
        normalize_is_decorative(Some(true)),
        (true, ColorSwatchBoolSource::IsProp)
    );
    assert_eq!(
        normalize_is_decorative(None),
        (false, ColorSwatchBoolSource::Default)
    );
}

#[test]
fn alpha_resolution_supports_hex_and_functional_colors() {
    assert_eq!(resolve_alpha(Some("#FF0000")), ColorSwatchAlpha::Opaque);
    assert_eq!(
        resolve_alpha(Some("#FF000080")),
        ColorSwatchAlpha::Translucent
    );
    assert_eq!(resolve_alpha(Some("#F000")), ColorSwatchAlpha::Transparent);
    assert_eq!(
        resolve_alpha(Some("rgba(255, 0, 0, 0.25)")),
        ColorSwatchAlpha::Translucent
    );
    assert_eq!(
        resolve_alpha(Some("hsl(0 100% 50% / 0%)")),
        ColorSwatchAlpha::Transparent
    );
    assert_eq!(resolve_alpha(None), ColorSwatchAlpha::None);
}

#[test]
fn aria_label_uses_color_name_and_context_when_provided() {
    let (label, is_custom) = normalize_aria_label(
        Some("Background".to_string()),
        Some("Fire truck red".to_string()),
        Some("#f00"),
        ColorSwatchAlpha::Opaque,
    );
    assert_eq!(label, "Fire truck red, Background");
    assert!(is_custom);

    let (label, is_custom) = normalize_aria_label(
        None,
        None,
        Some("rgba(255, 0, 0, 0.4)"),
        ColorSwatchAlpha::Translucent,
    );
    assert_eq!(label, "Translucent rgba(255, 0, 0, 0.4)");
    assert!(!is_custom);
}

#[test]
fn state_and_class_composition_track_markers() {
    let state = resolve_state(ColorSwatchStateInput {
        size: ColorSwatchSize::Lg,
        rounding: ColorSwatchRounding::Full,
        shape: ColorSwatchShape::Wide,
        bordered: true,
        alpha: ColorSwatchAlpha::Transparent,
        has_color: true,
        has_custom_aria_label: true,
        has_custom_class_name: true,
    });

    assert_eq!(state.data_state_attr, "transparent");
    assert_eq!(state.aria_source_attr, "custom");
    assert_eq!(state.class_source_attr, "custom");

    let class_name = compose_class_name(Some("docs-color-swatch".to_string()), state);
    for token in [
        "ui-color-swatch",
        "ui-color-swatch--size-lg",
        "ui-color-swatch--rounding-full",
        "ui-color-swatch--shape-wide",
        "ui-color-swatch--alpha-transparent",
        "ui-color-swatch--bordered",
        "ui-color-swatch--custom-class",
        "docs-color-swatch",
    ] {
        assert!(
            class_name.contains(token),
            "composed class name should include `{token}`"
        );
    }

    assert_eq!(
        compose_inline_style(Some("#ff0000")),
        Some("--ui-color-swatch-color: #ff0000;".to_string())
    );
    assert_eq!(
        resolve_inline_style(Some("#ff0000")),
        "--ui-color-swatch-color: #ff0000;".to_string()
    );
    assert_eq!(resolve_inline_style(None), String::new());
}

#[test]
fn render_state_centralizes_input_normalization_and_derivation() {
    let render_state = resolve_render_state(ColorSwatchRenderInput {
        color: Some("  #ff000080  ".to_string()),
        color_name: None,
        size: ColorSwatchSize::Sm,
        rounding: ColorSwatchRounding::None,
        shape: ColorSwatchShape::Square,
        is_bordered: None,
        is_decorative: Some(true),
        aria_label: Some("  Preview  ".to_string()),
        class_name: Some("  docs-color-swatch  ".to_string()),
    });

    assert_eq!(render_state.color, Some("#ff000080".to_string()));
    assert_eq!(render_state.state.alpha, ColorSwatchAlpha::Translucent);
    assert_eq!(render_state.state.data_state_attr, "translucent");
    assert_eq!(render_state.bordered_source, ColorSwatchBoolSource::Default);
    assert_eq!(
        render_state.decorative_source,
        ColorSwatchBoolSource::IsProp
    );
    assert_eq!(render_state.aria_label, "Translucent #ff000080, Preview");
    assert_eq!(
        render_state.inline_style,
        "--ui-color-swatch-color: #ff000080;".to_string()
    );
    for token in [
        "ui-color-swatch--size-sm",
        "ui-color-swatch--rounding-none",
        "ui-color-swatch--shape-square",
        "ui-color-swatch--alpha-translucent",
        "ui-color-swatch--custom-class",
        "docs-color-swatch",
    ] {
        assert!(
            render_state.class_name.contains(token),
            "render-state class name should include `{token}`"
        );
    }
}
