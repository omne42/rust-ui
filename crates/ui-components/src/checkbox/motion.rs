#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CheckboxMotion {
    pub spring: ui_motion::spring::SpringConfig,
    pub hover_scale: f64,
    pub tap_scale: f64,
    pub indicator_spring: ui_motion::spring::SpringConfig,
}

impl Default for CheckboxMotion {
    fn default() -> Self {
        Self {
            spring: ui_motion::spring::SpringConfig {
                stiffness: 260.0,
                damping: 16.0,
                mass: 1.0,
                ..Default::default()
            },
            hover_scale: 1.05,
            tap_scale: 0.95,
            indicator_spring: ui_motion::spring::SpringConfig {
                stiffness: 340.0,
                damping: 22.0,
                mass: 1.0,
                ..Default::default()
            },
        }
    }
}

#[cfg(target_arch = "wasm32")]
pub fn attach_root_motion(
    node_ref: leptos::prelude::NodeRef<leptos::html::Button>,
    is_hovered: leptos::prelude::ReadSignal<bool>,
    is_pressed: leptos::prelude::ReadSignal<bool>,
    is_disabled: bool,
    motion: CheckboxMotion,
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
            let _ = style.set_property("--ui-checkbox-scale", &format!("{scale}"));
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
pub fn attach_root_motion(
    _node_ref: leptos::prelude::NodeRef<leptos::html::Button>,
    _is_hovered: leptos::prelude::ReadSignal<bool>,
    _is_pressed: leptos::prelude::ReadSignal<bool>,
    _is_disabled: bool,
    _motion: CheckboxMotion,
) {
}

#[cfg(target_arch = "wasm32")]
pub fn attach_indicator_motion(
    node_ref: leptos::prelude::NodeRef<leptos::html::Span>,
    is_checked: leptos::prelude::ReadSignal<bool>,
    motion: CheckboxMotion,
) {
    use leptos::prelude::*;
    use leptos::wasm_bindgen::JsCast;

    let motion = StoredValue::new(motion);
    let last_checked = StoredValue::new(None::<bool>);
    let spring = StoredValue::new_local(None::<ui_motion::spring::SpringAnimator>);

    Effect::new(move |_| {
        let config = motion.get_value().indicator_spring;
        let Some(span) = node_ref.get() else {
            return;
        };

        if spring.get_value().is_some() {
            return;
        }

        let element: leptos::web_sys::HtmlElement = span.unchecked_into();
        let style = element.style();
        let initial = if is_checked.get_untracked() { 1.0 } else { 0.0 };

        let animator = ui_motion::spring::SpringAnimator::new(initial, config, move |value| {
            let value = value.clamp(0.0, 1.0);
            let _ = style.set_property("--ui-checkbox-indicator", &format!("{value}"));
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
        let checked = is_checked.get();
        let Some(prev) = last_checked.get_value() else {
            last_checked.set_value(Some(checked));
            return;
        };
        if prev == checked {
            return;
        }
        last_checked.set_value(Some(checked));

        let Some(spring) = spring.get_value() else {
            return;
        };
        spring.set_target(if checked { 1.0 } else { 0.0 });
    });
}

#[cfg(not(target_arch = "wasm32"))]
pub fn attach_indicator_motion(
    _node_ref: leptos::prelude::NodeRef<leptos::html::Span>,
    _is_checked: leptos::prelude::ReadSignal<bool>,
    _motion: CheckboxMotion,
) {
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_motion_has_reasonable_params() {
        let motion = CheckboxMotion::default();
        assert!(motion.spring.stiffness > 0.0);
        assert!(motion.spring.damping > 0.0);
        assert!(motion.spring.mass > 0.0);
        assert!(motion.indicator_spring.stiffness > 0.0);
        assert!(motion.indicator_spring.damping > 0.0);
        assert!(motion.indicator_spring.mass > 0.0);
        assert!(motion.hover_scale >= 1.0);
        assert!(motion.tap_scale > 0.0);
        assert!(motion.tap_scale <= 1.0);
    }
}
