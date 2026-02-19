use ui_theme::default_button_motion_tokens;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ButtonMotion {
    pub spring: ui_motion::spring::SpringConfig,
    pub hover_scale: f64,
    pub tap_scale: f64,
}

impl Default for ButtonMotion {
    fn default() -> Self {
        let tokens = default_button_motion_tokens();
        Self {
            spring: ui_motion::spring::SpringConfig {
                stiffness: tokens.spring.stiffness,
                damping: tokens.spring.damping,
                mass: tokens.spring.mass,
                precision: tokens.spring.precision,
            },
            hover_scale: tokens.hover_scale,
            tap_scale: tokens.tap_scale,
        }
    }
}

fn sanitize_number(value: f64, fallback: f64) -> f64 {
    if value.is_finite() { value } else { fallback }
}

pub fn sanitize_hover_scale_with_fallback(value: f64, fallback: f64) -> f64 {
    sanitize_number(value, fallback).clamp(0.5, 2.0)
}

pub fn sanitize_tap_scale_with_fallback(value: f64, fallback: f64) -> f64 {
    sanitize_number(value, fallback).clamp(0.5, 1.5)
}

pub fn sanitize_spring_with_fallback(
    value: ui_motion::spring::SpringConfig,
    fallback: ui_motion::spring::SpringConfig,
) -> ui_motion::spring::SpringConfig {
    ui_motion::spring::SpringConfig {
        stiffness: if value.stiffness.is_finite() && value.stiffness > 0.0 {
            value.stiffness
        } else {
            fallback.stiffness
        },
        damping: if value.damping.is_finite() && value.damping > 0.0 {
            value.damping
        } else {
            fallback.damping
        },
        mass: if value.mass.is_finite() && value.mass > 0.0 {
            value.mass
        } else {
            fallback.mass
        },
        precision: if value.precision.is_finite() && value.precision > 0.0 {
            value.precision
        } else {
            fallback.precision
        },
    }
}

fn sanitize_spring(value: ui_motion::spring::SpringConfig) -> ui_motion::spring::SpringConfig {
    sanitize_spring_with_fallback(value, ButtonMotion::default().spring)
}

pub fn sanitize_motion(motion: ButtonMotion) -> ButtonMotion {
    let default = ButtonMotion::default();

    ButtonMotion {
        spring: sanitize_spring(motion.spring),
        hover_scale: sanitize_hover_scale_with_fallback(motion.hover_scale, default.hover_scale),
        tap_scale: sanitize_tap_scale_with_fallback(motion.tap_scale, default.tap_scale),
    }
}

#[cfg(target_arch = "wasm32")]
pub fn attach_motion(
    node_ref: leptos::prelude::NodeRef<leptos::html::Button>,
    is_hovered: leptos::prelude::ReadSignal<bool>,
    is_pressed: leptos::prelude::ReadSignal<bool>,
    is_disabled: bool,
    motion: ButtonMotion,
) {
    use crate::observability::warn_js_error;
    use leptos::prelude::*;
    use leptos::wasm_bindgen::JsCast;

    if is_disabled {
        return;
    }

    let motion = StoredValue::new(sanitize_motion(motion));
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
            if let Err(error) = style.set_property("--ui-button-scale", &format!("{scale}")) {
                warn_js_error("button.motion.scale", &error);
            }
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
    motion: ButtonMotion,
) {
    std::hint::black_box(sanitize_motion(motion)); // drop(sanitize_motion(motion));
}

#[cfg(feature = "component-button_group")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ButtonGroupMotion {
    pub spring: ui_motion::spring::SpringConfig,
    pub enter_scale: f64,
}

#[cfg(feature = "component-button_group")]
impl Default for ButtonGroupMotion {
    fn default() -> Self {
        Self {
            spring: ui_motion::spring::SpringConfig {
                stiffness: 240.0,
                damping: 20.0,
                mass: 1.0,
                ..Default::default()
            },
            enter_scale: 0.985,
        }
    }
}

#[cfg(feature = "component-button_group")]
pub fn sanitize_button_group_motion(motion: ButtonGroupMotion) -> ButtonGroupMotion {
    let default = ButtonGroupMotion::default();

    ButtonGroupMotion {
        spring: sanitize_spring_with_fallback(motion.spring, default.spring),
        enter_scale: sanitize_number(motion.enter_scale, default.enter_scale).clamp(0.5, 1.5),
    }
}

#[cfg(all(feature = "component-button_group", target_arch = "wasm32"))]
pub fn attach_button_group_motion(
    node_ref: leptos::prelude::NodeRef<leptos::html::Div>,
    motion: ButtonGroupMotion,
) {
    use crate::observability::set_css_property_observed;
    use leptos::prelude::*;
    use leptos::wasm_bindgen::JsCast;

    let motion = StoredValue::new(sanitize_button_group_motion(motion));
    let initialized = StoredValue::new(false);
    let spring = StoredValue::new_local(None::<ui_motion::spring::SpringAnimator>);

    Effect::new(move |_| {
        if initialized.get_value() {
            return;
        }

        let Some(node) = node_ref.get() else {
            return;
        };
        initialized.set_value(true);

        let motion = motion.get_value();
        let element: leptos::web_sys::HtmlElement = node.unchecked_into();
        let style = element.style();

        set_css_property_observed(
            &style,
            "--ui-button-group-scale",
            &format!("{}", motion.enter_scale),
            "button.group.motion.initial_scale",
        );

        let animator = ui_motion::spring::SpringAnimator::new(
            motion.enter_scale,
            motion.spring,
            move |scale| {
                let scale = scale.clamp(0.0, 10.0);
                set_css_property_observed(
                    &style,
                    "--ui-button-group-scale",
                    &format!("{scale}"),
                    "button.group.motion.scale",
                );
            },
        );
        animator.set_target(1.0);

        let spring_for_cleanup = spring;
        on_cleanup(move || {
            if let Some(animator) = spring_for_cleanup.get_value() {
                animator.stop();
            }
        });

        spring.set_value(Some(animator));
    });
}

#[cfg(all(feature = "component-button_group", not(target_arch = "wasm32")))]
pub fn attach_button_group_motion(
    _node_ref: leptos::prelude::NodeRef<leptos::html::Div>,
    motion: ButtonGroupMotion,
) {
    let _sanitized_motion = sanitize_button_group_motion(motion);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_motion_matches_bb_params() {
        let motion = ButtonMotion::default();
        let tokens = default_button_motion_tokens();
        assert_eq!(
            motion.spring,
            ui_motion::spring::SpringConfig {
                stiffness: tokens.spring.stiffness,
                damping: tokens.spring.damping,
                mass: tokens.spring.mass,
                precision: tokens.spring.precision,
            }
        );
        assert_eq!(motion.hover_scale, tokens.hover_scale);
        assert_eq!(motion.tap_scale, tokens.tap_scale);
    }

    #[test]
    fn sanitize_motion_falls_back_for_invalid_values() {
        let motion = sanitize_motion(ButtonMotion {
            spring: ui_motion::spring::SpringConfig {
                stiffness: f64::NAN,
                damping: -1.0,
                mass: 0.0,
                precision: f64::INFINITY,
            },
            hover_scale: f64::NAN,
            tap_scale: f64::NAN,
        });

        let default = ButtonMotion::default();
        assert_eq!(motion.spring.stiffness, default.spring.stiffness);
        assert_eq!(motion.spring.damping, default.spring.damping);
        assert_eq!(motion.spring.mass, default.spring.mass);
        assert_eq!(motion.spring.precision, default.spring.precision);
        assert_eq!(motion.hover_scale, default.hover_scale);
        assert_eq!(motion.tap_scale, default.tap_scale);
    }

    #[test]
    fn sanitize_motion_clamps_scale_values() {
        let motion = sanitize_motion(ButtonMotion {
            spring: ui_motion::spring::SpringConfig {
                stiffness: 320.0,
                damping: 20.0,
                mass: 1.1,
                precision: 0.002,
            },
            hover_scale: 5.0,
            tap_scale: -2.0,
        });

        assert_eq!(motion.spring.stiffness, 320.0);
        assert_eq!(motion.spring.damping, 20.0);
        assert_eq!(motion.spring.mass, 1.1);
        assert_eq!(motion.spring.precision, 0.002);
        assert_eq!(motion.hover_scale, 2.0);
        assert_eq!(motion.tap_scale, 0.5);
    }

    #[cfg(feature = "component-button_group")]
    #[test]
    fn default_button_group_motion_matches_spring_contract() {
        let motion = ButtonGroupMotion::default();
        assert!(motion.spring.stiffness > 0.0);
        assert!(motion.spring.damping > 0.0);
        assert!(motion.spring.mass > 0.0);
        assert!(motion.enter_scale > 0.0);
        assert!(motion.enter_scale <= 1.0);
    }

    #[cfg(feature = "component-button_group")]
    #[test]
    fn sanitize_button_group_motion_keeps_valid_values() {
        let motion = sanitize_button_group_motion(ButtonGroupMotion {
            spring: ui_motion::spring::SpringConfig {
                stiffness: 300.0,
                damping: 22.0,
                mass: 1.2,
                precision: 0.002,
            },
            enter_scale: 1.08,
        });

        assert_eq!(motion.spring.stiffness, 300.0);
        assert_eq!(motion.spring.damping, 22.0);
        assert_eq!(motion.spring.mass, 1.2);
        assert_eq!(motion.spring.precision, 0.002);
        assert_eq!(motion.enter_scale, 1.08);
    }

    #[cfg(feature = "component-button_group")]
    #[test]
    fn sanitize_button_group_motion_falls_back_for_invalid_values() {
        let motion = sanitize_button_group_motion(ButtonGroupMotion {
            spring: ui_motion::spring::SpringConfig {
                stiffness: f64::NAN,
                damping: -1.0,
                mass: 0.0,
                precision: f64::INFINITY,
            },
            enter_scale: f64::NAN,
        });

        let default = ButtonGroupMotion::default();
        assert_eq!(motion.spring.stiffness, default.spring.stiffness);
        assert_eq!(motion.spring.damping, default.spring.damping);
        assert_eq!(motion.spring.mass, default.spring.mass);
        assert_eq!(motion.spring.precision, default.spring.precision);
        assert_eq!(motion.enter_scale, default.enter_scale);
    }
}
