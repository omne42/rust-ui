use ui_theme::default_text_field_motion_tokens;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CalendarMotion {
    pub enabled: bool,
    pub duration_ms: f64,
    pub spring: ui_motion::spring::SpringConfig,
}

impl Default for CalendarMotion {
    fn default() -> Self {
        let tokens = default_text_field_motion_tokens();
        Self {
            enabled: true,
            duration_ms: f64::from(tokens.duration_ms),
            spring: ui_motion::presets::spring_soft(),
        }
    }
}

fn sanitize_number(value: f64, fallback: f64) -> f64 {
    if value.is_finite() { value } else { fallback }
}

pub fn sanitize_motion(motion: CalendarMotion) -> CalendarMotion {
    let default = CalendarMotion::default();

    CalendarMotion {
        enabled: motion.enabled,
        duration_ms: sanitize_number(motion.duration_ms, default.duration_ms).clamp(1.0, 1000.0),
        spring: ui_motion::spring::sanitize_config(motion.spring, default.spring),
    }
}

pub fn source_attr(motion: CalendarMotion) -> &'static str {
    if sanitize_motion(motion) == CalendarMotion::default() {
        "default"
    } else {
        "custom"
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EffectiveCalendarMotion {
    pub duration_ms: f64,
    pub spring: ui_motion::spring::SpringConfig,
    pub reduced: bool,
}

pub fn resolve_effective_motion(
    motion: CalendarMotion,
    prefers_reduced_motion: bool,
) -> EffectiveCalendarMotion {
    let motion = sanitize_motion(motion);
    let reduced = !motion.enabled || prefers_reduced_motion;

    EffectiveCalendarMotion {
        duration_ms: if reduced { 1.0 } else { motion.duration_ms },
        spring: motion.spring,
        reduced,
    }
}

pub fn attach_motion(base_vars: Option<String>, motion: CalendarMotion) -> String {
    let mut style = base_vars.unwrap_or_default();

    if !style.trim().is_empty() && !style.trim_end().ends_with(';') {
        style.push(';');
    }

    let effective = resolve_effective_motion(motion, ui_motion::web::prefers_reduced_motion());
    style.push_str(&format!(
        " --ui-calendar-motion-duration: {}ms;",
        effective.duration_ms
    ));
    style.push_str(&format!(
        " --ui-calendar-motion-stiffness: {}; --ui-calendar-motion-damping: {}; --ui-calendar-motion-mass: {}; --ui-calendar-motion-precision: {}; --ui-calendar-motion-reduced: {};",
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
