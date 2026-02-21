use super::*;

#[test]
fn header_is_hidden_when_all_optional_parts_absent() {
    let view = resolve_view_state("let x = 1;", None, None, false);
    assert!(!view.show_header);
    assert_eq!(view.header_attr, "hidden");
}

#[test]
fn header_is_shown_for_label_language_or_copyable() {
    assert!(resolve_view_state("x", Some("Code"), None, false).show_header);
    assert!(resolve_view_state("x", None, Some("rs"), false).show_header);
    assert!(resolve_view_state("x", None, None, true).show_header);
}

#[test]
fn multiline_detection() {
    assert_eq!(
        resolve_view_state("x", None, None, false).state_attr,
        "single-line"
    );
    assert_eq!(
        resolve_view_state("x\ny", None, None, false).state_attr,
        "multiline"
    );
}

#[test]
fn normalize_optional_text_filters_blank_values() {
    assert_eq!(normalize_optional_text(None), None);
    assert_eq!(normalize_optional_text(Some("\n\t".to_string())), None);
    assert_eq!(
        normalize_optional_text(Some("  rust  ".to_string())),
        Some("rust".to_string())
    );
}

#[test]
fn resolve_state_tracks_custom_sources_and_flags() {
    let state = resolve_state(CodeBlockStateInput {
        is_multiline: true,
        is_empty: false,
        has_label: true,
        has_language: true,
        copyable: true,
        has_custom_class_name: true,
        has_custom_motion: true,
    });

    assert!(state.show_header);
    assert_eq!(state.state_class, "ui-code-block--state-multiline");
    assert_eq!(state.header_class, "ui-code-block--header-visible");
    assert_eq!(state.motion_source_class, "ui-code-block--motion-custom");
    assert!(state.has_custom_class_name);
}

#[test]
fn resolve_state_from_content_derives_textual_flags() {
    let view = resolve_state_from_content(CodeBlockContentInput {
        code: "x\ny",
        label: Some("  Demo  "),
        language: Some("rs"),
        copyable: true,
        has_custom_class_name: true,
        has_custom_motion: false,
    });

    assert_eq!(view.state_attr, "multiline");
    assert!(!view.is_empty);
    assert!(view.has_label);
    assert!(view.has_language);
    assert!(view.copyable);
    assert_eq!(view.motion_source_attr, "default");
    assert!(view.has_custom_class_name);
}
