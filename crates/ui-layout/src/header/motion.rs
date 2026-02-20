use ui_theme::default_text_field_motion_tokens;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HeaderMotion {
    pub duration_ms: f64,
}

impl Default for HeaderMotion {
    fn default() -> Self {
        let tokens = default_text_field_motion_tokens();
        Self {
            duration_ms: f64::from(tokens.duration_ms),
        }
    }
}

fn sanitize_number(value: f64, fallback: f64) -> f64 {
    if value.is_finite() { value } else { fallback }
}

pub fn sanitize_motion(motion: HeaderMotion) -> HeaderMotion {
    let default = HeaderMotion::default();

    HeaderMotion {
        duration_ms: sanitize_number(motion.duration_ms, default.duration_ms).clamp(1.0, 1000.0),
    }
}

pub fn source_attr(motion: HeaderMotion) -> &'static str {
    if sanitize_motion(motion) == HeaderMotion::default() {
        "default"
    } else {
        "custom"
    }
}

pub fn attach_motion(base_vars: Option<String>, motion: HeaderMotion) -> String {
    let motion = sanitize_motion(motion);
    let mut style = base_vars.unwrap_or_default();

    if !style.trim().is_empty() && !style.trim_end().ends_with(';') {
        style.push(';');
    }

    style.push_str(&format!(
        " --ui-header-motion-duration: {}ms;",
        motion.duration_ms
    ));
    style
}

#[cfg(test)]
#[path = "test/motion.rs"]
mod tests;
