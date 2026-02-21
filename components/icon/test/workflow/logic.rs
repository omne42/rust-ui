use super::*;

#[test]
fn normalize_icon_reference_tracks_default_explicit_prefixed_paths() {
    assert_eq!(
        normalize_icon_reference("workflow:success".to_string()),
        ("workflow:success".to_string(), "explicit", true, false)
    );
    assert_eq!(
        normalize_icon_reference("success".to_string()),
        ("workflow:success".to_string(), "prefixed", false, false)
    );
    assert_eq!(
        normalize_icon_reference("".to_string()),
        ("workflow:help".to_string(), "default", false, true)
    );
}

#[test]
fn default_workflow_glyphs_includes_help_and_common_contract_entries() {
    let glyphs = default_workflow_glyphs();
    assert!(glyphs.iter().any(|glyph| glyph.name == "workflow:help"));
    assert!(glyphs.iter().any(|glyph| glyph.name == "workflow:success"));
}

#[test]
fn inner_defaults_are_resolved_in_logic() {
    assert_eq!(resolve_inner_aria_label(None), "");
    assert_eq!(
        resolve_inner_aria_label(Some("workflow label".to_string())),
        "workflow label"
    );

    assert_eq!(resolve_inner_class_name(None), "ui-icons-workflow");
    assert_eq!(
        resolve_inner_class_name(Some("docs-workflow".to_string())),
        "ui-icons-workflow docs-workflow"
    );
}

#[test]
fn resolve_state_tracks_sources_and_markers() {
    let state = resolve_state(IconsWorkflowStateInput {
        disabled: false,
        decorative: false,
        has_explicit_icon_reference: true,
        used_default_icon_reference: false,
        has_custom_aria_label: true,
        has_custom_class_name: false,
        has_custom_glyphs: true,
        has_custom_size: true,
        has_custom_tone: false,
    });

    assert_eq!(state.state_attr, "ready");
    assert_eq!(state.icon_reference_source_attr, "explicit");
    assert_eq!(state.aria_source_attr, "custom");
    assert_eq!(state.class_source_attr, "default");
    assert_eq!(state.glyph_source_attr, "custom");
    assert_eq!(state.size_source_attr, "custom");
    assert_eq!(state.tone_source_attr, "default");
}

#[test]
fn compose_class_name_includes_state_and_custom_markers() {
    let class_name = compose_class_name(
        Some("docs-icons-workflow-state".to_string()),
        resolve_state(IconsWorkflowStateInput {
            disabled: true,
            decorative: true,
            has_explicit_icon_reference: false,
            used_default_icon_reference: true,
            has_custom_aria_label: false,
            has_custom_class_name: true,
            has_custom_glyphs: true,
            has_custom_size: true,
            has_custom_tone: true,
        }),
    );

    for token in [
        "ui-icons-workflow",
        "ui-icons-workflow--disabled",
        "ui-icons-workflow--decorative",
        "ui-icons-workflow--custom-glyphs",
        "ui-icons-workflow--custom-size",
        "ui-icons-workflow--custom-tone",
        "ui-icons-workflow--custom-class",
        "docs-icons-workflow-state",
    ] {
        assert!(
            class_name.contains(token),
            "composed class name should include `{token}`"
        );
    }
}
