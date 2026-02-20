use super::*;

#[test]
fn sanitize_motion_falls_back_for_invalid_values() {
    let default = IllustratedMessageMotion::default();

    let motion = sanitize_motion(IllustratedMessageMotion {
        spring: ui_motion::spring::SpringConfig {
            stiffness: f64::NAN,
            damping: -1.0,
            mass: 0.0,
            precision: f64::INFINITY,
        },
        initial_y_px: f64::NAN,
    });

    assert_eq!(motion.spring.stiffness, default.spring.stiffness);
    assert_eq!(motion.spring.damping, default.spring.damping);
    assert_eq!(motion.spring.mass, default.spring.mass);
    assert_eq!(motion.spring.precision, default.spring.precision);
    assert_eq!(motion.initial_y_px, default.initial_y_px);

    let capped = sanitize_motion(IllustratedMessageMotion {
        initial_y_px: -999.0,
        ..IllustratedMessageMotion::default()
    });
    assert_eq!(capped.initial_y_px, 120.0);
}

#[test]
fn default_motion_has_reasonable_params() {
    let motion = IllustratedMessageMotion::default();
    assert_eq!(motion.spring, ui_motion::presets::spring_soft());
    assert!(motion.initial_y_px.abs() > 0.0);
}
