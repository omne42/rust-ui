#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LegendMotion {
    pub duration_ms: f64,
}

impl Default for LegendMotion {
    fn default() -> Self {
        Self { duration_ms: 140.0 }
    }
}

fn sanitize_number(value: f64, fallback: f64) -> f64 {
    if value.is_finite() { value } else { fallback }
}

pub fn sanitize_motion(motion: LegendMotion) -> LegendMotion {
    let default = LegendMotion::default();

    LegendMotion {
        duration_ms: sanitize_number(motion.duration_ms, default.duration_ms).clamp(1.0, 800.0),
    }
}

pub fn attach_motion(motion: LegendMotion) -> String {
    let motion = sanitize_motion(motion);
    format!("--ui-legend-motion-duration: {}ms;", motion.duration_ms)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_motion_is_stable() {
        assert_eq!(LegendMotion::default(), LegendMotion { duration_ms: 140.0 });
    }

    #[test]
    fn sanitize_motion_clamps_values() {
        assert_eq!(
            sanitize_motion(LegendMotion {
                duration_ms: f64::NAN
            }),
            LegendMotion::default()
        );
        assert_eq!(
            sanitize_motion(LegendMotion { duration_ms: -10.0 }),
            LegendMotion { duration_ms: 1.0 }
        );
        assert_eq!(
            sanitize_motion(LegendMotion {
                duration_ms: 9999.0
            }),
            LegendMotion { duration_ms: 800.0 }
        );
    }

    #[test]
    fn attach_motion_outputs_css_variable() {
        assert_eq!(
            attach_motion(LegendMotion { duration_ms: 220.0 }),
            "--ui-legend-motion-duration: 220ms;"
        );
    }
}
