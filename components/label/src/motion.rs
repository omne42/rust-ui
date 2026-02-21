use ui_theme::default_label_motion_tokens;

const MIN_DURATION_MS: u16 = 1;
const MAX_DURATION_MS: u16 = 2_000;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LabelMotion {
    pub color_transition_ms: u16,
    pub weight_transition_ms: u16,
}

impl Default for LabelMotion {
    fn default() -> Self {
        let tokens = default_label_motion_tokens();
        Self {
            color_transition_ms: tokens.color_duration_ms,
            weight_transition_ms: tokens.weight_duration_ms,
        }
    }
}

pub fn sanitize_motion(motion: LabelMotion) -> LabelMotion {
    LabelMotion {
        color_transition_ms: motion
            .color_transition_ms
            .clamp(MIN_DURATION_MS, MAX_DURATION_MS),
        weight_transition_ms: motion
            .weight_transition_ms
            .clamp(MIN_DURATION_MS, MAX_DURATION_MS),
    }
}

pub fn motion_source_attr(motion: LabelMotion) -> &'static str {
    if sanitize_motion(motion) == LabelMotion::default() {
        "default"
    } else {
        "custom"
    }
}

pub fn attach_motion(base_vars: Option<String>, motion: LabelMotion) -> String {
    let motion = sanitize_motion(motion);
    let reduced_motion = ui_motion::web::prefers_reduced_motion();
    let color_transition_ms = if reduced_motion {
        MIN_DURATION_MS
    } else {
        motion.color_transition_ms
    };
    let weight_transition_ms = if reduced_motion {
        MIN_DURATION_MS
    } else {
        motion.weight_transition_ms
    };

    let mut style = base_vars.unwrap_or_default();

    if !style.trim().is_empty() && !style.trim_end().ends_with(';') {
        style.push(';');
    }

    style.push_str(&format!(
        " --ui-label-motion-color-duration: {color_transition_ms}ms; --ui-label-motion-weight-duration: {weight_transition_ms}ms;",
    ));

    style
}

#[cfg(test)]
#[path = "../test/motion.rs"]
mod tests;
