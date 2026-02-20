use super::*;

#[test]
fn normalize_helpers_track_defaults_and_custom_sources() {
    assert_eq!(
        normalize_aria_label(Some("  Toggle workspace  ".to_string())),
        ("Toggle workspace".to_string(), true)
    );
    assert_eq!(
        normalize_aria_label(None),
        (DEFAULT_ARIA_LABEL.into(), false)
    );

    assert_eq!(
        normalize_label(Some("  Collapse  ".to_string())),
        ("Collapse".to_string(), true)
    );
    assert_eq!(normalize_label(None), (DEFAULT_LABEL.into(), false));
    assert!(normalize_default_open(None));
}

#[test]
fn resolve_state_reports_open_control_and_source_markers() {
    let state = resolve_state(SidebarTriggerStateInput {
        open: false,
        disabled: false,
        is_controlled: true,
        has_custom_aria_label: false,
        has_custom_label: true,
        has_custom_class_name: true,
    });

    assert_eq!(state.state_attr, "closed");
    assert_eq!(state.control_attr, "controlled");
    assert_eq!(state.aria_source_attr, "default");
    assert_eq!(state.label_source_attr, "custom");
    assert_eq!(state.class_source_attr, "custom");
}

#[test]
fn compose_class_name_includes_state_control_and_custom_markers() {
    let state = resolve_state(SidebarTriggerStateInput {
        open: true,
        disabled: true,
        is_controlled: false,
        has_custom_aria_label: true,
        has_custom_label: true,
        has_custom_class_name: true,
    });
    let class_name = compose_class_name(Some("docs-sidebar-trigger-custom".to_string()), state);

    for needle in [
        "ui-sidebar__trigger",
        "ui-sidebar-trigger",
        "ui-sidebar-trigger--open",
        "ui-sidebar-trigger--disabled",
        "ui-sidebar-trigger--uncontrolled",
        "ui-sidebar-trigger--custom-class",
        "docs-sidebar-trigger-custom",
    ] {
        assert!(
            class_name.contains(needle),
            "missing `{needle}` in sidebar trigger class contract"
        );
    }
}
