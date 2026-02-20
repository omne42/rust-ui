use super::*;

#[test]
fn normalize_control_state_prefers_is_prefixed_props() {
    let normalized = normalize_control_state(SurfaceControlInput {
        is_bordered: Some(true),
        is_padded: Some(false),
    });

    assert!(normalized.bordered);
    assert!(!normalized.padded);
    assert_eq!(normalized.bordered_source_attr, "is-prop");
    assert_eq!(normalized.padded_source_attr, "is-prop");
}

#[test]
fn normalize_control_state_uses_defaults_when_props_absent() {
    let normalized = normalize_control_state(SurfaceControlInput {
        is_bordered: None,
        is_padded: None,
    });

    assert!(!normalized.bordered);
    assert!(normalized.padded);
    assert_eq!(normalized.bordered_source_attr, "default");
    assert_eq!(normalized.padded_source_attr, "default");
}

#[test]
fn normalize_root_state_centralizes_defaults_and_sources() {
    let state = normalize_root_state(SurfaceRootInput {
        tone: SurfaceTone::Subtle,
        elevation: SurfaceElevation::Flat,
        control: SurfaceControlInput {
            is_bordered: None,
            is_padded: None,
        },
        aria_label: None,
        class_name: Some("  docs-surface-custom ".to_string()),
    });

    assert_eq!(state.aria_label, DEFAULT_ARIA_LABEL);
    assert_eq!(state.class_name, Some("docs-surface-custom".to_string()));
    assert_eq!(state.state.tone_attr, "subtle");
    assert_eq!(state.state.elevation_attr, "flat");
    assert_eq!(state.state.data_state_attr, "padded");
    assert_eq!(state.state.aria_source_attr, "default");
    assert_eq!(state.state.class_source_attr, "custom");
    assert_eq!(state.bordered_source_attr, "default");
    assert_eq!(state.padded_source_attr, "default");
}

#[test]
fn compose_class_name_merges_custom_class() {
    let state = resolve_state(SurfaceStateInput {
        tone: SurfaceTone::Subtle,
        elevation: SurfaceElevation::Raised,
        bordered: false,
        padded: true,
        has_custom_aria_label: false,
        has_custom_class_name: true,
    });

    let class_name = compose_class_name(Some("docs-surface-custom".to_string()), state);

    assert!(class_name.contains("ui-surface"));
    assert!(class_name.contains("ui-surface--tone-subtle"));
    assert!(class_name.contains("ui-surface--elevation-raised"));
    assert!(class_name.contains("ui-surface--padded"));
    assert!(class_name.contains("ui-surface--custom-class"));
    assert!(class_name.contains("docs-surface-custom"));
}
