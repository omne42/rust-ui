use leptos::{html, prelude::*};
use ui_visual_primitive::active_highlight::attach_active_highlight_motion;

pub type CommandMotion = ui_visual_primitive::active_highlight::ActiveHighlightMotion;

pub fn sanitize_motion(motion: CommandMotion) -> CommandMotion {
    let default = CommandMotion::default();
    let mut spring = motion.spring;

    if !spring.stiffness.is_finite() || spring.stiffness <= 0.0 {
        spring.stiffness = default.spring.stiffness;
    }
    if !spring.damping.is_finite() || spring.damping <= 0.0 {
        spring.damping = default.spring.damping;
    }
    if !spring.mass.is_finite() || spring.mass <= 0.0 {
        spring.mass = default.spring.mass;
    }
    if !spring.precision.is_finite() || spring.precision <= 0.0 {
        spring.precision = default.spring.precision;
    }

    CommandMotion { spring }
}

pub fn is_custom_motion(motion: CommandMotion) -> bool {
    sanitize_motion(motion) != CommandMotion::default()
}

pub fn attach_motion(
    container_ref: NodeRef<html::Div>,
    highlight_ref: NodeRef<html::Div>,
    active_index: ReadSignal<usize>,
    option_id: Callback<usize, String>,
    motion: CommandMotion,
) {
    attach_active_highlight_motion(
        container_ref,
        highlight_ref,
        active_index,
        option_id,
        sanitize_motion(motion),
    );
}

#[cfg(test)]
#[path = "../test/motion.rs"]
mod tests;
