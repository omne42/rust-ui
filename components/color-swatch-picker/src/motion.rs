use ui_theme::{default_swatch_motion_tokens, default_text_field_motion_tokens};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ColorSwatchPickerMotion {
    pub enabled: bool,
    pub transition_ms: u16,
    pub focus_ring_width_px: u16,
    pub spring: ui_motion::spring::SpringConfig,
}

impl Default for ColorSwatchPickerMotion {
    fn default() -> Self {
        let transition_tokens = default_text_field_motion_tokens();
        let swatch_tokens = default_swatch_motion_tokens();
        Self {
            enabled: true,
            transition_ms: transition_tokens.duration_ms,
            focus_ring_width_px: 5,
            spring: ui_motion::spring::SpringConfig {
                stiffness: swatch_tokens.spring.stiffness,
                damping: swatch_tokens.spring.damping,
                mass: swatch_tokens.spring.mass,
                precision: swatch_tokens.spring.precision,
            },
        }
    }
}

pub fn sanitize_motion(motion: ColorSwatchPickerMotion) -> ColorSwatchPickerMotion {
    let default = ColorSwatchPickerMotion::default();

    ColorSwatchPickerMotion {
        enabled: motion.enabled,
        transition_ms: if motion.transition_ms == 0 {
            default.transition_ms
        } else {
            motion.transition_ms.min(1200)
        },
        focus_ring_width_px: motion.focus_ring_width_px.clamp(2, 12),
        spring: ui_motion::spring::sanitize_config(motion.spring, default.spring),
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EffectiveColorSwatchPickerMotion {
    pub transition_ms: u16,
    pub focus_ring_width_px: u16,
    pub spring: ui_motion::spring::SpringConfig,
}

pub fn resolve_effective_motion(
    motion: ColorSwatchPickerMotion,
    prefers_reduced_motion: bool,
) -> EffectiveColorSwatchPickerMotion {
    let motion = sanitize_motion(motion);
    if prefers_reduced_motion || !motion.enabled {
        return EffectiveColorSwatchPickerMotion {
            transition_ms: 1,
            focus_ring_width_px: motion.focus_ring_width_px,
            spring: motion.spring,
        };
    }

    EffectiveColorSwatchPickerMotion {
        transition_ms: motion.transition_ms,
        focus_ring_width_px: motion.focus_ring_width_px,
        spring: motion.spring,
    }
}

pub fn attach_motion(base_vars: Option<String>, motion: ColorSwatchPickerMotion) -> String {
    let mut style = base_vars.unwrap_or_default();
    if !style.trim().is_empty() && !style.trim_end().ends_with(';') {
        style.push(';');
    }

    let effective = resolve_effective_motion(motion, ui_motion::web::prefers_reduced_motion());
    style.push_str(&format!(
        " --ui-color-swatch-picker-transition-ms:{}ms; --ui-color-swatch-picker-focus-ring-width:{}px; --ui-color-swatch-picker-spring-stiffness:{}; --ui-color-swatch-picker-spring-damping:{}; --ui-color-swatch-picker-spring-mass:{}; --ui-color-swatch-picker-spring-precision:{};",
        effective.transition_ms,
        effective.focus_ring_width_px,
        effective.spring.stiffness,
        effective.spring.damping,
        effective.spring.mass,
        effective.spring.precision
    ));

    style
}

pub fn compose_style_vars(motion: ColorSwatchPickerMotion) -> String {
    let effective = resolve_effective_motion(motion, false);
    format!(
        "--ui-color-swatch-picker-transition-ms:{}ms;--ui-color-swatch-picker-focus-ring-width:{}px;--ui-color-swatch-picker-spring-stiffness:{};--ui-color-swatch-picker-spring-damping:{};--ui-color-swatch-picker-spring-mass:{};--ui-color-swatch-picker-spring-precision:{};",
        effective.transition_ms,
        effective.focus_ring_width_px,
        effective.spring.stiffness,
        effective.spring.damping,
        effective.spring.mass,
        effective.spring.precision
    )
}

#[cfg(test)]
#[path = "../test/motion.rs"]
mod tests;
