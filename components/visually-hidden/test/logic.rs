use super::*;

#[test]
fn focus_mode_source_and_attr_mappings_are_stable() {
    assert_eq!(VisuallyHiddenFocusMode::Hidden.as_attr(), "hidden");
    assert_eq!(VisuallyHiddenFocusMode::Focusable.as_attr(), "focusable");

    assert_eq!(FocusPropSource::Default.as_attr(), "default");
    assert_eq!(FocusPropSource::IsFocusable.as_attr(), "is_focusable");
    assert_eq!(FocusPropSource::FocusableAlias.as_attr(), "focusable");

    assert_eq!(
        FocusPropSource::resolve(None, None),
        (VisuallyHiddenFocusMode::Hidden, FocusPropSource::Default)
    );
    assert_eq!(
        FocusPropSource::resolve(Some(true), None),
        (
            VisuallyHiddenFocusMode::Focusable,
            FocusPropSource::IsFocusable
        )
    );
    assert_eq!(
        FocusPropSource::resolve(Some(false), Some(true)),
        (
            VisuallyHiddenFocusMode::Hidden,
            FocusPropSource::IsFocusable
        )
    );
    assert_eq!(
        FocusPropSource::resolve(None, Some(true)),
        (
            VisuallyHiddenFocusMode::Focusable,
            FocusPropSource::FocusableAlias
        )
    );
}

#[test]
fn class_name_source_uses_normalized_class_name() {
    let default_state = normalize_props(VisuallyHiddenLogicInput {
        is_focusable: None,
        focusable: None,
        class_name: Some("   ".to_string()),
    });
    assert_eq!(default_state.class_name_source, ClassNameSource::Default);
    assert_eq!(default_state.class_name, None);

    let custom_state = normalize_props(VisuallyHiddenLogicInput {
        is_focusable: None,
        focusable: None,
        class_name: Some(" docs-hidden ".to_string()),
    });
    assert_eq!(custom_state.class_name_source, ClassNameSource::Custom);
    assert_eq!(ClassNameSource::Default.as_attr(), "default");
    assert_eq!(ClassNameSource::Custom.as_attr(), "custom");
    assert_eq!(custom_state.class_name.as_deref(), Some("docs-hidden"));
}

#[test]
fn normalize_props_delegates_state_machine_to_ui_state_primitives() {
    let state = normalize_props(VisuallyHiddenLogicInput {
        is_focusable: Some(true),
        focusable: Some(false),
        class_name: Some("docs-hidden".to_string()),
    });

    assert_eq!(state.focus_mode, VisuallyHiddenFocusMode::Focusable);
    assert_eq!(state.focus_prop_source, FocusPropSource::IsFocusable);
    assert_eq!(state.class_name_source, ClassNameSource::Custom);
    assert!(state.primitive_state.is_focusable);
    assert_eq!(
        state.primitive_state.focusable_class,
        Some("ui-visually-hidden--focusable")
    );
    assert_eq!(state.primitive_state.custom_class_attr, Some("true"));
}

#[test]
fn compose_class_name_tracks_focus_and_custom_class() {
    let state = normalize_props(VisuallyHiddenLogicInput {
        is_focusable: Some(true),
        focusable: None,
        class_name: Some("docs-hidden".to_string()),
    });
    let class = compose_class_name(state.class_name, state.primitive_state);
    for token in [
        "ui-visually-hidden",
        "ui-visually-hidden--focusable",
        "docs-hidden",
    ] {
        assert!(
            class.contains(token),
            "composed class should include `{token}`"
        );
    }
}
