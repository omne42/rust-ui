use super::*;

#[test]
fn normalize_aria_label_tracks_default_and_custom_sources() {
    assert_eq!(
        normalize_aria_label(Some("  Workspace inset panel  ".to_string())),
        ("Workspace inset panel".to_string(), true)
    );
    assert_eq!(
        normalize_aria_label(None),
        (DEFAULT_ARIA_LABEL.into(), false)
    );
}

#[test]
fn resolve_state_reports_side_padding_and_surface_markers() {
    let state = resolve_state(SidebarInsetStateInput {
        side: SidebarSide::Right,
        padded: false,
        recessed: true,
        disabled: false,
        has_custom_aria_label: false,
        has_custom_class_name: true,
    });

    assert_eq!(state.side_attr, "right");
    assert_eq!(state.padding_attr, "compact");
    assert_eq!(state.surface_attr, "recessed");
    assert_eq!(state.state_attr, "recessed");
    assert_eq!(state.aria_source_attr, "default");
    assert_eq!(state.class_source_attr, "custom");
}

#[test]
fn compose_class_name_includes_state_and_custom_markers() {
    let state = resolve_state(SidebarInsetStateInput {
        side: SidebarSide::Left,
        padded: true,
        recessed: true,
        disabled: true,
        has_custom_aria_label: true,
        has_custom_class_name: true,
    });
    let class_name = compose_class_name(Some("docs-sidebar-inset-custom".to_string()), state);

    for needle in [
        "ui-sidebar-inset",
        "ui-sidebar-inset--left",
        "ui-sidebar-inset--padded",
        "ui-sidebar-inset--recessed",
        "ui-sidebar-inset--disabled",
        "ui-sidebar-inset--custom-class",
        "docs-sidebar-inset-custom",
    ] {
        assert!(
            class_name.contains(needle),
            "missing `{needle}` in sidebar inset class contract"
        );
    }
}
