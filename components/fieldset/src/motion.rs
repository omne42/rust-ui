#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FieldsetMotion {
    pub duration_ms: f64,
    pub distance_px: f64,
}

impl Default for FieldsetMotion {
    fn default() -> Self {
        Self {
            duration_ms: 170.0,
            distance_px: 4.0,
        }
    }
}

pub fn sanitize_motion(motion: FieldsetMotion) -> FieldsetMotion {
    let default = FieldsetMotion::default();

    FieldsetMotion {
        duration_ms: if motion.duration_ms.is_finite() {
            motion.duration_ms.clamp(0.0, 1200.0)
        } else {
            default.duration_ms
        },
        distance_px: if motion.distance_px.is_finite() {
            motion.distance_px.clamp(0.0, 32.0)
        } else {
            default.distance_px
        },
    }
}

pub fn attach_motion(motion: FieldsetMotion) -> String {
    let motion = sanitize_motion(motion);
    format!(
        "--ui-fieldset-motion-duration: {:.3}ms; --ui-fieldset-motion-distance: {:.3}px;",
        motion.duration_ms, motion.distance_px
    )
}

#[cfg(test)]
#[path = "../test/motion.rs"]
mod tests;
