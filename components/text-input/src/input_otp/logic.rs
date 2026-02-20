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
#[path = "../../test/input_otp/logic.rs"]
mod tests;
