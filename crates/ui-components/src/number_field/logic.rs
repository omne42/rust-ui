pub fn clamp_i64(value: i64, min: Option<i64>, max: Option<i64>) -> i64 {
    let mut v = value;
    if let Some(min) = min {
        v = v.max(min);
    }
    if let Some(max) = max {
        v = v.min(max);
    }
    v
}

pub fn step_i64(
    value: i64,
    delta_steps: i64,
    step: i64,
    min: Option<i64>,
    max: Option<i64>,
) -> i64 {
    let step = step.max(1);
    let delta = delta_steps.saturating_mul(step);
    clamp_i64(value.saturating_add(delta), min, max)
}

pub fn parse_i64(text: &str) -> Option<i64> {
    let trimmed = text.trim();
    if trimmed.is_empty() {
        return None;
    }
    trimmed.parse::<i64>().ok()
}

#[cfg(test)]
mod tests {
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
}
