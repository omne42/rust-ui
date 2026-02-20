use super::*;

#[test]
fn normalize_contracts_use_defaults_and_trim_custom_values() {
    assert_eq!(normalize_label(None), (DEFAULT_LABEL.into(), false));
    assert_eq!(
        normalize_label(Some("  Fill color  ".to_string())),
        ("Fill color".to_string(), true)
    );

    assert_eq!(
        normalize_placeholder(None),
        (DEFAULT_PLACEHOLDER.into(), false)
    );
    assert_eq!(
        normalize_placeholder(Some("  #ABCDEF  ".to_string())),
        ("#ABCDEF".to_string(), true)
    );

    assert_eq!(
        normalize_aria_label(None, "Fill color"),
        ("Fill color value".to_string(), false)
    );
    assert_eq!(
        normalize_aria_label(Some("  Theme color  ".to_string()), "Fill color"),
        ("Theme color".to_string(), true)
    );
}

#[test]
fn preview_color_sanitization_rejects_unsafe_values() {
    assert_eq!(
        sanitize_preview_color(Some("#09f".to_string())),
        Some("#09f".to_string())
    );
    assert_eq!(
        sanitize_preview_color(Some("rgba(12, 34, 56, 0.5)".to_string())),
        Some("rgba(12, 34, 56, 0.5)".to_string())
    );
    assert_eq!(
        sanitize_preview_color(Some("javascript:alert(1)".to_string())),
        None
    );
}

#[test]
fn resolve_state_and_class_name_track_state_and_sources() {
    let valid = resolve_state(ColorFieldStateInput {
        disabled: false,
        has_value: true,
        has_valid_value: true,
        has_preview: true,
        has_custom_label: true,
        has_custom_placeholder: true,
        has_custom_aria_label: false,
        has_custom_class_name: true,
    });

    assert_eq!(valid.data_state_attr, "valid");
    assert_eq!(valid.label_source_attr, "custom");
    assert_eq!(valid.placeholder_source_attr, "custom");
    assert_eq!(valid.aria_source_attr, "default");
    assert_eq!(valid.class_source_attr, "custom");

    let class_name = compose_class_name(Some("docs-color-field".to_string()), valid);
    assert!(class_name.contains("ui-color-field"));
    assert!(class_name.contains("ui-color-field--custom-class"));
    assert!(class_name.contains("docs-color-field"));

    let invalid = resolve_state(ColorFieldStateInput {
        disabled: false,
        has_value: true,
        has_valid_value: false,
        has_preview: false,
        has_custom_label: false,
        has_custom_placeholder: false,
        has_custom_aria_label: false,
        has_custom_class_name: false,
    });
    assert_eq!(invalid.data_state_attr, "invalid");

    let empty = resolve_state(ColorFieldStateInput {
        disabled: false,
        has_value: false,
        has_valid_value: false,
        has_preview: false,
        has_custom_label: false,
        has_custom_placeholder: false,
        has_custom_aria_label: false,
        has_custom_class_name: false,
    });
    assert_eq!(empty.data_state_attr, "empty");

    let disabled = resolve_state(ColorFieldStateInput {
        disabled: true,
        has_value: true,
        has_valid_value: true,
        has_preview: true,
        has_custom_label: false,
        has_custom_placeholder: false,
        has_custom_aria_label: false,
        has_custom_class_name: false,
    });
    assert_eq!(disabled.data_state_attr, "disabled");
}
