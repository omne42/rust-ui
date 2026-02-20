#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ActionButtonGroupMotion {
    pub duration_ms: f64,
}

impl Default for ActionButtonGroupMotion {
    fn default() -> Self {
        Self { duration_ms: 160.0 }
    }
}

fn sanitize_number(value: f64, fallback: f64) -> f64 {
    if value.is_finite() { value } else { fallback }
}

pub fn sanitize_motion(motion: ActionButtonGroupMotion) -> ActionButtonGroupMotion {
    let default = ActionButtonGroupMotion::default();

    ActionButtonGroupMotion {
        duration_ms: sanitize_number(motion.duration_ms, default.duration_ms).clamp(1.0, 800.0),
    }
}

pub fn attach_motion(motion: ActionButtonGroupMotion) -> String {
    let motion = sanitize_motion(motion);
    format!(
        "--ui-action-button-group-motion-duration: {}ms;",
        motion.duration_ms
    )
}

#[cfg(test)]
#[path = "../../test/action/motion.rs"]
mod tests;
