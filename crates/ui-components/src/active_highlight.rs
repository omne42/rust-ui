use leptos::{html, prelude::*};

pub const CSS: &str = r#"
.ui-active-highlight {
  position: absolute;
  left: 4px;
  right: 4px;
  top: 0;
  height: var(--ui-active-highlight-h, 0px);
  transform: translateY(var(--ui-active-highlight-y, 0px));
  opacity: var(--ui-active-highlight-o, 0);
  background: var(--ui-accent-soft);
  border-radius: 8px;
  pointer-events: none;
  will-change: transform, height, opacity;
}
"#;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ActiveHighlightMotion {
    pub spring: ui_motion::spring::SpringConfig,
}

impl Default for ActiveHighlightMotion {
    fn default() -> Self {
        Self {
            spring: ui_motion::presets::spring_slide(),
        }
    }
}

#[cfg(target_arch = "wasm32")]
pub fn attach_active_highlight_motion(
    container_ref: NodeRef<html::Div>,
    highlight_ref: NodeRef<html::Div>,
    active_index: ReadSignal<usize>,
    option_id: Callback<usize, String>,
    motion: ActiveHighlightMotion,
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
        let Some(highlight) = highlight_ref.get() else {
            return;
        };
        if springs.get_value().is_some() {
            return;
        }

        let element: leptos::web_sys::HtmlElement = highlight.unchecked_into();
        let style = element.style();

        let _ = style.set_property("--ui-active-highlight-y", "0px");
        let _ = style.set_property("--ui-active-highlight-h", "0px");
        let _ = style.set_property("--ui-active-highlight-o", "0");

        let style_for_y = style.clone();
        let y = ui_motion::spring::SpringAnimator::new(0.0, config, move |v| {
            let v = v.clamp(-10000.0, 10000.0);
            let _ = style_for_y.set_property("--ui-active-highlight-y", &format!("{v}px"));
        });

        let style_for_h = style.clone();
        let h = ui_motion::spring::SpringAnimator::new(0.0, config, move |v| {
            let v = v.clamp(0.0, 10000.0);
            let _ = style_for_h.set_property("--ui-active-highlight-h", &format!("{v}px"));
        });

        let style_for_o = style.clone();
        let o = ui_motion::spring::SpringAnimator::new(0.0, config, move |v| {
            let v = v.clamp(0.0, 1.0);
            let _ = style_for_o.set_property("--ui-active-highlight-o", &format!("{v}"));
        });

        let springs_for_cleanup = springs;
        on_cleanup(move || {
            if let Some((y, h, o)) = springs_for_cleanup.get_value() {
                y.stop();
                h.stop();
                o.stop();
            }
        });

        springs.set_value(Some((y, h, o)));
    });

    Effect::new(move |_| {
        let active = active_index.get();
        let _ = container_ref.get();

        let Some((y, h, o)) = springs.get_value() else {
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

        let target_y = option_rect.top() - container_rect.top();
        let target_h = option_rect.height();

        y.set_target(target_y);
        h.set_target(target_h);
        o.set_target(1.0);
    });
}

#[cfg(not(target_arch = "wasm32"))]
pub fn attach_active_highlight_motion(
    _container_ref: NodeRef<html::Div>,
    _highlight_ref: NodeRef<html::Div>,
    _active_index: ReadSignal<usize>,
    _option_id: Callback<usize, String>,
    _motion: ActiveHighlightMotion,
) {
}
