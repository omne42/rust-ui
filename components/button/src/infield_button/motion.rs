#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct InfieldButtonMotion {
    pub transition_duration_ms: u16,
}

impl Default for InfieldButtonMotion {
    fn default() -> Self {
        Self {
            transition_duration_ms: 120,
        }
    }
}

pub fn sanitize_motion(motion: InfieldButtonMotion) -> InfieldButtonMotion {
    InfieldButtonMotion {
        transition_duration_ms: motion.transition_duration_ms.clamp(40, 1200),
    }
}

pub fn source_attr(motion: InfieldButtonMotion) -> &'static str {
    if sanitize_motion(motion) == InfieldButtonMotion::default() {
        "default"
    } else {
        "custom"
    }
}

pub fn attach_motion(base_vars: Option<String>, motion: InfieldButtonMotion) -> String {
    let motion = sanitize_motion(motion);
    let mut style = base_vars.unwrap_or_default();

    if !style.trim().is_empty() && !style.trim_end().ends_with(';') {
        style.push(';');
    }

    style.push_str(&format!(
        " --ui-infield-button-motion-duration: {}ms;",
        motion.transition_duration_ms
    ));

    style
}

#[cfg(test)]
#[path = "../../test/infield_button/motion.rs"]
mod tests;
