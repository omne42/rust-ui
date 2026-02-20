use super::*;

#[test]
fn sanitize_duration_is_bounded() {
    assert_eq!(sanitize_duration_ms(0), 120);
    assert_eq!(sanitize_duration_ms(180), 180);
    assert_eq!(sanitize_duration_ms(2_000), 900);
}

#[test]
fn disabled_constructor_turns_motion_off() {
    assert!(!LabeledValueMotion::disabled().enabled);
}
