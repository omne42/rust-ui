use super::*;

#[test]
fn default_motion_has_reasonable_params() {
    let motion = SwitchMotion::default();
    let tokens = default_switch_motion_tokens();
    assert_eq!(
        motion.spring,
        ui_motion::spring::SpringConfig {
            stiffness: tokens.spring.stiffness,
            damping: tokens.spring.damping,
            mass: tokens.spring.mass,
            precision: tokens.spring.precision,
        }
    );
}

#[test]
fn sanitize_motion_falls_back_for_invalid_values() {
    let motion = sanitize_motion(SwitchMotion {
        spring: ui_motion::spring::SpringConfig {
            stiffness: f64::NAN,
            damping: -1.0,
            mass: 0.0,
            precision: f64::INFINITY,
        },
    });

    let default = SwitchMotion::default();
    assert_eq!(motion.spring.stiffness, default.spring.stiffness);
    assert_eq!(motion.spring.damping, default.spring.damping);
    assert_eq!(motion.spring.mass, default.spring.mass);
    assert_eq!(motion.spring.precision, default.spring.precision);
}

#[test]
fn sanitize_pressed_width_clamps_and_uses_fallback() {
    let tokens = default_switch_motion_tokens();
    assert_eq!(sanitize_pressed_width_px(24.0), 24.0);
    assert_eq!(sanitize_pressed_width_px(4.0), tokens.pressed_width_min_px);
    assert_eq!(
        sanitize_pressed_width_px(500.0),
        tokens.pressed_width_max_px
    );
    assert_eq!(
        sanitize_pressed_width_px(f64::NAN),
        tokens.pressed_width_default_px
    );
}
