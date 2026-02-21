use ui_headless::PopoverPlacement;
pub use ui_popover::PopoverMotion;
use ui_visual_primitive::active_highlight::ActiveHighlightMotion;

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct ComboBoxMotion {
    pub popover: PopoverMotion,
    pub highlight: ActiveHighlightMotion,
}

fn sanitize_spring(value: ui_motion::spring::SpringConfig) -> ui_motion::spring::SpringConfig {
    let default = ActiveHighlightMotion::default().spring;
    ui_motion::spring::sanitize_config(value, default)
}

fn sanitize_highlight(motion: ActiveHighlightMotion) -> ActiveHighlightMotion {
    ActiveHighlightMotion {
        spring: sanitize_spring(motion.spring),
    }
}

fn sanitize_popover_spring(
    value: ui_motion::spring::SpringConfig,
) -> ui_motion::spring::SpringConfig {
    let default = ui_popover::PopoverMotion::default().spring;
    ui_motion::spring::sanitize_config(value, default)
}

pub fn sanitize_popover_motion(motion: PopoverMotion) -> PopoverMotion {
    ui_popover::motion::sanitize_motion(PopoverMotion {
        spring: sanitize_popover_spring(motion.spring),
        ..motion
    })
}

pub fn sanitize_motion(motion: ComboBoxMotion) -> ComboBoxMotion {
    ComboBoxMotion {
        popover: sanitize_popover_motion(motion.popover),
        highlight: sanitize_highlight(motion.highlight),
    }
}

#[cfg(test)]
fn placement_offset_y(placement: PopoverPlacement, base: f64) -> f64 {
    match placement {
        PopoverPlacement::BottomStart | PopoverPlacement::BottomEnd => base.abs(),
        PopoverPlacement::TopStart | PopoverPlacement::TopEnd => -base.abs(),
    }
}

pub fn attach_popover_motion(
    node_ref: leptos::prelude::NodeRef<leptos::html::Div>,
    is_open: leptos::prelude::Signal<bool>,
    placement: leptos::prelude::Signal<PopoverPlacement>,
    on_exit_complete: leptos::prelude::Callback<()>,
    motion: PopoverMotion,
) {
    ui_popover::motion::attach_motion(
        node_ref,
        is_open,
        placement,
        on_exit_complete,
        sanitize_popover_motion(motion),
    );
}

#[cfg(test)]
#[path = "../test/motion.rs"]
mod tests;
