#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AlertMotion {
    pub spring: ui_motion::spring::SpringConfig,
}

impl Default for AlertMotion {
    fn default() -> Self {
        Self {
            spring: ui_motion::presets::spring_soft(),
        }
    }
}

fn sanitize_spring(value: ui_motion::spring::SpringConfig) -> ui_motion::spring::SpringConfig {
    let default = AlertMotion::default().spring;

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

pub fn sanitize_motion(motion: AlertMotion) -> AlertMotion {
    AlertMotion {
        spring: sanitize_spring(motion.spring),
    }
}

#[cfg(target_arch = "wasm32")]
pub fn attach_motion(
    node_ref: leptos::prelude::NodeRef<leptos::html::Section>,
    motion: AlertMotion,
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
        let is_inline = element
            .get_attribute("data-layout")
            .is_some_and(|layout| layout == "inline");
        let translate_y_start = if is_inline { 6.0 } else { 8.0 };
        let scale_start = if is_inline { 0.98 } else { 0.985 };
        let scale_delta = if is_inline { 0.02 } else { 0.015 };

        ui_observability::set_css_property_observed_auto!(&(style), "--ui-alert-opacity", "0");
        ui_observability::set_css_property_observed_auto!(
            &(style),
            "--ui-alert-translate-y",
            &format!("{translate_y_start}px")
        );
        ui_observability::set_css_property_observed_auto!(
            &(style),
            "--ui-alert-scale",
            &format!("{scale_start}")
        );
        let style_for_apply = style.clone();
        let animator = ui_motion::spring::SpringAnimator::new(0.0, config, move |v| {
            let v = v.clamp(0.0, 1.0);

            let opacity = v;
            let translate_y_px = (1.0 - v) * translate_y_start;
            let scale = scale_start + (scale_delta * v);

            ui_observability::set_css_property_observed_auto!(
                &(style_for_apply),
                "--ui-alert-opacity",
                &format!("{opacity}")
            );
            ui_observability::set_css_property_observed_auto!(
                &(style_for_apply),
                "--ui-alert-translate-y",
                &format!("{translate_y_px}px")
            );
            ui_observability::set_css_property_observed_auto!(
                &(style_for_apply),
                "--ui-alert-scale",
                &format!("{scale}")
            );
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
    motion: AlertMotion,
) {
    std::hint::black_box(sanitize_motion(motion)); // drop(sanitize_motion(motion));
}

#[cfg(test)]
#[path = "../test/motion.rs"]
mod tests;
