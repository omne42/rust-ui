use super::*;

#[test]
fn normalize_aria_label_tracks_default_and_custom_sources() {
    assert_eq!(
        normalize_aria_label(Some("  Open reviews  ".to_string())),
        ("Open reviews".to_string(), true)
    );
    assert_eq!(
        normalize_aria_label(None),
        (DEFAULT_ARIA_LABEL.into(), false)
    );
}

#[test]
fn resolve_state_tracks_tone_disabled_and_source_markers() {
    let muted = resolve_state(SidebarMenuBadgeStateInput {
        muted: true,
        disabled: false,
        has_custom_aria_label: false,
        has_custom_class_name: true,
    });

    assert_eq!(muted.state_attr, "muted");
    assert_eq!(muted.tone_attr, "muted");
    assert!(muted.enabled);
    assert_eq!(muted.aria_source_attr, "default");
    assert_eq!(muted.class_source_attr, "custom");

    let disabled = resolve_state(SidebarMenuBadgeStateInput {
        muted: false,
        disabled: true,
        has_custom_aria_label: true,
        has_custom_class_name: false,
    });

    assert_eq!(disabled.state_attr, "disabled");
    assert_eq!(disabled.tone_attr, "emphasized");
    assert!(disabled.disabled);
    assert_eq!(disabled.aria_source_attr, "custom");
    assert_eq!(disabled.class_source_attr, "default");
}

#[test]
fn compose_class_name_includes_state_and_custom_markers() {
    let state = resolve_state(SidebarMenuBadgeStateInput {
        muted: true,
        disabled: true,
        has_custom_aria_label: true,
        has_custom_class_name: true,
    });
    let class_name = compose_class_name(Some("docs-sidebar-menu-badge-custom".to_string()), state);

    for token in [
        "ui-sidebar-menu-badge",
        "ui-sidebar-menu-badge--muted",
        "ui-sidebar-menu-badge--disabled",
        "ui-sidebar-menu-badge--custom-class",
        "docs-sidebar-menu-badge-custom",
    ] {
        assert!(
            class_name.contains(token),
            "composed class name should include `{token}`"
        );
    }
}
