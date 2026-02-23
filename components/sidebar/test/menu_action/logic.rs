use super::*;

#[test]
fn normalize_defaults_and_trimmed_values_are_stable() {
    assert_eq!(
        normalize_aria_label(Some("  Item options  ".to_string())),
        ("Item options".to_string(), true)
    );
    assert_eq!(
        normalize_aria_label(None),
        (DEFAULT_ARIA_LABEL.into(), false)
    );
    assert_eq!(normalize_label(None), DEFAULT_LABEL.to_string());
    assert_eq!(
        normalize_label(Some("  Open actions  ".to_string())),
        "Open actions".to_string()
    );
}

#[test]
fn resolve_default_priority_prefers_prefixed_flags() {
    assert!(resolve_hover_only(Some(true), false));
    assert!(!resolve_hover_only(None, false));
    assert!(resolve_disabled(Some(true), false));
    assert!(!resolve_disabled(None, false));
}

#[test]
fn resolve_state_tracks_visibility_and_source_markers() {
    let hover = resolve_state(SidebarMenuActionStateInput {
        hover_only: true,
        disabled: false,
        has_custom_aria_label: false,
        has_custom_class_name: true,
    });
    assert_eq!(hover.state_attr, "hover-only");
    assert_eq!(hover.visibility_attr, "hover");
    assert_eq!(hover.class_source_attr, "custom");

    let disabled = resolve_state(SidebarMenuActionStateInput {
        hover_only: false,
        disabled: true,
        has_custom_aria_label: true,
        has_custom_class_name: false,
    });
    assert_eq!(disabled.state_attr, "disabled");
    assert_eq!(disabled.visibility_attr, "always");
    assert_eq!(disabled.aria_source_attr, "custom");
}

#[test]
fn compose_class_name_includes_state_and_custom_markers() {
    let state = resolve_state(SidebarMenuActionStateInput {
        hover_only: true,
        disabled: true,
        has_custom_aria_label: true,
        has_custom_class_name: true,
    });
    let class_name = compose_class_name(Some("docs-sidebar-menu-action-custom".to_string()), state);

    for token in [
        "ui-sidebar-menu-action",
        "ui-sidebar-menu-action--hover-only",
        "ui-sidebar-menu-action--disabled",
        "ui-sidebar-menu-action--custom-class",
        "docs-sidebar-menu-action-custom",
    ] {
        assert!(
            class_name.contains(token),
            "composed class name should include `{token}`"
        );
    }
}
