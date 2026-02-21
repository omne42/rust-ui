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
fn resolve_state_tracks_state_markers_and_sources() {
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

    let framed = resolve_state(ColorSwatchStateInput {
        alpha: ColorSwatchAlpha::Opaque,
        ..ColorSwatchStateInput {
            size: ColorSwatchSize::Md,
            rounding: ColorSwatchRounding::Default,
            shape: ColorSwatchShape::Square,
            bordered: true,
            alpha: ColorSwatchAlpha::None,
            has_color: false,
            has_custom_aria_label: false,
            has_custom_class_name: false,
        }
    });
    assert_eq!(framed.data_state_attr, "framed");
}
