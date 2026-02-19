#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DateFieldMotion {
    pub enabled: bool,
    pub duration_ms: u32,
}

impl Default for DateFieldMotion {
    fn default() -> Self {
        Self {
            enabled: true,
            duration_ms: 180,
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

pub fn sanitize_duration_ms(duration_ms: u32) -> u32 {
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
                duration_ms: motion.duration_ms,
                easing: "cubic-bezier(0.22, 1, 0.36, 1)",
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
    _motion: DateFieldMotion,
) {
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanitize_motion_clamps_duration() {
        assert_eq!(sanitize_duration_ms(0), 120);
        assert_eq!(sanitize_duration_ms(180), 180);
        assert_eq!(sanitize_duration_ms(2_000), 1_000);
    }

    #[test]
    fn disabled_constructor_turns_motion_off() {
        assert!(!DateFieldMotion::disabled().enabled);
    }
}
