use super::*;

#[test]
fn tone_and_elevation_contracts_are_stable() {
    assert_eq!(
        SurfaceTone::Default.class_name(),
        "ui-surface--tone-default"
    );
    assert_eq!(SurfaceTone::Subtle.class_name(), "ui-surface--tone-subtle");
    assert_eq!(SurfaceTone::Strong.class_name(), "ui-surface--tone-strong");

    assert_eq!(
        SurfaceElevation::Flat.class_name(),
        "ui-surface--elevation-flat"
    );
    assert_eq!(
        SurfaceElevation::Raised.class_name(),
        "ui-surface--elevation-raised"
    );
    assert_eq!(
        SurfaceElevation::Floating.class_name(),
        "ui-surface--elevation-floating"
    );
}

#[test]
fn normalize_aria_label_falls_back_to_default() {
    assert_eq!(
        normalize_aria_label(Some("  Dashboard card  ".to_string())),
        ("Dashboard card".to_string(), true)
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
fn resolve_state_tracks_state_and_source_markers() {
    let state = resolve_state(SurfaceStateInput {
        tone: SurfaceTone::Strong,
        elevation: SurfaceElevation::Floating,
        bordered: true,
        padded: true,
        has_custom_aria_label: true,
        has_custom_class_name: true,
    });

    assert_eq!(state.tone_attr, "strong");
    assert_eq!(state.elevation_attr, "floating");
    assert!(state.is_bordered);
    assert!(state.is_padded);
    assert!(!state.is_plain);
    assert_eq!(state.data_state_attr, "framed");
    assert_eq!(state.aria_source_attr, "custom");
    assert_eq!(state.class_source_attr, "custom");
}
