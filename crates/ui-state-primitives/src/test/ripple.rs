use super::*;

#[test]
fn phase_contract_is_stable() {
    assert_eq!(
        RipplePhase::Animated.class_name(),
        "ui-ripple--state-animated"
    );
    assert_eq!(RipplePhase::Animated.as_str(), "animated");
    assert_eq!(RipplePhase::Static.class_name(), "ui-ripple--state-static");
    assert_eq!(RipplePhase::Static.as_str(), "static");
}

#[test]
fn boundary_contract_is_stable() {
    assert_eq!(
        RippleBoundary::Bounded.class_name(),
        "ui-ripple--boundary-bounded"
    );
    assert_eq!(RippleBoundary::Bounded.as_str(), "bounded");
    assert!(RippleBoundary::Bounded.is_bounded());

    assert_eq!(
        RippleBoundary::Unbounded.class_name(),
        "ui-ripple--boundary-unbounded"
    );
    assert_eq!(RippleBoundary::Unbounded.as_str(), "unbounded");
    assert!(!RippleBoundary::Unbounded.is_bounded());
}

#[test]
fn normalize_optional_text_filters_blank_values() {
    assert_eq!(normalize_optional_text(None), None);
    assert_eq!(normalize_optional_text(Some("  \n\t  ".to_string())), None);
    assert_eq!(
        normalize_optional_text(Some("  docs-ripple-item  ".to_string())),
        Some("docs-ripple-item".to_string())
    );
}

#[test]
fn resolve_helpers_keep_state_derivation_explicit() {
    assert_eq!(resolve_phase(true), RipplePhase::Animated);
    assert_eq!(resolve_phase(false), RipplePhase::Static);
    assert_eq!(resolve_boundary(true), RippleBoundary::Bounded);
    assert_eq!(resolve_boundary(false), RippleBoundary::Unbounded);
}

#[test]
fn resolve_state_tracks_source_markers() {
    let state = resolve_state(RippleStateInput {
        phase: RipplePhase::Animated,
        boundary: RippleBoundary::Unbounded,
        has_custom_motion: true,
        has_custom_class_name: true,
    });

    assert_eq!(state.phase_class, "ui-ripple--state-animated");
    assert_eq!(state.phase_attr, "animated");
    assert_eq!(state.boundary_class, "ui-ripple--boundary-unbounded");
    assert_eq!(state.boundary_attr, "unbounded");
    assert!(state.is_unbounded);
    assert!(!state.is_bounded);
    assert_eq!(state.motion_source_class, "ui-ripple--motion-custom");
    assert_eq!(state.motion_source_attr, "custom");
    assert_eq!(state.class_source_attr, "custom");
}

#[test]
fn compose_class_name_includes_state_markers() {
    let state = resolve_state(RippleStateInput {
        phase: RipplePhase::Static,
        boundary: RippleBoundary::Bounded,
        has_custom_motion: false,
        has_custom_class_name: true,
    });
    let class_name = compose_class_name(Some("docs-ripple-item".to_string()), state);

    for token in [
        "ui-ripple",
        "ui-ripple--state-static",
        "ui-ripple--boundary-bounded",
        "ui-ripple--motion-default",
        "ui-ripple--custom-class",
        "docs-ripple-item",
    ] {
        assert!(
            class_name.contains(token),
            "composed class name should include `{token}`"
        );
    }
}
