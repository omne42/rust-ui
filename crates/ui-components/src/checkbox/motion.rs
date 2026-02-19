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

fn sanitize_number(value: f64, fallback: f64) -> f64 {
    if value.is_finite() { value } else { fallback }
}

fn sanitize_spring(value: ui_motion::spring::SpringConfig) -> ui_motion::spring::SpringConfig {
    let default = CheckboxMotion::default().spring;

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

fn sanitize_indicator_spring(
    value: ui_motion::spring::SpringConfig,
) -> ui_motion::spring::SpringConfig {
    let default = CheckboxMotion::default().indicator_spring;

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

pub fn sanitize_motion(motion: CheckboxMotion) -> CheckboxMotion {
    let default = CheckboxMotion::default();

    CheckboxMotion {
        spring: sanitize_spring(motion.spring),
        hover_scale: sanitize_number(motion.hover_scale, default.hover_scale).clamp(0.5, 2.0),
        tap_scale: sanitize_number(motion.tap_scale, default.tap_scale).clamp(0.5, 1.5),
        indicator_spring: sanitize_indicator_spring(motion.indicator_spring),
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
            drop(style.set_property("--ui-checkbox-scale", &format!("{scale}")));
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
    motion: CheckboxMotion,
) {
    std::hint::black_box(sanitize_motion(motion)); // drop(sanitize_motion(motion));
}

#[cfg(target_arch = "wasm32")]
pub fn attach_indicator_motion(
    node_ref: leptos::prelude::NodeRef<leptos::html::Span>,
    is_checked: leptos::prelude::ReadSignal<bool>,
    motion: CheckboxMotion,
) {
    use leptos::prelude::*;
    use leptos::wasm_bindgen::JsCast;

    let motion = StoredValue::new(sanitize_motion(motion));
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
            drop(style.set_property("--ui-checkbox-indicator", &format!("{value}")));
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
    motion: CheckboxMotion,
) {
    std::hint::black_box(sanitize_motion(motion)); // drop(sanitize_motion(motion));
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

    #[test]
    fn sanitize_motion_falls_back_for_invalid_values() {
        let motion = sanitize_motion(CheckboxMotion {
            spring: ui_motion::spring::SpringConfig {
                stiffness: f64::NAN,
                damping: -1.0,
                mass: 0.0,
                precision: f64::INFINITY,
            },
            hover_scale: f64::NAN,
            tap_scale: f64::NAN,
            indicator_spring: ui_motion::spring::SpringConfig {
                stiffness: f64::NAN,
                damping: -1.0,
                mass: 0.0,
                precision: f64::INFINITY,
            },
        });

        let default = CheckboxMotion::default();
        assert_eq!(motion.spring.stiffness, default.spring.stiffness);
        assert_eq!(motion.spring.damping, default.spring.damping);
        assert_eq!(motion.spring.mass, default.spring.mass);
        assert_eq!(motion.spring.precision, default.spring.precision);
        assert_eq!(motion.hover_scale, default.hover_scale);
        assert_eq!(motion.tap_scale, default.tap_scale);
        assert_eq!(
            motion.indicator_spring.stiffness,
            default.indicator_spring.stiffness
        );
        assert_eq!(
            motion.indicator_spring.damping,
            default.indicator_spring.damping
        );
        assert_eq!(motion.indicator_spring.mass, default.indicator_spring.mass);
        assert_eq!(
            motion.indicator_spring.precision,
            default.indicator_spring.precision
        );
    }

    #[test]
    fn sanitize_motion_clamps_scale_values_and_keeps_valid_springs() {
        let motion = sanitize_motion(CheckboxMotion {
            spring: ui_motion::spring::SpringConfig {
                stiffness: 320.0,
                damping: 20.0,
                mass: 1.1,
                precision: 0.002,
            },
            hover_scale: 5.0,
            tap_scale: -2.0,
            indicator_spring: ui_motion::spring::SpringConfig {
                stiffness: 420.0,
                damping: 24.0,
                mass: 1.2,
                precision: 0.003,
            },
        });

        assert_eq!(motion.spring.stiffness, 320.0);
        assert_eq!(motion.spring.damping, 20.0);
        assert_eq!(motion.spring.mass, 1.1);
        assert_eq!(motion.spring.precision, 0.002);
        assert_eq!(motion.hover_scale, 2.0);
        assert_eq!(motion.tap_scale, 0.5);
        assert_eq!(motion.indicator_spring.stiffness, 420.0);
        assert_eq!(motion.indicator_spring.damping, 24.0);
        assert_eq!(motion.indicator_spring.mass, 1.2);
        assert_eq!(motion.indicator_spring.precision, 0.003);
    }
}

#[cfg(feature = "component-checkbox_group")]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CheckboxGroupMotion {
    pub transition_duration_ms: u16,
}

#[cfg(feature = "component-checkbox_group")]
impl Default for CheckboxGroupMotion {
    fn default() -> Self {
        Self {
            transition_duration_ms: 140,
        }
    }
}

#[cfg(feature = "component-checkbox_group")]
pub fn sanitize_checkbox_group_motion(motion: CheckboxGroupMotion) -> CheckboxGroupMotion {
    CheckboxGroupMotion {
        transition_duration_ms: motion.transition_duration_ms.clamp(60, 1200),
    }
}

#[cfg(feature = "component-checkbox_group")]
pub fn checkbox_group_motion_source_attr(motion: CheckboxGroupMotion) -> &'static str {
    if sanitize_checkbox_group_motion(motion) == CheckboxGroupMotion::default() {
        "default"
    } else {
        "custom"
    }
}

#[cfg(feature = "component-checkbox_group")]
pub fn attach_checkbox_group_motion(
    base_vars: Option<String>,
    motion: CheckboxGroupMotion,
) -> String {
    let motion = sanitize_checkbox_group_motion(motion);
    let mut style = base_vars.unwrap_or_default();

    if !style.trim().is_empty() && !style.trim_end().ends_with(';') {
        style.push(';');
    }

    style.push_str(&format!(
        " --ui-checkbox-group-motion-duration: {}ms;",
        motion.transition_duration_ms
    ));

    style
}

#[cfg(all(test, feature = "component-checkbox_group"))]
mod checkbox_group_tests {
    use super::*;

    #[test]
    fn sanitize_checkbox_group_motion_clamps_duration_to_contract_range() {
        assert_eq!(
            sanitize_checkbox_group_motion(CheckboxGroupMotion {
                transition_duration_ms: 10,
            }),
            CheckboxGroupMotion {
                transition_duration_ms: 60,
            }
        );

        assert_eq!(
            sanitize_checkbox_group_motion(CheckboxGroupMotion {
                transition_duration_ms: 2600,
            }),
            CheckboxGroupMotion {
                transition_duration_ms: 1200,
            }
        );
    }

    #[test]
    fn attach_checkbox_group_motion_appends_css_variable_contract() {
        let style = attach_checkbox_group_motion(
            Some("--ui-local-var: 1".to_string()),
            CheckboxGroupMotion {
                transition_duration_ms: 220,
            },
        );

        assert!(style.contains("--ui-local-var: 1;"));
        assert!(style.contains("--ui-checkbox-group-motion-duration: 220ms;"));
    }
}
