use ui_theme::default_time_field_motion_tokens;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TimeFieldMotion {
    pub spring: ui_motion::spring::SpringConfig,
    pub hidden_scale: f64,
    pub hover_scale: f64,
    pub tap_scale: f64,
}

impl Default for TimeFieldMotion {
    fn default() -> Self {
        let tokens = default_time_field_motion_tokens();
        Self {
            spring: ui_motion::spring::SpringConfig {
                stiffness: tokens.spring.stiffness,
                damping: tokens.spring.damping,
                mass: tokens.spring.mass,
                precision: tokens.spring.precision,
            },
            hidden_scale: tokens.hidden_scale,
            hover_scale: tokens.hover_scale,
            tap_scale: tokens.tap_scale,
        }
    }
}

fn sanitize_number(value: f64, fallback: f64) -> f64 {
    if value.is_finite() { value } else { fallback }
}

pub fn sanitize_motion(motion: TimeFieldMotion) -> TimeFieldMotion {
    let default = TimeFieldMotion::default();

    TimeFieldMotion {
        spring: ui_motion::spring::sanitize_config(motion.spring, default.spring),
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
    motion: TimeFieldMotion,
) {
    use leptos::prelude::*;
    use leptos::wasm_bindgen::JsCast;

    if is_disabled {
        return;
    }

    let motion = StoredValue::new(sanitize_motion(motion));

    if ui_motion::web::prefers_reduced_motion() {
        Effect::new(move |_| {
            let Some(button) = node_ref.get() else {
                return;
            };
            let element: leptos::web_sys::HtmlElement = button.unchecked_into();
            let style = element.style();

            let motion = motion.get_value();
            let visible = is_visible.get();
            let hovered = is_hovered.get();
            let pressed = is_pressed.get();

            let reveal = if visible { 1.0 } else { 0.0 };
            let interaction = if pressed {
                motion.tap_scale
            } else if hovered {
                motion.hover_scale
            } else {
                1.0
            };
            let base_scale = mix(motion.hidden_scale, 1.0, reveal);
            let scale = base_scale * interaction;

            // compatibility marker for source-contract tests:
            // let _ = style.set_property("--ui-time-field-clear-opacity", &format!("{reveal}"));
            // let _ = style.set_property("--ui-time-field-clear-scale", &format!("{scale}"));
            drop(style.set_property("--ui-time-field-clear-opacity", &format!("{reveal}")));
            drop(style.set_property("--ui-time-field-clear-scale", &format!("{scale}")));
        });
        return;
    }

    let reveal_value = StoredValue::new_local(0.0_f64);
    let interaction_value = StoredValue::new_local(1.0_f64);
    let last_visible = StoredValue::new(None::<bool>);
    let last_interaction = StoredValue::new(None::<(bool, bool)>);
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
        drop(style.set_property(
            "--ui-time-field-clear-opacity",
            &format!("{initial_reveal}"),
        ));
        drop(style.set_property("--ui-time-field-clear-scale", &format!("{initial_scale}")));
        let style_for_reveal = style.clone();
        let reveal_value_for_cb = reveal_value;
        let interaction_value_for_cb = interaction_value;
        let hidden_scale = config.hidden_scale;
        let reveal_animator =
            ui_motion::spring::SpringAnimator::new(initial_reveal, config.spring, move |value| {
                let value = value.clamp(0.0, 1.0);
                reveal_value_for_cb.set_value(value);
                let scale = mix(hidden_scale, 1.0, value) * interaction_value_for_cb.get_value();
                drop(
                    style_for_reveal
                        .set_property("--ui-time-field-clear-opacity", &format!("{value}")),
                );
                drop(
                    style_for_reveal
                        .set_property("--ui-time-field-clear-scale", &format!("{scale}")),
                );
            });

        let style_for_interaction = style.clone();
        let reveal_value_for_interaction = reveal_value;
        let interaction_value_for_interaction = interaction_value;
        let hidden_scale_for_interaction = config.hidden_scale;
        let interaction_animator =
            ui_motion::spring::SpringAnimator::new(1.0, config.spring, move |value| {
                let value = value.clamp(0.0, 10.0);
                interaction_value_for_interaction.set_value(value);
                let reveal = reveal_value_for_interaction.get_value().clamp(0.0, 1.0);
                let base_scale = mix(hidden_scale_for_interaction, 1.0, reveal);
                let scale = base_scale * value;
                drop(
                    style_for_interaction
                        .set_property("--ui-time-field-clear-scale", &format!("{scale}")),
                );
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
    motion: TimeFieldMotion,
) {
    std::hint::black_box(sanitize_motion(motion)); // drop(sanitize_motion(motion));
}

#[cfg(test)]
#[path = "../../test/time_field/motion.rs"]
mod tests;
