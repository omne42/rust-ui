use super::*;

#[test]
fn sanitize_motion_falls_back_for_invalid_values() {
    let motion = sanitize_motion(ErrorViewMotion {
        enabled: true,
        spring: ui_motion::spring::SpringConfig {
            stiffness: f64::NAN,
            damping: -1.0,
            mass: 0.0,
            precision: f64::INFINITY,
        },
        hidden_translate_px: f64::NAN,
        hidden_opacity: f64::INFINITY,
        hidden_scale: f64::NAN,
    });

    let default = ErrorViewMotion::default();
    assert_eq!(motion.spring.stiffness, default.spring.stiffness);
    assert_eq!(motion.spring.damping, default.spring.damping);
    assert_eq!(motion.spring.mass, default.spring.mass);
    assert_eq!(motion.spring.precision, default.spring.precision);
    assert_eq!(motion.hidden_translate_px, default.hidden_translate_px);
    assert_eq!(motion.hidden_opacity, default.hidden_opacity);
    assert_eq!(motion.hidden_scale, default.hidden_scale);
}

#[test]
fn disabled_constructor_turns_motion_off() {
    assert!(!ErrorViewMotion::disabled().enabled);
}
