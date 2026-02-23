use ui_theme::default_text_field_motion_tokens;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PaginationMotion {
    pub enabled: bool,
    pub duration_ms: u16,
    pub easing: &'static str,
}

impl Default for PaginationMotion {
    fn default() -> Self {
        let tokens = default_text_field_motion_tokens();
        Self {
            enabled: true,
            duration_ms: tokens.duration_ms,
            easing: tokens.easing,
        }
    }
}

impl PaginationMotion {
    pub fn disabled() -> Self {
        Self {
            enabled: false,
            ..Self::default()
        }
    }
}

pub fn sanitize_motion(motion: PaginationMotion) -> PaginationMotion {
    let default = PaginationMotion::default();
    PaginationMotion {
        enabled: motion.enabled,
        duration_ms: motion.duration_ms.clamp(0, 2000),
        easing: if motion.easing.trim().is_empty() {
            default.easing
        } else {
            motion.easing
        },
    }
}

pub fn source_attr(motion: PaginationMotion) -> &'static str {
    if sanitize_motion(motion) == PaginationMotion::default() {
        "default"
    } else {
        "custom"
    }
}

pub fn attach_motion(base_vars: Option<String>, motion: PaginationMotion) -> String {
    let motion = sanitize_motion(motion);
    let mut style = base_vars.unwrap_or_default();

    if !style.trim().is_empty() && !style.trim_end().ends_with(';') {
        style.push(';');
    }

    let effective_duration_ms = if !motion.enabled || ui_motion::web::prefers_reduced_motion() {
        0
    } else {
        motion.duration_ms
    };

    style.push_str(&format!(
        " --ui-pagination-motion-duration: {effective_duration_ms}ms;"
    ));
    style.push_str(&format!(
        " --ui-pagination-motion-easing: {};",
        motion.easing
    ));

    style
}

#[cfg(test)]
#[path = "../test/motion.rs"]
mod tests;
