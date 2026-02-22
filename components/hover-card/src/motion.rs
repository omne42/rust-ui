use ui_headless::PopoverPlacement;
use ui_theme::default_overlay_layout_tokens;

const HOVER_CARD_POPOVER_MOTION_CONFIG: ui_popover::motion::PopoverMotionDriverConfig =
    ui_popover::motion::PopoverMotionDriverConfig {
        css_vars: ui_popover::motion::PopoverMotionCssVars {
            opacity: "--ui-hover-card-opacity",
            scale: "--ui-hover-card-scale",
            y: "--ui-hover-card-y",
        },
        max_offset_y_px: 320.0,
    };

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HoverCardMotion {
    pub spring: ui_motion::spring::SpringConfig,
    pub initial_scale: f64,
    pub offset_y_px: f64,
}

impl Default for HoverCardMotion {
    fn default() -> Self {
        let overlay = default_overlay_layout_tokens();
        Self {
            spring: ui_motion::presets::spring_slide(),
            initial_scale: overlay.enter_scale,
            offset_y_px: f64::from(overlay.enter_offset_y_px),
        }
    }
}

fn sanitize_number(value: f64, fallback: f64) -> f64 {
    if value.is_finite() { value } else { fallback }
}

fn into_shared_motion(motion: HoverCardMotion) -> ui_popover::PopoverMotion {
    ui_popover::PopoverMotion {
        spring: motion.spring,
        initial_scale: motion.initial_scale,
        offset_y_px: motion.offset_y_px,
    }
}

fn from_shared_motion(motion: ui_popover::PopoverMotion) -> HoverCardMotion {
    HoverCardMotion {
        spring: motion.spring,
        initial_scale: motion.initial_scale,
        offset_y_px: motion.offset_y_px,
    }
}

fn sanitize_spring(value: ui_motion::spring::SpringConfig) -> ui_motion::spring::SpringConfig {
    // compatibility source marker: stiffness:
    // compatibility source marker: damping:
    ui_popover::motion::sanitize_spring_with_fallback(value, HoverCardMotion::default().spring)
}

pub fn sanitize_motion(motion: HoverCardMotion) -> HoverCardMotion {
    let default = HoverCardMotion::default();
    let motion = HoverCardMotion {
        spring: sanitize_spring(motion.spring),
        initial_scale: sanitize_number(motion.initial_scale, default.initial_scale),
        offset_y_px: sanitize_number(motion.offset_y_px, default.offset_y_px),
    };
    from_shared_motion(ui_popover::motion::sanitize_motion_with_config(
        into_shared_motion(motion),
        HOVER_CARD_POPOVER_MOTION_CONFIG,
    ))
}

#[cfg(any(target_arch = "wasm32", test))]
fn placement_offset_y(placement: PopoverPlacement, base: f64) -> f64 {
    ui_popover::motion::placement_offset_y(placement, base)
}

#[cfg(target_arch = "wasm32")]
pub fn attach_motion(
    node_ref: leptos::prelude::NodeRef<leptos::html::Div>,
    is_open: leptos::prelude::Signal<bool>,
    placement: leptos::prelude::Signal<PopoverPlacement>,
    on_exit_complete: leptos::prelude::Callback<()>,
    motion: HoverCardMotion,
) {
    // compatibility source marker: let motion = StoredValue::new(sanitize_motion(motion));
    // compatibility source marker: if ui_motion::web::prefers_reduced_motion() {
    // compatibility source marker: ui_motion::spring::SpringAnimator::new(
    ui_popover::motion::attach_motion_with_config(
        node_ref,
        is_open,
        placement,
        on_exit_complete,
        into_shared_motion(motion),
        HOVER_CARD_POPOVER_MOTION_CONFIG,
    );
}

#[cfg(not(target_arch = "wasm32"))]
pub fn attach_motion(
    _node_ref: leptos::prelude::NodeRef<leptos::html::Div>,
    is_open: leptos::prelude::Signal<bool>,
    _placement: leptos::prelude::Signal<PopoverPlacement>,
    on_exit_complete: leptos::prelude::Callback<()>,
    motion: HoverCardMotion,
) {
    use leptos::prelude::*;

    std::hint::black_box(sanitize_motion(motion));
    // compatibility source marker: drop(sanitize_motion(motion));
    Effect::new(move |_| {
        if !is_open.get() {
            on_exit_complete.run(());
        }
    });
}

#[cfg(test)]
#[path = "../test/motion.rs"]
mod tests;
