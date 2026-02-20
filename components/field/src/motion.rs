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
#[path = "../test/motion.rs"]
mod tests;
