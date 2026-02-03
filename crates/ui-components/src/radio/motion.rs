#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RadioMotion {
    pub spring: ui_motion::spring::SpringConfig,
    pub hover_scale: f64,
    pub tap_scale: f64,
}

impl Default for RadioMotion {
    fn default() -> Self {
        Self {
            spring: ui_motion::spring::SpringConfig {
                stiffness: 260.0,
                damping: 16.0,
                mass: 1.0,
                ..Default::default()
            },
            hover_scale: 1.03,
            tap_scale: 0.97,
        }
    }
}

#[cfg(target_arch = "wasm32")]
pub fn attach_motion(
    node_ref: leptos::prelude::NodeRef<leptos::html::Button>,
    is_hovered: leptos::prelude::ReadSignal<bool>,
    is_pressed: leptos::prelude::ReadSignal<bool>,
    is_disabled: bool,
    motion: RadioMotion,
) {
    use leptos::prelude::*;
    use leptos::wasm_bindgen::JsCast;

    if is_disabled {
        return;
    }

    let motion = StoredValue::new(motion);
    let last_state = StoredValue::new(None::<(bool, bool)>);
    let spring = StoredValue::new_local(None::<ui_motion::spring::SpringAnimator>);

    Effect::new(move |_| {
        let config = motion.get_value().spring;

        let Some(button) = node_ref.get() else {
            return;
        };

        if spring.get_value().is_some() {
            return;
        }

        let element: leptos::web_sys::HtmlElement = button.unchecked_into();
        let style = element.style();

        let animator = ui_motion::spring::SpringAnimator::new(1.0, config, move |scale| {
            let scale = scale.clamp(0.0, 10.0);
            let _ = style.set_property("--ui-radio-scale", &format!("{scale}"));
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
        let hovered = is_hovered.get();
        let pressed = is_pressed.get();
        let Some(prev) = last_state.get_value() else {
            last_state.set_value(Some((hovered, pressed)));
            return;
        };
        if prev == (hovered, pressed) {
            return;
        }
        last_state.set_value(Some((hovered, pressed)));

        let motion = motion.get_value();
        let target = if pressed {
            motion.tap_scale
        } else if hovered {
            motion.hover_scale
        } else {
            1.0
        };

        let Some(spring) = spring.get_value() else {
            return;
        };
        spring.set_target(target);
    });
}

#[cfg(not(target_arch = "wasm32"))]
pub fn attach_motion(
    _node_ref: leptos::prelude::NodeRef<leptos::html::Button>,
    _is_hovered: leptos::prelude::ReadSignal<bool>,
    _is_pressed: leptos::prelude::ReadSignal<bool>,
    _is_disabled: bool,
    _motion: RadioMotion,
) {
}
