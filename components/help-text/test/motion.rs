use super::*;
use ui_theme::default_text_field_motion_tokens;

#[test]
fn sanitize_duration_is_bounded() {
    let tokens = default_text_field_motion_tokens();
    assert_eq!(sanitize_duration_ms(0), u32::from(tokens.duration_ms));
    assert_eq!(sanitize_duration_ms(160), 160);
    assert_eq!(sanitize_duration_ms(1_400), 1_200);
}

#[test]
fn disabled_constructor_turns_motion_off() {
    assert!(!HelpTextMotion::disabled().enabled);
}

#[test]
fn default_motion_uses_theme_duration_token() {
    let tokens = default_text_field_motion_tokens();
    assert_eq!(
        HelpTextMotion::default().duration_ms,
        u32::from(tokens.duration_ms)
    );
}

#[test]
fn resolved_motion_options_use_theme_easing_and_fill() {
    let tokens = default_text_field_motion_tokens();
    let options = resolve_motion_options(HelpTextMotion::default());

    assert_eq!(options.easing, tokens.easing);
    assert_eq!(options.duration_ms, u32::from(tokens.duration_ms));
    assert!(matches!(options.fill, ui_motion::options::FillMode::Both));
}
