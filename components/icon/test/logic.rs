use super::*;

#[test]
fn size_and_tone_contracts_are_stable() {
    assert_eq!(IconSize::Sm.class_name(), "ui-icon--size-sm");
    assert_eq!(IconSize::Md.class_name(), "ui-icon--size-md");
    assert_eq!(IconSize::Lg.class_name(), "ui-icon--size-lg");
    assert_eq!(IconSize::Sm.as_attr(), "sm");
    assert_eq!(IconSize::Md.as_attr(), "md");
    assert_eq!(IconSize::Lg.as_attr(), "lg");

    assert_eq!(IconTone::Default.class_name(), "ui-icon--tone-default");
    assert_eq!(IconTone::Muted.class_name(), "ui-icon--tone-muted");
    assert_eq!(IconTone::Accent.class_name(), "ui-icon--tone-accent");
    assert_eq!(IconTone::Danger.class_name(), "ui-icon--tone-danger");
    assert_eq!(IconTone::Default.as_attr(), "default");
    assert_eq!(IconTone::Muted.as_attr(), "muted");
    assert_eq!(IconTone::Accent.as_attr(), "accent");
    assert_eq!(IconTone::Danger.as_attr(), "danger");
}

#[test]
fn normalize_helpers_trim_and_fallback() {
    assert_eq!(normalize_optional_text(None), None);
    assert_eq!(normalize_optional_text(Some("  \n\t ".to_string())), None);
    assert_eq!(
        normalize_optional_text(Some("  Completion state  ".to_string())),
        Some("Completion state".to_string())
    );

    assert_eq!(
        normalize_aria_label(Some("  Save success  ".to_string())),
        ("Save success".to_string(), true)
    );
    assert_eq!(
        normalize_aria_label(None),
        (DEFAULT_ARIA_LABEL.into(), false)
    );
}

#[test]
fn slot_kind_contract_is_stable() {
    assert_eq!(resolve_slot_kind_attr(None), "none");
    assert_eq!(resolve_slot_kind_attr(Some("label")), "label");
    assert_eq!(resolve_slot_kind_attr(Some("DESCRIPTION")), "description");
    assert_eq!(resolve_slot_kind_attr(Some("icon")), "icon");
    assert_eq!(resolve_slot_kind_attr(Some("trailing")), "custom");
}

#[test]
fn resolve_state_tracks_accessibility_and_sources() {
    let state = resolve_state(IconStateInput {
        size: IconSize::Lg,
        tone: IconTone::Accent,
        disabled: false,
        decorative: false,
        has_custom_aria_label: true,
        has_custom_class_name: true,
        slot_kind_attr: "icon",
        has_named_slot: true,
    });

    assert_eq!(state.data_state_attr, "labeled");
    assert!(state.has_accessible_name);
    assert_eq!(state.aria_source_attr, "custom");
    assert_eq!(state.class_source_attr, "custom");
    assert_eq!(state.slot_kind_attr, "icon");
    assert!(state.has_named_slot);

    let decorative = resolve_state(IconStateInput {
        size: IconSize::Md,
        tone: IconTone::Default,
        disabled: false,
        decorative: true,
        has_custom_aria_label: false,
        has_custom_class_name: false,
        slot_kind_attr: "none",
        has_named_slot: false,
    });

    assert_eq!(decorative.data_state_attr, "decorative");
    assert!(decorative.is_decorative);
    assert!(!decorative.has_accessible_name);

    let non_decorative_with_default_label = resolve_state(IconStateInput {
        size: IconSize::Md,
        tone: IconTone::Default,
        disabled: false,
        decorative: false,
        has_custom_aria_label: false,
        has_custom_class_name: false,
        slot_kind_attr: "none",
        has_named_slot: false,
    });
    assert!(non_decorative_with_default_label.has_accessible_name);
    assert_eq!(
        non_decorative_with_default_label.aria_source_attr,
        "default"
    );
}

#[test]
fn compose_class_name_includes_state_markers() {
    let state = resolve_state(IconStateInput {
        size: IconSize::Sm,
        tone: IconTone::Danger,
        disabled: true,
        decorative: true,
        has_custom_aria_label: false,
        has_custom_class_name: true,
        slot_kind_attr: "none",
        has_named_slot: false,
    });

    let class_name = compose_class_name(Some("docs-icon-custom".to_string()), state);

    for token in [
        "ui-icon",
        "ui-icon--size-sm",
        "ui-icon--tone-danger",
        "ui-icon--disabled",
        "ui-icon--decorative",
        "ui-icon--custom-class",
        "docs-icon-custom",
    ] {
        assert!(
            class_name.contains(token),
            "composed class name should include `{token}`"
        );
    }
}
