use leptos::{html, prelude::*};
use std::sync::Arc;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TabsMotion {
    pub spring: ui_motion::spring::SpringConfig,
}

impl Default for TabsMotion {
    fn default() -> Self {
        Self {
            spring: ui_motion::presets::spring_slide(),
        }
    }
}

#[cfg(target_arch = "wasm32")]
pub fn attach_motion(
    list_ref: NodeRef<html::Div>,
    indicator_ref: NodeRef<html::Div>,
    tab_refs: Arc<Vec<NodeRef<html::Button>>>,
    selected_index: Signal<usize>,
    motion: TabsMotion,
) {
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

        let _ = style.set_property("--ui-tabs-indicator-x", "0px");
        let _ = style.set_property("--ui-tabs-indicator-w", "0px");
        let _ = style.set_property("--ui-tabs-indicator-o", "0");

        let style_for_x = style.clone();
        let x = ui_motion::spring::SpringAnimator::new(0.0, config, move |v| {
            let v = v.clamp(-10000.0, 10000.0);
            let _ = style_for_x.set_property("--ui-tabs-indicator-x", &format!("{v}px"));
        });

        let style_for_w = style.clone();
        let w = ui_motion::spring::SpringAnimator::new(0.0, config, move |v| {
            let v = v.clamp(0.0, 10000.0);
            let _ = style_for_w.set_property("--ui-tabs-indicator-w", &format!("{v}px"));
        });

        let style_for_o = style.clone();
        let o = ui_motion::spring::SpringAnimator::new(0.0, config, move |v| {
            let v = v.clamp(0.0, 1.0);
            let _ = style_for_o.set_property("--ui-tabs-indicator-o", &format!("{v}"));
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
        let selected = selected_index.get();
        let _ = list_ref.get();
        let _ = indicator_ref.get();

        let Some((x, w, o)) = springs.get_value() else {
            return;
        };

        let Some(tab_ref) = tab_refs.get(selected) else {
            o.set_target(0.0);
            return;
        };
        let Some(tab) = tab_ref.get_untracked() else {
            o.set_target(0.0);
            return;
        };

        let element: leptos::web_sys::HtmlElement = tab.unchecked_into();

        // The indicator is positioned with `left: 4px` to match the tab list padding, so we
        // subtract the same amount from `offsetLeft` (relative to the padding edge).
        let target_x = (element.offset_left() as f64) - 4.0;
        let target_w = element.offset_width() as f64;

        x.set_target(target_x);
        w.set_target(target_w);
        o.set_target(1.0);
    });
}

#[cfg(not(target_arch = "wasm32"))]
pub fn attach_motion(
    _list_ref: NodeRef<html::Div>,
    _indicator_ref: NodeRef<html::Div>,
    _tab_refs: Arc<Vec<NodeRef<html::Button>>>,
    _selected_index: Signal<usize>,
    _motion: TabsMotion,
) {
}
