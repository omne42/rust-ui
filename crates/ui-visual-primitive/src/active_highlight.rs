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

#[cfg(any(test, target_arch = "wasm32"))]
use std::{cell::RefCell, rc::Rc};

#[cfg(any(test, target_arch = "wasm32"))]
#[derive(Clone, Copy, Debug, PartialEq)]
struct HighlightLayout {
    y_px: f64,
    height_px: f64,
}

#[cfg(any(test, target_arch = "wasm32"))]
struct ActiveHighlightMotionDriver {
    measure_layout: Box<dyn FnMut() -> Option<HighlightLayout>>,
    y: ui_motion::spring::SpringAnimator,
    h: ui_motion::spring::SpringAnimator,
    o: ui_motion::spring::SpringAnimator,
    last_layout: Option<HighlightLayout>,
}

#[cfg(any(test, target_arch = "wasm32"))]
impl ActiveHighlightMotionDriver {
    fn new(
        config: ui_motion::spring::SpringConfig,
        measure_layout: impl FnMut() -> Option<HighlightLayout> + 'static,
        set_y: impl FnMut(f64) + 'static,
        set_h: impl FnMut(f64) + 'static,
        set_o: impl FnMut(f64) + 'static,
    ) -> Self {
        Self {
            measure_layout: Box::new(measure_layout),
            y: ui_motion::spring::SpringAnimator::new(0.0, config, set_y),
            h: ui_motion::spring::SpringAnimator::new(0.0, config, set_h),
            o: ui_motion::spring::SpringAnimator::new(0.0, config, set_o),
            last_layout: None,
        }
    }

    fn stop(&mut self) {
        self.y.stop();
        self.h.stop();
        self.o.stop();
    }

    fn sync_measured_layout(&mut self) {
        let layout = (self.measure_layout)();
        self.sync_layout(layout);
    }

    fn sync_layout(&mut self, layout: Option<HighlightLayout>) {
        let Some(layout) = layout else {
            self.last_layout = None;
            self.o.set_target(0.0);
            return;
        };

        if let Some(prev) = self.last_layout {
            let unchanged = (prev.y_px - layout.y_px).abs() < 0.5
                && (prev.height_px - layout.height_px).abs() < 0.5;
            if unchanged {
                self.o.set_target(1.0);
                return;
            }
        }

        self.last_layout = Some(layout);

        self.y.set_target(layout.y_px);
        self.h.set_target(layout.height_px);
        self.o.set_target(1.0);
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
    use leptos::wasm_bindgen::{JsCast, closure::Closure};

    let motion = StoredValue::new(motion);
    let driver = StoredValue::new_local(None::<Rc<RefCell<ActiveHighlightMotionDriver>>>);
    let resize_observer = StoredValue::new_local(None::<leptos::web_sys::ResizeObserver>);
    let resize_closure = StoredValue::new_local(
        None::<Closure<dyn FnMut(js_sys::Array, leptos::web_sys::ResizeObserver)>>,
    );

    Effect::new(move |_| {
        let config = motion.get_value().spring;
        let Some(container) = container_ref.get() else {
            return;
        };
        let Some(highlight) = highlight_ref.get() else {
            return;
        };
        if driver.get_value().is_some() {
            return;
        }

        let element: leptos::web_sys::HtmlElement = highlight.unchecked_into();
        let style = element.style();

        drop(style.set_property("--ui-active-highlight-y", "0px"));
        drop(style.set_property("--ui-active-highlight-h", "0px"));
        drop(style.set_property("--ui-active-highlight-o", "0"));
        let measure_layout = {
            let container_ref = container_ref;
            let active_index = active_index;
            let option_id = option_id;
            move || {
                let Some(_container) = container_ref.get_untracked() else {
                    return None;
                };
                let Some(document) = leptos::web_sys::window().and_then(|w| w.document()) else {
                    return None;
                };

                let id = option_id.run(active_index.get_untracked());
                let option_el = document.get_element_by_id(&id)?;
                let option: leptos::web_sys::HtmlElement = option_el.unchecked_into();

                Some(HighlightLayout {
                    y_px: option.offset_top() as f64,
                    height_px: option.offset_height() as f64,
                })
            }
        };

        let style_for_y = style.clone();
        let set_y = move |v: f64| {
            let v = v.clamp(-10000.0, 10000.0);
            drop(style_for_y.set_property("--ui-active-highlight-y", &format!("{v}px")));
        };

        let style_for_h = style.clone();
        let set_h = move |v: f64| {
            let v = v.clamp(0.0, 10000.0);
            drop(style_for_h.set_property("--ui-active-highlight-h", &format!("{v}px")));
        };

        let style_for_o = style.clone();
        let set_o = move |v: f64| {
            let v = v.clamp(0.0, 1.0);
            drop(style_for_o.set_property("--ui-active-highlight-o", &format!("{v}")));
        };

        let driver_instance = Rc::new(RefCell::new(ActiveHighlightMotionDriver::new(
            config,
            measure_layout,
            set_y,
            set_h,
            set_o,
        )));

        driver_instance.borrow_mut().sync_measured_layout();
        driver.set_value(Some(Rc::clone(&driver_instance)));

        if resize_observer.get_value().is_none() {
            let container_el: leptos::web_sys::Element = container.unchecked_into();

            let driver_for_resize = Rc::clone(&driver_instance);
            let closure = Closure::wrap(Box::new(
                move |_: js_sys::Array, _: leptos::web_sys::ResizeObserver| {
                    driver_for_resize.borrow_mut().sync_measured_layout();
                },
            )
                as Box<dyn FnMut(js_sys::Array, leptos::web_sys::ResizeObserver)>);

            if let Ok(observer) =
                leptos::web_sys::ResizeObserver::new(closure.as_ref().unchecked_ref())
            {
                observer.observe(&container_el);
                resize_observer.set_value(Some(observer));
                resize_closure.set_value(Some(closure));
            }
        }

        let driver_for_cleanup = driver;
        let resize_observer_for_cleanup = resize_observer;
        let resize_closure_for_cleanup = resize_closure;
        on_cleanup(move || {
            if let Some(observer) = resize_observer_for_cleanup.get_value() {
                observer.disconnect();
            }
            resize_observer_for_cleanup.set_value(None);
            resize_closure_for_cleanup.set_value(None);

            if let Some(driver) = driver_for_cleanup.get_value() {
                driver.borrow_mut().stop();
            }
        });
    });

    Effect::new(move |_| {
        std::hint::black_box(active_index.get());
        std::hint::black_box(container_ref.get());
        std::hint::black_box(highlight_ref.get());
        let Some(driver) = driver.get_value() else {
            return;
        };
        driver.borrow_mut().sync_measured_layout();
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

#[cfg(test)]
#[path = "test/active_highlight.rs"]
mod tests;
