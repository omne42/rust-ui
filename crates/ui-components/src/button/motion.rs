#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ButtonMotion {
    pub press: PressMotion,
}

impl Default for ButtonMotion {
    fn default() -> Self {
        Self {
            press: PressMotion::Spring {
                spring: ui_motion::spring::SpringConfig {
                    stiffness: 420.0,
                    damping: 34.0,
                    ..Default::default()
                },
                pressed_scale: 0.97,
                pressed_translate_y_px: 1.0,
                pressed_brightness: 0.96,
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
    Spring {
        spring: ui_motion::spring::SpringConfig,
        pressed_scale: f64,
        pressed_translate_y_px: f64,
        pressed_brightness: f64,
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
    let spring = StoredValue::new_local(None::<ui_motion::spring::SpringAnimator>);

    Effect::new(move |_| {
        let motion = motion.get_value();
        let PressMotion::Spring {
            spring: config,
            pressed_scale,
            pressed_translate_y_px,
            pressed_brightness,
        } = motion.press
        else {
            return;
        };

        let Some(button) = node_ref.get() else {
            return;
        };

        if spring.get_value().is_some() {
            return;
        }

        let element: leptos::web_sys::HtmlElement = button.unchecked_into();
        let style = element.style();

        let animator = ui_motion::spring::SpringAnimator::new(0.0, config, move |t| {
            let t = t.clamp(0.0, 1.0);
            let scale = 1.0 + t * (pressed_scale - 1.0);
            let y_px = t * pressed_translate_y_px;
            let brightness = 1.0 + t * (pressed_brightness - 1.0);

            let _ =
                style.set_property("transform", &format!("translateY({y_px}px) scale({scale})"));
            let _ = style.set_property("filter", &format!("brightness({brightness})"));
        });

        let spring_for_cleanup = spring;
        on_cleanup(move || {
            if let Some(animator) = spring_for_cleanup.get_value() {
                animator.stop();
            }
        });

        spring.set_value(Some(animator));
    });

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
        match motion.press {
            PressMotion::None => {}
            PressMotion::Waapi {
                duration_ms,
                easing,
            } => {
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
            }
            PressMotion::Spring { .. } => {
                let Some(spring) = spring.get_value() else {
                    return;
                };
                spring.set_target(if pressed { 1.0 } else { 0.0 });
            }
        }
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
