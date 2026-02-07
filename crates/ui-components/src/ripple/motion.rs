#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RippleMotion {
    pub enabled: bool,
    pub duration_ms: u32,
}

impl RippleMotion {
    pub fn disabled() -> Self {
        Self {
            enabled: false,
            ..Self::default()
        }
    }
}

impl Default for RippleMotion {
    fn default() -> Self {
        Self {
            enabled: true,
            duration_ms: 420,
        }
    }
}

pub fn sanitize_duration_ms(duration_ms: u32) -> u32 {
    duration_ms.clamp(120, 1_600)
}

pub fn sanitize_motion(motion: RippleMotion) -> RippleMotion {
    RippleMotion {
        enabled: motion.enabled,
        duration_ms: sanitize_duration_ms(motion.duration_ms),
    }
}

#[cfg(target_arch = "wasm32")]
fn trigger_ripple_with_origin_internal(
    ripple_ref: leptos::prelude::NodeRef<leptos::html::Span>,
    motion: RippleMotion,
    origin_x_percent: f64,
    origin_y_percent: f64,
) {
    use leptos::prelude::*;
    use leptos::wasm_bindgen::JsCast;
    use ui_motion::{keyframes::MotionKeyframe, options::MotionOptions};

    let motion = sanitize_motion(motion);
    if !motion.enabled || ui_motion::web::prefers_reduced_motion() {
        return;
    }

    let Some(span) = ripple_ref.get() else {
        return;
    };

    let html_element: leptos::web_sys::HtmlElement = span.unchecked_into();
    let style = html_element.style();

    let origin_x_percent = origin_x_percent.clamp(0.0, 100.0);
    let origin_y_percent = origin_y_percent.clamp(0.0, 100.0);

    let _ = style.set_property("--ui-ripple-origin-x", &format!("{origin_x_percent}%"));
    let _ = style.set_property("--ui-ripple-origin-y", &format!("{origin_y_percent}%"));
    let _ = style.set_property("--ui-ripple-duration-ms", &motion.duration_ms.to_string());

    let element: leptos::web_sys::Element = html_element.unchecked_into();
    let frames = [
        MotionKeyframe::new()
            .with_offset(0.0)
            .prop("opacity", "0.34")
            .prop("transform", "scale(0)"),
        MotionKeyframe::new()
            .with_offset(0.5)
            .prop("opacity", "0.16")
            .prop("transform", "scale(0.55)"),
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

#[cfg(target_arch = "wasm32")]
pub fn trigger_ripple(
    ripple_ref: leptos::prelude::NodeRef<leptos::html::Span>,
    motion: RippleMotion,
) {
    trigger_ripple_with_origin_internal(ripple_ref, motion, 50.0, 50.0);
}

#[cfg(target_arch = "wasm32")]
pub fn trigger_ripple_at(
    ripple_ref: leptos::prelude::NodeRef<leptos::html::Span>,
    motion: RippleMotion,
    origin_x_percent: f64,
    origin_y_percent: f64,
) {
    trigger_ripple_with_origin_internal(ripple_ref, motion, origin_x_percent, origin_y_percent);
}

#[cfg(not(target_arch = "wasm32"))]
pub fn trigger_ripple(
    _ripple_ref: leptos::prelude::NodeRef<leptos::html::Span>,
    _motion: RippleMotion,
) {
}

#[cfg(not(target_arch = "wasm32"))]
pub fn trigger_ripple_at(
    _ripple_ref: leptos::prelude::NodeRef<leptos::html::Span>,
    _motion: RippleMotion,
    _origin_x_percent: f64,
    _origin_y_percent: f64,
) {
}
