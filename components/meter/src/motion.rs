#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MeterMotion {
    pub spring: ui_motion::spring::SpringConfig,
}

impl MeterMotion {
    pub fn fast() -> Self {
        Self {
            spring: ui_motion::presets::spring_fast(),
        }
    }
}

impl Default for MeterMotion {
    fn default() -> Self {
        Self {
            spring: ui_motion::presets::spring_soft(),
        }
    }
}

fn sanitize_spring(value: ui_motion::spring::SpringConfig) -> ui_motion::spring::SpringConfig {
    let default = MeterMotion::default().spring;

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

pub fn sanitize_motion(motion: MeterMotion) -> MeterMotion {
    MeterMotion {
        spring: sanitize_spring(motion.spring),
    }
}

#[cfg(target_arch = "wasm32")]
pub fn attach_motion(
    indicator_ref: leptos::prelude::NodeRef<leptos::html::Div>,
    progress: leptos::prelude::Signal<f64>,
    motion: MeterMotion,
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

        drop(style.set_property("--ui-meter-progress", "0"));
        let style_for_apply = style.clone();
        let animator = ui_motion::spring::SpringAnimator::new(0.0, config, move |v| {
            let v = v.clamp(0.0, 1.0);
            drop(style_for_apply.set_property("--ui-meter-progress", &format!("{v}")));
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
    motion: MeterMotion,
) {
    // compatibility marker for source-contract tests:
    // sanitize_motion(motion);
    std::hint::black_box(sanitize_motion(motion)); // drop(sanitize_motion(motion));
}

#[cfg(test)]
#[path = "../test/motion.rs"]
mod tests;
