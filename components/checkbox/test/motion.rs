use super::*;

#[test]
fn default_motion_has_reasonable_params() {
    let motion = CheckboxMotion::default();
    assert!(motion.spring.stiffness > 0.0);
    assert!(motion.spring.damping > 0.0);
    assert!(motion.spring.mass > 0.0);
    assert!(motion.indicator_spring.stiffness > 0.0);
    assert!(motion.indicator_spring.damping > 0.0);
    assert!(motion.indicator_spring.mass > 0.0);
    assert!(motion.hover_scale >= 1.0);
    assert!(motion.tap_scale > 0.0);
    assert!(motion.tap_scale <= 1.0);
}

#[test]
fn sanitize_motion_falls_back_for_invalid_values() {
    let motion = sanitize_motion(CheckboxMotion {
        spring: ui_motion::spring::SpringConfig {
            stiffness: f64::NAN,
            damping: -1.0,
            mass: 0.0,
            precision: f64::INFINITY,
        },
        hover_scale: f64::NAN,
        tap_scale: f64::NAN,
        indicator_spring: ui_motion::spring::SpringConfig {
            stiffness: f64::NAN,
            damping: -1.0,
            mass: 0.0,
            precision: f64::INFINITY,
        },
    });

    let default = CheckboxMotion::default();
    assert_eq!(motion.spring.stiffness, default.spring.stiffness);
    assert_eq!(motion.spring.damping, default.spring.damping);
    assert_eq!(motion.spring.mass, default.spring.mass);
    assert_eq!(motion.spring.precision, default.spring.precision);
    assert_eq!(motion.hover_scale, default.hover_scale);
    assert_eq!(motion.tap_scale, default.tap_scale);
    assert_eq!(
        motion.indicator_spring.stiffness,
        default.indicator_spring.stiffness
    );
    assert_eq!(
        motion.indicator_spring.damping,
        default.indicator_spring.damping
    );
    assert_eq!(motion.indicator_spring.mass, default.indicator_spring.mass);
    assert_eq!(
        motion.indicator_spring.precision,
        default.indicator_spring.precision
    );
}

#[test]
fn sanitize_motion_clamps_scale_values_and_keeps_valid_springs() {
    let motion = sanitize_motion(CheckboxMotion {
        spring: ui_motion::spring::SpringConfig {
            stiffness: 320.0,
            damping: 20.0,
            mass: 1.1,
            precision: 0.002,
        },
        hover_scale: 5.0,
        tap_scale: -2.0,
        indicator_spring: ui_motion::spring::SpringConfig {
            stiffness: 420.0,
            damping: 24.0,
            mass: 1.2,
            precision: 0.003,
        },
    });

    assert_eq!(motion.spring.stiffness, 320.0);
    assert_eq!(motion.spring.damping, 20.0);
    assert_eq!(motion.spring.mass, 1.1);
    assert_eq!(motion.spring.precision, 0.002);
    assert_eq!(motion.hover_scale, 2.0);
    assert_eq!(motion.tap_scale, 0.5);
    assert_eq!(motion.indicator_spring.stiffness, 420.0);
    assert_eq!(motion.indicator_spring.damping, 24.0);
    assert_eq!(motion.indicator_spring.mass, 1.2);
    assert_eq!(motion.indicator_spring.precision, 0.003);
}
