use ui_theme::default_text_field_motion_tokens;

const MIN_TRANSITION_MS: u16 = 1;
const MAX_TRANSITION_MS: u16 = 1_200;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ErrorMessageMotion {
    pub transition_ms: u16,
}

impl Default for ErrorMessageMotion {
    fn default() -> Self {
        let tokens = default_text_field_motion_tokens();
        Self {
            transition_ms: tokens.duration_ms,
        }
    }
}

pub fn sanitize_motion(motion: ErrorMessageMotion) -> ErrorMessageMotion {
    let default = ErrorMessageMotion::default();

    ErrorMessageMotion {
        transition_ms: if motion.transition_ms == 0 {
            default.transition_ms
        } else {
            motion
                .transition_ms
                .clamp(MIN_TRANSITION_MS, MAX_TRANSITION_MS)
        },
    }
}

pub fn source_attr(motion: ErrorMessageMotion) -> &'static str {
    if sanitize_motion(motion) == ErrorMessageMotion::default() {
        "default"
    } else {
        "custom"
    }
}

pub fn resolve_effective_transition_ms(
    motion: ErrorMessageMotion,
    prefers_reduced_motion: bool,
) -> u16 {
    let motion = sanitize_motion(motion);
    if prefers_reduced_motion {
        MIN_TRANSITION_MS
    } else {
        motion.transition_ms
    }
}

pub fn attach_motion(base_vars: Option<String>, motion: ErrorMessageMotion) -> String {
    let mut style = base_vars.unwrap_or_default();
    if !style.trim().is_empty() && !style.trim_end().ends_with(';') {
        style.push(';');
    }

    let effective_transition_ms =
        resolve_effective_transition_ms(motion, ui_motion::web::prefers_reduced_motion());
    style.push_str(&format!(
        " --ui-error-message-transition-ms:{effective_transition_ms}ms;"
    ));

    style
}

#[cfg(test)]
#[path = "../test/motion.rs"]
mod tests;
