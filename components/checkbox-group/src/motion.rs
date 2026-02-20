#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CheckboxGroupMotion {
    pub transition_duration_ms: u16,
}

impl Default for CheckboxGroupMotion {
    fn default() -> Self {
        Self {
            transition_duration_ms: 140,
        }
    }
}

pub fn sanitize_checkbox_group_motion(motion: CheckboxGroupMotion) -> CheckboxGroupMotion {
    CheckboxGroupMotion {
        transition_duration_ms: motion.transition_duration_ms.clamp(60, 1200),
    }
}

pub fn checkbox_group_motion_source_attr(motion: CheckboxGroupMotion) -> &'static str {
    if sanitize_checkbox_group_motion(motion) == CheckboxGroupMotion::default() {
        "default"
    } else {
        "custom"
    }
}

pub fn attach_checkbox_group_motion(
    base_vars: Option<String>,
    motion: CheckboxGroupMotion,
) -> String {
    let motion = sanitize_checkbox_group_motion(motion);
    let mut style = base_vars.unwrap_or_default();

    if !style.trim().is_empty() && !style.trim_end().ends_with(';') {
        style.push(';');
    }

    style.push_str(&format!(
        " --ui-checkbox-group-motion-duration: {}ms;",
        motion.transition_duration_ms
    ));

    style
}

#[cfg(test)]
#[path = "../test/motion.rs"]
mod tests;
