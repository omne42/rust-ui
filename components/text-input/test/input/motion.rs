use super::*;

#[test]
fn default_motion_has_reasonable_params() {
    let motion = InputMotion::default();
    assert!(motion.hidden_scale > 0.0);
    assert!(motion.hidden_scale < 1.0);
    assert!(motion.hover_scale >= 1.0);
    assert!(motion.tap_scale > 0.0);
    assert!(motion.tap_scale <= 1.0);
}

#[test]
fn sanitize_motion_falls_back_for_invalid_values() {
    let motion = sanitize_motion(InputMotion {
        spring: ui_motion::spring::SpringConfig {
            stiffness: f64::NAN,
            damping: -1.0,
            mass: 0.0,
            precision: f64::INFINITY,
        },
        hidden_scale: f64::NAN,
        hover_scale: f64::NAN,
        tap_scale: f64::NAN,
    });

    let default = InputMotion::default();
    assert_eq!(motion.spring.stiffness, default.spring.stiffness);
    assert_eq!(motion.spring.damping, default.spring.damping);
    assert_eq!(motion.spring.mass, default.spring.mass);
    assert_eq!(motion.spring.precision, default.spring.precision);
    assert_eq!(motion.hidden_scale, default.hidden_scale);
    assert_eq!(motion.hover_scale, default.hover_scale);
    assert_eq!(motion.tap_scale, default.tap_scale);
}

#[test]
fn sanitize_motion_clamps_scale_values() {
    let motion = sanitize_motion(InputMotion {
        spring: ui_motion::spring::SpringConfig {
            stiffness: 320.0,
            damping: 20.0,
            mass: 1.1,
            precision: 0.002,
        },
        hidden_scale: 5.0,
        hover_scale: 6.0,
        tap_scale: -2.0,
    });

    assert_eq!(motion.spring.stiffness, 320.0);
    assert_eq!(motion.spring.damping, 20.0);
    assert_eq!(motion.spring.mass, 1.1);
    assert_eq!(motion.spring.precision, 0.002);
    assert_eq!(motion.hidden_scale, 1.0);
    assert_eq!(motion.hover_scale, 2.0);
    assert_eq!(motion.tap_scale, 0.5);
}
