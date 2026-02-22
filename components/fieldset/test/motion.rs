use super::*;
use ui_theme::{Theme, ThemeContext, default_text_field_motion_tokens};

#[test]
fn sanitize_motion_clamps_invalid_values() {
    let motion = sanitize_motion(FieldsetMotion {
        duration_ms: f64::NAN,
        distance_px: -10.0,
        stiffness: f64::NAN,
        damping: -1.0,
    });

    let default = FieldsetMotion::default();
    assert_eq!(motion.duration_ms, default.duration_ms);
    assert_eq!(motion.distance_px, 0.0);
    assert_eq!(motion.stiffness, default.stiffness);
    assert_eq!(motion.damping, 0.1);
}

#[test]
fn default_motion_uses_theme_token_baseline() {
    let default = FieldsetMotion::default();
    let text_field_motion = default_text_field_motion_tokens();
    let theme = Theme::new(ThemeContext::default());

    assert_eq!(
        default.duration_ms,
        f64::from(text_field_motion.duration_ms)
    );
    assert_eq!(
        default.distance_px,
        f64::from(theme.tokens.layout.space.space_2xs_px)
    );
}

#[test]
fn resolve_effective_motion_honors_reduced_motion() {
    let default = FieldsetMotion::default();
    let custom_motion = FieldsetMotion {
        duration_ms: default.duration_ms + 40.0,
        distance_px: default.distance_px + 2.0,
        stiffness: default.stiffness + 20.0,
        damping: default.damping + 2.0,
    };
    let effective = resolve_effective_motion(custom_motion, true);

    assert_eq!(effective.duration_ms, 1.0);
    assert_eq!(effective.distance_px, 0.0);
    assert_eq!(effective.stiffness, custom_motion.stiffness);
    assert_eq!(effective.damping, custom_motion.damping);
}

#[test]
fn attach_motion_serializes_css_vars() {
    let default = FieldsetMotion::default();
    let style = attach_motion(FieldsetMotion {
        duration_ms: default.duration_ms + 40.0,
        distance_px: default.distance_px + 2.0,
        stiffness: default.stiffness + 40.0,
        damping: default.damping + 4.0,
    });

    assert!(style.contains("--ui-fieldset-motion-duration"));
    assert!(style.contains("--ui-fieldset-motion-distance"));
    assert!(style.contains("--ui-fieldset-motion-stiffness"));
    assert!(style.contains("--ui-fieldset-motion-damping"));
}
