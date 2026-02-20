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
#[path = "../../test/number_field/logic.rs"]
mod tests;
