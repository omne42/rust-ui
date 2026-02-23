use super::*;

#[test]
fn normalizes_digits_only() {
    assert_eq!(normalize_otp_value("a1b2c3", 6), "123");
}

#[test]
fn insert_overwrites_and_extends() {
    let (next, focus) = apply_otp_input("12", 1, "9", 6);
    assert_eq!(next, "19");
    assert_eq!(focus, Some(2));

    let (next, focus) = apply_otp_input("12", 9, "9", 6);
    assert_eq!(next, "129");
    assert_eq!(focus, Some(3));
}

#[test]
fn backspace_deletes_current_or_last() {
    let (next, focus) = apply_otp_backspace("1234", 1, 6);
    assert_eq!(next, "134");
    assert_eq!(focus, 1);

    let (next, focus) = apply_otp_backspace("1234", 10, 6);
    assert_eq!(next, "123");
    assert_eq!(focus, 3);
}
