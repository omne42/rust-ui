use super::*;

#[test]
fn clamps_min_and_max() {
    assert_eq!(clamp_i64(5, Some(10), None), 10);
    assert_eq!(clamp_i64(5, None, Some(3)), 3);
    assert_eq!(clamp_i64(5, Some(0), Some(10)), 5);
}

#[test]
fn steps_with_limits() {
    assert_eq!(step_i64(0, 1, 1, Some(0), Some(2)), 1);
    assert_eq!(step_i64(2, 1, 1, Some(0), Some(2)), 2);
    assert_eq!(step_i64(2, -1, 1, Some(0), Some(2)), 1);
}

#[test]
fn parses_trimmed_numbers() {
    assert_eq!(parse_i64(" 42 "), Some(42));
    assert_eq!(parse_i64(""), None);
    assert_eq!(parse_i64("nope"), None);
}
