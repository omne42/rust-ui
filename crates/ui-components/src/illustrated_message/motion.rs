#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IllustratedMessageMotion {
    pub spring: ui_motion::spring::SpringConfig,
    pub initial_y_px: f64,
}

impl Default for IllustratedMessageMotion {
    fn default() -> Self {
        Self {
            spring: ui_motion::presets::spring_soft(),
            initial_y_px: 8.0,
        }
    }
}

fn sanitize_number(value: f64, fallback: f64) -> f64 {
    if value.is_finite() { value } else { fallback }
}

fn sanitize_spring(value: ui_motion::spring::SpringConfig) -> ui_motion::spring::SpringConfig {
    let default = IllustratedMessageMotion::default().spring;

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

pub fn sanitize_motion(motion: IllustratedMessageMotion) -> IllustratedMessageMotion {
    let default = IllustratedMessageMotion::default();

    IllustratedMessageMotion {
        spring: sanitize_spring(motion.spring),
        initial_y_px: sanitize_number(motion.initial_y_px, default.initial_y_px)
            .abs()
            .clamp(0.0, 120.0),
    }
}

#[cfg(target_arch = "wasm32")]
pub fn attach_motion(
    node_ref: leptos::prelude::NodeRef<leptos::html::Div>,
    motion: IllustratedMessageMotion,
) {
    use leptos::prelude::*;
    use leptos::wasm_bindgen::JsCast;

    let motion = StoredValue::new(sanitize_motion(motion));
    let spring = StoredValue::new_local(None::<ui_motion::spring::SpringAnimator>);

    Effect::new(move |_| {
        let config = motion.get_value().spring;
        let Some(div) = node_ref.get() else {
            return;
        };
        if spring.get_value().is_some() {
            return;
        }

        let element: leptos::web_sys::HtmlElement = div.unchecked_into();
        let style = element.style();
        let motion = motion.get_value();

        drop(style.set_property("--ui-im-opacity", "0"));
        drop(style.set_property("--ui-im-y", &format!("{}px", motion.initial_y_px)));
        let style_for_apply = style.clone();
        let animator = ui_motion::spring::SpringAnimator::new(0.0, config, move |progress| {
            let progress = progress.clamp(0.0, 1.0);
            let y = motion.initial_y_px * (1.0 - progress);
            drop(style_for_apply.set_property("--ui-im-opacity", &format!("{progress}")));
            drop(style_for_apply.set_property("--ui-im-y", &format!("{y}px")));
        });

        let spring_for_cleanup = spring;
        on_cleanup(move || {
            if let Some(animator) = spring_for_cleanup.get_value() {
                animator.stop();
            }
        });

        animator.set_target(1.0);
        spring.set_value(Some(animator));
    });
}

#[cfg(not(target_arch = "wasm32"))]
pub fn attach_motion(
    _node_ref: leptos::prelude::NodeRef<leptos::html::Div>,
    motion: IllustratedMessageMotion,
) {
    std::hint::black_box(sanitize_motion(motion)); // drop(sanitize_motion(motion));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanitize_motion_falls_back_for_invalid_values() {
        let default = IllustratedMessageMotion::default();

        let motion = sanitize_motion(IllustratedMessageMotion {
            spring: ui_motion::spring::SpringConfig {
                stiffness: f64::NAN,
                damping: -1.0,
                mass: 0.0,
                precision: f64::INFINITY,
            },
            initial_y_px: f64::NAN,
        });

        assert_eq!(motion.spring.stiffness, default.spring.stiffness);
        assert_eq!(motion.spring.damping, default.spring.damping);
        assert_eq!(motion.spring.mass, default.spring.mass);
        assert_eq!(motion.spring.precision, default.spring.precision);
        assert_eq!(motion.initial_y_px, default.initial_y_px);

        let capped = sanitize_motion(IllustratedMessageMotion {
            initial_y_px: -999.0,
            ..IllustratedMessageMotion::default()
        });
        assert_eq!(capped.initial_y_px, 120.0);
    }

    #[test]
    fn default_motion_has_reasonable_params() {
        let motion = IllustratedMessageMotion::default();
        assert_eq!(motion.spring, ui_motion::presets::spring_soft());
        assert!(motion.initial_y_px.abs() > 0.0);
    }
}
