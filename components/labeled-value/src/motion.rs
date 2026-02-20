#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LabeledValueMotion {
    pub enabled: bool,
    pub duration_ms: u32,
}

impl Default for LabeledValueMotion {
    fn default() -> Self {
        Self {
            enabled: true,
            duration_ms: 180,
        }
    }
}

impl LabeledValueMotion {
    pub fn disabled() -> Self {
        Self {
            enabled: false,
            ..Self::default()
        }
    }
}

pub fn sanitize_duration_ms(duration_ms: u32) -> u32 {
    duration_ms.clamp(120, 900)
}

pub fn sanitize_motion(motion: LabeledValueMotion) -> LabeledValueMotion {
    LabeledValueMotion {
        enabled: motion.enabled,
        duration_ms: sanitize_duration_ms(motion.duration_ms),
    }
}

#[cfg(target_arch = "wasm32")]
pub fn attach_motion(
    node_ref: leptos::prelude::NodeRef<leptos::html::Div>,
    has_description: leptos::prelude::Signal<bool>,
    motion: LabeledValueMotion,
) {
    use leptos::prelude::*;
    use leptos::wasm_bindgen::JsCast;
    use ui_motion::{
        keyframes::MotionKeyframe,
        options::{FillMode, MotionOptions},
    };

    let motion = sanitize_motion(motion);
    let last_has_description = StoredValue::new(None::<bool>);

    Effect::new(move |_| {
        let now_has_description = has_description.get();
        let Some(previous_has_description) = last_has_description.get_value() else {
            last_has_description.set_value(Some(now_has_description));
            return;
        };

        if now_has_description == previous_has_description {
            return;
        }
        last_has_description.set_value(Some(now_has_description));

        if !motion.enabled || ui_motion::web::prefers_reduced_motion() {
            return;
        }

        let Some(node) = node_ref.get() else {
            return;
        };

        let element: leptos::web_sys::Element = node.unchecked_into();
        let frames = if now_has_description {
            [
                MotionKeyframe::new()
                    .with_offset(0.0)
                    .prop("opacity", "0.9")
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
                easing: "cubic-bezier(0.2, 0.8, 0.2, 1)",
                fill: FillMode::Both,
                ..Default::default()
            },
        );
    });
}

#[cfg(not(target_arch = "wasm32"))]
pub fn attach_motion(
    _node_ref: leptos::prelude::NodeRef<leptos::html::Div>,
    _has_description: leptos::prelude::Signal<bool>,
    _motion: LabeledValueMotion,
) {
}

#[cfg(test)]
#[path = "../test/motion.rs"]
mod tests;
