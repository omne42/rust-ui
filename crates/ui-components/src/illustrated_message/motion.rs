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

#[cfg(target_arch = "wasm32")]
pub fn attach_motion(
    node_ref: leptos::prelude::NodeRef<leptos::html::Div>,
    motion: IllustratedMessageMotion,
) {
    use leptos::prelude::*;
    use leptos::wasm_bindgen::JsCast;

    let motion = StoredValue::new(motion);
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

        let _ = style.set_property("--ui-im-opacity", "0");
        let _ = style.set_property("--ui-im-y", &format!("{}px", motion.initial_y_px));

        let style_for_apply = style.clone();
        let animator = ui_motion::spring::SpringAnimator::new(0.0, config, move |progress| {
            let progress = progress.clamp(0.0, 1.0);
            let y = motion.initial_y_px * (1.0 - progress);
            let _ = style_for_apply.set_property("--ui-im-opacity", &format!("{progress}"));
            let _ = style_for_apply.set_property("--ui-im-y", &format!("{y}px"));
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
    _motion: IllustratedMessageMotion,
) {
}
