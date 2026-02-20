use super::*;

#[test]
fn normalize_id_base_sanitizes_whitespace_and_symbols() {
    assert_eq!(normalize_id_base("  My Panel  ".to_string()), "my-panel");
    assert_eq!(
        normalize_id_base("Settings/Panel#1".to_string()),
        "settings-panel-1"
    );
    assert_eq!(normalize_id_base("   ".to_string()), DEFAULT_ID_BASE);
}

#[test]
fn resolve_title_and_aria_label_fall_back_to_defaults() {
    assert_eq!(resolve_title("  ".to_string()), DEFAULT_TITLE);
    assert_eq!(
        resolve_title("  Advanced Options  ".to_string()),
        "Advanced Options"
    );

    let (aria_label, custom) = resolve_aria_label("Advanced Options", None);
    assert_eq!(aria_label, "Advanced Options");
    assert!(!custom);

    let (aria_label, custom) =
        resolve_aria_label("Advanced Options", Some("  Settings panel  ".to_string()));
    assert_eq!(aria_label, "Settings panel");
    assert!(custom);
}

#[test]
fn resolve_state_tracks_open_mode_sources_and_motion() {
    let state = resolve_state(CollapsibleStateInput {
        is_open: true,
        is_disabled: false,
        is_controlled: true,
        has_custom_aria_label: true,
        has_custom_class_name: false,
        has_custom_motion: true,
    });

    assert_eq!(state.state_attr, "open");
    assert_eq!(state.open_mode_attr, "controlled");
    assert_eq!(state.label_source_attr, "custom");
    assert_eq!(state.class_source_attr, "default");
    assert_eq!(state.motion_source_attr, "custom");
    assert!(state.is_open);
    assert!(!state.is_closed);
}
