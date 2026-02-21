use ui_theme::default_checkbox_group_motion_tokens;

const MIN_DURATION_MS: u16 = 60;
const MAX_DURATION_MS: u16 = 1200;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CheckboxGroupMotion {
    pub enabled: bool,
    pub transition_duration_ms: u16,
    pub transition_easing: &'static str,
    pub spring: ui_motion::spring::SpringConfig,
}

impl Default for CheckboxGroupMotion {
    fn default() -> Self {
        let tokens = default_checkbox_group_motion_tokens();
        Self {
            enabled: true,
            transition_duration_ms: tokens.duration_ms,
            transition_easing: tokens.easing,
            spring: ui_motion::spring::SpringConfig {
                stiffness: tokens.spring.stiffness,
                damping: tokens.spring.damping,
                mass: tokens.spring.mass,
                precision: tokens.spring.precision,
            },
        }
    }
}

pub fn sanitize_motion(motion: CheckboxGroupMotion) -> CheckboxGroupMotion {
    let default = CheckboxGroupMotion::default();
    let transition_easing = if motion.transition_easing.trim().is_empty() {
        default.transition_easing
    } else {
        motion.transition_easing
    };

    CheckboxGroupMotion {
        enabled: motion.enabled,
        transition_duration_ms: motion
            .transition_duration_ms
            .clamp(MIN_DURATION_MS, MAX_DURATION_MS),
        transition_easing,
        spring: ui_motion::spring::sanitize_config(motion.spring, default.spring),
    }
}

pub fn motion_source_attr(motion: CheckboxGroupMotion) -> &'static str {
    if sanitize_motion(motion) == CheckboxGroupMotion::default() {
        "default"
    } else {
        "custom"
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EffectiveCheckboxGroupMotion {
    pub transition_duration_ms: u16,
    pub transition_easing: &'static str,
    pub spring: ui_motion::spring::SpringConfig,
    pub reduced: bool,
}

pub fn resolve_effective_motion(
    motion: CheckboxGroupMotion,
    prefers_reduced_motion: bool,
) -> EffectiveCheckboxGroupMotion {
    let motion = sanitize_motion(motion);
    let reduced = !motion.enabled || prefers_reduced_motion;

    EffectiveCheckboxGroupMotion {
        transition_duration_ms: if reduced {
            1
        } else {
            motion.transition_duration_ms
        },
        transition_easing: motion.transition_easing,
        spring: motion.spring,
        reduced,
    }
}

pub fn attach_motion(base_vars: Option<String>, motion: CheckboxGroupMotion) -> String {
    let mut style = base_vars.unwrap_or_default();

    if !style.trim().is_empty() && !style.trim_end().ends_with(';') {
        style.push(';');
    }

    // Non-wasm path is a predictable reduced-motion no-op in `ui-motion`.
    let effective = resolve_effective_motion(motion, ui_motion::web::prefers_reduced_motion());

    style.push_str(&format!(
        " --ui-checkbox-group-motion-duration: {}ms; --ui-checkbox-group-motion-easing: {}; --ui-checkbox-group-motion-stiffness: {}; --ui-checkbox-group-motion-damping: {}; --ui-checkbox-group-motion-mass: {}; --ui-checkbox-group-motion-precision: {}; --ui-checkbox-group-motion-reduced: {};",
        effective.transition_duration_ms,
        effective.transition_easing,
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
