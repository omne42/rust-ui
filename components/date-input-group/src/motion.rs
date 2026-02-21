#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DateInputGroupMotion {
    pub spring: ui_motion::spring::SpringConfig,
    pub enter_scale: f64,
}

const DEFAULT_ENTER_SCALE: f64 = 0.99;

impl Default for DateInputGroupMotion {
    fn default() -> Self {
        Self {
            spring: ui_motion::presets::spring_soft(),
            enter_scale: DEFAULT_ENTER_SCALE,
        }
    }
}

fn sanitize_number(value: f64, fallback: f64) -> f64 {
    if value.is_finite() { value } else { fallback }
}

fn sanitize_spring(value: ui_motion::spring::SpringConfig) -> ui_motion::spring::SpringConfig {
    let default = DateInputGroupMotion::default().spring;
    ui_motion::spring::sanitize_config(value, default)
}

pub fn sanitize_motion(motion: DateInputGroupMotion) -> DateInputGroupMotion {
    let default = DateInputGroupMotion::default();

    DateInputGroupMotion {
        spring: sanitize_spring(motion.spring),
        enter_scale: sanitize_number(motion.enter_scale, default.enter_scale).clamp(0.5, 1.5),
    }
}

#[cfg(target_arch = "wasm32")]
pub fn attach_motion(
    node_ref: leptos::prelude::NodeRef<leptos::html::Div>,
    motion: DateInputGroupMotion,
) {
    use leptos::prelude::*;
    use leptos::wasm_bindgen::JsCast;

    let motion = StoredValue::new(sanitize_motion(motion));
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

        if ui_motion::web::prefers_reduced_motion() {
            ui_observability::set_css_property_observed_auto!(
                &(style),
                "--ui-date-input-group-scale",
                "1"
            );
            return;
        }

        ui_observability::set_css_property_observed_auto!(
            &(style),
            "--ui-date-input-group-scale",
            &format!("{}", motion.enter_scale),
        );

        let animator = ui_motion::spring::SpringAnimator::new(
            motion.enter_scale,
            motion.spring,
            move |scale| {
                let scale = scale.clamp(0.0, 10.0);
                ui_observability::set_css_property_observed_auto!(
                    &(style),
                    "--ui-date-input-group-scale",
                    &format!("{scale}")
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

#[cfg(not(target_arch = "wasm32"))]
pub fn attach_motion(
    _node_ref: leptos::prelude::NodeRef<leptos::html::Div>,
    motion: DateInputGroupMotion,
) {
    std::hint::black_box(sanitize_motion(motion)); // drop(sanitize_motion(motion));
}

#[cfg(test)]
#[path = "../test/motion.rs"]
mod tests;
