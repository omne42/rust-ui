use super::*;

#[test]
fn resolve_state_core_tracks_interaction_and_source_metadata() {
    let active = resolve_state_core(FlipButtonStateCoreInput {
        is_hovered: true,
        is_focus_within: false,
        has_custom_class_name: true,
        has_custom_motion: true,
    });

    assert!(active.is_active);
    assert!(!active.is_inactive);
    assert_eq!(active.state_attr, "active");
    assert_eq!(active.hover_attr, "hovered");
    assert_eq!(active.focus_within_attr, "inactive");
    assert_eq!(active.class_source_attr, "custom");
    assert_eq!(active.motion_source_attr, "custom");

    let inactive = resolve_state_core(FlipButtonStateCoreInput {
        is_hovered: false,
        is_focus_within: false,
        has_custom_class_name: false,
        has_custom_motion: false,
    });

    assert!(!inactive.is_active);
    assert!(inactive.is_inactive);
    assert_eq!(inactive.state_attr, "inactive");
    assert_eq!(inactive.hover_attr, "resting");
    assert_eq!(inactive.focus_within_attr, "inactive");
    assert_eq!(inactive.class_source_attr, "default");
    assert_eq!(inactive.motion_source_attr, "default");
}
