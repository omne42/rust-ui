use ui_theme::default_slider_motion_tokens;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SliderMotion {
    pub enabled: bool,
    pub spring: ui_motion::spring::SpringConfig,
}

impl SliderMotion {
    pub fn disabled() -> Self {
        Self {
            enabled: false,
            ..Self::default()
        }
    }
}

impl Default for SliderMotion {
    fn default() -> Self {
        let tokens = default_slider_motion_tokens();
        Self {
            enabled: true,
            spring: ui_motion::spring::SpringConfig {
                stiffness: tokens.spring.stiffness,
                damping: tokens.spring.damping,
                mass: tokens.spring.mass,
                precision: tokens.spring.precision,
            },
        }
    }
}

pub fn sanitize_percent(percent: f64) -> f64 {
    if percent.is_finite() {
        percent.clamp(0.0, 100.0)
    } else {
        0.0
    }
}

fn sanitize_spring(value: ui_motion::spring::SpringConfig) -> ui_motion::spring::SpringConfig {
    ui_motion::spring::sanitize_config(value, SliderMotion::default().spring)
}

pub fn sanitize_motion(motion: SliderMotion) -> SliderMotion {
    SliderMotion {
        enabled: motion.enabled,
        spring: sanitize_spring(motion.spring),
    }
}

#[cfg(target_arch = "wasm32")]
pub fn attach_motion(
    root_ref: leptos::prelude::NodeRef<leptos::html::Div>,
    visual_percent: leptos::prelude::Signal<f64>,
    motion: SliderMotion,
) {
    use leptos::prelude::*;
    use leptos::wasm_bindgen::JsCast;

    let motion = StoredValue::new(sanitize_motion(motion));
    let spring = StoredValue::new_local(None::<ui_motion::spring::SpringAnimator>);

    Effect::new(move |_| {
        let Some(root) = root_ref.get() else {
            return;
        };

        if spring.get_value().is_some() {
            return;
        }

        let element: leptos::web_sys::HtmlElement = root.unchecked_into();
        let style = element.style();
        let initial = sanitize_percent(visual_percent.get());

        let _ = style.set_property("--ui-slider-visual-percent", &format!("{initial:.4}"));

        let motion = motion.get_value();
        if !motion.enabled || ui_motion::web::prefers_reduced_motion() {
            return;
        }

        let animator = ui_motion::spring::SpringAnimator::new(initial, motion.spring, move |v| {
            let v = sanitize_percent(v);
            let _ = style.set_property("--ui-slider-visual-percent", &format!("{v:.4}"));
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
        let Some(root) = root_ref.get() else {
            return;
        };

        let element: leptos::web_sys::HtmlElement = root.unchecked_into();
        let style = element.style();

        let target = sanitize_percent(visual_percent.get());
        let motion = motion.get_value();

        if !motion.enabled || ui_motion::web::prefers_reduced_motion() {
            let _ = style.set_property("--ui-slider-visual-percent", &format!("{target:.4}"));
            return;
        }

        if let Some(spring) = spring.get_value() {
            spring.set_target(target);
        }
    });
}

#[cfg(not(target_arch = "wasm32"))]
pub fn attach_motion(
    _root_ref: leptos::prelude::NodeRef<leptos::html::Div>,
    _visual_percent: leptos::prelude::Signal<f64>,
    _motion: SliderMotion,
) {
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanitize_percent_clamps_and_handles_nan() {
        assert_eq!(sanitize_percent(42.0), 42.0);
        assert_eq!(sanitize_percent(-2.0), 0.0);
        assert_eq!(sanitize_percent(140.0), 100.0);
        assert_eq!(sanitize_percent(f64::NAN), 0.0);
    }

    #[test]
    fn sanitize_motion_falls_back_for_invalid_spring_values() {
        let motion = sanitize_motion(SliderMotion {
            enabled: true,
            spring: ui_motion::spring::SpringConfig {
                stiffness: f64::NAN,
                damping: -1.0,
                mass: 0.0,
                precision: f64::INFINITY,
            },
        });

        let default = SliderMotion::default().spring;
        assert_eq!(motion.spring.stiffness, default.stiffness);
        assert_eq!(motion.spring.damping, default.damping);
        assert_eq!(motion.spring.mass, default.mass);
        assert_eq!(motion.spring.precision, default.precision);
    }

    #[test]
    fn disabled_constructor_turns_motion_off() {
        assert!(!SliderMotion::disabled().enabled);
    }
}
