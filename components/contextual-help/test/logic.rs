use super::*;

#[test]
fn variant_default_labels_are_stable() {
    assert_eq!(ContextualHelpVariant::Help.default_label(), "Help");
    assert_eq!(ContextualHelpVariant::Info.default_label(), "Info");

    assert_eq!(ContextualHelpVariant::Help.as_attr(), "help");
    assert_eq!(ContextualHelpVariant::Info.as_attr(), "info");
}

#[test]
fn normalize_optional_text_trims_and_filters_blank_values() {
    assert_eq!(normalize_optional_text(None), None);
    assert_eq!(normalize_optional_text(Some("  \n\t".to_string())), None);
    assert_eq!(
        normalize_optional_text(Some(" docs-contextual-help ".to_string())),
        Some("docs-contextual-help".to_string())
    );
}

#[test]
fn resolve_trigger_aria_label_uses_custom_or_default() {
    assert_eq!(
        resolve_trigger_aria_label(ContextualHelpVariant::Help, None),
        ("Help".to_string(), false)
    );
    assert_eq!(
        resolve_trigger_aria_label(
            ContextualHelpVariant::Info,
            Some("  Learn more  ".to_string())
        ),
        ("Learn more".to_string(), true)
    );
    assert_eq!(
        resolve_trigger_aria_label(ContextualHelpVariant::Info, Some("  ".to_string())),
        ("Info".to_string(), false)
    );
}

#[test]
fn resolve_id_uses_custom_or_fallback() {
    assert_eq!(
        resolve_id(Some(" docs-help ".to_string()), "fallback".to_string()),
        ("docs-help".to_string(), true)
    );
    assert_eq!(
        resolve_id(Some("   ".to_string()), "fallback".to_string()),
        ("fallback".to_string(), false)
    );
}

#[test]
fn resolve_state_tracks_flags_and_attrs() {
    let state = resolve_state(ContextualHelpStateInput {
        variant: ContextualHelpVariant::Info,
        placement: PopoverPlacement::TopEnd,
        disabled: true,
        has_heading: false,
        has_footer: true,
        has_custom_aria_label: true,
        has_custom_class_name: true,
        has_custom_id: false,
        has_custom_motion: true,
        is_controlled: true,
    });

    assert_eq!(state.variant, ContextualHelpVariant::Info);
    assert_eq!(state.variant_class, "ui-contextual-help--variant-info");
    assert_eq!(state.variant_attr, "info");

    assert_eq!(state.placement, PopoverPlacement::TopEnd);
    assert_eq!(
        state.placement_class,
        "ui-contextual-help--placement-top-end"
    );
    assert_eq!(state.placement_attr, "top-end");

    assert!(state.is_disabled);
    assert_eq!(state.state_attr, "disabled");

    assert!(!state.has_heading);
    assert_eq!(state.heading_attr, "absent");

    assert!(state.has_footer);
    assert_eq!(state.footer_attr, "present");

    assert_eq!(state.label_source_attr, "custom");
    assert_eq!(state.class_source_attr, "custom");
    assert_eq!(state.id_source_attr, "auto");
    assert_eq!(state.motion_source_attr, "custom");

    assert!(state.is_controlled);
    assert_eq!(state.open_mode_attr, "controlled");

    assert!(state.has_custom_class_name);
    assert!(state.has_custom_motion);
}

#[test]
fn compose_class_name_includes_state_markers() {
    let class_name = compose_class_name(
        Some("docs-help".to_string()),
        resolve_state(ContextualHelpStateInput {
            variant: ContextualHelpVariant::Help,
            placement: PopoverPlacement::BottomStart,
            disabled: false,
            has_heading: true,
            has_footer: false,
            has_custom_aria_label: false,
            has_custom_class_name: true,
            has_custom_id: true,
            has_custom_motion: true,
            is_controlled: false,
        }),
    );

    for token in [
        "ui-contextual-help",
        "ui-contextual-help--variant-help",
        "ui-contextual-help--placement-bottom-start",
        "ui-contextual-help--enabled",
        "ui-contextual-help--with-heading",
        "ui-contextual-help--no-footer",
        "ui-contextual-help--uncontrolled",
        "ui-contextual-help--custom-class",
        "ui-contextual-help--custom-motion",
        "docs-help",
    ] {
        assert!(
            class_name.contains(token),
            "composed class name should include `{token}`"
        );
    }
}
