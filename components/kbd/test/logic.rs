use super::*;

#[test]
fn size_class_names_and_attrs_are_stable() {
    assert_eq!(KbdSize::Sm.class_name(), "ui-kbd--size-sm");
    assert_eq!(KbdSize::Md.class_name(), "ui-kbd--size-md");

    assert_eq!(KbdSize::Sm.as_attr(), "sm");
    assert_eq!(KbdSize::Md.as_attr(), "md");
}

#[test]
fn normalize_optional_text_filters_blank_values() {
    assert_eq!(normalize_optional_text(None), None);
    assert_eq!(normalize_optional_text(Some("\n\t".to_string())), None);
    assert_eq!(
        normalize_optional_text(Some("  Ctrl+K  ".to_string())),
        Some("Ctrl+K".to_string())
    );
}

#[test]
fn normalize_size_defaults_to_md() {
    assert_eq!(normalize_size(None), KbdSize::Md);
    assert_eq!(normalize_size(Some(KbdSize::Sm)), KbdSize::Sm);
}

#[test]
fn resolve_state_tracks_size_keys_and_class_source() {
    let state = resolve_state(KbdStateInput {
        size: KbdSize::Sm,
        has_keys: true,
        has_custom_class_name: true,
    });

    assert_eq!(state.size, KbdSize::Sm);
    assert_eq!(state.size_class, "ui-kbd--size-sm");
    assert_eq!(state.size_attr, "sm");
    assert_eq!(state.state_class, "ui-kbd--state-with-keys");
    assert_eq!(state.state_attr, "with-keys");
    assert!(state.has_keys);
    assert!(state.has_custom_class_name);
}

#[test]
fn compose_class_name_includes_state_markers() {
    let class_name = compose_class_name(
        Some("docs-kbd".to_string()),
        resolve_state(KbdStateInput {
            size: KbdSize::Md,
            has_keys: false,
            has_custom_class_name: true,
        }),
    );

    for token in [
        "ui-kbd",
        "ui-kbd--size-md",
        "ui-kbd--state-label-only",
        "ui-kbd--custom-class",
        "docs-kbd",
    ] {
        assert!(
            class_name.contains(token),
            "composed class name should include `{token}`"
        );
    }
}

#[test]
fn resolve_view_model_normalizes_inputs_and_derives_state_in_logic() {
    let view_model = resolve_view_model(KbdLogicInput {
        size: None,
        keys: Some("  Ctrl+K  ".to_string()),
        class_name: Some("  docs-kbd  ".to_string()),
    });

    assert_eq!(view_model.state.size, KbdSize::Md);
    assert_eq!(view_model.state.size_attr, "md");
    assert_eq!(view_model.keys, Some("Ctrl+K".to_string()));
    assert_eq!(view_model.state.state_attr, "with-keys");
    assert!(view_model.state.has_custom_class_name);
    assert!(view_model.class.contains("ui-kbd"));
    assert!(view_model.class.contains("docs-kbd"));
}
