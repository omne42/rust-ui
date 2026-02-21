use ui_theme::default_text_field_motion_tokens;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LegendMotion {
    pub duration_ms: f64,
    pub spring: ui_motion::spring::SpringConfig,
}

impl Default for LegendMotion {
    fn default() -> Self {
        let tokens = default_text_field_motion_tokens();
        Self {
            duration_ms: f64::from(tokens.duration_ms),
            spring: ui_motion::presets::spring_soft(),
        }
    }
}

fn sanitize_number(value: f64, fallback: f64) -> f64 {
    if value.is_finite() { value } else { fallback }
}

pub fn sanitize_motion(motion: LegendMotion) -> LegendMotion {
    let default = LegendMotion::default();

    LegendMotion {
        duration_ms: sanitize_number(motion.duration_ms, default.duration_ms).clamp(1.0, 800.0),
        spring: ui_motion::spring::sanitize_config(motion.spring, default.spring),
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EffectiveLegendMotion {
    pub duration_ms: f64,
    pub spring: ui_motion::spring::SpringConfig,
    pub reduced: bool,
}

pub fn resolve_effective_motion(
    motion: LegendMotion,
    prefers_reduced_motion: bool,
) -> EffectiveLegendMotion {
    let motion = sanitize_motion(motion);
    let reduced = prefers_reduced_motion;

    EffectiveLegendMotion {
        duration_ms: if reduced { 1.0 } else { motion.duration_ms },
        spring: motion.spring,
        reduced,
    }
}

pub fn attach_motion(motion: LegendMotion) -> String {
    let effective = resolve_effective_motion(motion, ui_motion::web::prefers_reduced_motion());
    let duration_ms = effective.duration_ms;
    let mut style = format!("--ui-legend-motion-duration: {duration_ms}ms;");
    style.push_str(&format!(
        "--ui-legend-motion-stiffness: {};--ui-legend-motion-damping: {};--ui-legend-motion-mass: {};--ui-legend-motion-precision: {};--ui-legend-motion-reduced: {};",
        effective.spring.stiffness,
        effective.spring.damping,
        effective.spring.mass,
        effective.spring.precision,
        if effective.reduced { "true" } else { "false" }
    ));
    style
}

#[cfg(test)]
#[path = "../test/motion.rs"]
mod tests;
