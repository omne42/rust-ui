use ui_theme::default_text_field_motion_tokens;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ColorAreaMotion {
    pub duration_ms: f64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColorAreaMotionSource {
    Default,
    Custom,
}

impl ColorAreaMotionSource {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Custom => "custom",
        }
    }

    pub const fn is_custom(self) -> bool {
        matches!(self, Self::Custom)
    }
}

impl Default for ColorAreaMotion {
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

pub fn sanitize_motion(motion: ColorAreaMotion) -> ColorAreaMotion {
    let default = ColorAreaMotion::default();

    ColorAreaMotion {
        duration_ms: sanitize_number(motion.duration_ms, default.duration_ms).clamp(1.0, 1000.0),
    }
}

pub fn resolve_source(motion: ColorAreaMotion) -> ColorAreaMotionSource {
    if sanitize_motion(motion) == ColorAreaMotion::default() {
        ColorAreaMotionSource::Default
    } else {
        ColorAreaMotionSource::Custom
    }
}

pub fn source_attr(motion: ColorAreaMotion) -> &'static str {
    resolve_source(motion).as_attr()
}

pub fn attach_motion(base_vars: Option<String>, motion: ColorAreaMotion) -> String {
    let mut style = base_vars.unwrap_or_default();

    if !style.trim().is_empty() && !style.trim_end().ends_with(';') {
        style.push(';');
    }

    if resolve_source(motion).is_custom() {
        let motion = sanitize_motion(motion);

        if !style.trim().is_empty() {
            style.push(' ');
        }

        style.push_str(
            format!("--ui-color-area-motion-duration: {}ms;", motion.duration_ms).as_str(),
        );
    }
    style
}

#[cfg(test)]
#[path = "../test/motion.rs"]
mod tests;
