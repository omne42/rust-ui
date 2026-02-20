#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ColorAreaMotion {
    pub duration_ms: f64,
}

impl Default for ColorAreaMotion {
    fn default() -> Self {
        Self { duration_ms: 180.0 }
    }
}

fn sanitize_number(value: f64, fallback: f64) -> f64 {
    if value.is_finite() { value } else { fallback }
}

pub fn sanitize_motion(motion: ColorAreaMotion) -> ColorAreaMotion {
    let default = ColorAreaMotion::default();

    ColorAreaMotion {
        duration_ms: sanitize_number(motion.duration_ms, default.duration_ms).clamp(1.0, 1000.0),
    }
}

pub fn source_attr(motion: ColorAreaMotion) -> &'static str {
    if sanitize_motion(motion) == ColorAreaMotion::default() {
        "default"
    } else {
        "custom"
    }
}

pub fn attach_motion(base_vars: Option<String>, motion: ColorAreaMotion) -> String {
    let motion = sanitize_motion(motion);
    let mut style = base_vars.unwrap_or_default();

    if !style.trim().is_empty() && !style.trim_end().ends_with(';') {
        style.push(';');
    }

    if !style.trim().is_empty() {
        style.push(' ');
    }

    style.push_str(format!("--ui-color-area-motion-duration: {}ms;", motion.duration_ms).as_str());
    style
}

#[cfg(test)]
#[path = "../test/motion.rs"]
mod tests;
