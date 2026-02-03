#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SegmentedControlMotion {
    pub spring: ui_motion::spring::SpringConfig,
}

impl Default for SegmentedControlMotion {
    fn default() -> Self {
        Self {
            spring: ui_motion::presets::spring_slide(),
        }
    }
}

#[cfg(target_arch = "wasm32")]
pub fn attach_indicator_motion(
    container_ref: leptos::prelude::NodeRef<leptos::html::Div>,
    indicator_ref: leptos::prelude::NodeRef<leptos::html::Div>,
    active_index: leptos::prelude::ReadSignal<usize>,
    option_id: leptos::prelude::Callback<usize, String>,
    motion: SegmentedControlMotion,
) {
    use leptos::prelude::*;
    use leptos::wasm_bindgen::JsCast;

    let motion = StoredValue::new(motion);
    let springs = StoredValue::new_local(
        None::<(
            ui_motion::spring::SpringAnimator,
            ui_motion::spring::SpringAnimator,
            ui_motion::spring::SpringAnimator,
        )>,
    );

    Effect::new(move |_| {
        let config = motion.get_value().spring;
        let Some(indicator) = indicator_ref.get() else {
            return;
        };
        if springs.get_value().is_some() {
            return;
        }

        let element: leptos::web_sys::HtmlElement = indicator.unchecked_into();
        let style = element.style();

        let _ = style.set_property("--ui-segmented-control-indicator-x", "0px");
        let _ = style.set_property("--ui-segmented-control-indicator-w", "0px");
        let _ = style.set_property("--ui-segmented-control-indicator-o", "0");

        let style_for_x = style.clone();
        let x = ui_motion::spring::SpringAnimator::new(0.0, config, move |v| {
            let v = v.clamp(-10000.0, 10000.0);
            let _ =
                style_for_x.set_property("--ui-segmented-control-indicator-x", &format!("{v}px"));
        });

        let style_for_w = style.clone();
        let w = ui_motion::spring::SpringAnimator::new(0.0, config, move |v| {
            let v = v.clamp(0.0, 10000.0);
            let _ =
                style_for_w.set_property("--ui-segmented-control-indicator-w", &format!("{v}px"));
        });

        let style_for_o = style.clone();
        let o = ui_motion::spring::SpringAnimator::new(0.0, config, move |v| {
            let v = v.clamp(0.0, 1.0);
            let _ = style_for_o.set_property("--ui-segmented-control-indicator-o", &format!("{v}"));
        });

        let springs_for_cleanup = springs;
        on_cleanup(move || {
            if let Some((x, w, o)) = springs_for_cleanup.get_value() {
                x.stop();
                w.stop();
                o.stop();
            }
        });

        springs.set_value(Some((x, w, o)));
    });

    Effect::new(move |_| {
        let active = active_index.get();
        let _ = container_ref.get();

        let Some((x, w, o)) = springs.get_value() else {
            return;
        };

        let Some(container) = container_ref.get_untracked() else {
            return;
        };

        let Some(document) = leptos::web_sys::window().and_then(|w| w.document()) else {
            return;
        };

        let id = option_id.run(active);
        let Some(option_el) = document.get_element_by_id(&id) else {
            o.set_target(0.0);
            return;
        };

        let container_el: &leptos::web_sys::Element = container.as_ref();
        let container_rect = container_el.get_bounding_client_rect();
        let option_rect = option_el.get_bounding_client_rect();

        let target_x = option_rect.left() - container_rect.left();
        let target_w = option_rect.width();

        x.set_target(target_x);
        w.set_target(target_w);
        o.set_target(1.0);
    });
}

#[cfg(not(target_arch = "wasm32"))]
pub fn attach_indicator_motion(
    _container_ref: leptos::prelude::NodeRef<leptos::html::Div>,
    _indicator_ref: leptos::prelude::NodeRef<leptos::html::Div>,
    _active_index: leptos::prelude::ReadSignal<usize>,
    _option_id: leptos::prelude::Callback<usize, String>,
    _motion: SegmentedControlMotion,
) {
}
