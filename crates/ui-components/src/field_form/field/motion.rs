#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FieldMotion {
    pub duration_ms: f64,
}

impl Default for FieldMotion {
    fn default() -> Self {
        Self { duration_ms: 160.0 }
    }
}

fn sanitize_number(value: f64, fallback: f64) -> f64 {
    if value.is_finite() { value } else { fallback }
}

pub fn sanitize_motion(motion: FieldMotion) -> FieldMotion {
    let default = FieldMotion::default();

    FieldMotion {
        duration_ms: sanitize_number(motion.duration_ms, default.duration_ms).clamp(1.0, 800.0),
    }
}

pub fn attach_motion(motion: FieldMotion) -> String {
    let motion = sanitize_motion(motion);
    format!("--ui-field-motion-duration: {}ms;", motion.duration_ms)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_motion_is_stable() {
        assert_eq!(FieldMotion::default(), FieldMotion { duration_ms: 160.0 });
    }

    #[test]
    fn sanitize_motion_clamps_values() {
        assert_eq!(
            sanitize_motion(FieldMotion {
                duration_ms: f64::NAN
            }),
            FieldMotion::default()
        );
        assert_eq!(
            sanitize_motion(FieldMotion { duration_ms: -20.0 }),
            FieldMotion { duration_ms: 1.0 }
        );
        assert_eq!(
            sanitize_motion(FieldMotion {
                duration_ms: 9999.0
            }),
            FieldMotion { duration_ms: 800.0 }
        );
    }

    #[test]
    fn attach_motion_outputs_css_variable() {
        assert_eq!(
            attach_motion(FieldMotion { duration_ms: 200.0 }),
            "--ui-field-motion-duration: 200ms;"
        );
    }
}
