use super::*;

#[test]
fn sanitize_motion_falls_back_for_invalid_values() {
    let motion = sanitize_motion(PressableFeedbackMotion {
        enabled: true,
        spring: ui_motion::spring::SpringConfig {
            stiffness: f64::NAN,
            damping: -1.0,
            mass: 0.0,
            precision: f64::INFINITY,
        },
        pressed_scale: f64::NAN,
        highlight_opacity: f64::INFINITY,
        ripple: RippleMotion {
            enabled: true,
            duration_ms: 999_999,
        },
    });

    let default = PressableFeedbackMotion::default();
    assert_eq!(motion.spring.stiffness, default.spring.stiffness);
    assert_eq!(motion.spring.damping, default.spring.damping);
    assert_eq!(motion.spring.mass, default.spring.mass);
    assert_eq!(motion.spring.precision, default.spring.precision);
    assert_eq!(motion.pressed_scale, default.pressed_scale);
    assert_eq!(motion.highlight_opacity, default.highlight_opacity);
    assert_eq!(motion.ripple.duration_ms, 1600);
}

#[test]
fn disabled_constructor_turns_motion_off() {
    let motion = PressableFeedbackMotion::disabled();
    assert!(!motion.enabled);
    assert!(!motion.ripple.enabled);
}
