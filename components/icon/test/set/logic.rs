use super::*;

#[test]
fn normalize_and_parse_helpers_trim_values() {
    assert_eq!(normalize_optional_text(None), None);
    assert_eq!(normalize_optional_text(Some(" \n ".to_string())), None);
    assert_eq!(
        normalize_optional_text(Some("  workflow  ".to_string())),
        Some("workflow".to_string())
    );

    assert_eq!(
        parse_icon_reference("workflow:check"),
        (Some("workflow".to_string()), "check".to_string())
    );
    assert_eq!(parse_icon_reference("alert"), (None, "alert".to_string()));
}

#[test]
fn resolve_iconset_namespace_tracks_source_priority() {
    assert_eq!(
        resolve_iconset_namespace(Some("ui".to_string()), Some("workflow".to_string())),
        ("ui".to_string(), "prop", true, true)
    );
    assert_eq!(
        resolve_iconset_namespace(None, Some("workflow".to_string())),
        ("workflow".to_string(), "icon", false, true)
    );
    assert_eq!(
        resolve_iconset_namespace(None, None),
        (DEFAULT_ICONSET_NAMESPACE.into(), "default", false, false)
    );
}

#[test]
fn registry_helpers_resolve_matches_and_fallbacks() {
    assert!(glyph_matches("workflow:check", "workflow", "check"));
    assert!(glyph_matches("check", "workflow", "check"));
    assert!(!glyph_matches("ui:check", "workflow", "check"));

    let (glyph, matched, label) = resolve_registry_glyph(
        vec![IconsetGlyph::new("workflow:check", "✓").with_aria_label("Workflow Check")],
        "workflow",
        "check",
    );
    assert_eq!(glyph, "✓");
    assert!(matched);
    assert_eq!(label, Some("Workflow Check".to_string()));

    let (glyph, matched, label) = resolve_registry_glyph(vec![], "workflow", "alert");
    assert_eq!(glyph, FALLBACK_GLYPH);
    assert!(!matched);
    assert_eq!(label, None);
}

#[test]
fn resolve_state_and_class_name_surface_all_markers() {
    let state = resolve_state(IconsetStateInput {
        disabled: false,
        decorative: false,
        has_registry_match: true,
        has_registry_label: true,
        has_custom_iconset_prop: true,
        has_iconset_in_icon_reference: true,
        has_custom_aria_label: false,
        has_custom_class_name: true,
        has_custom_size: true,
        has_custom_tone: false,
    });

    assert_eq!(state.state_attr, "resolved");
    assert_eq!(state.icon_source_attr, "registry");
    assert_eq!(state.iconset_source_attr, "prop");
    assert_eq!(state.label_source_attr, "registry");
    assert_eq!(state.class_source_attr, "custom");
    assert_eq!(state.size_source_attr, "custom");
    assert_eq!(state.tone_source_attr, "default");

    let class_name = compose_class_name(Some("docs-iconset-state".to_string()), state);

    for token in [
        "ui-iconset",
        "ui-iconset--registry",
        "ui-iconset--custom-size",
        "ui-iconset--custom-class",
        "docs-iconset-state",
    ] {
        assert!(
            class_name.contains(token),
            "composed class name should include `{token}`"
        );
    }
}

#[test]
fn resolve_accessible_label_prioritizes_custom_then_registry_then_fallback() {
    assert_eq!(
        resolve_accessible_label(
            false,
            Some("  Custom Label  ".to_string()),
            Some("Registry".to_string()),
            "workflow-check"
        ),
        "Custom Label"
    );
    assert_eq!(
        resolve_accessible_label(false, None, Some("Registry".to_string()), "workflow-check"),
        "Registry"
    );
    assert_eq!(
        resolve_accessible_label(false, None, None, "workflow-check"),
        "workflow check"
    );
    assert_eq!(
        resolve_accessible_label(
            true,
            Some("Custom".to_string()),
            Some("Registry".to_string()),
            "workflow-check"
        ),
        ""
    );
}
