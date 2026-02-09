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

    let motion = StoredValue::new(motion);
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

        let _ = style.set_property("--ui-alert-banner-opacity", "0");
        let _ = style.set_property("--ui-alert-banner-translate-y", "8px");
        let _ = style.set_property("--ui-alert-banner-scale", "0.985");

        let style_for_apply = style.clone();
        let animator = ui_motion::spring::SpringAnimator::new(0.0, config, move |v| {
            let v = v.clamp(0.0, 1.0);

            let opacity = v;
            let translate_y_px = (1.0 - v) * 8.0;
            let scale = 0.985 + (0.015 * v);

            let _ =
                style_for_apply.set_property("--ui-alert-banner-opacity", &format!("{opacity}"));
            let _ = style_for_apply.set_property(
                "--ui-alert-banner-translate-y",
                &format!("{translate_y_px}px"),
            );
            let _ = style_for_apply.set_property("--ui-alert-banner-scale", &format!("{scale}"));
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
    _motion: AlertBannerMotion,
) {
}
