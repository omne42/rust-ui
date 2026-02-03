use leptos::{html, prelude::*};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum PopoverPlacement {
    #[default]
    BottomStart,
    BottomEnd,
}

#[derive(Clone, Copy)]
pub struct PopoverPositionOptions {
    pub anchor_ref: NodeRef<html::Button>,
    pub panel_ref: NodeRef<html::Div>,
    pub placement: PopoverPlacement,
    pub offset_px: f64,
    pub padding_px: f64,
}

impl Default for PopoverPositionOptions {
    fn default() -> Self {
        Self {
            anchor_ref: NodeRef::new(),
            panel_ref: NodeRef::new(),
            placement: PopoverPlacement::BottomStart,
            offset_px: 8.0,
            padding_px: 8.0,
        }
    }
}

#[derive(Clone)]
pub struct PopoverPositionState {
    pub top_px: ReadSignal<f64>,
    pub left_px: ReadSignal<f64>,
}

pub fn use_popover_position(_options: PopoverPositionOptions) -> PopoverPositionState {
    let (top_px, _set_top_px) = signal(0.0);
    let (left_px, _set_left_px) = signal(0.0);

    #[cfg(all(feature = "web", target_arch = "wasm32"))]
    {
        use send_wrapper::SendWrapper;
        use std::rc::Rc;
        use wasm_bindgen::{JsCast, closure::Closure};

        let compute: Rc<dyn Fn()> = Rc::new({
            let anchor_ref = _options.anchor_ref;
            let panel_ref = _options.panel_ref;
            let placement = _options.placement;
            let offset_px = _options.offset_px;
            let padding_px = _options.padding_px;
            move || {
                let Some(window) = web_sys::window() else {
                    return;
                };
                let Some(anchor) = anchor_ref.get_untracked() else {
                    return;
                };
                let Some(panel) = panel_ref.get_untracked() else {
                    return;
                };

                let viewport_w = window
                    .inner_width()
                    .ok()
                    .and_then(|v| v.as_f64())
                    .unwrap_or(0.0);
                let viewport_h = window
                    .inner_height()
                    .ok()
                    .and_then(|v| v.as_f64())
                    .unwrap_or(0.0);

                let anchor_el: &web_sys::Element = anchor.as_ref();
                let panel_el: &web_sys::Element = panel.as_ref();
                let anchor_rect = anchor_el.get_bounding_client_rect();
                let panel_rect = panel_el.get_bounding_client_rect();

                let mut top = anchor_rect.bottom() + offset_px;
                let mut left = match placement {
                    PopoverPlacement::BottomStart => anchor_rect.left(),
                    PopoverPlacement::BottomEnd => anchor_rect.right() - panel_rect.width(),
                };

                if viewport_w > 0.0 && panel_rect.width() > 0.0 {
                    let min_left = padding_px;
                    let max_left = (viewport_w - panel_rect.width() - padding_px).max(min_left);
                    left = left.clamp(min_left, max_left);
                }

                if viewport_h > 0.0 && panel_rect.height() > 0.0 {
                    let min_top = padding_px;
                    let max_top = (viewport_h - panel_rect.height() - padding_px).max(min_top);
                    top = top.clamp(min_top, max_top);
                }

                _set_top_px.set(top);
                _set_left_px.set(left);
            }
        });

        // Compute once when refs become available.
        Effect::new({
            let compute = compute.clone();
            let anchor_ref = _options.anchor_ref;
            let panel_ref = _options.panel_ref;
            move |_| {
                let _ = anchor_ref.get();
                let _ = panel_ref.get();
                compute();
            }
        });

        // Recompute on resize/scroll while mounted.
        if let Some(window) = web_sys::window() {
            let window = SendWrapper::new(window);

            let on_resize: SendWrapper<Closure<dyn FnMut()>> = SendWrapper::new({
                let compute = compute.clone();
                Closure::wrap(Box::new(move || compute()) as Box<dyn FnMut()>)
            });
            let on_scroll: SendWrapper<Closure<dyn FnMut()>> = SendWrapper::new({
                let compute = compute.clone();
                Closure::wrap(Box::new(move || compute()) as Box<dyn FnMut()>)
            });

            let _ = window
                .add_event_listener_with_callback("resize", on_resize.as_ref().unchecked_ref());
            let _ = window
                .add_event_listener_with_callback("scroll", on_scroll.as_ref().unchecked_ref());

            on_cleanup(move || {
                let _ = window.remove_event_listener_with_callback(
                    "resize",
                    on_resize.as_ref().unchecked_ref(),
                );
                let _ = window.remove_event_listener_with_callback(
                    "scroll",
                    on_scroll.as_ref().unchecked_ref(),
                );
            });
        }
    }

    PopoverPositionState { top_px, left_px }
}
