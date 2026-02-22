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
    let default = CheckboxMotion::default();
    let custom_spring = ui_motion::spring::SpringConfig {
        stiffness: default.spring.stiffness + 60.0,
        damping: default.spring.damping + 4.0,
        mass: default.spring.mass + 0.1,
        precision: default.spring.precision * 2.0,
    };
    let custom_indicator_spring = ui_motion::spring::SpringConfig {
        stiffness: default.indicator_spring.stiffness + 80.0,
        damping: default.indicator_spring.damping + 8.0,
        mass: default.indicator_spring.mass + 0.2,
        precision: default.indicator_spring.precision * 3.0,
    };
    let motion = sanitize_motion(CheckboxMotion {
        spring: custom_spring,
        hover_scale: 5.0,
        tap_scale: -2.0,
        indicator_spring: custom_indicator_spring,
    });

    assert_eq!(motion.spring.stiffness, custom_spring.stiffness);
    assert_eq!(motion.spring.damping, custom_spring.damping);
    assert_eq!(motion.spring.mass, custom_spring.mass);
    assert_eq!(motion.spring.precision, custom_spring.precision);
    assert_eq!(motion.hover_scale, 2.0);
    assert_eq!(motion.tap_scale, 0.5);
    assert_eq!(
        motion.indicator_spring.stiffness,
        custom_indicator_spring.stiffness
    );
    assert_eq!(
        motion.indicator_spring.damping,
        custom_indicator_spring.damping
    );
    assert_eq!(motion.indicator_spring.mass, custom_indicator_spring.mass);
    assert_eq!(
        motion.indicator_spring.precision,
        custom_indicator_spring.precision
    );
}
