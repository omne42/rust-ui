use super::*;

#[test]
fn tone_and_align_contracts_are_stable() {
    assert_eq!(
        EmptyStateTone::Default.class_name(),
        "ui-empty-state--tone-default"
    );
    assert_eq!(
        EmptyStateTone::Muted.class_name(),
        "ui-empty-state--tone-muted"
    );
    assert_eq!(
        EmptyStateTone::Accent.class_name(),
        "ui-empty-state--tone-accent"
    );

    assert_eq!(EmptyStateTone::Default.as_attr(), "default");
    assert_eq!(EmptyStateTone::Muted.as_attr(), "muted");
    assert_eq!(EmptyStateTone::Accent.as_attr(), "accent");

    assert_eq!(
        EmptyStateAlign::Start.class_name(),
        "ui-empty-state--align-start"
    );
    assert_eq!(
        EmptyStateAlign::Center.class_name(),
        "ui-empty-state--align-center"
    );

    assert_eq!(EmptyStateAlign::Start.as_attr(), "start");
    assert_eq!(EmptyStateAlign::Center.as_attr(), "center");
}

#[test]
fn normalize_helpers_trim_and_fallback() {
    assert_eq!(normalize_optional_text(None), None);
    assert_eq!(normalize_optional_text(Some(" \n\t ".to_string())), None);
    assert_eq!(
        normalize_optional_text(Some("  add filters  ".to_string())),
        Some("add filters".to_string())
    );

    let (title, custom_title) = normalize_title(Some("  No matches  ".to_string()), "Default");
    assert_eq!(title, "No matches");
    assert!(custom_title);

    let (title, custom_title) = normalize_title(None, "Default title");
    assert_eq!(title, "Default title");
    assert!(!custom_title);

    let (description, custom_description) = normalize_description(
        Some("  Try another keyword  ".to_string()),
        "Default description",
    );
    assert_eq!(description, "Try another keyword");
    assert!(custom_description);

    let (description, custom_description) = normalize_description(None, "Default description");
    assert_eq!(description, "Default description");
    assert!(!custom_description);

    let (label, custom_label) =
        normalize_aria_label(Some("  Project state  ".to_string()), "Default label");
    assert_eq!(label, "Project state");
    assert!(custom_label);

    let (label, custom_label) = normalize_aria_label(None, "Default label");
    assert_eq!(label, "Default label");
    assert!(!custom_label);
}

#[test]
fn resolve_state_tracks_markers_and_sources() {
    let state = resolve_state(EmptyStateStateInput {
        tone: EmptyStateTone::Accent,
        align: EmptyStateAlign::Center,
        compact: true,
        bordered: true,
        has_icon: true,
        has_actions: false,
        has_custom_title: true,
        has_custom_description: false,
        has_custom_aria_label: true,
        has_custom_class_name: true,
    });

    assert_eq!(state.tone_attr, "accent");
    assert_eq!(state.align_attr, "center");
    assert_eq!(state.data_state_attr, "icon");
    assert!(state.is_compact);
    assert!(state.is_bordered);
    assert_eq!(state.title_source_attr, "custom");
    assert_eq!(state.description_source_attr, "default");
    assert_eq!(state.aria_source_attr, "custom");
    assert_eq!(state.class_source_attr, "custom");
}

#[test]
fn compose_class_name_includes_state_markers() {
    let class_name = compose_class_name(
        Some("docs-empty-state".to_string()),
        resolve_state(EmptyStateStateInput {
            tone: EmptyStateTone::Muted,
            align: EmptyStateAlign::Center,
            compact: true,
            bordered: true,
            has_icon: true,
            has_actions: true,
            has_custom_title: false,
            has_custom_description: false,
            has_custom_aria_label: false,
            has_custom_class_name: true,
        }),
    );

    for token in [
        "ui-empty-state",
        "ui-empty-state--tone-muted",
        "ui-empty-state--align-center",
        "ui-empty-state--compact",
        "ui-empty-state--bordered",
        "ui-empty-state--with-icon",
        "ui-empty-state--with-actions",
        "ui-empty-state--custom-class",
        "docs-empty-state",
    ] {
        assert!(
            class_name.contains(token),
            "composed class should include `{token}`"
        );
    }
}
