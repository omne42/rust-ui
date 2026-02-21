use ui_theme::{Theme, ThemeContext, default_text_field_motion_tokens};

const MIN_DURATION_MS: f64 = 1.0;
const MAX_DURATION_MS: f64 = 1200.0;
const MIN_DISTANCE_PX: f64 = 0.0;
const MAX_DISTANCE_PX: f64 = 32.0;
const MIN_STIFFNESS: f64 = 1.0;
const MAX_STIFFNESS: f64 = 2000.0;
const MIN_DAMPING: f64 = 0.1;
const MAX_DAMPING: f64 = 400.0;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FieldsetMotion {
    pub duration_ms: f64,
    pub distance_px: f64,
    pub stiffness: f64,
    pub damping: f64,
}

impl Default for FieldsetMotion {
    fn default() -> Self {
        let text_field_motion = default_text_field_motion_tokens();
        let theme = Theme::new(ThemeContext::default());
        let spring = ui_motion::spring::SpringConfig::default();
        Self {
            duration_ms: f64::from(text_field_motion.duration_ms),
            distance_px: f64::from(theme.tokens.layout.space.space_2xs_px),
            stiffness: spring.stiffness,
            damping: spring.damping,
        }
    }
}

pub fn sanitize_motion(motion: FieldsetMotion) -> FieldsetMotion {
    let default = FieldsetMotion::default();

    FieldsetMotion {
        duration_ms: if motion.duration_ms.is_finite() {
            motion.duration_ms.clamp(MIN_DURATION_MS, MAX_DURATION_MS)
        } else {
            default.duration_ms
        },
        distance_px: if motion.distance_px.is_finite() {
            motion.distance_px.clamp(MIN_DISTANCE_PX, MAX_DISTANCE_PX)
        } else {
            default.distance_px
        },
        stiffness: if motion.stiffness.is_finite() {
            motion.stiffness.clamp(MIN_STIFFNESS, MAX_STIFFNESS)
        } else {
            default.stiffness
        },
        damping: if motion.damping.is_finite() {
            motion.damping.clamp(MIN_DAMPING, MAX_DAMPING)
        } else {
            default.damping
        },
    }
}

pub fn resolve_effective_motion(
    motion: FieldsetMotion,
    prefers_reduced_motion: bool,
) -> FieldsetMotion {
    let motion = sanitize_motion(motion);
    if prefers_reduced_motion {
        return FieldsetMotion {
            duration_ms: MIN_DURATION_MS,
            distance_px: 0.0,
            stiffness: motion.stiffness,
            damping: motion.damping,
        };
    }
    motion
}

pub fn attach_motion(motion: FieldsetMotion) -> String {
    let motion = resolve_effective_motion(motion, ui_motion::web::prefers_reduced_motion());
    format!(
        "--ui-fieldset-motion-duration: {:.3}ms; --ui-fieldset-motion-distance: {:.3}px; --ui-fieldset-motion-stiffness: {:.3}; --ui-fieldset-motion-damping: {:.3};",
        motion.duration_ms, motion.distance_px, motion.stiffness, motion.damping
    )
}

#[cfg(test)]
#[path = "../test/motion.rs"]
mod tests;
