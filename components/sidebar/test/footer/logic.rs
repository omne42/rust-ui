use super::*;
use crate::sidebar_footer::DEFAULT_ARIA_LABEL;

#[test]
fn normalize_aria_label_tracks_default_and_custom_sources() {
    assert_eq!(
        normalize_aria_label(Some("  Workspace status footer  ".to_string())),
        ("Workspace status footer".to_string(), true)
    );
    assert_eq!(
        normalize_aria_label(Some("\n\t".to_string())),
        (DEFAULT_ARIA_LABEL.into(), false)
    );
    assert_eq!(
        normalize_aria_label(None),
        (DEFAULT_ARIA_LABEL.into(), false)
    );
}

#[test]
fn resolve_default_priority_prefers_prefixed_flags() {
    assert!(resolve_disabled(Some(true), false));
    assert!(!resolve_disabled(None, false));
    assert!(resolve_bordered(Some(true), false));
    assert!(!resolve_bordered(None, false));
}

#[test]
fn resolve_state_reports_disabled_border_and_source_markers() {
    let state = resolve_state(SidebarFooterStateInput {
        disabled: true,
        bordered: true,
        has_custom_aria_label: false,
        has_custom_class_name: true,
    });

    assert_eq!(state.state_attr, "disabled");
    assert_eq!(state.border_attr, "bordered");
    assert_eq!(state.aria_source_attr, "default");
    assert_eq!(state.class_source_attr, "custom");
    assert!(state.disabled);
    assert!(!state.enabled);
}

#[test]
fn compose_class_name_includes_state_border_and_custom_markers() {
    let state = resolve_state(SidebarFooterStateInput {
        disabled: true,
        bordered: true,
        has_custom_aria_label: true,
        has_custom_class_name: true,
    });
    let class_name = compose_class_name(Some("docs-sidebar-footer-custom".to_string()), state);

    for needle in [
        "ui-sidebar__footer",
        "ui-sidebar-footer",
        "ui-sidebar-footer--disabled",
        "ui-sidebar-footer--bordered",
        "ui-sidebar-footer--custom-class",
        "docs-sidebar-footer-custom",
    ] {
        assert!(
            class_name.contains(needle),
            "missing `{needle}` in sidebar footer class contract"
        );
    }
}
