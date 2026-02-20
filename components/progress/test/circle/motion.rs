use super::*;

#[test]
fn default_motion_matches_progress_circle_spring_contract() {
    let motion = ProgressCircleMotion::default();
    let expected = ui_motion::presets::spring_soft();

    assert_eq!(motion.spring, expected);
}

#[test]
fn sanitize_motion_falls_back_for_invalid_values() {
    let default = ProgressCircleMotion::default();

    let motion = sanitize_motion(ProgressCircleMotion {
        spring: ui_motion::spring::SpringConfig {
            stiffness: f64::NAN,
            damping: -1.0,
            mass: 0.0,
            precision: f64::INFINITY,
        },
    });

    assert_eq!(motion.spring.stiffness, default.spring.stiffness);
    assert_eq!(motion.spring.damping, default.spring.damping);
    assert_eq!(motion.spring.mass, default.spring.mass);
    assert_eq!(motion.spring.precision, default.spring.precision);
}

#[test]
fn supports_custom_spring_motion_contract() {
    let motion = ProgressCircleMotion {
        spring: ui_motion::spring::SpringConfig {
            stiffness: 310.0,
            damping: 22.0,
            mass: 1.1,
            precision: 0.002,
        },
    };

    assert_eq!(motion.spring.stiffness, 310.0);
    assert_eq!(motion.spring.damping, 22.0);
    assert_eq!(motion.spring.mass, 1.1);
    assert_eq!(motion.spring.precision, 0.002);
}
