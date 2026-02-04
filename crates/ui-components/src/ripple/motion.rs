#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RippleMotion {
    pub duration_ms: u32,
}

impl Default for RippleMotion {
    fn default() -> Self {
        Self { duration_ms: 420 }
    }
}

#[cfg(target_arch = "wasm32")]
pub fn trigger_ripple(
    ripple_ref: leptos::prelude::NodeRef<leptos::html::Span>,
    motion: RippleMotion,
) {
    use leptos::prelude::*;
    use leptos::wasm_bindgen::JsCast;
    use ui_motion::{keyframes::MotionKeyframe, options::MotionOptions};

    if ui_motion::web::prefers_reduced_motion() {
        return;
    }

    let Some(span) = ripple_ref.get() else {
        return;
    };

    let element: leptos::web_sys::Element = span.unchecked_into();

    let frames = [
        MotionKeyframe::new()
            .with_offset(0.0)
            .prop("opacity", "0.35")
            .prop("transform", "scale(0)"),
        MotionKeyframe::new()
            .with_offset(1.0)
            .prop("opacity", "0")
            .prop("transform", "scale(1)"),
    ];

    ui_motion::web::animate(
        &element,
        &frames,
        MotionOptions {
            duration_ms: motion.duration_ms,
            easing: "cubic-bezier(0.2, 0, 0, 1)",
            ..Default::default()
        },
    );
}

#[cfg(not(target_arch = "wasm32"))]
pub fn trigger_ripple(
    _ripple_ref: leptos::prelude::NodeRef<leptos::html::Span>,
    _motion: RippleMotion,
) {
}
