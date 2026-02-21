use super::*;
use crate::icons::IconsScale;

#[test]
fn parse_and_resolve_set_follow_name_prefix_priority() {
    assert_eq!(
        parse_set_from_name("workflow:check"),
        Some(IconsSet::Workflow)
    );
    assert_eq!(parse_set_from_name("ui:check"), Some(IconsSet::Ui));
    assert_eq!(parse_set_from_name("check"), None);

    assert_eq!(
        resolve_set("workflow:check", IconsSet::Ui),
        (IconsSet::Workflow, true)
    );
    assert_eq!(
        resolve_set("check", IconsSet::Workflow),
        (IconsSet::Workflow, false)
    );
}

#[test]
fn normalize_name_preserves_prefix_and_applies_defaults() {
    assert_eq!(
        normalize_name("check".to_string(), IconsSet::Ui),
        "ui:check"
    );
    assert_eq!(
        normalize_name("workflow:check".to_string(), IconsSet::Ui),
        "workflow:check"
    );
    assert_eq!(
        normalize_name("".to_string(), IconsSet::Workflow),
        "workflow:help"
    );
}

#[test]
fn inner_defaults_are_resolved_in_logic() {
    assert_eq!(resolve_inner_aria_label(None), "");
    assert_eq!(
        resolve_inner_aria_label(Some("icons label".to_string())),
        "icons label"
    );

    assert_eq!(resolve_inner_class_name(None), "ui-icons");
    assert_eq!(
        resolve_inner_class_name(Some("docs-icons".to_string())),
        "ui-icons docs-icons"
    );
}

#[test]
fn resolve_state_tracks_sources_and_state_markers() {
    let state = resolve_state(IconsStateInput {
        set: IconsSet::Workflow,
        scale: IconsScale::Large,
        disabled: false,
        decorative: false,
        has_set_prefix_in_name: true,
        has_custom_set_prop: true,
        has_custom_aria_label: true,
        has_custom_class_name: false,
        has_custom_glyphs: true,
        has_custom_tone: false,
    });

    assert_eq!(state.set_attr, "workflow");
    assert_eq!(state.scale_attr, "large");
    assert_eq!(state.state_attr, "ready");
    assert_eq!(state.set_source_attr, "name");
    assert_eq!(state.aria_source_attr, "custom");
    assert_eq!(state.class_source_attr, "default");
    assert_eq!(state.glyph_source_attr, "custom");
    assert_eq!(state.tone_source_attr, "default");
}

#[test]
fn compose_class_name_includes_state_and_custom_markers() {
    let class_name = compose_class_name(
        Some("docs-icons-state".to_string()),
        resolve_state(IconsStateInput {
            set: IconsSet::Ui,
            scale: IconsScale::Medium,
            disabled: true,
            decorative: true,
            has_set_prefix_in_name: false,
            has_custom_set_prop: false,
            has_custom_aria_label: false,
            has_custom_class_name: true,
            has_custom_glyphs: true,
            has_custom_tone: true,
        }),
    );

    for token in [
        "ui-icons",
        "ui-icons--set-ui",
        "ui-icons--scale-medium",
        "ui-icons--disabled",
        "ui-icons--decorative",
        "ui-icons--custom-glyphs",
        "ui-icons--custom-tone",
        "ui-icons--custom-class",
        "docs-icons-state",
    ] {
        assert!(
            class_name.contains(token),
            "composed class name should include `{token}`"
        );
    }
}
