#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ProgressMotion {
    pub spring: ui_motion::spring::SpringConfig,
}

impl ProgressMotion {
    pub fn fast() -> Self {
        Self {
            spring: ui_motion::presets::spring_fast(),
        }
    }
}

impl Default for ProgressMotion {
    fn default() -> Self {
        Self {
            spring: ui_motion::presets::spring_soft(),
        }
    }
}

fn sanitize_spring(value: ui_motion::spring::SpringConfig) -> ui_motion::spring::SpringConfig {
    let default = ProgressMotion::default().spring;

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

pub fn sanitize_motion(motion: ProgressMotion) -> ProgressMotion {
    ProgressMotion {
        spring: sanitize_spring(motion.spring),
    }
}

#[cfg(target_arch = "wasm32")]
pub fn attach_motion(
    indicator_ref: leptos::prelude::NodeRef<leptos::html::Div>,
    progress: leptos::prelude::Signal<f64>,
    motion: ProgressMotion,
) {
    use leptos::prelude::*;
    use leptos::wasm_bindgen::JsCast;

    let motion = StoredValue::new(sanitize_motion(motion));
    let spring = StoredValue::new_local(None::<ui_motion::spring::SpringAnimator>);

    Effect::new(move |_| {
        let config = motion.get_value().spring;
        let Some(div) = indicator_ref.get() else {
            return;
        };
        if spring.get_value().is_some() {
            return;
        }

        let element: leptos::web_sys::HtmlElement = div.unchecked_into();
        let style = element.style();

        let _ = style.set_property("--ui-progress-progress", "0");

        let style_for_apply = style.clone();
        let animator = ui_motion::spring::SpringAnimator::new(0.0, config, move |v| {
            let v = v.clamp(0.0, 1.0);
            let _ = style_for_apply.set_property("--ui-progress-progress", &format!("{v}"));
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
        let v = progress.get();
        if let Some(animator) = spring.get_value() {
            animator.set_target(v.clamp(0.0, 1.0));
        }
    });
}

#[cfg(not(target_arch = "wasm32"))]
pub fn attach_motion(
    _indicator_ref: leptos::prelude::NodeRef<leptos::html::Div>,
    _progress: leptos::prelude::Signal<f64>,
    motion: ProgressMotion,
) {
    let _ = sanitize_motion(motion);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_motion_matches_progress_spring_contract() {
        let motion = ProgressMotion::default();
        let expected = ui_motion::presets::spring_soft();

        assert_eq!(motion.spring, expected);
    }

    #[test]
    fn sanitize_motion_falls_back_for_invalid_values() {
        let default = ProgressMotion::default();

        let motion = sanitize_motion(ProgressMotion {
            spring: ui_motion::spring::SpringConfig {
                stiffness: f64::NAN,
                damping: -1.0,
                mass: 0.0,
                precision: f64::INFINITY,
            },
        });

        assert_eq!(motion.spring.stiffness, default.spring.stiffness);
        assert_eq!(motion.spring.damping, default.spring.damping);
        assert_eq!(motion.spring.mass, default.spring.mass);
        assert_eq!(motion.spring.precision, default.spring.precision);
    }

    #[test]
    fn supports_custom_spring_motion_contract() {
        let motion = ProgressMotion {
            spring: ui_motion::spring::SpringConfig {
                stiffness: 310.0,
                damping: 22.0,
                mass: 1.1,
                precision: 0.002,
            },
        };

        assert_eq!(motion.spring.stiffness, 310.0);
        assert_eq!(motion.spring.damping, 22.0);
        assert_eq!(motion.spring.mass, 1.1);
        assert_eq!(motion.spring.precision, 0.002);
    }
}
