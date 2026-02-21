use super::*;

#[test]
fn normalize_default_inputs_centralizes_default_fallbacks() {
    let normalized = normalize_default_inputs(ColorEditorDefaultInput {
        default_selected_color: None,
        default_format: None,
        default_hue: None,
        default_alpha: None,
        default_area: None,
        area_label: None,
        area_aria_label: None,
        hue_label: None,
        alpha_label: None,
        value_label: None,
        format_aria_label: None,
        preview_color: None,
        class_name: Some("  docs-color-editor  ".to_string()),
        lang: Some("  en-US  ".to_string()),
    });

    assert_eq!(normalized.default_selected_color, None);
    assert_eq!(normalized.default_format, ColorEditorFormat::default());
    assert_eq!(normalized.default_hue, DEFAULT_HUE);
    assert_eq!(normalized.default_alpha, DEFAULT_ALPHA);
    assert_eq!(normalized.default_area, DEFAULT_AREA);
    assert_eq!(normalized.area_label, "Saturation / Brightness");
    assert_eq!(normalized.area_aria_label, "Color area");
    assert_eq!(normalized.hue_label, "Hue");
    assert_eq!(normalized.alpha_label, "Alpha");
    assert_eq!(normalized.value_label, "Value");
    assert_eq!(normalized.format_aria_label, "Color format");
    assert_eq!(
        normalized.preview_color,
        compose_color_from_hsb(
            DEFAULT_HUE,
            f64::from(DEFAULT_AREA.0 * 100.0),
            f64::from(DEFAULT_AREA.1 * 100.0),
            DEFAULT_ALPHA,
            true,
        )
    );
    assert_eq!(normalized.class_name, Some("docs-color-editor".to_string()));
    assert_eq!(normalized.normalized_lang, Some("en-US".to_string()));
}

#[test]
fn normalize_default_inputs_sanitizes_custom_values_before_use() {
    let normalized = normalize_default_inputs(ColorEditorDefaultInput {
        default_selected_color: Some("  #0Ea5E9  ".to_string()),
        default_format: Some(ColorEditorFormat::Hsb),
        default_hue: Some(540.0),
        default_alpha: Some(140.0),
        default_area: Some((2.0, -1.0)),
        area_label: Some("  Saturation  ".to_string()),
        area_aria_label: Some("  ".to_string()),
        hue_label: Some(" Tone ".to_string()),
        alpha_label: Some("Opacity".to_string()),
        value_label: Some("  Value ".to_string()),
        format_aria_label: Some(" Palette format ".to_string()),
        preview_color: Some("  ".to_string()),
        class_name: Some("  ".to_string()),
        lang: Some("  ".to_string()),
    });

    assert_eq!(
        normalized.default_selected_color,
        sanitize_color(Some("  #0Ea5E9  ".to_string()))
    );
    assert_eq!(normalized.default_format, ColorEditorFormat::Hsb);
    assert_eq!(normalized.default_hue, 180.0);
    assert_eq!(normalized.default_alpha, 100.0);
    assert_eq!(normalized.default_area, (1.0, 0.0));
    assert_eq!(normalized.area_label, "Saturation");
    assert_eq!(normalized.area_aria_label, "Color area");
    assert_eq!(normalized.hue_label, "Tone");
    assert_eq!(normalized.alpha_label, "Opacity");
    assert_eq!(normalized.value_label, "Value");
    assert_eq!(normalized.format_aria_label, "Palette format");
    assert_eq!(
        normalized.preview_color,
        compose_color_from_hsb(180.0, 100.0, 0.0, 100.0, true)
    );
    assert_eq!(normalized.class_name, None);
    assert_eq!(normalized.normalized_lang, None);
}

#[test]
fn resolve_selected_color_sanitizes_axes_before_formatting() {
    let color = resolve_selected_color(ColorEditorSelectionInput {
        hue: 540.0,
        area: (2.0, -0.5),
        alpha: 140.0,
        hide_alpha_channel: false,
    });

    assert_eq!(
        color,
        compose_color_from_hsb(180.0, 100.0, 0.0, 100.0, false)
    );
}

#[test]
fn resolve_change_helpers_keep_state_derivation_in_logic_layer() {
    let (area, area_color) = resolve_area_change((1.2, -0.2), 400.0, 101.0, true);
    assert_eq!(area, (1.0, 0.0));
    assert_eq!(
        area_color,
        resolve_selected_color(ColorEditorSelectionInput {
            hue: 400.0,
            area,
            alpha: 101.0,
            hide_alpha_channel: true,
        })
    );

    let (hue, hue_color) = resolve_hue_change(-30.0, (0.25, 0.75), 64.0, false);
    assert_eq!(hue, 330.0);
    assert_eq!(
        hue_color,
        resolve_selected_color(ColorEditorSelectionInput {
            hue,
            area: (0.25, 0.75),
            alpha: 64.0,
            hide_alpha_channel: false,
        })
    );

    let (alpha, alpha_color) = resolve_alpha_change(999.0, 120.0, (0.3, 0.4), false);
    assert_eq!(alpha, 100.0);
    assert_eq!(
        alpha_color,
        resolve_selected_color(ColorEditorSelectionInput {
            hue: 120.0,
            area: (0.3, 0.4),
            alpha,
            hide_alpha_channel: false,
        })
    );
}

#[test]
fn resolve_field_change_reuses_color_sanitizer() {
    assert_eq!(
        resolve_field_change(Some("  #0ea5e9  ".to_string())),
        sanitize_color(Some("  #0ea5e9  ".to_string()))
    );
    assert_eq!(resolve_field_change(Some("   ".to_string())), None);
}
