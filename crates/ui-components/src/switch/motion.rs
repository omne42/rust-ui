#[cfg(target_arch = "wasm32")]
use crate::switch::logic::{THUMB_WIDTH_PX, checked_thumb_x_px};
use ui_theme::default_switch_motion_tokens;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SwitchMotion {
    pub spring: ui_motion::spring::SpringConfig,
}

impl Default for SwitchMotion {
    fn default() -> Self {
        let tokens = default_switch_motion_tokens();
        Self {
            spring: ui_motion::spring::SpringConfig {
                stiffness: tokens.spring.stiffness,
                damping: tokens.spring.damping,
                mass: tokens.spring.mass,
                precision: tokens.spring.precision,
            },
        }
    }
}

pub fn default_pressed_width_px() -> f64 {
    default_switch_motion_tokens().pressed_width_default_px
}

fn sanitize_number(value: f64, fallback: f64) -> f64 {
    if value.is_finite() { value } else { fallback }
}

fn sanitize_spring(value: ui_motion::spring::SpringConfig) -> ui_motion::spring::SpringConfig {
    ui_motion::spring::sanitize_config(value, SwitchMotion::default().spring)
}

pub fn sanitize_motion(motion: SwitchMotion) -> SwitchMotion {
    SwitchMotion {
        spring: sanitize_spring(motion.spring),
    }
}

fn sanitize_pressed_width_px(value: f64) -> f64 {
    let tokens = default_switch_motion_tokens();
    sanitize_number(value, tokens.pressed_width_default_px)
        .clamp(tokens.pressed_width_min_px, tokens.pressed_width_max_px)
}

#[cfg(target_arch = "wasm32")]
pub fn attach_thumb_motion(
    node_ref: leptos::prelude::NodeRef<leptos::html::Span>,
    is_checked: leptos::prelude::ReadSignal<bool>,
    is_pressed: leptos::prelude::ReadSignal<bool>,
    pressed_width_px: f64,
    motion: SwitchMotion,
) {
    use leptos::prelude::*;
    use leptos::wasm_bindgen::JsCast;

    let motion = StoredValue::new(sanitize_motion(motion));
    let pressed_width_px = sanitize_pressed_width_px(pressed_width_px);
    let last_state = StoredValue::new(None::<(bool, bool)>);
    let springs = StoredValue::new_local(
        None::<(
            ui_motion::spring::SpringAnimator,
            ui_motion::spring::SpringAnimator,
        )>,
    );

    Effect::new(move |_| {
        let config = motion.get_value().spring;
        let Some(span) = node_ref.get() else {
            return;
        };
        if springs.get_value().is_some() {
            return;
        }

        let element: leptos::web_sys::HtmlElement = span.unchecked_into();
        let style = element.style();

        let checked = is_checked.get_untracked();
        let pressed = is_pressed.get_untracked();
        let initial_width = if pressed {
            pressed_width_px
        } else {
            THUMB_WIDTH_PX
        };
        let initial_x = if checked {
            checked_thumb_x_px(initial_width)
        } else {
            0.0
        };

        drop(style.set_property("--ui-switch-thumb-width", &format!("{initial_width}px")));
        drop(style.set_property("--ui-switch-thumb-x", &format!("{initial_x}px")));
        let style_for_width = style.clone();
        let width_anim =
            ui_motion::spring::SpringAnimator::new(initial_width, config, move |value| {
                let value = value.clamp(0.0, 1000.0);
                drop(
                    style_for_width.set_property("--ui-switch-thumb-width", &format!("{value}px")),
                );
            });

        let style_for_x = style.clone();
        let x_anim = ui_motion::spring::SpringAnimator::new(initial_x, config, move |value| {
            let value = value.clamp(-1000.0, 1000.0);
            drop(style_for_x.set_property("--ui-switch-thumb-x", &format!("{value}px")));
        });

        let springs_for_cleanup = springs;
        on_cleanup(move || {
            if let Some((width, x)) = springs_for_cleanup.get_value() {
                width.stop();
                x.stop();
            }
        });

        springs.set_value(Some((width_anim, x_anim)));
    });

    Effect::new(move |_| {
        let checked = is_checked.get();
        let pressed = is_pressed.get();

        let Some(prev) = last_state.get_value() else {
            last_state.set_value(Some((checked, pressed)));
            return;
        };
        if prev == (checked, pressed) {
            return;
        }
        last_state.set_value(Some((checked, pressed)));

        let Some((width, x)) = springs.get_value() else {
            return;
        };

        let target_width = if pressed {
            pressed_width_px
        } else {
            THUMB_WIDTH_PX
        };
        let target_x = if checked {
            checked_thumb_x_px(target_width)
        } else {
            0.0
        };

        width.set_target(target_width);
        x.set_target(target_x);
    });
}

#[cfg(not(target_arch = "wasm32"))]
pub fn attach_thumb_motion(
    _node_ref: leptos::prelude::NodeRef<leptos::html::Span>,
    _is_checked: leptos::prelude::ReadSignal<bool>,
    _is_pressed: leptos::prelude::ReadSignal<bool>,
    pressed_width_px: f64,
    motion: SwitchMotion,
) {
    std::hint::black_box(sanitize_motion(motion)); // drop(sanitize_motion(motion));
    sanitize_pressed_width_px(pressed_width_px);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_motion_has_reasonable_params() {
        let motion = SwitchMotion::default();
        let tokens = default_switch_motion_tokens();
        assert_eq!(
            motion.spring,
            ui_motion::spring::SpringConfig {
                stiffness: tokens.spring.stiffness,
                damping: tokens.spring.damping,
                mass: tokens.spring.mass,
                precision: tokens.spring.precision,
            }
        );
    }

    #[test]
    fn sanitize_motion_falls_back_for_invalid_values() {
        let motion = sanitize_motion(SwitchMotion {
            spring: ui_motion::spring::SpringConfig {
                stiffness: f64::NAN,
                damping: -1.0,
                mass: 0.0,
                precision: f64::INFINITY,
            },
        });

        let default = SwitchMotion::default();
        assert_eq!(motion.spring.stiffness, default.spring.stiffness);
        assert_eq!(motion.spring.damping, default.spring.damping);
        assert_eq!(motion.spring.mass, default.spring.mass);
        assert_eq!(motion.spring.precision, default.spring.precision);
    }

    #[test]
    fn sanitize_pressed_width_clamps_and_uses_fallback() {
        let tokens = default_switch_motion_tokens();
        assert_eq!(sanitize_pressed_width_px(24.0), 24.0);
        assert_eq!(sanitize_pressed_width_px(4.0), tokens.pressed_width_min_px);
        assert_eq!(
            sanitize_pressed_width_px(500.0),
            tokens.pressed_width_max_px
        );
        assert_eq!(
            sanitize_pressed_width_px(f64::NAN),
            tokens.pressed_width_default_px
        );
    }
}
