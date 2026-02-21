use super::*;

#[test]
fn sanitize_motion_preserves_contract() {
    let motion = ChartMotion::default();
    assert_eq!(sanitize_motion(motion), motion);
}

#[test]
fn sanitize_motion_falls_back_for_invalid_spring_values() {
    let fallback = ui_motion::presets::spring_slide();
    let motion = ChartMotion {
        spring: ui_motion::spring::SpringConfig {
            stiffness: f64::NAN,
            damping: -1.0,
            mass: 0.0,
            precision: f64::INFINITY,
        },
    };

    let sanitized = sanitize_motion(motion);
    assert_eq!(sanitized.spring, fallback);
}
