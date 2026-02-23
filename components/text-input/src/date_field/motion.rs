use ui_theme::default_text_field_motion_tokens;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DateFieldMotion {
    pub enabled: bool,
    pub duration_ms: u16,
}

impl Default for DateFieldMotion {
    fn default() -> Self {
        let tokens = default_text_field_motion_tokens();
        Self {
            enabled: true,
            duration_ms: tokens.duration_ms,
        }
    }
}

impl DateFieldMotion {
    pub fn disabled() -> Self {
        Self {
            enabled: false,
            ..Self::default()
        }
    }
}

pub fn sanitize_duration_ms(duration_ms: u16) -> u16 {
    duration_ms.clamp(120, 1_000)
}

pub fn sanitize_motion(motion: DateFieldMotion) -> DateFieldMotion {
    DateFieldMotion {
        enabled: motion.enabled,
        duration_ms: sanitize_duration_ms(motion.duration_ms),
    }
}

#[cfg(target_arch = "wasm32")]
pub fn attach_motion(
    node_ref: leptos::prelude::NodeRef<leptos::html::Div>,
    has_value: leptos::prelude::Signal<bool>,
    motion: DateFieldMotion,
) {
    use leptos::prelude::*;
    use leptos::wasm_bindgen::JsCast;
    use ui_motion::{
        keyframes::MotionKeyframe,
        options::{FillMode, MotionOptions},
    };

    let motion = sanitize_motion(motion);
    let easing = default_text_field_motion_tokens().easing;
    let last_has_value = StoredValue::new(None::<bool>);

    Effect::new(move |_| {
        let now_has_value = has_value.get();
        let Some(previous_has_value) = last_has_value.get_value() else {
            last_has_value.set_value(Some(now_has_value));
            return;
        };

        if now_has_value == previous_has_value {
            return;
        }
        last_has_value.set_value(Some(now_has_value));

        if !motion.enabled || ui_motion::web::prefers_reduced_motion() {
            return;
        }

        let Some(node) = node_ref.get() else {
            return;
        };

        let element: leptos::web_sys::Element = node.unchecked_into();
        let frames = if now_has_value {
            [
                MotionKeyframe::new()
                    .with_offset(0.0)
                    .prop("opacity", "0.92")
                    .prop("transform", "translateY(1px)"),
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
                duration_ms: u32::from(motion.duration_ms),
                easing,
                fill: FillMode::Both,
                ..Default::default()
            },
        );
    });
}

#[cfg(not(target_arch = "wasm32"))]
pub fn attach_motion(
    _node_ref: leptos::prelude::NodeRef<leptos::html::Div>,
    _has_value: leptos::prelude::Signal<bool>,
    motion: DateFieldMotion,
) {
    std::hint::black_box(sanitize_motion(motion));
}

#[cfg(test)]
#[path = "../../test/date_field/motion.rs"]
mod tests;
