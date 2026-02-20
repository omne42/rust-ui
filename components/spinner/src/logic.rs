use super::motion::{self, SpinnerMotion};

pub use ui_state_primitives::spinner::{
    SpinnerSize, SpinnerState, SpinnerStateInput, compose_class_name, normalize_optional_text,
    resolve_aria_label, resolve_state,
};

#[derive(Debug)]
pub struct SpinnerRenderInput<'a> {
    pub size: SpinnerSize,
    pub aria_label: Option<String>,
    pub class_name: Option<String>,
    pub motion: SpinnerMotion,
    pub default_aria_label: &'a str,
}

#[derive(Debug)]
pub struct SpinnerRenderState {
    pub aria_label: String,
    pub class_name: String,
    pub state: SpinnerState,
    pub motion_source: &'static str,
    pub style_vars: String,
}

pub fn resolve_render_state(input: SpinnerRenderInput<'_>) -> SpinnerRenderState {
    let class_name = normalize_optional_text(input.class_name);
    let (aria_label, has_custom_aria_label) =
        resolve_aria_label(input.aria_label, input.default_aria_label);

    let motion = motion::sanitize_motion(input.motion);
    let motion_source = motion::source_attr(motion);
    let style = motion::attach_motion(None, motion);

    let state = resolve_state(SpinnerStateInput {
        size: input.size,
        has_custom_aria_label,
        has_custom_class_name: class_name.is_some(),
    });

    let class_name = compose_class_name(class_name, state);

    SpinnerRenderState {
        aria_label,
        class_name,
        state,
        motion_source,
        style_vars: style,
    }
}

#[cfg(test)]
#[path = "../test/logic.rs"]
mod tests;
