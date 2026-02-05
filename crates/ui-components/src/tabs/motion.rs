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

#[cfg(any(test, target_arch = "wasm32"))]
use std::{cell::RefCell, rc::Rc};

#[cfg(target_arch = "wasm32")]
const INDICATOR_INSET_PX: f64 = 4.0;

#[cfg(any(test, target_arch = "wasm32"))]
#[derive(Clone, Copy, Debug, PartialEq)]
struct IndicatorLayout {
    x_px: f64,
    width_px: f64,
}

#[cfg(any(test, target_arch = "wasm32"))]
struct IndicatorMotionDriver {
    measure_layout: Box<dyn FnMut() -> Option<IndicatorLayout>>,
    x: ui_motion::spring::SpringAnimator,
    w: ui_motion::spring::SpringAnimator,
    o: ui_motion::spring::SpringAnimator,
    last_layout: Option<IndicatorLayout>,
}

#[cfg(any(test, target_arch = "wasm32"))]
impl IndicatorMotionDriver {
    fn new(
        config: ui_motion::spring::SpringConfig,
        measure_layout: impl FnMut() -> Option<IndicatorLayout> + 'static,
        set_x: impl FnMut(f64) + 'static,
        set_w: impl FnMut(f64) + 'static,
        set_o: impl FnMut(f64) + 'static,
    ) -> Self {
        Self {
            measure_layout: Box::new(measure_layout),
            x: ui_motion::spring::SpringAnimator::new(0.0, config, set_x),
            w: ui_motion::spring::SpringAnimator::new(0.0, config, set_w),
            o: ui_motion::spring::SpringAnimator::new(0.0, config, set_o),
            last_layout: None,
        }
    }

    fn stop(&mut self) {
        self.x.stop();
        self.w.stop();
        self.o.stop();
    }

    fn sync_measured_layout(&mut self) {
        let layout = (self.measure_layout)();
        self.sync_layout(layout);
    }

    fn sync_layout(&mut self, layout: Option<IndicatorLayout>) {
        let Some(layout) = layout else {
            self.last_layout = None;
            self.o.set_target(0.0);
            return;
        };

        if let Some(prev) = self.last_layout {
            let unchanged = (prev.x_px - layout.x_px).abs() < 0.5
                && (prev.width_px - layout.width_px).abs() < 0.5;
            if unchanged {
                self.o.set_target(1.0);
                return;
            }
        }

        self.last_layout = Some(layout);

        self.x.set_target(layout.x_px);
        self.w.set_target(layout.width_px);
        self.o.set_target(1.0);
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
    use leptos::wasm_bindgen::{JsCast, closure::Closure};

    let motion = StoredValue::new(motion);
    let driver = StoredValue::new_local(None::<Rc<RefCell<IndicatorMotionDriver>>>);
    let resize_observer = StoredValue::new_local(None::<leptos::web_sys::ResizeObserver>);
    let resize_closure = StoredValue::new_local(
        None::<Closure<dyn FnMut(js_sys::Array, leptos::web_sys::ResizeObserver)>>,
    );

    Effect::new(move |_| {
        let config = motion.get_value().spring;
        let Some(list) = list_ref.get() else {
            return;
        };
        let Some(indicator) = indicator_ref.get() else {
            return;
        };
        if driver.get_value().is_some() {
            return;
        }

        let indicator_el: leptos::web_sys::HtmlElement = indicator.unchecked_into();
        let style = indicator_el.style();

        let _ = style.set_property("--ui-tabs-indicator-x", "0px");
        let _ = style.set_property("--ui-tabs-indicator-w", "0px");
        let _ = style.set_property("--ui-tabs-indicator-o", "0");

        let measure_layout = {
            let tab_refs = Arc::clone(&tab_refs);
            let selected_index = selected_index;
            move || {
                let selected = selected_index.get_untracked();
                let tab_ref = tab_refs.get(selected)?;
                let tab = tab_ref.get_untracked()?;
                let element: leptos::web_sys::HtmlElement = tab.unchecked_into();

                Some(IndicatorLayout {
                    x_px: (element.offset_left() as f64) - INDICATOR_INSET_PX,
                    width_px: element.offset_width() as f64,
                })
            }
        };

        let style_for_x = style.clone();
        let set_x = move |v: f64| {
            let v = v.clamp(-10000.0, 10000.0);
            let _ = style_for_x.set_property("--ui-tabs-indicator-x", &format!("{v}px"));
        };

        let style_for_w = style.clone();
        let set_w = move |v: f64| {
            let v = v.clamp(0.0, 10000.0);
            let _ = style_for_w.set_property("--ui-tabs-indicator-w", &format!("{v}px"));
        };

        let style_for_o = style.clone();
        let set_o = move |v: f64| {
            let v = v.clamp(0.0, 1.0);
            let _ = style_for_o.set_property("--ui-tabs-indicator-o", &format!("{v}"));
        };

        let driver_instance = Rc::new(RefCell::new(IndicatorMotionDriver::new(
            config,
            measure_layout,
            set_x,
            set_w,
            set_o,
        )));
        driver_instance.borrow_mut().sync_measured_layout();
        driver.set_value(Some(Rc::clone(&driver_instance)));

        if resize_observer.get_value().is_none() {
            let list_el: leptos::web_sys::Element = list.unchecked_into();

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
                observer.observe(&list_el);
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
        let _ = selected_index.get();
        let _ = list_ref.get();
        let _ = indicator_ref.get();

        let Some(driver) = driver.get_value() else {
            return;
        };
        driver.borrow_mut().sync_measured_layout();
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;

    #[derive(Debug, PartialEq)]
    enum Event {
        X(f64),
        W(f64),
        O(f64),
    }

    fn record(events: &Rc<RefCell<Vec<Event>>>, event: Event) {
        events.borrow_mut().push(event);
    }

    #[test]
    fn driver_sync_layout_updates_all_css_vars() {
        let events: Rc<RefCell<Vec<Event>>> = Rc::new(RefCell::new(Vec::new()));

        let mut driver = IndicatorMotionDriver::new(
            ui_motion::presets::spring_slide(),
            || None,
            {
                let events = Rc::clone(&events);
                move |v| record(&events, Event::X(v))
            },
            {
                let events = Rc::clone(&events);
                move |v| record(&events, Event::W(v))
            },
            {
                let events = Rc::clone(&events);
                move |v| record(&events, Event::O(v))
            },
        );

        driver.sync_layout(Some(IndicatorLayout {
            x_px: 12.0,
            width_px: 88.0,
        }));

        assert_eq!(
            &*events.borrow(),
            &[Event::X(12.0), Event::W(88.0), Event::O(1.0)]
        );
    }

    #[test]
    fn driver_sync_layout_noops_when_geometry_is_unchanged() {
        let events: Rc<RefCell<Vec<Event>>> = Rc::new(RefCell::new(Vec::new()));

        let mut driver = IndicatorMotionDriver::new(
            ui_motion::presets::spring_slide(),
            || None,
            {
                let events = Rc::clone(&events);
                move |v| record(&events, Event::X(v))
            },
            {
                let events = Rc::clone(&events);
                move |v| record(&events, Event::W(v))
            },
            {
                let events = Rc::clone(&events);
                move |v| record(&events, Event::O(v))
            },
        );

        let layout = IndicatorLayout {
            x_px: 0.0,
            width_px: 88.0,
        };

        driver.sync_layout(Some(layout));
        events.borrow_mut().clear();

        driver.sync_layout(Some(layout));
        assert_eq!(&*events.borrow(), &[Event::O(1.0)]);
    }

    #[test]
    fn driver_sync_layout_hides_when_layout_is_missing() {
        let events: Rc<RefCell<Vec<Event>>> = Rc::new(RefCell::new(Vec::new()));

        let mut driver = IndicatorMotionDriver::new(
            ui_motion::presets::spring_slide(),
            || None,
            {
                let events = Rc::clone(&events);
                move |v| record(&events, Event::X(v))
            },
            {
                let events = Rc::clone(&events);
                move |v| record(&events, Event::W(v))
            },
            {
                let events = Rc::clone(&events);
                move |v| record(&events, Event::O(v))
            },
        );

        driver.sync_layout(None);
        assert_eq!(&*events.borrow(), &[Event::O(0.0)]);

        driver.stop();
    }

    #[test]
    fn driver_sync_measured_layout_reads_latest_values() {
        let events: Rc<RefCell<Vec<Event>>> = Rc::new(RefCell::new(Vec::new()));
        let layout = Rc::new(Cell::new(IndicatorLayout {
            x_px: 0.0,
            width_px: 80.0,
        }));

        let mut driver = IndicatorMotionDriver::new(
            ui_motion::presets::spring_slide(),
            {
                let layout = Rc::clone(&layout);
                move || Some(layout.get())
            },
            {
                let events = Rc::clone(&events);
                move |v| record(&events, Event::X(v))
            },
            {
                let events = Rc::clone(&events);
                move |v| record(&events, Event::W(v))
            },
            {
                let events = Rc::clone(&events);
                move |v| record(&events, Event::O(v))
            },
        );

        driver.sync_measured_layout();
        events.borrow_mut().clear();

        layout.set(IndicatorLayout {
            x_px: 12.0,
            width_px: 100.0,
        });
        driver.sync_measured_layout();

        assert_eq!(
            &*events.borrow(),
            &[Event::X(12.0), Event::W(100.0), Event::O(1.0)]
        );
    }
}
