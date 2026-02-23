use super::{RippleMotion, motion};
pub use ui_state_primitives::ripple::{
    RippleState, RippleStateInput, compose_class_name, normalize_optional_text, resolve_boundary,
    resolve_phase, resolve_state,
};

#[derive(Debug)]
pub struct RippleRenderInput {
    pub is_bounded: Option<bool>,
    pub motion: RippleMotion,
    pub class_name: Option<String>,
}

#[derive(Debug)]
pub struct RippleRenderState {
    pub class_name: String,
    pub style_vars: String,
    pub state: RippleState,
    pub motion: RippleMotion,
}

pub fn resolve_render_state(input: RippleRenderInput) -> RippleRenderState {
    let class_name = normalize_optional_text(input.class_name);
    let is_bounded = input.is_bounded.unwrap_or(true);

    let (motion, motion_source, style_vars) = motion::resolve_motion(input.motion);

    let state = resolve_state(RippleStateInput {
        phase: resolve_phase(motion.enabled),
        boundary: resolve_boundary(is_bounded),
        has_custom_motion: motion_source == "custom",
        has_custom_class_name: class_name.is_some(),
    });

    let class_name = compose_class_name(class_name, state);

    RippleRenderState {
        class_name,
        style_vars,
        state,
        motion,
    }
}

#[cfg(test)]
#[path = "../test/logic.rs"]
mod tests;
