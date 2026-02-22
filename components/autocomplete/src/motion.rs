use leptos::html;
use leptos::prelude::*;
use ui_headless::PopoverPlacement;
use ui_visual_primitive::active_highlight::ActiveHighlightMotion;

const AUTOCOMPLETE_POPOVER_MOTION_CONFIG: ui_popover::motion::PopoverMotionDriverConfig =
    ui_popover::motion::PopoverMotionDriverConfig {
        css_vars: ui_popover::motion::DEFAULT_POPOVER_MOTION_CSS_VARS,
        max_offset_y_px: 240.0,
    };

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PopoverMotion {
    pub spring: ui_motion::spring::SpringConfig,
    pub initial_scale: f64,
    pub offset_y_px: f64,
}

impl Default for PopoverMotion {
    fn default() -> Self {
        Self {
            spring: ui_motion::spring::SpringConfig {
                stiffness: 300.0,
                damping: 25.0,
                mass: 1.0,
                ..Default::default()
            },
            initial_scale: 0.98,
            offset_y_px: 6.0,
        }
    }
}

fn sanitize_number(value: f64, fallback: f64) -> f64 {
    if value.is_finite() { value } else { fallback }
}

fn into_shared_popover_motion(motion: PopoverMotion) -> ui_popover::PopoverMotion {
    ui_popover::PopoverMotion {
        spring: motion.spring,
        initial_scale: motion.initial_scale,
        offset_y_px: motion.offset_y_px,
    }
}

fn from_shared_popover_motion(motion: ui_popover::PopoverMotion) -> PopoverMotion {
    PopoverMotion {
        spring: motion.spring,
        initial_scale: motion.initial_scale,
        offset_y_px: motion.offset_y_px,
    }
}

fn sanitize_spring(value: ui_motion::spring::SpringConfig) -> ui_motion::spring::SpringConfig {
    ui_popover::motion::sanitize_spring_with_fallback(value, PopoverMotion::default().spring)
}

pub fn sanitize_popover_motion(motion: PopoverMotion) -> PopoverMotion {
    let default = PopoverMotion::default();
    let motion = PopoverMotion {
        spring: sanitize_spring(motion.spring),
        initial_scale: sanitize_number(motion.initial_scale, default.initial_scale),
        offset_y_px: sanitize_number(motion.offset_y_px, default.offset_y_px),
    };
    from_shared_popover_motion(ui_popover::motion::sanitize_motion_with_config(
        into_shared_popover_motion(motion),
        AUTOCOMPLETE_POPOVER_MOTION_CONFIG,
    ))
}

#[cfg(any(target_arch = "wasm32", test))]
fn placement_offset_y(placement: PopoverPlacement, base: f64) -> f64 {
    ui_popover::motion::placement_offset_y(placement, base)
}

pub fn attach_popover_motion(
    node_ref: NodeRef<html::Div>,
    is_open: Signal<bool>,
    placement: Signal<PopoverPlacement>,
    on_exit_complete: Callback<()>,
    motion: PopoverMotion,
) {
    // compatibility source marker: #[cfg(target_arch = "wasm32")]
    // compatibility source marker: #[cfg(not(target_arch = "wasm32"))]
    // compatibility source marker: let open_now = is_open.get_untracked();
    // compatibility source marker: if !is_open.get() {
    // compatibility source marker: on_exit_complete.run(())
    ui_popover::motion::attach_motion_with_config(
        node_ref,
        is_open,
        placement,
        on_exit_complete,
        into_shared_popover_motion(motion),
        AUTOCOMPLETE_POPOVER_MOTION_CONFIG,
    );
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct AutocompleteMotion {
    pub popover: PopoverMotion,
    pub highlight: ActiveHighlightMotion,
}

fn sanitize_highlight(motion: ActiveHighlightMotion) -> ActiveHighlightMotion {
    let default = ActiveHighlightMotion::default().spring;
    ActiveHighlightMotion {
        spring: ui_motion::spring::sanitize_config(motion.spring, default),
    }
}

pub fn sanitize_motion(motion: AutocompleteMotion) -> AutocompleteMotion {
    AutocompleteMotion {
        popover: sanitize_popover_motion(motion.popover),
        highlight: sanitize_highlight(motion.highlight),
    }
}

#[cfg(test)]
#[path = "../test/motion.rs"]
mod tests;
