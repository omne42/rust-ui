use super::*;

#[test]
fn normalize_helpers_track_defaults_and_custom_sources() {
    assert_eq!(
        normalize_aria_label(Some("  Toggle inspector rail  ".to_string())),
        ("Toggle inspector rail".to_string(), true)
    );
    assert_eq!(
        normalize_aria_label(None),
        (DEFAULT_ARIA_LABEL.into(), false)
    );

    assert_eq!(
        normalize_label(Some("  collapse  ".to_string())),
        ("collapse".to_string(), true)
    );
    assert_eq!(normalize_label(None), (DEFAULT_LABEL.into(), false));
    assert!(normalize_default_open(None));
}

#[test]
fn resolve_state_reports_side_control_and_source_markers() {
    let state = resolve_state(SidebarRailStateInput {
        open: false,
        side: SidebarSide::Right,
        disabled: false,
        is_controlled: true,
        has_custom_aria_label: false,
        has_custom_label: true,
        has_custom_class_name: true,
    });

    assert_eq!(state.state_attr, "closed");
    assert_eq!(state.side_attr, "right");
    assert_eq!(state.control_attr, "controlled");
    assert_eq!(state.aria_source_attr, "default");
    assert_eq!(state.label_source_attr, "custom");
    assert_eq!(state.class_source_attr, "custom");
}

#[test]
fn compose_class_name_includes_state_side_control_and_custom_markers() {
    let state = resolve_state(SidebarRailStateInput {
        open: true,
        side: SidebarSide::Left,
        disabled: true,
        is_controlled: false,
        has_custom_aria_label: true,
        has_custom_label: true,
        has_custom_class_name: true,
    });
    let class_name = compose_class_name(Some("docs-sidebar-rail-custom".to_string()), state);

    for needle in [
        "ui-sidebar__rail",
        "ui-sidebar-rail",
        "ui-sidebar-rail--open",
        "ui-sidebar-rail--left",
        "ui-sidebar-rail--disabled",
        "ui-sidebar-rail--uncontrolled",
        "ui-sidebar-rail--custom-class",
        "docs-sidebar-rail-custom",
    ] {
        assert!(
            class_name.contains(needle),
            "missing `{needle}` in sidebar rail class contract"
        );
    }
}
