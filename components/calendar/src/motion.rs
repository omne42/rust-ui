use ui_theme::default_text_field_motion_tokens;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CalendarMotion {
    pub enabled: bool,
    pub duration_ms: f64,
}

impl Default for CalendarMotion {
    fn default() -> Self {
        let tokens = default_text_field_motion_tokens();
        Self {
            enabled: true,
            duration_ms: f64::from(tokens.duration_ms),
        }
    }
}

fn sanitize_number(value: f64, fallback: f64) -> f64 {
    if value.is_finite() { value } else { fallback }
}

pub fn sanitize_motion(motion: CalendarMotion) -> CalendarMotion {
    let default = CalendarMotion::default();

    CalendarMotion {
        enabled: motion.enabled,
        duration_ms: sanitize_number(motion.duration_ms, default.duration_ms).clamp(1.0, 1000.0),
    }
}

pub fn source_attr(motion: CalendarMotion) -> &'static str {
    if sanitize_motion(motion) == CalendarMotion::default() {
        "default"
    } else {
        "custom"
    }
}

pub fn attach_motion(base_vars: Option<String>, motion: CalendarMotion) -> String {
    let motion = sanitize_motion(motion);
    let mut style = base_vars.unwrap_or_default();

    if !style.trim().is_empty() && !style.trim_end().ends_with(';') {
        style.push(';');
    }

    let duration_ms = if motion.enabled {
        motion.duration_ms
    } else {
        1.0
    };
    style.push_str(&format!(
        " --ui-calendar-motion-duration: {}ms;",
        duration_ms
    ));

    style
}

#[cfg(test)]
#[path = "../test/motion.rs"]
mod tests;
