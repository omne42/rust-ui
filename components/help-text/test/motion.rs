use super::*;

#[test]
fn sanitize_duration_is_bounded() {
    assert_eq!(sanitize_duration_ms(0), 100);
    assert_eq!(sanitize_duration_ms(160), 160);
    assert_eq!(sanitize_duration_ms(1_200), 800);
}

#[test]
fn disabled_constructor_turns_motion_off() {
    assert!(!HelpTextMotion::disabled().enabled);
}
