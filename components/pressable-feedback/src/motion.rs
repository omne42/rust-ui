use ui_visual_primitive::ripple::RippleMotion;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PressableFeedbackMotion {
    pub enabled: bool,
    pub spring: ui_motion::spring::SpringConfig,
    pub pressed_scale: f64,
    pub highlight_opacity: f64,
    pub ripple: RippleMotion,
}

impl Default for PressableFeedbackMotion {
    fn default() -> Self {
        Self {
            enabled: true,
            spring: ui_motion::presets::spring_soft(),
            pressed_scale: 0.97,
            highlight_opacity: 0.14,
            ripple: RippleMotion::default(),
        }
    }
}

impl PressableFeedbackMotion {
    pub fn disabled() -> Self {
        Self {
            enabled: false,
            ripple: RippleMotion::disabled(),
            ..Self::default()
        }
    }
}

fn sanitize_number(value: f64, fallback: f64) -> f64 {
    if value.is_finite() { value } else { fallback }
}

pub fn sanitize_motion(motion: PressableFeedbackMotion) -> PressableFeedbackMotion {
    let default = PressableFeedbackMotion::default();

    let spring = motion.spring;
    let stiffness = if spring.stiffness.is_finite() && spring.stiffness > 0.0 {
        spring.stiffness
    } else {
        default.spring.stiffness
    };
    let damping = if spring.damping.is_finite() && spring.damping > 0.0 {
        spring.damping
    } else {
        default.spring.damping
    };
    let mass = if spring.mass.is_finite() && spring.mass > 0.0 {
        spring.mass
    } else {
        default.spring.mass
    };
    let precision = if spring.precision.is_finite() && spring.precision > 0.0 {
        spring.precision
    } else {
        default.spring.precision
    };

    PressableFeedbackMotion {
        enabled: motion.enabled,
        spring: ui_motion::spring::SpringConfig {
            stiffness,
            damping,
            mass,
            precision,
        },
        pressed_scale: sanitize_number(motion.pressed_scale, default.pressed_scale)
            .clamp(0.82, 1.0),
        highlight_opacity: sanitize_number(motion.highlight_opacity, default.highlight_opacity)
            .clamp(0.0, 0.4),
        ripple: ui_visual_primitive::ripple::sanitize_motion(motion.ripple),
    }
}

#[cfg(target_arch = "wasm32")]
pub fn attach_motion(
    node_ref: leptos::prelude::NodeRef<leptos::html::Div>,
    pressed: leptos::prelude::Signal<bool>,
    motion: PressableFeedbackMotion,
    has_highlight: bool,
) {
    use leptos::prelude::*;
    use leptos::wasm_bindgen::JsCast;

    let motion = sanitize_motion(motion);

    let last_pressed = StoredValue::new(None::<bool>);
    let springs = StoredValue::new_local(
        None::<(
            ui_motion::spring::SpringAnimator,
            ui_motion::spring::SpringAnimator,
        )>,
    );

    Effect::new(move |_| {
        let Some(node) = node_ref.get() else {
            return;
        };

        if springs.get_value().is_some() {
            return;
        }

        let element: leptos::web_sys::HtmlElement = node.unchecked_into();
        let style = element.style();

        let is_pressed_now = pressed.get_untracked();
        let initial_scale = if is_pressed_now {
            motion.pressed_scale
        } else {
            1.0
        };
        let initial_highlight = if is_pressed_now && has_highlight {
            motion.highlight_opacity
        } else {
            0.0
        };

        ui_observability::set_css_property_observed_auto!(
            &(style),
            "--ui-pressable-feedback-scale",
            &format!("{initial_scale}")
        );
        ui_observability::set_css_property_observed_auto!(
            &(style),
            "--ui-pressable-feedback-highlight-opacity",
            &format!("{initial_highlight}"),
        );

        let style_scale = style.clone();
        let scale =
            ui_motion::spring::SpringAnimator::new(initial_scale, motion.spring, move |next| {
                let next = next.clamp(0.6, 1.05);
                ui_observability::set_css_property_observed_auto!(
                    &(style_scale),
                    "--ui-pressable-feedback-scale",
                    &format!("{next}")
                );
            });

        let style_highlight = style.clone();
        let highlight =
            ui_motion::spring::SpringAnimator::new(initial_highlight, motion.spring, move |next| {
                let next = next.clamp(0.0, 0.6);
                ui_observability::set_css_property_observed_auto!(
                    &(style_highlight),
                    "--ui-pressable-feedback-highlight-opacity",
                    &format!("{next}"),
                );
            });

        let springs_for_cleanup = springs;
        on_cleanup(move || {
            if let Some((scale, highlight)) = springs_for_cleanup.get_value() {
                scale.stop();
                highlight.stop();
            }
        });

        springs.set_value(Some((scale, highlight)));
    });

    Effect::new(move |_| {
        let is_pressed_now = pressed.get();
        let Some(previous_pressed) = last_pressed.get_value() else {
            last_pressed.set_value(Some(is_pressed_now));
            return;
        };

        if is_pressed_now == previous_pressed {
            return;
        }
        last_pressed.set_value(Some(is_pressed_now));

        let Some((scale, highlight)) = springs.get_value() else {
            return;
        };

        let target_scale = if is_pressed_now {
            motion.pressed_scale
        } else {
            1.0
        };
        let target_highlight = if is_pressed_now && has_highlight {
            motion.highlight_opacity
        } else {
            0.0
        };

        if !motion.enabled || ui_motion::web::prefers_reduced_motion() {
            scale.clear_on_rest();
            highlight.clear_on_rest();
            scale.set_target(target_scale);
            highlight.set_target(target_highlight);
            return;
        }

        scale.clear_on_rest();
        highlight.clear_on_rest();
        scale.set_target(target_scale);
        highlight.set_target(target_highlight);
    });
}

#[cfg(not(target_arch = "wasm32"))]
pub fn attach_motion(
    _node_ref: leptos::prelude::NodeRef<leptos::html::Div>,
    _pressed: leptos::prelude::Signal<bool>,
    _motion: PressableFeedbackMotion,
    _has_highlight: bool,
) {
}

#[cfg(test)]
#[path = "../test/motion.rs"]
mod tests;
