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

fn sanitize_number(value: f64, fallback: f64) -> f64 {
    if value.is_finite() { value } else { fallback }
}

fn sanitize_spring(value: ui_motion::spring::SpringConfig) -> ui_motion::spring::SpringConfig {
    let default = RadioMotion::default().spring;

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

pub fn sanitize_motion(motion: RadioMotion) -> RadioMotion {
    let default = RadioMotion::default();

    RadioMotion {
        spring: sanitize_spring(motion.spring),
        hover_scale: sanitize_number(motion.hover_scale, default.hover_scale).clamp(0.5, 2.0),
        tap_scale: sanitize_number(motion.tap_scale, default.tap_scale).clamp(0.5, 1.5),
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

    if is_disabled || ui_motion::web::prefers_reduced_motion() {
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
            drop(style.set_property("--ui-radio-scale", &format!("{scale}")));
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
    motion: RadioMotion,
) {
    std::hint::black_box(sanitize_motion(motion)); // drop(sanitize_motion(motion));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_motion_has_reasonable_params() {
        let motion = RadioMotion::default();
        assert!(motion.spring.stiffness > 0.0);
        assert!(motion.spring.damping > 0.0);
        assert!(motion.spring.mass > 0.0);
        assert!(motion.hover_scale >= 1.0);
        assert!(motion.tap_scale > 0.0);
        assert!(motion.tap_scale <= 1.0);
    }

    #[test]
    fn sanitize_motion_falls_back_for_invalid_values() {
        let motion = sanitize_motion(RadioMotion {
            spring: ui_motion::spring::SpringConfig {
                stiffness: f64::NAN,
                damping: -1.0,
                mass: 0.0,
                precision: f64::INFINITY,
            },
            hover_scale: f64::NAN,
            tap_scale: f64::NAN,
        });

        let default = RadioMotion::default();
        assert_eq!(motion.spring.stiffness, default.spring.stiffness);
        assert_eq!(motion.spring.damping, default.spring.damping);
        assert_eq!(motion.spring.mass, default.spring.mass);
        assert_eq!(motion.spring.precision, default.spring.precision);
        assert_eq!(motion.hover_scale, default.hover_scale);
        assert_eq!(motion.tap_scale, default.tap_scale);
    }

    #[test]
    fn sanitize_motion_clamps_scale_values() {
        let motion = sanitize_motion(RadioMotion {
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
}
