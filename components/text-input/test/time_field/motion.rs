use super::*;

#[test]
fn default_motion_reads_from_theme_tokens() {
    let motion = TimeFieldMotion::default();
    let tokens = default_time_field_motion_tokens();
    assert_eq!(
        motion.spring,
        ui_motion::spring::SpringConfig {
            stiffness: tokens.spring.stiffness,
            damping: tokens.spring.damping,
            mass: tokens.spring.mass,
            precision: tokens.spring.precision,
        }
    );
    assert_eq!(motion.hidden_scale, tokens.hidden_scale);
    assert_eq!(motion.hover_scale, tokens.hover_scale);
    assert_eq!(motion.tap_scale, tokens.tap_scale);
}

#[test]
fn sanitize_motion_falls_back_and_clamps() {
    let motion = sanitize_motion(TimeFieldMotion {
        spring: ui_motion::spring::SpringConfig {
            stiffness: f64::NAN,
            damping: -1.0,
            mass: 0.0,
            precision: f64::INFINITY,
        },
        hidden_scale: f64::NAN,
        hover_scale: 9.0,
        tap_scale: -2.0,
    });

    let default = TimeFieldMotion::default();
    assert_eq!(motion.spring.stiffness, default.spring.stiffness);
    assert_eq!(motion.spring.damping, default.spring.damping);
    assert_eq!(motion.spring.mass, default.spring.mass);
    assert_eq!(motion.spring.precision, default.spring.precision);
    assert_eq!(motion.hidden_scale, default.hidden_scale);
    assert_eq!(motion.hover_scale, 2.0);
    assert_eq!(motion.tap_scale, 0.5);
}
