#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CheckboxFieldMotion {
    pub transition_ms: u16,
    pub indicator_scale_pct: u16,
}

impl Default for CheckboxFieldMotion {
    fn default() -> Self {
        Self {
            transition_ms: 160,
            indicator_scale_pct: 100,
        }
    }
}

pub fn sanitize_motion(motion: CheckboxFieldMotion) -> CheckboxFieldMotion {
    let default = CheckboxFieldMotion::default();

    CheckboxFieldMotion {
        transition_ms: if motion.transition_ms == 0 {
            default.transition_ms
        } else {
            motion.transition_ms.min(1200)
        },
        indicator_scale_pct: motion.indicator_scale_pct.clamp(80, 140),
    }
}

pub fn compose_style_vars(motion: CheckboxFieldMotion) -> String {
    let scale = (motion.indicator_scale_pct as f64) / 100.0;
    format!(
        "--ui-checkbox-field-transition-ms:{}ms;--ui-checkbox-field-indicator-scale:{scale:.3};",
        motion.transition_ms
    )
}

#[cfg(test)]
#[path = "../test/motion.rs"]
mod tests;
