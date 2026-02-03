#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ButtonMotion {
    pub press: PressMotion,
}

impl Default for ButtonMotion {
    fn default() -> Self {
        Self {
            press: PressMotion::Waapi {
                duration_ms: 80,
                easing: "cubic-bezier(0.2, 0, 0, 1)",
            },
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PressMotion {
    None,
    Waapi {
        duration_ms: u32,
        easing: &'static str,
    },
}

#[cfg(target_arch = "wasm32")]
pub fn attach_motion(
    node_ref: leptos::prelude::NodeRef<leptos::html::Button>,
    is_pressed: leptos::prelude::ReadSignal<bool>,
    is_disabled: bool,
    motion: ButtonMotion,
) {
    use leptos::prelude::*;
    use leptos::wasm_bindgen::JsCast;

    if is_disabled {
        return;
    }

    let motion = StoredValue::new(motion);
    let last_pressed = StoredValue::new(None::<bool>);

    Effect::new(move |_| {
        let pressed = is_pressed.get();
        let Some(prev) = last_pressed.get_value() else {
            last_pressed.set_value(Some(pressed));
            return;
        };
        if prev == pressed {
            return;
        }
        last_pressed.set_value(Some(pressed));

        let motion = motion.get_value();
        if matches!(motion.press, PressMotion::None) {
            return;
        }

        let Some(button) = node_ref.get() else {
            return;
        };

        let element: leptos::web_sys::Element = button.unchecked_into();
        let (duration_ms, easing) = match motion.press {
            PressMotion::Waapi {
                duration_ms,
                easing,
            } => (duration_ms, easing),
            PressMotion::None => return,
        };

        let (from_transform, from_filter, to_transform, to_filter) = if pressed {
            (
                "translateY(0px)",
                "brightness(1)",
                "translateY(1px)",
                "brightness(0.96)",
            )
        } else {
            (
                "translateY(1px)",
                "brightness(0.96)",
                "translateY(0px)",
                "brightness(1)",
            )
        };

        ui_motion::web::animate(
            &element,
            &[
                ui_motion::keyframes::MotionKeyframe::new()
                    .with_offset(0.0)
                    .prop("transform", from_transform)
                    .prop("filter", from_filter),
                ui_motion::keyframes::MotionKeyframe::new()
                    .with_offset(1.0)
                    .prop("transform", to_transform)
                    .prop("filter", to_filter),
            ],
            ui_motion::options::MotionOptions {
                duration_ms,
                easing,
                fill: ui_motion::options::FillMode::Backwards,
                ..Default::default()
            },
        );
    });
}

#[cfg(not(target_arch = "wasm32"))]
pub fn attach_motion(
    _node_ref: leptos::prelude::NodeRef<leptos::html::Button>,
    _is_pressed: leptos::prelude::ReadSignal<bool>,
    _is_disabled: bool,
    _motion: ButtonMotion,
) {
}
