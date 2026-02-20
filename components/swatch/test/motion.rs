use super::*;

#[test]
fn sanitize_motion_falls_back_for_invalid_values() {
    let motion = sanitize_motion(SwatchMotion {
        enabled: true,
        spring: ui_motion::spring::SpringConfig {
            stiffness: f64::NAN,
            damping: -1.0,
            mass: 0.0,
            precision: f64::INFINITY,
        },
        selected_scale: f64::NAN,
        selected_ring_opacity: f64::INFINITY,
    });

    let default = SwatchMotion::default();
    assert_eq!(motion.spring.stiffness, default.spring.stiffness);
    assert_eq!(motion.spring.damping, default.spring.damping);
    assert_eq!(motion.spring.mass, default.spring.mass);
    assert_eq!(motion.spring.precision, default.spring.precision);
    assert_eq!(motion.selected_scale, default.selected_scale);
    assert_eq!(motion.selected_ring_opacity, 1.0);
}

#[test]
fn default_motion_reads_theme_tokens() {
    let motion = SwatchMotion::default();
    let tokens = default_swatch_motion_tokens();

    assert_eq!(
        motion.spring,
        ui_motion::spring::SpringConfig {
            stiffness: tokens.spring.stiffness,
            damping: tokens.spring.damping,
            mass: tokens.spring.mass,
            precision: tokens.spring.precision,
        }
    );
    assert_eq!(motion.selected_scale, tokens.selected_scale);
    assert_eq!(motion.selected_ring_opacity, tokens.selected_ring_opacity);
}

#[test]
fn disabled_constructor_turns_motion_off() {
    let motion = SwatchMotion::disabled();
    assert!(!motion.enabled);
}
