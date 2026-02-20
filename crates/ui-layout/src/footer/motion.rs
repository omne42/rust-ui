#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FooterMotion {
    pub transition_duration_ms: u16,
}

impl Default for FooterMotion {
    fn default() -> Self {
        Self {
            transition_duration_ms: 160,
        }
    }
}

pub fn sanitize_motion(motion: FooterMotion) -> FooterMotion {
    FooterMotion {
        transition_duration_ms: motion.transition_duration_ms.clamp(60, 1200),
    }
}

pub fn source_attr(motion: FooterMotion) -> &'static str {
    if sanitize_motion(motion) == FooterMotion::default() {
        "default"
    } else {
        "custom"
    }
}

pub fn attach_motion(base_vars: Option<String>, motion: FooterMotion) -> String {
    let motion = sanitize_motion(motion);
    let mut style = base_vars.unwrap_or_default();

    if !style.trim().is_empty() && !style.trim_end().ends_with(';') {
        style.push(';');
    }

    style.push_str(&format!(
        " --ui-footer-motion-duration: {}ms;",
        motion.transition_duration_ms
    ));

    style
}

#[cfg(test)]
#[path = "test/motion.rs"]
mod tests;
