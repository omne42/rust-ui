use ui_theme::default_text_field_motion_tokens;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FieldMotion {
    pub duration_ms: f64,
    pub spring: ui_motion::spring::SpringConfig,
}

impl Default for FieldMotion {
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

fn sanitize_spring(value: ui_motion::spring::SpringConfig) -> ui_motion::spring::SpringConfig {
    let default = FieldMotion::default().spring;
    ui_motion::spring::sanitize_config(value, default)
}

pub fn sanitize_motion(motion: FieldMotion) -> FieldMotion {
    let default = FieldMotion::default();

    FieldMotion {
        duration_ms: sanitize_number(motion.duration_ms, default.duration_ms).clamp(1.0, 800.0),
        spring: sanitize_spring(motion.spring),
    }
}

pub fn source_attr(motion: FieldMotion) -> &'static str {
    if sanitize_motion(motion) == FieldMotion::default() {
        "default"
    } else {
        "custom"
    }
}

pub fn attach_motion(motion: FieldMotion) -> String {
    let motion = sanitize_motion(motion);
    let duration_ms = if ui_motion::web::prefers_reduced_motion() {
        1.0
    } else {
        motion.duration_ms
    };
    let mut style = format!("--ui-field-motion-duration: {duration_ms}ms;");
    style.push_str(&format!(
        "--ui-field-motion-stiffness: {};",
        motion.spring.stiffness
    ));
    style.push_str(&format!(
        "--ui-field-motion-damping: {};",
        motion.spring.damping
    ));
    style.push_str(&format!("--ui-field-motion-mass: {};", motion.spring.mass));
    style.push_str(&format!(
        "--ui-field-motion-precision: {};",
        motion.spring.precision
    ));
    style
}

#[cfg(test)]
#[path = "../test/motion.rs"]
mod tests;
