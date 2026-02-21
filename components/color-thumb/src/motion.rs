use ui_theme::default_text_field_motion_tokens;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ColorThumbMotion {
    pub handle_duration_ms: u16,
    pub loupe_duration_ms: u16,
    pub spring: ui_motion::spring::SpringConfig,
}

impl Default for ColorThumbMotion {
    fn default() -> Self {
        let tokens = default_text_field_motion_tokens();
        Self {
            handle_duration_ms: tokens.duration_ms,
            loupe_duration_ms: tokens.duration_ms,
            spring: ui_motion::presets::spring_soft(),
        }
    }
}

pub fn sanitize_motion(motion: ColorThumbMotion) -> ColorThumbMotion {
    let default = ColorThumbMotion::default();

    ColorThumbMotion {
        handle_duration_ms: motion.handle_duration_ms.clamp(60, 1200),
        loupe_duration_ms: motion.loupe_duration_ms.clamp(60, 1200),
        spring: ui_motion::spring::sanitize_config(motion.spring, default.spring),
    }
}

pub fn source_attr(motion: ColorThumbMotion) -> &'static str {
    if sanitize_motion(motion) == ColorThumbMotion::default() {
        "default"
    } else {
        "custom"
    }
}

pub fn resolve_runtime_motion(motion: ColorThumbMotion) -> ColorThumbMotion {
    let motion = sanitize_motion(motion);

    if ui_motion::web::prefers_reduced_motion() {
        return ColorThumbMotion {
            handle_duration_ms: 1,
            loupe_duration_ms: 1,
            spring: motion.spring,
        };
    }

    motion
}

pub fn attach_motion(base_vars: Option<String>, motion: ColorThumbMotion) -> String {
    let motion = resolve_runtime_motion(motion);
    let mut style = base_vars.unwrap_or_default();

    if !style.trim().is_empty() && !style.trim_end().ends_with(';') {
        style.push(';');
    }

    style.push_str(&format!(
        " --ui-color-thumb-handle-duration: {}ms; --ui-color-thumb-loupe-duration: {}ms; --ui-color-thumb-motion-stiffness: {}; --ui-color-thumb-motion-damping: {}; --ui-color-thumb-motion-mass: {}; --ui-color-thumb-motion-precision: {};",
        motion.handle_duration_ms,
        motion.loupe_duration_ms,
        motion.spring.stiffness,
        motion.spring.damping,
        motion.spring.mass,
        motion.spring.precision
    ));

    style
}

#[cfg(test)]
#[path = "../test/motion.rs"]
mod tests;
