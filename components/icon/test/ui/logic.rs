use super::*;

#[test]
fn normalize_icon_reference_tracks_default_explicit_prefixed_paths() {
    assert_eq!(
        normalize_icon_reference("ui:check".to_string()),
        ("ui:check".to_string(), "explicit", true, false)
    );
    assert_eq!(
        normalize_icon_reference("check".to_string()),
        ("ui:check".to_string(), "prefixed", false, false)
    );
    assert_eq!(
        normalize_icon_reference("".to_string()),
        ("ui:help".to_string(), "default", false, true)
    );
}

#[test]
fn default_ui_glyphs_includes_help_and_common_contract_entries() {
    let glyphs = default_ui_glyphs();
    assert!(glyphs.iter().any(|glyph| glyph.name == "ui:help"));
    assert!(glyphs.iter().any(|glyph| glyph.name == "ui:check"));
}

#[test]
fn inner_defaults_are_resolved_in_logic() {
    assert_eq!(resolve_inner_aria_label(None), "");
    assert_eq!(
        resolve_inner_aria_label(Some("ui label".to_string())),
        "ui label"
    );

    assert_eq!(resolve_inner_class_name(None), "ui-icons-ui");
    assert_eq!(
        resolve_inner_class_name(Some("docs-ui".to_string())),
        "ui-icons-ui docs-ui"
    );
}

#[test]
fn resolve_state_tracks_sources_and_markers() {
    let state = resolve_state(IconsUiStateInput {
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
        Some("docs-icons-ui-state".to_string()),
        resolve_state(IconsUiStateInput {
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
        "ui-icons-ui",
        "ui-icons-ui--disabled",
        "ui-icons-ui--decorative",
        "ui-icons-ui--custom-glyphs",
        "ui-icons-ui--custom-size",
        "ui-icons-ui--custom-tone",
        "ui-icons-ui--custom-class",
        "docs-icons-ui-state",
    ] {
        assert!(
            class_name.contains(token),
            "composed class name should include `{token}`"
        );
    }
}
