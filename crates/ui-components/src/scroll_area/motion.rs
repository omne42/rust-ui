#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ScrollAreaMotion {
    pub duration_ms: f64,
}

impl Default for ScrollAreaMotion {
    fn default() -> Self {
        Self { duration_ms: 160.0 }
    }
}

fn sanitize_number(value: f64, fallback: f64) -> f64 {
    if value.is_finite() { value } else { fallback }
}

pub fn sanitize_motion(motion: ScrollAreaMotion) -> ScrollAreaMotion {
    let default = ScrollAreaMotion::default();

    ScrollAreaMotion {
        duration_ms: sanitize_number(motion.duration_ms, default.duration_ms).clamp(1.0, 1000.0),
    }
}

pub fn attach_motion(motion: ScrollAreaMotion) -> String {
    let motion = sanitize_motion(motion);
    format!(
        "--ui-scroll-area-motion-duration: {}ms;",
        motion.duration_ms
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_motion_is_stable() {
        assert_eq!(
            ScrollAreaMotion::default(),
            ScrollAreaMotion { duration_ms: 160.0 }
        );
    }

    #[test]
    fn sanitize_motion_clamps_values() {
        assert_eq!(
            sanitize_motion(ScrollAreaMotion {
                duration_ms: f64::NAN
            }),
            ScrollAreaMotion::default()
        );
        assert_eq!(
            sanitize_motion(ScrollAreaMotion { duration_ms: 0.0 }),
            ScrollAreaMotion { duration_ms: 1.0 }
        );
        assert_eq!(
            sanitize_motion(ScrollAreaMotion {
                duration_ms: 5000.0
            }),
            ScrollAreaMotion {
                duration_ms: 1000.0
            }
        );
    }

    #[test]
    fn attach_motion_outputs_css_variable() {
        assert_eq!(
            attach_motion(ScrollAreaMotion { duration_ms: 240.0 }),
            "--ui-scroll-area-motion-duration: 240ms;"
        );
    }
}
