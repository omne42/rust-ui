use super::*;

#[test]
fn default_motion_has_reasonable_params() {
    let motion = DateInputGroupMotion::default();
    assert!(motion.spring.stiffness > 0.0);
    assert!(motion.spring.damping > 0.0);
    assert!(motion.spring.mass > 0.0);
    assert!(motion.enter_scale > 0.0);
    assert!(motion.enter_scale <= 1.0);
}

#[test]
fn sanitize_motion_falls_back_for_invalid_values() {
    let motion = sanitize_motion(DateInputGroupMotion {
        spring: ui_motion::spring::SpringConfig {
            stiffness: f64::NAN,
            damping: -1.0,
            mass: 0.0,
            precision: f64::INFINITY,
        },
        enter_scale: f64::NAN,
    });

    let default = DateInputGroupMotion::default();
    assert_eq!(motion.spring.stiffness, default.spring.stiffness);
    assert_eq!(motion.spring.damping, default.spring.damping);
    assert_eq!(motion.spring.mass, default.spring.mass);
    assert_eq!(motion.spring.precision, default.spring.precision);
    assert_eq!(motion.enter_scale, default.enter_scale);
}

#[test]
fn sanitize_motion_clamps_scale_values() {
    let default = DateInputGroupMotion::default();
    let motion = sanitize_motion(DateInputGroupMotion {
        spring: ui_motion::spring::SpringConfig {
            stiffness: default.spring.stiffness + 20.0,
            damping: default.spring.damping + 2.0,
            mass: default.spring.mass,
            precision: default.spring.precision * 2.0,
        },
        enter_scale: 8.0,
    });
    assert_eq!(motion.enter_scale, 1.5);
}
