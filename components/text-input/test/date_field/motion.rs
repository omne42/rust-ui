use super::*;

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
