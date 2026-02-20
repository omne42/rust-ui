use super::*;

#[test]
fn normalize_aria_label_tracks_default_and_custom_sources() {
    assert_eq!(
        normalize_aria_label(Some("  Site navigation  ".to_string())),
        ("Site navigation".to_string(), true)
    );
    assert_eq!(
        normalize_aria_label(None),
        (DEFAULT_ARIA_LABEL.into(), false)
    );
}

#[test]
fn normalize_href_trims_blank_values() {
    assert_eq!(
        normalize_href(Some("  /components  ".to_string())),
        Some("/components".to_string())
    );
    assert_eq!(normalize_href(Some("  ".to_string())), None);
    assert_eq!(normalize_href(None), None);
}

#[test]
fn resolve_root_and_slot_states_track_source_contracts() {
    let root = resolve_root_state(BreadcrumbRootStateInput {
        has_custom_aria_label: true,
        has_custom_class_name: false,
    });

    assert_eq!(root.state_attr, "customized");
    assert_eq!(root.aria_source_attr, "custom");
    assert_eq!(root.class_source_attr, "default");

    let slot = resolve_slot_state(BreadcrumbSlotStateInput {
        has_custom_class_name: true,
    });

    assert_eq!(slot.state_attr, "customized");
    assert_eq!(slot.class_source_attr, "custom");
}

#[test]
fn resolve_link_and_separator_states_cover_customized_paths() {
    let link = resolve_link_state(BreadcrumbLinkStateInput {
        has_href: false,
        has_custom_class_name: true,
    });

    assert_eq!(link.state_attr, "placeholder-customized");
    assert_eq!(link.href_state_attr, "absent");
    assert!(!link.interactive);
    assert_eq!(link.class_source_attr, "custom");

    let separator = resolve_separator_state(BreadcrumbSeparatorStateInput {
        has_custom_content: true,
        has_custom_class_name: true,
    });

    assert_eq!(separator.state_attr, "customized");
    assert_eq!(separator.content_source_attr, "custom");
    assert_eq!(separator.class_source_attr, "custom");
}
