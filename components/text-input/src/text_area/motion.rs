use ui_theme::{default_text_field_motion_tokens, default_textarea_motion_tokens};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TextAreaMotion {
    pub enabled: bool,
    pub duration_ms: u32,
}

impl Default for TextAreaMotion {
    fn default() -> Self {
        let tokens = default_textarea_motion_tokens();
        Self {
            enabled: true,
            duration_ms: u32::from(tokens.duration_ms),
        }
    }
}

impl TextAreaMotion {
    pub fn disabled() -> Self {
        Self {
            enabled: false,
            ..Self::default()
        }
    }
}

pub fn sanitize_duration_ms(duration_ms: u32) -> u32 {
    duration_ms.clamp(80, 1_000)
}

pub fn sanitize_motion(motion: TextAreaMotion) -> TextAreaMotion {
    TextAreaMotion {
        enabled: motion.enabled,
        duration_ms: sanitize_duration_ms(motion.duration_ms),
    }
}

pub fn motion_style_vars(motion: TextAreaMotion) -> String {
    let motion = sanitize_motion(motion);
    let duration_ms = if motion.enabled {
        motion.duration_ms
    } else {
        0
    };
    let easing = default_text_field_motion_tokens().easing;
    format!(
        "--ui-text-area-motion-duration: {}ms; --ui-text-area-motion-easing: {};",
        duration_ms, easing
    )
}

#[cfg(target_arch = "wasm32")]
pub fn attach_motion(
    node_ref: leptos::prelude::NodeRef<leptos::html::Div>,
    is_active: leptos::prelude::Signal<bool>,
    motion: TextAreaMotion,
) {
    use leptos::prelude::*;
    use leptos::wasm_bindgen::JsCast;
    use ui_motion::{
        keyframes::MotionKeyframe,
        options::{FillMode, MotionOptions},
    };

    let motion = sanitize_motion(motion);
    let last_active = StoredValue::new(None::<bool>);

    Effect::new(move |_| {
        let now_active = is_active.get();
        let Some(previous_active) = last_active.get_value() else {
            last_active.set_value(Some(now_active));
            return;
        };

        if now_active == previous_active {
            return;
        }
        last_active.set_value(Some(now_active));

        if !motion.enabled || ui_motion::web::prefers_reduced_motion() {
            return;
        }

        let Some(node) = node_ref.get() else {
            return;
        };

        let element: leptos::web_sys::Element = node.unchecked_into();
        let frames = if now_active {
            [
                MotionKeyframe::new()
                    .with_offset(0.0)
                    .prop("opacity", "0.985")
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
                    .prop("opacity", "0.995")
                    .prop("transform", "translateY(1px)"),
            ]
        };

        ui_motion::web::animate(
            &element,
            &frames,
            MotionOptions {
                duration_ms: motion.duration_ms,
                fill: FillMode::Both,
                ..Default::default()
            },
        );
    });
}

#[cfg(not(target_arch = "wasm32"))]
pub fn attach_motion(
    _node_ref: leptos::prelude::NodeRef<leptos::html::Div>,
    _is_active: leptos::prelude::Signal<bool>,
    motion: TextAreaMotion,
) {
    std::hint::black_box(sanitize_motion(motion)); // drop(sanitize_motion(motion));
}

#[cfg(test)]
#[path = "../../test/text_area/motion.rs"]
mod tests;
