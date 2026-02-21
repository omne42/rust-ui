use ui_theme::default_text_field_motion_tokens;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CheckboxFieldMotion {
    pub enabled: bool,
    pub transition_ms: u16,
    pub indicator_scale_pct: u16,
}

impl Default for CheckboxFieldMotion {
    fn default() -> Self {
        let tokens = default_text_field_motion_tokens();
        Self {
            enabled: true,
            transition_ms: tokens.duration_ms,
            indicator_scale_pct: 100,
        }
    }
}

pub fn sanitize_motion(motion: CheckboxFieldMotion) -> CheckboxFieldMotion {
    let default = CheckboxFieldMotion::default();

    CheckboxFieldMotion {
        enabled: motion.enabled,
        transition_ms: if motion.transition_ms == 0 {
            default.transition_ms
        } else {
            motion.transition_ms.min(1200)
        },
        indicator_scale_pct: motion.indicator_scale_pct.clamp(80, 140),
    }
}

pub fn source_attr(motion: CheckboxFieldMotion) -> &'static str {
    if sanitize_motion(motion) == CheckboxFieldMotion::default() {
        "default"
    } else {
        "custom"
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EffectiveCheckboxFieldMotion {
    pub transition_ms: u16,
    pub indicator_scale_pct: u16,
}

pub fn resolve_effective_motion(
    motion: CheckboxFieldMotion,
    prefers_reduced_motion: bool,
) -> EffectiveCheckboxFieldMotion {
    let motion = sanitize_motion(motion);

    if !motion.enabled || prefers_reduced_motion {
        return EffectiveCheckboxFieldMotion {
            transition_ms: 1,
            indicator_scale_pct: 100,
        };
    }

    EffectiveCheckboxFieldMotion {
        transition_ms: motion.transition_ms,
        indicator_scale_pct: motion.indicator_scale_pct,
    }
}

pub fn attach_motion(base_vars: Option<String>, motion: CheckboxFieldMotion) -> String {
    let mut style = base_vars.unwrap_or_default();
    if !style.trim().is_empty() && !style.trim_end().ends_with(';') {
        style.push(';');
    }

    let effective = resolve_effective_motion(motion, ui_motion::web::prefers_reduced_motion());
    let scale = (effective.indicator_scale_pct as f64) / 100.0;

    style.push_str(&format!(
        " --ui-checkbox-field-transition-ms:{}ms; --ui-checkbox-field-indicator-scale:{scale:.3};",
        effective.transition_ms
    ));

    style
}

pub fn compose_style_vars(motion: CheckboxFieldMotion) -> String {
    let effective = resolve_effective_motion(motion, false);
    let scale = (effective.indicator_scale_pct as f64) / 100.0;
    format!(
        "--ui-checkbox-field-transition-ms:{}ms;--ui-checkbox-field-indicator-scale:{scale:.3};",
        effective.transition_ms
    )
}

#[cfg(test)]
#[path = "../test/motion.rs"]
mod tests;
