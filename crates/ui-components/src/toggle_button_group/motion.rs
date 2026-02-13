#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ToggleButtonGroupMotion {
    pub duration_ms: f64,
}

impl Default for ToggleButtonGroupMotion {
    fn default() -> Self {
        Self { duration_ms: 160.0 }
    }
}

fn sanitize_number(value: f64, fallback: f64) -> f64 {
    if value.is_finite() { value } else { fallback }
}

pub fn sanitize_motion(motion: ToggleButtonGroupMotion) -> ToggleButtonGroupMotion {
    let default = ToggleButtonGroupMotion::default();

    ToggleButtonGroupMotion {
        duration_ms: sanitize_number(motion.duration_ms, default.duration_ms).clamp(1.0, 1000.0),
    }
}

pub fn attach_motion(motion: ToggleButtonGroupMotion) -> String {
    let motion = sanitize_motion(motion);
    format!(
        "--ui-toggle-button-group-motion-duration: {}ms;",
        motion.duration_ms
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_motion_is_stable() {
        assert_eq!(
            ToggleButtonGroupMotion::default(),
            ToggleButtonGroupMotion { duration_ms: 160.0 }
        );
    }

    #[test]
    fn sanitize_motion_clamps_values() {
        assert_eq!(
            sanitize_motion(ToggleButtonGroupMotion {
                duration_ms: f64::NAN
            }),
            ToggleButtonGroupMotion::default()
        );
        assert_eq!(
            sanitize_motion(ToggleButtonGroupMotion { duration_ms: 0.0 }),
            ToggleButtonGroupMotion { duration_ms: 1.0 }
        );
        assert_eq!(
            sanitize_motion(ToggleButtonGroupMotion {
                duration_ms: 5000.0
            }),
            ToggleButtonGroupMotion {
                duration_ms: 1000.0
            }
        );
    }

    #[test]
    fn attach_motion_outputs_css_variable() {
        assert_eq!(
            attach_motion(ToggleButtonGroupMotion { duration_ms: 260.0 }),
            "--ui-toggle-button-group-motion-duration: 260ms;"
        );
    }
}
