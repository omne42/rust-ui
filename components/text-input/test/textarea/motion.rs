use super::*;

#[test]
fn default_motion_comes_from_theme_tokens() {
    let motion = TextareaMotion::default();
    let tokens = default_textarea_motion_tokens();
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
    let style = motion_style_vars(TextareaMotion {
        enabled: true,
        duration_ms: 220,
    });

    assert!(style.contains("--ui-textarea-motion-duration: 220ms;"));
    assert!(style.contains("--ui-textarea-motion-easing:"));
}
