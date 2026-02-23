use super::*;

#[test]
fn default_motion_comes_from_theme_tokens() {
    let motion = DateFieldMotion::default();
    let tokens = default_text_field_motion_tokens();
    assert_eq!(motion.duration_ms, tokens.duration_ms);
    assert!(motion.enabled);
}

#[test]
fn sanitize_motion_clamps_duration() {
    assert_eq!(sanitize_duration_ms(0), 120);
    assert_eq!(sanitize_duration_ms(180), 180);
    assert_eq!(sanitize_duration_ms(2_000), 1_000);
}

#[test]
fn disabled_constructor_turns_motion_off() {
    assert!(!DateFieldMotion::disabled().enabled);
}
