use super::*;

#[test]
fn default_motion_comes_from_theme_tokens() {
    let motion = TextFieldMotion::default();
    let tokens = default_text_field_motion_tokens();
    assert_eq!(motion.duration_ms, u32::from(tokens.duration_ms));
    assert!(motion.enabled);
}

#[test]
fn sanitize_duration_is_bounded() {
    assert_eq!(sanitize_duration_ms(0), 80);
    assert_eq!(sanitize_duration_ms(180), 180);
    assert_eq!(sanitize_duration_ms(9_999), 1_000);
}

#[test]
fn motion_style_vars_exposes_css_variables() {
    let style = motion_style_vars(TextFieldMotion {
        enabled: true,
        duration_ms: 220,
    });

    assert!(style.contains("--ui-text-field-motion-duration: 220ms;"));
    assert!(style.contains("--ui-text-field-motion-easing: cubic-bezier(0.2, 0, 0, 1);"));
}

#[test]
fn disabled_motion_uses_zero_duration_css_var() {
    let style = motion_style_vars(TextFieldMotion {
        enabled: false,
        duration_ms: 220,
    });

    assert!(style.contains("--ui-text-field-motion-duration: 0ms;"));
}
