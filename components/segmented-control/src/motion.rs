use ui_theme::default_button_motion_tokens;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SegmentedControlMotion {
    pub spring: ui_motion::spring::SpringConfig,
}

impl Default for SegmentedControlMotion {
    fn default() -> Self {
        let tokens = default_button_motion_tokens();
        Self {
            spring: ui_motion::spring::SpringConfig {
                stiffness: tokens.spring.stiffness,
                damping: tokens.spring.damping,
                mass: tokens.spring.mass,
                precision: tokens.spring.precision,
            },
        }
    }
}

fn sanitize_spring(value: ui_motion::spring::SpringConfig) -> ui_motion::spring::SpringConfig {
    let default = SegmentedControlMotion::default().spring;

    ui_motion::spring::SpringConfig {
        stiffness: if value.stiffness.is_finite() && value.stiffness > 0.0 {
            value.stiffness
        } else {
            default.stiffness
        },
        damping: if value.damping.is_finite() && value.damping > 0.0 {
            value.damping
        } else {
            default.damping
        },
        mass: if value.mass.is_finite() && value.mass > 0.0 {
            value.mass
        } else {
            default.mass
        },
        precision: if value.precision.is_finite() && value.precision > 0.0 {
            value.precision
        } else {
            default.precision
        },
    }
}

pub fn sanitize_motion(motion: SegmentedControlMotion) -> SegmentedControlMotion {
    SegmentedControlMotion {
        spring: sanitize_spring(motion.spring),
    }
}

#[cfg(any(test, target_arch = "wasm32"))]
const INDICATOR_INSET_PX: f64 = 4.0;

#[cfg(any(test, target_arch = "wasm32"))]
#[derive(Clone, Copy, Debug, PartialEq)]
struct Rect {
    left: f64,
    top: f64,
    width: f64,
    height: f64,
}

#[cfg(any(test, target_arch = "wasm32"))]
impl Rect {
    fn new(left: f64, top: f64, width: f64, height: f64) -> Self {
        Self {
            left,
            top,
            width,
            height,
        }
    }
}

#[cfg(any(test, target_arch = "wasm32"))]
#[derive(Clone, Copy, Debug, PartialEq)]
struct IndicatorLayout {
    x_px: f64,
    y_px: f64,
    width_px: f64,
    height_px: f64,
}

#[cfg(any(test, target_arch = "wasm32"))]
fn compute_indicator_layout(container: Rect, option: Rect) -> IndicatorLayout {
    IndicatorLayout {
        x_px: option.left - container.left - INDICATOR_INSET_PX,
        y_px: option.top - container.top - INDICATOR_INSET_PX,
        width_px: option.width,
        height_px: option.height,
    }
}

#[cfg(any(test, target_arch = "wasm32"))]
use std::{cell::RefCell, rc::Rc};

#[cfg(any(test, target_arch = "wasm32"))]
struct IndicatorMotionDriver {
    measure_layout: Box<dyn FnMut() -> Option<IndicatorLayout>>,
    x: ui_motion::spring::SpringAnimator,
    y: ui_motion::spring::SpringAnimator,
    w: ui_motion::spring::SpringAnimator,
    h: ui_motion::spring::SpringAnimator,
    o: ui_motion::spring::SpringAnimator,
    last_layout: Option<IndicatorLayout>,
}

#[cfg(any(test, target_arch = "wasm32"))]
impl IndicatorMotionDriver {
    fn new(
        config: ui_motion::spring::SpringConfig,
        measure_layout: impl FnMut() -> Option<IndicatorLayout> + 'static,
        set_x: impl FnMut(f64) + 'static,
        set_y: impl FnMut(f64) + 'static,
        set_w: impl FnMut(f64) + 'static,
        set_h: impl FnMut(f64) + 'static,
        set_o: impl FnMut(f64) + 'static,
    ) -> Self {
        Self {
            measure_layout: Box::new(measure_layout),
            x: ui_motion::spring::SpringAnimator::new(0.0, config, set_x),
            y: ui_motion::spring::SpringAnimator::new(0.0, config, set_y),
            w: ui_motion::spring::SpringAnimator::new(0.0, config, set_w),
            h: ui_motion::spring::SpringAnimator::new(0.0, config, set_h),
            o: ui_motion::spring::SpringAnimator::new(0.0, config, set_o),
            last_layout: None,
        }
    }

    fn stop(&mut self) {
        self.x.stop();
        self.y.stop();
        self.w.stop();
        self.h.stop();
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
                && (prev.y_px - layout.y_px).abs() < 0.5
                && (prev.width_px - layout.width_px).abs() < 0.5
                && (prev.height_px - layout.height_px).abs() < 0.5;
            if unchanged {
                self.o.set_target(1.0);
                return;
            }
        }

        self.last_layout = Some(layout);

        self.x.set_target(layout.x_px);
        self.y.set_target(layout.y_px);
        self.w.set_target(layout.width_px);
        self.h.set_target(layout.height_px);
        self.o.set_target(1.0);
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
    use leptos::wasm_bindgen::{JsCast, closure::Closure};

    let motion = StoredValue::new(sanitize_motion(motion));
    let driver = StoredValue::new_local(None::<Rc<RefCell<IndicatorMotionDriver>>>);
    let resize_observer = StoredValue::new_local(None::<leptos::web_sys::ResizeObserver>);
    let resize_closure = StoredValue::new_local(
        None::<Closure<dyn FnMut(js_sys::Array, leptos::web_sys::ResizeObserver)>>,
    );

    Effect::new(move |_| {
        let config = motion.get_value().spring;
        let Some(indicator) = indicator_ref.get() else {
            return;
        };
        let Some(container) = container_ref.get() else {
            return;
        };
        if driver.get_value().is_some() {
            return;
        }

        let element: leptos::web_sys::HtmlElement = indicator.unchecked_into();
        let style = element.style();

        drop(style.set_property("--ui-segmented-control-indicator-x", "0px"));
        drop(style.set_property("--ui-segmented-control-indicator-y", "0px"));
        drop(style.set_property("--ui-segmented-control-indicator-w", "0px"));
        drop(style.set_property("--ui-segmented-control-indicator-h", "0px"));
        drop(style.set_property("--ui-segmented-control-indicator-o", "0"));
        let measure_layout = {
            let container_ref = container_ref;
            let active_index = active_index;
            let option_id = option_id;
            move || {
                let Some(container) = container_ref.get_untracked() else {
                    return None;
                };
                let Some(document) = leptos::web_sys::window().and_then(|w| w.document()) else {
                    return None;
                };

                let id = option_id.run(active_index.get_untracked());
                let option_el = document.get_element_by_id(&id)?;

                let container_el: &leptos::web_sys::Element = container.as_ref();
                let container_rect = container_el.get_bounding_client_rect();
                let option_rect = option_el.get_bounding_client_rect();

                Some(compute_indicator_layout(
                    Rect::new(
                        container_rect.left(),
                        container_rect.top(),
                        container_rect.width(),
                        container_rect.height(),
                    ),
                    Rect::new(
                        option_rect.left(),
                        option_rect.top(),
                        option_rect.width(),
                        option_rect.height(),
                    ),
                ))
            }
        };

        let style_for_x = style.clone();
        let set_x = move |v: f64| {
            let v = v.clamp(-10000.0, 10000.0);
            drop(style_for_x.set_property("--ui-segmented-control-indicator-x", &format!("{v}px")));
        };

        let style_for_y = style.clone();
        let set_y = move |v: f64| {
            let v = v.clamp(-10000.0, 10000.0);
            drop(style_for_y.set_property("--ui-segmented-control-indicator-y", &format!("{v}px")));
        };

        let style_for_w = style.clone();
        let set_w = move |v: f64| {
            let v = v.clamp(0.0, 10000.0);
            drop(style_for_w.set_property("--ui-segmented-control-indicator-w", &format!("{v}px")));
        };

        let style_for_h = style.clone();
        let set_h = move |v: f64| {
            let v = v.clamp(0.0, 10000.0);
            drop(style_for_h.set_property("--ui-segmented-control-indicator-h", &format!("{v}px")));
        };

        let style_for_o = style.clone();
        let set_o = move |v: f64| {
            let v = v.clamp(0.0, 1.0);
            drop(style_for_o.set_property("--ui-segmented-control-indicator-o", &format!("{v}")));
        };

        let driver_instance = Rc::new(RefCell::new(IndicatorMotionDriver::new(
            config,
            measure_layout,
            set_x,
            set_y,
            set_w,
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
        std::hint::black_box(indicator_ref.get());
        let Some(driver) = driver.get_value() else {
            return;
        };
        driver.borrow_mut().sync_measured_layout();
    });
}

#[cfg(not(target_arch = "wasm32"))]
pub fn attach_indicator_motion(
    _container_ref: leptos::prelude::NodeRef<leptos::html::Div>,
    _indicator_ref: leptos::prelude::NodeRef<leptos::html::Div>,
    _active_index: leptos::prelude::ReadSignal<usize>,
    _option_id: leptos::prelude::Callback<usize, String>,
    motion: SegmentedControlMotion,
) {
    std::hint::black_box(sanitize_motion(motion)); // drop(sanitize_motion(motion));
}

#[cfg(test)]
#[path = "../test/motion.rs"]
mod tests;
