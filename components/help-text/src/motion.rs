#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HelpTextMotion {
    pub enabled: bool,
    pub duration_ms: u32,
}

impl Default for HelpTextMotion {
    fn default() -> Self {
        Self {
            enabled: true,
            duration_ms: 160,
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
    duration_ms.clamp(100, 800)
}

pub fn sanitize_motion(motion: HelpTextMotion) -> HelpTextMotion {
    HelpTextMotion {
        enabled: motion.enabled,
        duration_ms: sanitize_duration_ms(motion.duration_ms),
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
    use ui_motion::{
        keyframes::MotionKeyframe,
        options::{FillMode, MotionOptions},
    };

    let motion = sanitize_motion(motion);
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
                    .prop("opacity", "0.92")
                    .prop("transform", "translateY(-1px)"),
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
                    .prop("opacity", "0.96")
                    .prop("transform", "translateY(1px)"),
            ]
        };

        ui_motion::web::animate(
            &element,
            &frames,
            MotionOptions {
                duration_ms: motion.duration_ms,
                easing: "cubic-bezier(0.2, 0, 0, 1)",
                fill: FillMode::Both,
                ..Default::default()
            },
        );
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
