#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ColorAreaMotion {
    pub duration_ms: f64,
}

impl Default for ColorAreaMotion {
    fn default() -> Self {
        Self { duration_ms: 180.0 }
    }
}

fn sanitize_number(value: f64, fallback: f64) -> f64 {
    if value.is_finite() { value } else { fallback }
}

pub fn sanitize_motion(motion: ColorAreaMotion) -> ColorAreaMotion {
    let default = ColorAreaMotion::default();

    ColorAreaMotion {
        duration_ms: sanitize_number(motion.duration_ms, default.duration_ms).clamp(1.0, 1000.0),
    }
}

pub fn attach_motion(motion: ColorAreaMotion) -> String {
    let motion = sanitize_motion(motion);
    format!("--ui-color-area-motion-duration: {}ms;", motion.duration_ms)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_motion_is_stable() {
        assert_eq!(
            ColorAreaMotion::default(),
            ColorAreaMotion { duration_ms: 180.0 }
        );
    }

    #[test]
    fn sanitize_motion_clamps_values() {
        assert_eq!(
            sanitize_motion(ColorAreaMotion {
                duration_ms: f64::NAN
            }),
            ColorAreaMotion::default()
        );
        assert_eq!(
            sanitize_motion(ColorAreaMotion { duration_ms: 0.0 }),
            ColorAreaMotion { duration_ms: 1.0 }
        );
        assert_eq!(
            sanitize_motion(ColorAreaMotion {
                duration_ms: 9999.0
            }),
            ColorAreaMotion {
                duration_ms: 1000.0
            }
        );
    }

    #[test]
    fn attach_motion_outputs_css_variable() {
        assert_eq!(
            attach_motion(ColorAreaMotion { duration_ms: 220.0 }),
            "--ui-color-area-motion-duration: 220ms;"
        );
    }
}
