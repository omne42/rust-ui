use super::*;

#[test]
fn default_motion_matches_bb_params() {
    let motion = ButtonMotion::default();
    let tokens = default_button_motion_tokens();
    assert_eq!(
        motion.spring,
        ui_motion::spring::SpringConfig {
            stiffness: tokens.spring.stiffness,
            damping: tokens.spring.damping,
            mass: tokens.spring.mass,
            precision: tokens.spring.precision,
        }
    );
    assert_eq!(motion.hover_scale, tokens.hover_scale);
    assert_eq!(motion.tap_scale, tokens.tap_scale);
}

#[test]
fn sanitize_motion_falls_back_for_invalid_values() {
    let motion = sanitize_motion(ButtonMotion {
        spring: ui_motion::spring::SpringConfig {
            stiffness: f64::NAN,
            damping: -1.0,
            mass: 0.0,
            precision: f64::INFINITY,
        },
        hover_scale: f64::NAN,
        tap_scale: f64::NAN,
    });

    let default = ButtonMotion::default();
    assert_eq!(motion.spring.stiffness, default.spring.stiffness);
    assert_eq!(motion.spring.damping, default.spring.damping);
    assert_eq!(motion.spring.mass, default.spring.mass);
    assert_eq!(motion.spring.precision, default.spring.precision);
    assert_eq!(motion.hover_scale, default.hover_scale);
    assert_eq!(motion.tap_scale, default.tap_scale);
}

#[test]
fn sanitize_motion_clamps_scale_values() {
    let motion = sanitize_motion(ButtonMotion {
        spring: ui_motion::spring::SpringConfig {
            stiffness: 320.0,
            damping: 20.0,
            mass: 1.1,
            precision: 0.002,
        },
        hover_scale: 5.0,
        tap_scale: -2.0,
    });

    assert_eq!(motion.spring.stiffness, 320.0);
    assert_eq!(motion.spring.damping, 20.0);
    assert_eq!(motion.spring.mass, 1.1);
    assert_eq!(motion.spring.precision, 0.002);
    assert_eq!(motion.hover_scale, 2.0);
    assert_eq!(motion.tap_scale, 0.5);
}

#[cfg(feature = "component-button_group")]
#[test]
fn default_button_group_motion_matches_spring_contract() {
    let motion = ButtonGroupMotion::default();
    assert!(motion.spring.stiffness > 0.0);
    assert!(motion.spring.damping > 0.0);
    assert!(motion.spring.mass > 0.0);
    assert!(motion.enter_scale > 0.0);
    assert!(motion.enter_scale <= 1.0);
}

#[cfg(feature = "component-button_group")]
#[test]
fn sanitize_button_group_motion_keeps_valid_values() {
    let motion = sanitize_button_group_motion(ButtonGroupMotion {
        spring: ui_motion::spring::SpringConfig {
            stiffness: 300.0,
            damping: 22.0,
            mass: 1.2,
            precision: 0.002,
        },
        enter_scale: 1.08,
    });

    assert_eq!(motion.spring.stiffness, 300.0);
    assert_eq!(motion.spring.damping, 22.0);
    assert_eq!(motion.spring.mass, 1.2);
    assert_eq!(motion.spring.precision, 0.002);
    assert_eq!(motion.enter_scale, 1.08);
}

#[cfg(feature = "component-button_group")]
#[test]
fn sanitize_button_group_motion_falls_back_for_invalid_values() {
    let motion = sanitize_button_group_motion(ButtonGroupMotion {
        spring: ui_motion::spring::SpringConfig {
            stiffness: f64::NAN,
            damping: -1.0,
            mass: 0.0,
            precision: f64::INFINITY,
        },
        enter_scale: f64::NAN,
    });

    let default = ButtonGroupMotion::default();
    assert_eq!(motion.spring.stiffness, default.spring.stiffness);
    assert_eq!(motion.spring.damping, default.spring.damping);
    assert_eq!(motion.spring.mass, default.spring.mass);
    assert_eq!(motion.spring.precision, default.spring.precision);
    assert_eq!(motion.enter_scale, default.enter_scale);
}
