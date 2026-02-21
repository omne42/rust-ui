use super::*;

#[test]
fn sanitize_motion_preserves_default_contract() {
    let motion = sanitize_motion(ColorSwatchMotion::default());
    assert_eq!(motion, ColorSwatchMotion::default());
}

#[test]
fn sanitize_motion_falls_back_for_invalid_values() {
    let motion = sanitize_motion(ColorSwatchMotion {
        enabled: true,
        spring: ui_motion::spring::SpringConfig {
            stiffness: f64::NAN,
            damping: -1.0,
            mass: 0.0,
            precision: f64::INFINITY,
        },
        initial_y_px: f64::NAN,
        initial_opacity: f64::INFINITY,
    });

    let default = ColorSwatchMotion::default();
    assert_eq!(motion.spring.stiffness, default.spring.stiffness);
    assert_eq!(motion.spring.damping, default.spring.damping);
    assert_eq!(motion.spring.mass, default.spring.mass);
    assert_eq!(motion.spring.precision, default.spring.precision);
    assert_eq!(motion.initial_y_px, default.initial_y_px);
    assert_eq!(motion.initial_opacity, default.initial_opacity);
}

#[test]
fn disabled_constructor_turns_motion_off() {
    assert!(!ColorSwatchMotion::disabled().enabled);
}
