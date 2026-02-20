use ui_theme::default_button_layout_tokens;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SpinnerMotion {
    pub rotation_duration_ms: u16,
}

impl Default for SpinnerMotion {
    fn default() -> Self {
        let tokens = default_button_layout_tokens();
        Self {
            rotation_duration_ms: tokens.spinner_duration_ms,
        }
    }
}

pub fn sanitize_motion(motion: SpinnerMotion) -> SpinnerMotion {
    SpinnerMotion {
        rotation_duration_ms: motion.rotation_duration_ms.clamp(240, 4000),
    }
}

pub fn source_attr(motion: SpinnerMotion) -> &'static str {
    if sanitize_motion(motion) == SpinnerMotion::default() {
        "default"
    } else {
        "custom"
    }
}

pub fn attach_motion(base_vars: Option<String>, motion: SpinnerMotion) -> String {
    let motion = sanitize_motion(motion);
    let mut style = base_vars.unwrap_or_default();

    if !style.trim().is_empty() && !style.trim_end().ends_with(';') {
        style.push(';');
    }

    style.push_str(&format!(
        " --ui-spinner-rotation-duration: {}ms;",
        motion.rotation_duration_ms
    ));

    style
}

#[cfg(test)]
#[path = "../test/motion.rs"]
mod tests;
