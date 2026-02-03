fn digits_only(value: &str) -> impl Iterator<Item = char> + '_ {
    value.chars().filter(|c| c.is_ascii_digit())
}

pub fn normalize_otp_value(value: &str, length: usize) -> String {
    digits_only(value).take(length).collect()
}

pub fn apply_otp_input(
    current: &str,
    index: usize,
    raw: &str,
    length: usize,
) -> (String, Option<usize>) {
    if length == 0 {
        return (String::new(), None);
    }

    let current = normalize_otp_value(current, length);
    let inserted: String = digits_only(raw).collect();
    let inserted_len = inserted.len();

    if inserted.is_empty() {
        // Treat empty input as delete-at-index when possible.
        let mut chars: Vec<char> = current.chars().collect();
        if index < chars.len() {
            chars.remove(index);
        }
        return (chars.into_iter().take(length).collect(), None);
    }

    let mut chars: Vec<char> = current.chars().collect();
    let mut pos = index.min(chars.len());
    let start_pos = pos;

    for digit in inserted.chars() {
        if pos < chars.len() {
            chars[pos] = digit;
        } else {
            chars.push(digit);
        }
        pos += 1;
        if pos >= length {
            break;
        }
    }

    chars.truncate(length);
    let next_focus = (start_pos + inserted_len).min(length);
    let next_focus = (next_focus < length).then_some(next_focus);
    (chars.into_iter().collect(), next_focus)
}

pub fn apply_otp_backspace(current: &str, index: usize, length: usize) -> (String, usize) {
    if length == 0 {
        return (String::new(), 0);
    }

    let current = normalize_otp_value(current, length);
    let mut chars: Vec<char> = current.chars().collect();
    if chars.is_empty() {
        return (String::new(), index.min(length.saturating_sub(1)));
    }

    if index < chars.len() {
        chars.remove(index);
        let focus = index.min(length.saturating_sub(1));
        return (chars.into_iter().collect(), focus);
    }

    // If the focused cell is empty, delete the last digit.
    chars.pop();
    let focus = chars.len().min(length.saturating_sub(1));
    (chars.into_iter().collect(), focus)
}

#[cfg(test)]
mod tests {
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
    fn paste_fills_forward() {
        let (next, focus) = apply_otp_input("12", 1, "3456", 6);
        assert_eq!(next, "13456");
        assert_eq!(focus, Some(5));
    }

    #[test]
    fn empty_input_deletes_at_index() {
        let (next, focus) = apply_otp_input("1234", 1, "", 6);
        assert_eq!(next, "134");
        assert_eq!(focus, None);
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
}
