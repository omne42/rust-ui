use ui_theme::default_text_field_motion_tokens;

const MIN_DURATION_MS: u32 = 1;
const MAX_DURATION_MS: u32 = 1_200;
#[cfg(target_arch = "wasm32")]
const ENTER_FROM_OPACITY: &str = "0.92";
#[cfg(target_arch = "wasm32")]
const EXIT_TO_OPACITY: &str = "0.96";
#[cfg(target_arch = "wasm32")]
const ENTER_FROM_TRANSFORM: &str = "translateY(-1px)";
#[cfg(target_arch = "wasm32")]
const EXIT_TO_TRANSFORM: &str = "translateY(1px)";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HelpTextMotion {
    pub enabled: bool,
    pub duration_ms: u32,
}

impl Default for HelpTextMotion {
    fn default() -> Self {
        let tokens = default_text_field_motion_tokens();
        Self {
            enabled: true,
            duration_ms: u32::from(tokens.duration_ms),
        }
    }
}

impl HelpTextMotion {
    pub fn disabled() -> Self {
        Self {
            enabled: false,
            ..Self::default()
        }
    }
}

pub fn sanitize_duration_ms(duration_ms: u32) -> u32 {
    let default_duration = HelpTextMotion::default().duration_ms;
    if duration_ms == 0 {
        default_duration
    } else {
        duration_ms.clamp(MIN_DURATION_MS, MAX_DURATION_MS)
    }
}

pub fn sanitize_motion(motion: HelpTextMotion) -> HelpTextMotion {
    HelpTextMotion {
        enabled: motion.enabled,
        duration_ms: sanitize_duration_ms(motion.duration_ms),
    }
}

pub fn resolve_motion_options(motion: HelpTextMotion) -> ui_motion::options::MotionOptions {
    let motion = sanitize_motion(motion);
    let tokens = default_text_field_motion_tokens();

    ui_motion::options::MotionOptions {
        duration_ms: motion.duration_ms,
        easing: tokens.easing,
        fill: ui_motion::options::FillMode::Both,
    }
}

#[cfg(target_arch = "wasm32")]
pub fn attach_motion(
    node_ref: leptos::prelude::NodeRef<leptos::html::Div>,
    is_error: leptos::prelude::Signal<bool>,
    motion: HelpTextMotion,
) {
    use leptos::prelude::*;
    use leptos::wasm_bindgen::JsCast;
    use ui_motion::keyframes::MotionKeyframe;

    let motion = sanitize_motion(motion);
    let motion_options = resolve_motion_options(motion);
    let last_is_error = StoredValue::new(None::<bool>);

    Effect::new(move |_| {
        let now_is_error = is_error.get();
        let Some(previous_is_error) = last_is_error.get_value() else {
            last_is_error.set_value(Some(now_is_error));
            return;
        };

        if now_is_error == previous_is_error {
            return;
        }
        last_is_error.set_value(Some(now_is_error));

        if !motion.enabled || ui_motion::web::prefers_reduced_motion() {
            return;
        }

        let Some(node) = node_ref.get() else {
            return;
        };

        let element: leptos::web_sys::Element = node.unchecked_into();
        let frames = if now_is_error {
            [
                MotionKeyframe::new()
                    .with_offset(0.0)
                    .prop("opacity", ENTER_FROM_OPACITY)
                    .prop("transform", ENTER_FROM_TRANSFORM),
                MotionKeyframe::new()
                    .with_offset(1.0)
                    .prop("opacity", "1")
                    .prop("transform", "translateY(0px)"),
            ]
        } else {
            [
                MotionKeyframe::new()
                    .with_offset(0.0)
                    .prop("opacity", "1")
                    .prop("transform", "translateY(0px)"),
                MotionKeyframe::new()
                    .with_offset(1.0)
                    .prop("opacity", EXIT_TO_OPACITY)
                    .prop("transform", EXIT_TO_TRANSFORM),
            ]
        };

        ui_motion::web::animate(&element, &frames, motion_options);
    });
}

#[cfg(not(target_arch = "wasm32"))]
pub fn attach_motion(
    _node_ref: leptos::prelude::NodeRef<leptos::html::Div>,
    _is_error: leptos::prelude::Signal<bool>,
    _motion: HelpTextMotion,
) {
}

#[cfg(test)]
#[path = "../test/motion.rs"]
mod tests;
