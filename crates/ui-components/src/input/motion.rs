#[derive(Clone, Copy, Debug, PartialEq)]
pub struct InputMotion {
    pub spring: ui_motion::spring::SpringConfig,
    pub hidden_scale: f64,
    pub hover_scale: f64,
    pub tap_scale: f64,
}

impl Default for InputMotion {
    fn default() -> Self {
        Self {
            spring: ui_motion::spring::SpringConfig {
                stiffness: 280.0,
                damping: 18.0,
                mass: 1.0,
                ..Default::default()
            },
            hidden_scale: 0.85,
            hover_scale: 1.05,
            tap_scale: 0.95,
        }
    }
}

fn sanitize_number(value: f64, fallback: f64) -> f64 {
    if value.is_finite() { value } else { fallback }
}

fn sanitize_spring(value: ui_motion::spring::SpringConfig) -> ui_motion::spring::SpringConfig {
    let default = InputMotion::default().spring;

    ui_motion::spring::SpringConfig {
        stiffness: if value.stiffness.is_finite() && value.stiffness > 0.0 {
            value.stiffness
        } else {
            default.stiffness
        },
        damping: if value.damping.is_finite() && value.damping > 0.0 {
            value.damping
        } else {
            default.damping
        },
        mass: if value.mass.is_finite() && value.mass > 0.0 {
            value.mass
        } else {
            default.mass
        },
        precision: if value.precision.is_finite() && value.precision > 0.0 {
            value.precision
        } else {
            default.precision
        },
    }
}

pub fn sanitize_motion(motion: InputMotion) -> InputMotion {
    let default = InputMotion::default();

    InputMotion {
        spring: sanitize_spring(motion.spring),
        hidden_scale: sanitize_number(motion.hidden_scale, default.hidden_scale).clamp(0.0, 1.0),
        hover_scale: sanitize_number(motion.hover_scale, default.hover_scale).clamp(0.5, 2.0),
        tap_scale: sanitize_number(motion.tap_scale, default.tap_scale).clamp(0.5, 1.5),
    }
}

#[cfg(target_arch = "wasm32")]
fn mix(from: f64, to: f64, t: f64) -> f64 {
    from + ((to - from) * t)
}

#[cfg(target_arch = "wasm32")]
pub fn attach_clear_button_motion(
    node_ref: leptos::prelude::NodeRef<leptos::html::Button>,
    is_visible: leptos::prelude::Signal<bool>,
    is_hovered: leptos::prelude::ReadSignal<bool>,
    is_pressed: leptos::prelude::ReadSignal<bool>,
    is_disabled: bool,
    motion: InputMotion,
) {
    use leptos::prelude::*;
    use leptos::wasm_bindgen::JsCast;

    if is_disabled {
        return;
    }

    let motion = StoredValue::new(sanitize_motion(motion));
    let reveal_value = StoredValue::new_local(0.0_f64);
    let interaction_value = StoredValue::new_local(1.0_f64);

    let last_interaction = StoredValue::new(None::<(bool, bool)>);
    let last_visible = StoredValue::new(None::<bool>);

    let reveal_spring = StoredValue::new_local(None::<ui_motion::spring::SpringAnimator>);
    let interaction_spring = StoredValue::new_local(None::<ui_motion::spring::SpringAnimator>);

    Effect::new(move |_| {
        let config = motion.get_value();
        let Some(button) = node_ref.get() else {
            return;
        };
        if reveal_spring.get_value().is_some() {
            return;
        }

        let element: leptos::web_sys::HtmlElement = button.unchecked_into();
        let style = element.style();

        let initial_reveal = if is_visible.get_untracked() { 1.0 } else { 0.0 };
        reveal_value.set_value(initial_reveal);
        interaction_value.set_value(1.0);

        let initial_scale = mix(config.hidden_scale, 1.0, initial_reveal);
        let _ = style.set_property("--ui-input-clear-opacity", &format!("{initial_reveal}"));
        let _ = style.set_property("--ui-input-clear-scale", &format!("{initial_scale}"));

        let style_for_reveal = style.clone();
        let reveal_value_for_cb = reveal_value;
        let interaction_value_for_cb = interaction_value;
        let hidden_scale = config.hidden_scale;
        let reveal_animator =
            ui_motion::spring::SpringAnimator::new(initial_reveal, config.spring, move |v| {
                let v = v.clamp(0.0, 1.0);
                reveal_value_for_cb.set_value(v);

                let scale = mix(hidden_scale, 1.0, v) * interaction_value_for_cb.get_value();
                let _ = style_for_reveal.set_property("--ui-input-clear-opacity", &format!("{v}"));
                let _ =
                    style_for_reveal.set_property("--ui-input-clear-scale", &format!("{scale}"));
            });

        let style_for_interaction = style.clone();
        let reveal_value_for_interaction = reveal_value;
        let interaction_value_for_interaction = interaction_value;
        let hidden_scale_for_interaction = config.hidden_scale;
        let interaction_animator =
            ui_motion::spring::SpringAnimator::new(1.0, config.spring, move |v| {
                let v = v.clamp(0.0, 10.0);
                interaction_value_for_interaction.set_value(v);

                let reveal = reveal_value_for_interaction.get_value().clamp(0.0, 1.0);
                let base_scale = mix(hidden_scale_for_interaction, 1.0, reveal);
                let scale = base_scale * v;
                let _ = style_for_interaction
                    .set_property("--ui-input-clear-scale", &format!("{scale}"));
            });

        let reveal_for_cleanup = reveal_spring;
        let interaction_for_cleanup = interaction_spring;
        on_cleanup(move || {
            if let Some(animator) = reveal_for_cleanup.get_value() {
                animator.stop();
            }
            if let Some(animator) = interaction_for_cleanup.get_value() {
                animator.stop();
            }
        });

        reveal_spring.set_value(Some(reveal_animator));
        interaction_spring.set_value(Some(interaction_animator));
    });

    Effect::new(move |_| {
        let visible = is_visible.get();
        let Some(prev) = last_visible.get_value() else {
            last_visible.set_value(Some(visible));
            return;
        };
        if prev == visible {
            return;
        }
        last_visible.set_value(Some(visible));

        let Some(animator) = reveal_spring.get_value() else {
            return;
        };
        animator.set_target(if visible { 1.0 } else { 0.0 });
    });

    Effect::new(move |_| {
        let hovered = is_hovered.get();
        let pressed = is_pressed.get();
        let Some(prev) = last_interaction.get_value() else {
            last_interaction.set_value(Some((hovered, pressed)));
            return;
        };
        if prev == (hovered, pressed) {
            return;
        }
        last_interaction.set_value(Some((hovered, pressed)));

        let motion = motion.get_value();
        let target = if pressed {
            motion.tap_scale
        } else if hovered {
            motion.hover_scale
        } else {
            1.0
        };

        let Some(animator) = interaction_spring.get_value() else {
            return;
        };
        animator.set_target(target);
    });
}

#[cfg(not(target_arch = "wasm32"))]
pub fn attach_clear_button_motion(
    _node_ref: leptos::prelude::NodeRef<leptos::html::Button>,
    _is_visible: leptos::prelude::Signal<bool>,
    _is_hovered: leptos::prelude::ReadSignal<bool>,
    _is_pressed: leptos::prelude::ReadSignal<bool>,
    _is_disabled: bool,
    motion: InputMotion,
) {
    let _ = sanitize_motion(motion);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_motion_has_reasonable_params() {
        let motion = InputMotion::default();
        assert!(motion.hidden_scale > 0.0);
        assert!(motion.hidden_scale < 1.0);
        assert!(motion.hover_scale >= 1.0);
        assert!(motion.tap_scale > 0.0);
        assert!(motion.tap_scale <= 1.0);
    }

    #[test]
    fn sanitize_motion_falls_back_for_invalid_values() {
        let motion = sanitize_motion(InputMotion {
            spring: ui_motion::spring::SpringConfig {
                stiffness: f64::NAN,
                damping: -1.0,
                mass: 0.0,
                precision: f64::INFINITY,
            },
            hidden_scale: f64::NAN,
            hover_scale: f64::NAN,
            tap_scale: f64::NAN,
        });

        let default = InputMotion::default();
        assert_eq!(motion.spring.stiffness, default.spring.stiffness);
        assert_eq!(motion.spring.damping, default.spring.damping);
        assert_eq!(motion.spring.mass, default.spring.mass);
        assert_eq!(motion.spring.precision, default.spring.precision);
        assert_eq!(motion.hidden_scale, default.hidden_scale);
        assert_eq!(motion.hover_scale, default.hover_scale);
        assert_eq!(motion.tap_scale, default.tap_scale);
    }

    #[test]
    fn sanitize_motion_clamps_scale_values() {
        let motion = sanitize_motion(InputMotion {
            spring: ui_motion::spring::SpringConfig {
                stiffness: 320.0,
                damping: 20.0,
                mass: 1.1,
                precision: 0.002,
            },
            hidden_scale: 5.0,
            hover_scale: 6.0,
            tap_scale: -2.0,
        });

        assert_eq!(motion.spring.stiffness, 320.0);
        assert_eq!(motion.spring.damping, 20.0);
        assert_eq!(motion.spring.mass, 1.1);
        assert_eq!(motion.spring.precision, 0.002);
        assert_eq!(motion.hidden_scale, 1.0);
        assert_eq!(motion.hover_scale, 2.0);
        assert_eq!(motion.tap_scale, 0.5);
    }
}
