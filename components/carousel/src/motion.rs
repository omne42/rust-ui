use leptos::{html, prelude::*};
use ui_visual_primitive::active_highlight::attach_active_highlight_motion;

pub fn sanitize_motion(motion: super::CarouselMotion) -> super::CarouselMotion {
    let default = super::CarouselMotion::default();
    super::CarouselMotion {
        spring: ui_motion::spring::sanitize_config(motion.spring, default.spring),
    }
}

pub fn attach_carousel_indicator_motion(
    container_ref: NodeRef<html::Div>,
    highlight_ref: NodeRef<html::Div>,
    active_index: ReadSignal<usize>,
    option_id: Callback<usize, String>,
    motion: super::CarouselMotion,
) {
    // Component-level contract: sanitize user-facing motion input, then delegate runtime driver.
    let motion = sanitize_motion(motion);
    attach_active_highlight_motion(
        container_ref,
        highlight_ref,
        active_index,
        option_id,
        motion,
    );
}

#[cfg(test)]
#[path = "../test/motion.rs"]
mod tests;
