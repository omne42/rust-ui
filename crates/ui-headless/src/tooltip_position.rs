use leptos::{html, prelude::*};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum TooltipPlacement {
    #[default]
    Top,
    Bottom,
}

impl TooltipPlacement {
    pub const fn is_top(self) -> bool {
        matches!(self, Self::Top)
    }

    pub const fn is_bottom(self) -> bool {
        matches!(self, Self::Bottom)
    }

    pub const fn flip_vertical(self) -> Self {
        match self {
            Self::Top => Self::Bottom,
            Self::Bottom => Self::Top,
        }
    }

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Top => "top",
            Self::Bottom => "bottom",
        }
    }
}

#[derive(Clone, Copy)]
pub struct TooltipPositionOptions {
    pub anchor_ref: NodeRef<html::Span>,
    pub panel_ref: NodeRef<html::Div>,
    pub placement: TooltipPlacement,
    pub offset_px: f64,
    pub padding_px: f64,
}

impl Default for TooltipPositionOptions {
    fn default() -> Self {
        Self {
            anchor_ref: NodeRef::new(),
            panel_ref: NodeRef::new(),
            placement: TooltipPlacement::Top,
            offset_px: 7.0,
            padding_px: 8.0,
        }
    }
}

#[derive(Clone)]
pub struct TooltipPositionState {
    pub top_px: ReadSignal<f64>,
    pub left_px: ReadSignal<f64>,
    pub placement: ReadSignal<TooltipPlacement>,
}

#[cfg(any(test, all(feature = "web", target_arch = "wasm32")))]
#[derive(Clone, Copy, Debug, PartialEq)]
struct Rect {
    top: f64,
    left: f64,
    width: f64,
    height: f64,
}

#[cfg(any(test, all(feature = "web", target_arch = "wasm32")))]
impl Rect {
    fn bottom(self) -> f64 {
        self.top + self.height
    }

    fn center_x(self) -> f64 {
        self.left + self.width / 2.0
    }
}

#[cfg(any(test, all(feature = "web", target_arch = "wasm32")))]
#[derive(Clone, Copy, Debug, PartialEq)]
struct Size {
    width: f64,
    height: f64,
}

#[cfg(any(test, all(feature = "web", target_arch = "wasm32")))]
#[derive(Clone, Copy, Debug, PartialEq)]
struct ComputedPosition {
    top: f64,
    left: f64,
    placement: TooltipPlacement,
}

#[cfg(any(test, all(feature = "web", target_arch = "wasm32")))]
fn compute_tooltip_position(
    anchor: Rect,
    panel: Size,
    viewport: Size,
    preferred: TooltipPlacement,
    offset_px: f64,
    padding_px: f64,
) -> ComputedPosition {
    let padding_px = padding_px.max(0.0);
    let offset_px = offset_px.max(0.0);

    let mut resolved = preferred;

    if viewport.height > 0.0 && panel.height > 0.0 {
        let space_above = (anchor.top - padding_px).max(0.0);
        let space_below = (viewport.height - anchor.bottom() - padding_px).max(0.0);
        let needed = panel.height + offset_px;

        let fits_above = needed <= space_above;
        let fits_below = needed <= space_below;

        let should_flip = (preferred.is_top()
            && !fits_above
            && (fits_below || space_below > space_above))
            || (preferred.is_bottom() && !fits_below && (fits_above || space_above > space_below));

        if should_flip {
            resolved = preferred.flip_vertical();
        }
    }

    let mut top = if resolved.is_bottom() {
        anchor.bottom() + offset_px
    } else {
        anchor.top - offset_px - panel.height
    };

    let mut left = anchor.center_x() - panel.width / 2.0;

    if viewport.width > 0.0 && panel.width > 0.0 {
        let min_left = padding_px;
        let max_left = (viewport.width - panel.width - padding_px).max(min_left);
        left = left.clamp(min_left, max_left);
    }

    if viewport.height > 0.0 && panel.height > 0.0 {
        let min_top = padding_px;
        let max_top = (viewport.height - panel.height - padding_px).max(min_top);
        top = top.clamp(min_top, max_top);
    }

    ComputedPosition {
        top,
        left,
        placement: resolved,
    }
}

pub fn use_tooltip_position(_options: TooltipPositionOptions) -> TooltipPositionState {
    let (top_px, _set_top_px) = signal(0.0);
    let (left_px, _set_left_px) = signal(0.0);
    let (placement, _set_placement) = signal(_options.placement);

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

                let computed = compute_tooltip_position(
                    Rect {
                        top: anchor_rect.top(),
                        left: anchor_rect.left(),
                        width: anchor_rect.width(),
                        height: anchor_rect.height(),
                    },
                    Size {
                        width: panel_rect.width(),
                        height: panel_rect.height(),
                    },
                    Size {
                        width: viewport_w,
                        height: viewport_h,
                    },
                    placement,
                    offset_px,
                    padding_px,
                );

                _set_top_px.set(computed.top);
                _set_left_px.set(computed.left);
                _set_placement.set(computed.placement);
            }
        });

        let resize_observer = StoredValue::new_local(None::<web_sys::ResizeObserver>);
        let resize_closure = StoredValue::new_local(
            None::<Closure<dyn FnMut(js_sys::Array, web_sys::ResizeObserver)>>,
        );

        Effect::new({
            let compute = compute.clone();
            let anchor_ref = _options.anchor_ref;
            let panel_ref = _options.panel_ref;
            move |_| {
                drop(anchor_ref.get());
                drop(panel_ref.get());
                compute();

                if resize_observer.get_value().is_some() {
                    return;
                }

                let Some(anchor) = anchor_ref.get_untracked() else {
                    return;
                };
                let Some(panel) = panel_ref.get_untracked() else {
                    return;
                };

                let anchor_el: web_sys::Element = anchor.unchecked_into();
                let panel_el: web_sys::Element = panel.unchecked_into();

                let compute_for_resize = compute.clone();
                let closure = Closure::wrap(Box::new(
                    move |_: js_sys::Array, _: web_sys::ResizeObserver| {
                        compute_for_resize();
                    },
                )
                    as Box<dyn FnMut(js_sys::Array, web_sys::ResizeObserver)>);

                if let Ok(observer) = web_sys::ResizeObserver::new(closure.as_ref().unchecked_ref())
                {
                    observer.observe(&anchor_el);
                    observer.observe(&panel_el);
                    resize_observer.set_value(Some(observer));
                    resize_closure.set_value(Some(closure));
                }

                let resize_observer_for_cleanup = resize_observer;
                let resize_closure_for_cleanup = resize_closure;
                on_cleanup(move || {
                    if let Some(observer) = resize_observer_for_cleanup.get_value() {
                        observer.disconnect();
                    }
                    resize_observer_for_cleanup.set_value(None);
                    resize_closure_for_cleanup.set_value(None);
                });
            }
        });

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

            drop(
                window
                    .add_event_listener_with_callback("resize", on_resize.as_ref().unchecked_ref()),
            );
            drop(window.add_event_listener_with_callback_and_bool(
                "scroll",
                on_scroll.as_ref().unchecked_ref(),
                true,
            ));

            on_cleanup(move || {
                drop(window.remove_event_listener_with_callback(
                    "resize",
                    on_resize.as_ref().unchecked_ref(),
                ));
                drop(window.remove_event_listener_with_callback_and_bool(
                    "scroll",
                    on_scroll.as_ref().unchecked_ref(),
                    true,
                ));
            });
        }
    }

    TooltipPositionState {
        top_px,
        left_px,
        placement,
    }
}

#[cfg(test)]
#[path = "test/tooltip_position.rs"]
mod tests;
