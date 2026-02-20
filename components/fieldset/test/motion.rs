use super::*;

#[test]
fn sanitize_motion_clamps_invalid_values() {
    let motion = sanitize_motion(FieldsetMotion {
        duration_ms: f64::NAN,
        distance_px: -10.0,
    });

    let default = FieldsetMotion::default();
    assert_eq!(motion.duration_ms, default.duration_ms);
    assert_eq!(motion.distance_px, 0.0);
}

#[test]
fn attach_motion_serializes_css_vars() {
    let style = attach_motion(FieldsetMotion {
        duration_ms: 220.0,
        distance_px: 6.0,
    });

    assert!(style.contains("--ui-fieldset-motion-duration"));
    assert!(style.contains("--ui-fieldset-motion-distance"));
}
