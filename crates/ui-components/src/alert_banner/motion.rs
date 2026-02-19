#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AlertBannerMotion {
    pub spring: ui_motion::spring::SpringConfig,
}

impl Default for AlertBannerMotion {
    fn default() -> Self {
        Self {
            spring: ui_motion::spring::SpringConfig {
                stiffness: 260.0,
                damping: 18.0,
                mass: 1.0,
                ..Default::default()
            },
        }
    }
}

fn sanitize_spring(value: ui_motion::spring::SpringConfig) -> ui_motion::spring::SpringConfig {
    let default = AlertBannerMotion::default().spring;

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

pub fn sanitize_motion(motion: AlertBannerMotion) -> AlertBannerMotion {
    AlertBannerMotion {
        spring: sanitize_spring(motion.spring),
    }
}

#[cfg(target_arch = "wasm32")]
pub fn attach_motion(
    node_ref: leptos::prelude::NodeRef<leptos::html::Section>,
    motion: AlertBannerMotion,
) {
    use leptos::prelude::*;
    use leptos::wasm_bindgen::JsCast;

    if ui_motion::web::prefers_reduced_motion() {
        return;
    }

    let motion = StoredValue::new(sanitize_motion(motion));
    let spring = StoredValue::new_local(None::<ui_motion::spring::SpringAnimator>);

    Effect::new(move |_| {
        let config = motion.get_value().spring;

        let Some(section) = node_ref.get() else {
            return;
        };

        if spring.get_value().is_some() {
            return;
        }

        let element: leptos::web_sys::HtmlElement = section.unchecked_into();
        let style = element.style();

        drop(style.set_property("--ui-alert-banner-opacity", "0"));
        drop(style.set_property("--ui-alert-banner-translate-y", "8px"));
        drop(style.set_property("--ui-alert-banner-scale", "0.985"));
        let style_for_apply = style.clone();
        let animator = ui_motion::spring::SpringAnimator::new(0.0, config, move |v| {
            let v = v.clamp(0.0, 1.0);

            let opacity = v;
            let translate_y_px = (1.0 - v) * 8.0;
            let scale = 0.985 + (0.015 * v);

            drop(style_for_apply.set_property("--ui-alert-banner-opacity", &format!("{opacity}")));
            drop(style_for_apply.set_property(
                "--ui-alert-banner-translate-y",
                &format!("{translate_y_px}px"),
            ));
            drop(style_for_apply.set_property("--ui-alert-banner-scale", &format!("{scale}")));
        });

        animator.set_target(1.0);

        let spring_for_cleanup = spring;
        on_cleanup(move || {
            if let Some(animator) = spring_for_cleanup.get_value() {
                animator.stop();
            }
            spring_for_cleanup.set_value(None);
        });

        spring.set_value(Some(animator));
    });
}

#[cfg(not(target_arch = "wasm32"))]
pub fn attach_motion(
    _node_ref: leptos::prelude::NodeRef<leptos::html::Section>,
    motion: AlertBannerMotion,
) {
    std::hint::black_box(sanitize_motion(motion)); // drop(sanitize_motion(motion));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_motion_matches_alert_banner_spring_contract() {
        let motion = AlertBannerMotion::default();

        assert_eq!(
            motion.spring,
            ui_motion::spring::SpringConfig {
                stiffness: 260.0,
                damping: 18.0,
                mass: 1.0,
                ..Default::default()
            }
        );
    }

    #[test]
    fn supports_custom_spring_motion_contract() {
        let motion = AlertBannerMotion {
            spring: ui_motion::spring::SpringConfig {
                stiffness: 284.0,
                damping: 21.0,
                mass: 1.0,
                precision: 0.002,
            },
        };

        assert_eq!(motion.spring.stiffness, 284.0);
        assert_eq!(motion.spring.damping, 21.0);
        assert_eq!(motion.spring.mass, 1.0);
        assert_eq!(motion.spring.precision, 0.002);
    }

    #[test]
    fn sanitize_motion_falls_back_for_invalid_values() {
        let default = AlertBannerMotion::default();

        let motion = sanitize_motion(AlertBannerMotion {
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
}
