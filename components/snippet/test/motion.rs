use super::*;

#[test]
fn default_motion_reads_theme_tokens() {
    let motion = SnippetMotion::default();
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
    assert_eq!(motion.copied_scale, tokens.hover_scale);
}

#[test]
fn sanitize_motion_falls_back_for_invalid_values() {
    let motion = sanitize_motion(SnippetMotion {
        enabled: true,
        spring: ui_motion::spring::SpringConfig {
            stiffness: f64::NAN,
            damping: -1.0,
            mass: 0.0,
            precision: f64::INFINITY,
        },
        copied_scale: f64::NAN,
    });

    let default = SnippetMotion::default();
    assert_eq!(motion.spring.stiffness, default.spring.stiffness);
    assert_eq!(motion.spring.damping, default.spring.damping);
    assert_eq!(motion.spring.mass, default.spring.mass);
    assert_eq!(motion.spring.precision, default.spring.precision);
    assert_eq!(motion.copied_scale, default.copied_scale);
}

#[test]
fn disabled_constructor_turns_motion_off() {
    assert!(!SnippetMotion::disabled().enabled);
}
