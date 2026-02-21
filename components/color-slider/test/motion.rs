use super::*;

#[test]
fn sanitize_percent_clamps_and_handles_nan() {
    assert_eq!(sanitize_percent(42.0), 42.0);
    assert_eq!(sanitize_percent(-2.0), 0.0);
    assert_eq!(sanitize_percent(140.0), 100.0);
    assert_eq!(sanitize_percent(f64::NAN), 0.0);
}

#[test]
fn sanitize_motion_falls_back_for_invalid_spring_values() {
    let motion = sanitize_motion(ColorSliderMotion {
        enabled: true,
        spring: ui_motion::spring::SpringConfig {
            stiffness: f64::NAN,
            damping: -1.0,
            mass: 0.0,
            precision: f64::INFINITY,
        },
    });

    let default = ColorSliderMotion::default().spring;
    assert_eq!(motion.spring.stiffness, default.stiffness);
    assert_eq!(motion.spring.damping, default.damping);
    assert_eq!(motion.spring.mass, default.mass);
    assert_eq!(motion.spring.precision, default.precision);
}

#[test]
fn disabled_constructor_turns_motion_off() {
    assert!(!ColorSliderMotion::disabled().enabled);
}
