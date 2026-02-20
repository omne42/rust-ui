use super::*;

#[test]
fn normalize_helpers_apply_defaults() {
    assert_eq!(normalize_aria_label(None), DEFAULT_ARIA_LABEL);
    assert_eq!(normalize_label(None), DEFAULT_LABEL);
    assert_eq!(normalize_action_label(None), DEFAULT_ACTION_LABEL);
    assert!(normalize_default_open(None));
}

#[test]
fn resolve_state_tracks_flags_and_attrs() {
    let state = resolve_state(SidebarGroupStateInput {
        open: false,
        collapsible: true,
        disabled: false,
        show_label: true,
        show_action: false,
        has_label: true,
        has_action: false,
        is_controlled: true,
        has_custom_class_name: true,
    });

    assert!(state.closed);
    assert_eq!(state.state_attr, "closed");
    assert_eq!(state.collapse_attr, "collapsible");
    assert_eq!(state.control_attr, "controlled");
    assert_eq!(state.class_source_attr, "custom");
}

#[test]
fn compose_class_name_includes_markers() {
    let class_name = compose_class_name(
        Some("demo".to_string()),
        resolve_state(SidebarGroupStateInput {
            open: true,
            collapsible: true,
            disabled: true,
            show_label: false,
            show_action: true,
            has_label: true,
            has_action: true,
            is_controlled: false,
            has_custom_class_name: true,
        }),
    );

    for needle in [
        "ui-sidebar-group",
        "ui-sidebar-group--collapsible",
        "ui-sidebar-group--open",
        "ui-sidebar-group--disabled",
        "ui-sidebar-group--uncontrolled",
        "ui-sidebar-group--label-hidden",
        "ui-sidebar-group--custom-class",
        "demo",
    ] {
        assert!(
            class_name.contains(needle),
            "missing `{needle}` in class_name"
        );
    }
}
