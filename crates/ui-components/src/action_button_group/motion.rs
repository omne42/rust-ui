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
mod tests {
    use super::*;

    #[test]
    fn default_motion_is_stable() {
        assert_eq!(
            ActionButtonGroupMotion::default(),
            ActionButtonGroupMotion { duration_ms: 160.0 }
        );
    }

    #[test]
    fn sanitize_motion_clamps_and_falls_back() {
        assert_eq!(
            sanitize_motion(ActionButtonGroupMotion {
                duration_ms: f64::NAN
            }),
            ActionButtonGroupMotion::default()
        );
        assert_eq!(
            sanitize_motion(ActionButtonGroupMotion { duration_ms: -10.0 }),
            ActionButtonGroupMotion { duration_ms: 1.0 }
        );
        assert_eq!(
            sanitize_motion(ActionButtonGroupMotion {
                duration_ms: 9999.0
            }),
            ActionButtonGroupMotion { duration_ms: 800.0 }
        );
    }

    #[test]
    fn attach_motion_only_outputs_css_variable() {
        assert_eq!(
            attach_motion(ActionButtonGroupMotion { duration_ms: 240.0 }),
            "--ui-action-button-group-motion-duration: 240ms;"
        );
    }
}
