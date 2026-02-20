use super::*;

#[test]
fn sanitize_motion_falls_back_for_invalid_values() {
    let default = ToastMotion::default();

    let motion = sanitize_motion(ToastMotion {
        spring: ui_motion::spring::SpringConfig {
            stiffness: f64::NAN,
            damping: -1.0,
            mass: 0.0,
            precision: f64::INFINITY,
        },
        initial_y_px: f64::INFINITY,
        initial_scale: 0.0,
    });

    assert_eq!(motion.spring.stiffness, default.spring.stiffness);
    assert_eq!(motion.spring.damping, default.spring.damping);
    assert_eq!(motion.spring.mass, default.spring.mass);
    assert_eq!(motion.spring.precision, default.spring.precision);
    assert_eq!(motion.initial_y_px, default.initial_y_px);
    assert_eq!(motion.initial_scale, default.initial_scale);
}

#[test]
fn supports_custom_spring_motion_contract() {
    let motion = sanitize_motion(ToastMotion {
        spring: ui_motion::spring::SpringConfig {
            stiffness: 320.0,
            damping: 24.0,
            mass: 1.0,
            precision: 0.002,
        },
        initial_y_px: 20.0,
        initial_scale: 0.94,
    });

    assert_eq!(motion.spring.stiffness, 320.0);
    assert_eq!(motion.spring.damping, 24.0);
    assert_eq!(motion.spring.mass, 1.0);
    assert_eq!(motion.spring.precision, 0.002);
    assert_eq!(motion.initial_y_px, 20.0);
    assert_eq!(motion.initial_scale, 0.94);
}

#[test]
fn default_motion_matches_slide_preset() {
    let motion = ToastMotion::default();
    assert_eq!(motion.spring, ui_motion::presets::spring_slide());
    assert!(motion.initial_y_px.abs() > 0.0);
    assert!(motion.initial_scale > 0.0);
    assert!(motion.initial_scale <= 1.0);
}
