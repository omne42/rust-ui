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

#[cfg(target_arch = "wasm32")]
pub fn attach_motion(
    indicator_ref: leptos::prelude::NodeRef<leptos::html::Div>,
    progress: leptos::prelude::Signal<f64>,
    motion: ProgressMotion,
) {
    use leptos::prelude::*;
    use leptos::wasm_bindgen::JsCast;

    let motion = StoredValue::new(motion);
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
    _motion: ProgressMotion,
) {
}
