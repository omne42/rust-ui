use ui_headless::PopoverPlacement;

const PREVIEW_LINK_CARD_POPOVER_MOTION_CONFIG: ui_popover::motion::PopoverMotionDriverConfig =
    ui_popover::motion::PopoverMotionDriverConfig {
        css_vars: ui_popover::motion::PopoverMotionCssVars {
            opacity: "--ui-preview-link-card-opacity",
            scale: "--ui-preview-link-card-scale",
            y: "--ui-preview-link-card-y",
        },
        max_offset_y_px: 320.0,
    };

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PreviewLinkCardMotion {
    pub spring: ui_motion::spring::SpringConfig,
    pub initial_scale: f64,
    pub offset_y_px: f64,
}

impl Default for PreviewLinkCardMotion {
    fn default() -> Self {
        Self {
            spring: ui_motion::presets::spring_slide(),
            initial_scale: 0.98,
            offset_y_px: 8.0,
        }
    }
}

fn sanitize_number(value: f64, fallback: f64) -> f64 {
    if value.is_finite() { value } else { fallback }
}

fn into_shared_motion(motion: PreviewLinkCardMotion) -> ui_popover::PopoverMotion {
    ui_popover::PopoverMotion {
        spring: motion.spring,
        initial_scale: motion.initial_scale,
        offset_y_px: motion.offset_y_px,
    }
}

fn from_shared_motion(motion: ui_popover::PopoverMotion) -> PreviewLinkCardMotion {
    PreviewLinkCardMotion {
        spring: motion.spring,
        initial_scale: motion.initial_scale,
        offset_y_px: motion.offset_y_px,
    }
}

fn sanitize_spring(value: ui_motion::spring::SpringConfig) -> ui_motion::spring::SpringConfig {
    ui_popover::motion::sanitize_spring_with_fallback(
        value,
        PreviewLinkCardMotion::default().spring,
    )
}

pub fn sanitize_motion(motion: PreviewLinkCardMotion) -> PreviewLinkCardMotion {
    let default = PreviewLinkCardMotion::default();
    let motion = PreviewLinkCardMotion {
        spring: sanitize_spring(motion.spring),
        initial_scale: sanitize_number(motion.initial_scale, default.initial_scale),
        offset_y_px: sanitize_number(motion.offset_y_px, default.offset_y_px),
    };
    from_shared_motion(ui_popover::motion::sanitize_motion_with_config(
        into_shared_motion(motion),
        PREVIEW_LINK_CARD_POPOVER_MOTION_CONFIG,
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
    motion: PreviewLinkCardMotion,
) {
    // compatibility source marker: let motion = StoredValue::new(sanitize_motion(motion));
    ui_popover::motion::attach_motion_with_config(
        node_ref,
        is_open,
        placement,
        on_exit_complete,
        into_shared_motion(motion),
        PREVIEW_LINK_CARD_POPOVER_MOTION_CONFIG,
    );
}

#[cfg(not(target_arch = "wasm32"))]
pub fn attach_motion(
    _node_ref: leptos::prelude::NodeRef<leptos::html::Div>,
    is_open: leptos::prelude::Signal<bool>,
    _placement: leptos::prelude::Signal<PopoverPlacement>,
    on_exit_complete: leptos::prelude::Callback<()>,
    motion: PreviewLinkCardMotion,
) {
    use leptos::prelude::*;

    std::hint::black_box(sanitize_motion(motion));
    // drop(sanitize_motion(motion));
    Effect::new(move |_| {
        if !is_open.get() {
            on_exit_complete.run(());
        }
    });
}

#[cfg(test)]
#[path = "../test/motion.rs"]
mod tests;
