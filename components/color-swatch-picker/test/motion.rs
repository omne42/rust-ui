use super::*;
use ui_theme::{default_swatch_motion_tokens, default_text_field_motion_tokens};

#[test]
fn default_motion_matches_contract() {
    let motion = ColorSwatchPickerMotion::default();
    let transition_tokens = default_text_field_motion_tokens();
    let swatch_tokens = default_swatch_motion_tokens();

    assert!(motion.enabled);
    assert_eq!(motion.transition_ms, transition_tokens.duration_ms);
    assert_eq!(motion.focus_ring_width_px, 5);
    assert_eq!(motion.spring.stiffness, swatch_tokens.spring.stiffness);
    assert_eq!(motion.spring.damping, swatch_tokens.spring.damping);
    assert_eq!(motion.spring.mass, swatch_tokens.spring.mass);
    assert_eq!(motion.spring.precision, swatch_tokens.spring.precision);
}

#[test]
fn sanitize_motion_clamps_values() {
    let motion = sanitize_motion(ColorSwatchPickerMotion {
        enabled: true,
        transition_ms: 0,
        focus_ring_width_px: 0,
        spring: ui_motion::spring::SpringConfig {
            stiffness: 0.0,
            damping: f64::NAN,
            mass: -1.0,
            precision: 0.0,
        },
    });

    assert_eq!(
        motion.transition_ms,
        default_text_field_motion_tokens().duration_ms
    );
    assert_eq!(motion.focus_ring_width_px, 2);
    let default = ColorSwatchPickerMotion::default();
    assert_eq!(motion.spring, default.spring);

    let motion = sanitize_motion(ColorSwatchPickerMotion {
        enabled: true,
        transition_ms: 5000,
        focus_ring_width_px: 18,
        spring: ui_motion::spring::SpringConfig {
            stiffness: 450.0,
            damping: 31.0,
            mass: 1.4,
            precision: 0.002,
        },
    });

    assert_eq!(motion.transition_ms, 1200);
    assert_eq!(motion.focus_ring_width_px, 12);
    assert_eq!(motion.spring.stiffness, 450.0);
    assert_eq!(motion.spring.damping, 31.0);
    assert_eq!(motion.spring.mass, 1.4);
    assert_eq!(motion.spring.precision, 0.002);
}

#[test]
fn compose_style_vars_exposes_css_variables() {
    let vars = compose_style_vars(ColorSwatchPickerMotion {
        enabled: true,
        transition_ms: 220,
        focus_ring_width_px: 7,
        spring: ui_motion::spring::SpringConfig {
            stiffness: 333.0,
            damping: 29.0,
            mass: 1.2,
            precision: 0.003,
        },
    });

    assert!(vars.contains("--ui-color-swatch-picker-transition-ms:220ms"));
    assert!(vars.contains("--ui-color-swatch-picker-focus-ring-width:7px"));
    assert!(vars.contains("--ui-color-swatch-picker-spring-stiffness:333"));
    assert!(vars.contains("--ui-color-swatch-picker-spring-damping:29"));
    assert!(vars.contains("--ui-color-swatch-picker-spring-mass:1.2"));
    assert!(vars.contains("--ui-color-swatch-picker-spring-precision:0.003"));
}

#[test]
fn reduced_motion_collapses_transition_duration() {
    let effective = resolve_effective_motion(ColorSwatchPickerMotion::default(), true);
    assert_eq!(effective.transition_ms, 1);
}

#[test]
fn disabled_motion_collapses_transition_duration() {
    let effective = resolve_effective_motion(
        ColorSwatchPickerMotion {
            enabled: false,
            ..ColorSwatchPickerMotion::default()
        },
        false,
    );
    assert_eq!(effective.transition_ms, 1);
}
