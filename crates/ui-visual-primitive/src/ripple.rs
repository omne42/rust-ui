use ui_theme::default_text_field_motion_tokens;

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
        let tokens = default_text_field_motion_tokens();
        Self {
            enabled: true,
            duration_ms: u32::from(tokens.duration_ms),
        }
    }
}

fn duration_min_ms() -> u32 {
    120
}

fn duration_max_ms() -> u32 {
    1_600
}

pub fn easing() -> &'static str {
    default_text_field_motion_tokens().easing
}

pub fn sanitize_duration_ms(duration_ms: u32) -> u32 {
    duration_ms.clamp(duration_min_ms(), duration_max_ms())
}

pub fn sanitize_motion(motion: RippleMotion) -> RippleMotion {
    RippleMotion {
        enabled: motion.enabled,
        duration_ms: sanitize_duration_ms(motion.duration_ms),
    }
}

pub fn source_attr(motion: RippleMotion) -> &'static str {
    if sanitize_motion(motion) == RippleMotion::default() {
        "default"
    } else {
        "custom"
    }
}

pub fn attach_motion(base_vars: Option<String>, motion: RippleMotion) -> String {
    let motion = sanitize_motion(motion);
    let mut style = base_vars.unwrap_or_default();

    if !style.trim().is_empty() && !style.trim_end().ends_with(';') {
        style.push(';');
    }

    style.push_str(&format!(
        " --ui-ripple-duration-ms: {}ms;",
        motion.duration_ms
    ));
    style
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

    ui_observability::set_css_property_observed_auto!(
        &(style),
        "--ui-ripple-origin-x",
        &format!("{origin_x_percent}%")
    );
    ui_observability::set_css_property_observed_auto!(
        &(style),
        "--ui-ripple-origin-y",
        &format!("{origin_y_percent}%")
    );
    ui_observability::set_css_property_observed_auto!(
        &(style),
        "--ui-ripple-duration-ms",
        &format!("{}ms", motion.duration_ms),
    );

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
            easing: easing(),
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
