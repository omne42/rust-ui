use crate::logic;
use ui_motion::spring::SpringConfig;
use ui_theme::default_accordion_motion_tokens;

#[cfg(any(test, target_arch = "wasm32"))]
use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AccordionMotion {
    pub spring: SpringConfig,
    pub indicator_closed_rotation_deg: f64,
    pub indicator_open_rotation_deg: f64,
    pub panel_offset_y_px: f64,
}

impl Default for AccordionMotion {
    fn default() -> Self {
        let tokens = default_accordion_motion_tokens();
        Self {
            spring: SpringConfig {
                stiffness: tokens.spring.stiffness,
                damping: tokens.spring.damping,
                mass: tokens.spring.mass,
                precision: tokens.spring.precision,
            },
            indicator_closed_rotation_deg: tokens.indicator_closed_rotation_deg,
            indicator_open_rotation_deg: tokens.indicator_open_rotation_deg,
            panel_offset_y_px: tokens.panel_offset_y_px,
        }
    }
}

fn sanitize_number(value: f64, fallback: f64) -> f64 {
    if value.is_finite() { value } else { fallback }
}

fn sanitize_spring(value: SpringConfig) -> SpringConfig {
    let default = AccordionMotion::default().spring;

    SpringConfig {
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

fn panel_lifecycle_event_from_hidden(hidden: bool) -> logic::AccordionPanelLifecycleEvent {
    if hidden {
        logic::AccordionPanelLifecycleEvent::NotifyHidden
    } else {
        logic::AccordionPanelLifecycleEvent::NotifyShown
    }
}

pub fn sanitize_motion(motion: AccordionMotion) -> AccordionMotion {
    let default = AccordionMotion::default();

    AccordionMotion {
        spring: sanitize_spring(motion.spring),
        indicator_closed_rotation_deg: sanitize_number(
            motion.indicator_closed_rotation_deg,
            default.indicator_closed_rotation_deg,
        )
        .clamp(-360.0, 360.0),
        indicator_open_rotation_deg: sanitize_number(
            motion.indicator_open_rotation_deg,
            default.indicator_open_rotation_deg,
        )
        .clamp(-360.0, 360.0),
        panel_offset_y_px: sanitize_number(motion.panel_offset_y_px, default.panel_offset_y_px)
            .abs()
            .clamp(0.0, 240.0),
    }
}

#[cfg(target_arch = "wasm32")]
fn prefers_reduced_motion() -> bool {
    let Some(window) = leptos::web_sys::window() else {
        return false;
    };

    window
        .match_media("(prefers-reduced-motion: reduce)")
        .ok()
        .flatten()
        .is_some_and(|query| query.matches())
}

#[cfg(target_arch = "wasm32")]
pub fn attach_indicator_motion(
    node_ref: leptos::prelude::NodeRef<leptos::html::Span>,
    is_open: leptos::prelude::Signal<bool>,
    motion: AccordionMotion,
) {
    use leptos::prelude::*;
    use leptos::wasm_bindgen::JsCast;
    use ui_observability::set_css_property_observed;

    let motion = StoredValue::new(sanitize_motion(motion));
    if prefers_reduced_motion() {
        Effect::new(move |_| {
            let Some(indicator) = node_ref.get() else {
                return;
            };
            let element: leptos::web_sys::HtmlElement = indicator.unchecked_into();
            let style = element.style();
            let motion = motion.get_value();
            let target = if is_open.get() {
                motion.indicator_open_rotation_deg
            } else {
                motion.indicator_closed_rotation_deg
            };
            set_css_property_observed(
                &style,
                "--ui-accordion-indicator-rotation",
                &format!("{target}deg"),
                "accordion.motion.indicator.reduced",
            );
        });
        return;
    }

    let spring = StoredValue::new_local(None::<ui_motion::spring::SpringAnimator>);
    let last_open = StoredValue::new(None::<bool>);

    Effect::new(move |_| {
        let config = motion.get_value().spring;
        let Some(indicator) = node_ref.get() else {
            return;
        };
        if spring.get_value().is_some() {
            return;
        }

        let element: leptos::web_sys::HtmlElement = indicator.unchecked_into();
        let style = element.style();
        let initial_open = is_open.get_untracked();
        let initial = if initial_open {
            motion.get_value().indicator_open_rotation_deg
        } else {
            motion.get_value().indicator_closed_rotation_deg
        };
        last_open.set_value(Some(initial_open));

        let animator = ui_motion::spring::SpringAnimator::new(initial, config, move |deg| {
            set_css_property_observed(
                &style,
                "--ui-accordion-indicator-rotation",
                &format!("{deg}deg"),
                "accordion.motion.indicator",
            );
        });

        let spring_for_cleanup = spring;
        on_cleanup(move || {
            if let Some(animator) = spring_for_cleanup.get_value() {
                animator.stop();
            }
        });

        spring.set_value(Some(animator));
    });

    Effect::new(move |_| {
        let open = is_open.get();
        if last_open.get_value() == Some(open) {
            return;
        }
        last_open.set_value(Some(open));

        let motion = motion.get_value();
        let target = if open {
            motion.indicator_open_rotation_deg
        } else {
            motion.indicator_closed_rotation_deg
        };

        let Some(spring) = spring.get_value() else {
            return;
        };
        spring.set_target(target);
    });
}

#[cfg(not(target_arch = "wasm32"))]
pub fn attach_indicator_motion(
    _node_ref: leptos::prelude::NodeRef<leptos::html::Span>,
    _is_open: leptos::prelude::Signal<bool>,
    motion: AccordionMotion,
) {
    sanitize_motion(motion);
}

#[cfg(any(test, target_arch = "wasm32"))]
type SetHidden = Rc<RefCell<Box<dyn FnMut(bool)>>>;
#[cfg(any(test, target_arch = "wasm32"))]
type MeasureHeight = Box<dyn FnMut() -> f64>;

#[cfg(any(test, target_arch = "wasm32"))]
type SetPanelHeight = Rc<RefCell<Box<dyn FnMut(f64)>>>;
#[cfg(any(test, target_arch = "wasm32"))]
type SetPanelOpacity = Rc<RefCell<Box<dyn FnMut(f64)>>>;
#[cfg(any(test, target_arch = "wasm32"))]
type SetPanelY = Rc<RefCell<Box<dyn FnMut(f64)>>>;

#[cfg(any(test, target_arch = "wasm32"))]
struct PanelMotionDriver {
    motion: AccordionMotion,
    set_hidden: SetHidden,
    measure_height_px: MeasureHeight,
    set_height: SetPanelHeight,
    set_opacity: SetPanelOpacity,
    set_y: SetPanelY,
    height_spring: Option<ui_motion::spring::SpringAnimator>,
    opacity_spring: Option<ui_motion::spring::SpringAnimator>,
    y_spring: Option<ui_motion::spring::SpringAnimator>,
    last_measured_height_px: Option<f64>,
    last_open: Option<bool>,
    current_height_px: Rc<Cell<f64>>,
    current_opacity: Rc<Cell<f64>>,
    current_y_px: Rc<Cell<f64>>,
    is_closing: Rc<Cell<bool>>,
}

#[cfg(any(test, target_arch = "wasm32"))]
impl PanelMotionDriver {
    fn close_spring_config(&self) -> SpringConfig {
        // Use a more damped spring on collapse to avoid visual wobble while keeping spring timing.
        let spring = self.motion.spring;
        let critical_damping = 2.0 * (spring.stiffness * spring.mass).sqrt();
        let damping = spring.damping.max(critical_damping * 0.92);
        SpringConfig { damping, ..spring }
    }

    fn new(
        motion: AccordionMotion,
        set_hidden: impl FnMut(bool) + 'static,
        measure_height_px: impl FnMut() -> f64 + 'static,
        set_height: impl FnMut(f64) + 'static,
        set_opacity: impl FnMut(f64) + 'static,
        set_y: impl FnMut(f64) + 'static,
    ) -> Self {
        Self {
            motion,
            set_hidden: Rc::new(RefCell::new(Box::new(set_hidden))),
            measure_height_px: Box::new(measure_height_px),
            set_height: Rc::new(RefCell::new(Box::new(set_height))),
            set_opacity: Rc::new(RefCell::new(Box::new(set_opacity))),
            set_y: Rc::new(RefCell::new(Box::new(set_y))),
            height_spring: None,
            opacity_spring: None,
            y_spring: None,
            last_measured_height_px: None,
            last_open: None,
            current_height_px: Rc::new(Cell::new(0.0)),
            current_opacity: Rc::new(Cell::new(0.0)),
            current_y_px: Rc::new(Cell::new(motion.panel_offset_y_px)),
            is_closing: Rc::new(Cell::new(false)),
        }
    }

    fn stop(&mut self) {
        if let Some(spring) = self.height_spring.take() {
            spring.stop();
        }
        if let Some(spring) = self.opacity_spring.take() {
            spring.stop();
        }
        if let Some(spring) = self.y_spring.take() {
            spring.stop();
        }
    }

    fn make_height_spring(
        &self,
        initial: f64,
        config: SpringConfig,
    ) -> ui_motion::spring::SpringAnimator {
        let set_height = Rc::clone(&self.set_height);
        let current_height_px = Rc::clone(&self.current_height_px);
        ui_motion::spring::SpringAnimator::new(initial, config, move |value| {
            let value = value.clamp(0.0, 100000.0);
            current_height_px.set(value);
            (set_height.borrow_mut())(value);
        })
    }

    fn make_opacity_spring(
        &self,
        initial: f64,
        config: SpringConfig,
    ) -> ui_motion::spring::SpringAnimator {
        let set_opacity = Rc::clone(&self.set_opacity);
        let current_opacity = Rc::clone(&self.current_opacity);
        let is_closing = Rc::clone(&self.is_closing);
        ui_motion::spring::SpringAnimator::new(initial, config, move |value| {
            let mut value = value.clamp(0.0, 1.0);
            let prev = current_opacity.get();
            if is_closing.get() {
                value = value.min(prev);
            }
            current_opacity.set(value);
            (set_opacity.borrow_mut())(value);
        })
    }

    fn make_y_spring(
        &self,
        initial: f64,
        config: SpringConfig,
    ) -> ui_motion::spring::SpringAnimator {
        let set_y = Rc::clone(&self.set_y);
        let current_y_px = Rc::clone(&self.current_y_px);
        let is_closing = Rc::clone(&self.is_closing);
        ui_motion::spring::SpringAnimator::new(initial, config, move |value| {
            let mut value = value.clamp(-1000.0, 1000.0);
            let prev = current_y_px.get();
            if is_closing.get() {
                value = value.max(prev);
            }
            current_y_px.set(value);
            (set_y.borrow_mut())(value);
        })
    }

    fn reset_springs(&mut self, config: SpringConfig) {
        self.stop();
        self.height_spring = Some(self.make_height_spring(self.current_height_px.get(), config));
        self.opacity_spring = Some(self.make_opacity_spring(self.current_opacity.get(), config));
        self.y_spring = Some(self.make_y_spring(self.current_y_px.get(), config));
    }

    fn sync_initial_state(&mut self, open: bool) {
        self.last_measured_height_px = None;
        self.last_open = Some(open);

        if open {
            self.is_closing.set(false);
            (self.set_hidden.borrow_mut())(false);

            let height_px = (self.measure_height_px)().max(0.0);
            self.last_measured_height_px = Some(height_px);
            self.current_height_px.set(height_px);
            self.current_opacity.set(1.0);
            self.current_y_px.set(0.0);
            (self.set_height.borrow_mut())(height_px);
            (self.set_opacity.borrow_mut())(1.0);
            (self.set_y.borrow_mut())(0.0);
            self.reset_springs(self.motion.spring);
        } else {
            self.is_closing.set(true);
            (self.set_hidden.borrow_mut())(true);
            self.last_measured_height_px = Some(0.0);
            self.current_height_px.set(0.0);
            self.current_opacity.set(0.0);
            self.current_y_px.set(self.motion.panel_offset_y_px);
            (self.set_height.borrow_mut())(0.0);
            (self.set_opacity.borrow_mut())(0.0);
            (self.set_y.borrow_mut())(self.motion.panel_offset_y_px);
            self.reset_springs(self.motion.spring);
        }
    }

    fn set_open(&mut self, open: bool) {
        if self.last_open.is_none() {
            self.sync_initial_state(open);
            return;
        }
        if self.last_open == Some(open) {
            return;
        }
        self.last_open = Some(open);

        if open {
            self.open_panel();
        } else {
            self.close_panel();
        }
    }

    fn open_panel(&mut self) {
        self.is_closing.set(false);
        // Restore default motion contract and clear residual velocity from collapse.
        self.reset_springs(self.motion.spring);

        let Some(height) = self.height_spring.as_ref() else {
            return;
        };
        let Some(opacity) = self.opacity_spring.as_ref() else {
            return;
        };
        let Some(y) = self.y_spring.as_ref() else {
            return;
        };

        (self.set_hidden.borrow_mut())(false);

        let target_height = (self.measure_height_px)().max(0.0);
        self.last_measured_height_px = Some(target_height);

        height.clear_on_rest();
        height.set_target(target_height);

        opacity.clear_on_rest();
        opacity.set_target(1.0);

        y.clear_on_rest();
        y.set_target(0.0);
    }

    fn close_panel(&mut self) {
        self.is_closing.set(true);
        // Collapse uses a dedicated damped config to remove weird wobble on close.
        self.reset_springs(self.close_spring_config());

        let Some(height) = self.height_spring.as_ref() else {
            return;
        };
        let Some(opacity) = self.opacity_spring.as_ref() else {
            return;
        };
        let Some(y) = self.y_spring.as_ref() else {
            return;
        };

        opacity.clear_on_rest();
        let pending_hide = Rc::new(Cell::new(2_u8));
        let set_hidden_for_opacity = Rc::clone(&self.set_hidden);
        let pending_hide_for_opacity = Rc::clone(&pending_hide);
        opacity.set_on_rest(move || {
            let remaining = pending_hide_for_opacity.get().saturating_sub(1);
            pending_hide_for_opacity.set(remaining);
            if remaining == 0 {
                (set_hidden_for_opacity.borrow_mut())(true);
            }
        });
        opacity.set_target(0.0);

        y.clear_on_rest();
        y.set_target(self.motion.panel_offset_y_px);

        height.clear_on_rest();
        let set_hidden_for_height = Rc::clone(&self.set_hidden);
        let pending_hide_for_height = Rc::clone(&pending_hide);
        height.set_on_rest(move || {
            let remaining = pending_hide_for_height.get().saturating_sub(1);
            pending_hide_for_height.set(remaining);
            if remaining == 0 {
                (set_hidden_for_height.borrow_mut())(true);
            }
        });
        height.set_target(0.0);
        self.last_measured_height_px = Some(0.0);
    }

    fn sync_open_height(&mut self) {
        if self.last_open != Some(true) {
            return;
        }

        if self.height_spring.is_none() {
            return;
        }

        let measured_height = (self.measure_height_px)().max(0.0);
        if let Some(prev_height) = self.last_measured_height_px
            && (prev_height - measured_height).abs() < 0.5
        {
            return;
        }
        self.last_measured_height_px = Some(measured_height);
        self.current_height_px.set(measured_height);

        (self.set_height.borrow_mut())(measured_height);

        if let Some(height) = self.height_spring.take() {
            height.stop();
        }

        let height = self.make_height_spring(measured_height, self.motion.spring);
        self.height_spring = Some(height);
    }
}

#[cfg(target_arch = "wasm32")]
pub fn attach_panel_motion(
    panel_ref: leptos::prelude::NodeRef<leptos::html::Div>,
    surface_ref: leptos::prelude::NodeRef<leptos::html::Div>,
    is_open: leptos::prelude::Signal<bool>,
    is_hidden: leptos::prelude::RwSignal<bool>,
    motion: AccordionMotion,
    slot_projection: logic::AccordionSlotProjection,
    on_panel_lifecycle: leptos::prelude::Callback<logic::AccordionPanelLifecycleEvent>,
) {
    use leptos::prelude::*;
    use leptos::wasm_bindgen::{JsCast, closure::Closure};
    use ui_observability::set_css_property_observed;

    let motion = StoredValue::new(sanitize_motion(motion));
    let last_hidden = StoredValue::new_local(None::<bool>);
    if prefers_reduced_motion() {
        Effect::new(move |_| {
            let Some(panel) = panel_ref.get() else {
                return;
            };

            let element: leptos::web_sys::HtmlElement = panel.unchecked_into();
            let style = element.style();
            let motion = motion.get_value();
            let open = is_open.get();
            let hidden = !open;

            is_hidden.set(hidden);
            element.set_hidden(hidden);
            if last_hidden.get_value() != Some(hidden) {
                last_hidden.set_value(Some(hidden));
                on_panel_lifecycle.run(panel_lifecycle_event_from_hidden(hidden));
            }

            if open {
                set_css_property_observed(
                    &style,
                    "--ui-accordion-panel-height",
                    "auto",
                    "accordion.motion.panel.reduced.height",
                );
                set_css_property_observed(
                    &style,
                    "--ui-accordion-panel-opacity",
                    "1",
                    "accordion.motion.panel.reduced.opacity",
                );
                set_css_property_observed(
                    &style,
                    "--ui-accordion-panel-y",
                    "0px",
                    "accordion.motion.panel.reduced.y",
                );
            } else {
                set_css_property_observed(
                    &style,
                    "--ui-accordion-panel-height",
                    "0px",
                    "accordion.motion.panel.reduced.height",
                );
                set_css_property_observed(
                    &style,
                    "--ui-accordion-panel-opacity",
                    "0",
                    "accordion.motion.panel.reduced.opacity",
                );
                set_css_property_observed(
                    &style,
                    "--ui-accordion-panel-y",
                    &format!("{}px", motion.panel_offset_y_px),
                    "accordion.motion.panel.reduced.y",
                );
            }
        });
        return;
    }

    let driver = StoredValue::new_local(None::<Rc<RefCell<PanelMotionDriver>>>);
    let resize_observer = StoredValue::new_local(None::<leptos::web_sys::ResizeObserver>);
    let observed_surface = StoredValue::new_local(None::<leptos::web_sys::Element>);
    let is_observing = StoredValue::new_local(false);
    let resize_closure = StoredValue::new_local(
        None::<Closure<dyn FnMut(js_sys::Array, leptos::web_sys::ResizeObserver)>>,
    );

    Effect::new(move |_| {
        let motion = motion.get_value();

        let Some(panel) = panel_ref.get() else {
            return;
        };
        let Some(surface) = surface_ref.get() else {
            return;
        };
        if driver.get_value().is_some() {
            return;
        }

        let element: leptos::web_sys::HtmlElement = panel.unchecked_into();
        let style = element.style();
        let open_now = is_open.get_untracked();

        let set_hidden = {
            let element = element.clone();
            move |hidden: bool| {
                is_hidden.set(hidden);
                element.set_hidden(hidden);
                if last_hidden.get_value() != Some(hidden) {
                    last_hidden.set_value(Some(hidden));
                    on_panel_lifecycle.run(panel_lifecycle_event_from_hidden(hidden));
                }
            }
        };

        let set_height = {
            let style = style.clone();
            move |height_px: f64| {
                let height_px = height_px.clamp(0.0, 100000.0);
                set_css_property_observed(
                    &style,
                    "--ui-accordion-panel-height",
                    &format!("{height_px}px"),
                    "accordion.motion.panel.height",
                );
            }
        };

        let set_opacity = {
            let style = style.clone();
            move |opacity: f64| {
                set_css_property_observed(
                    &style,
                    "--ui-accordion-panel-opacity",
                    &format!("{opacity}"),
                    "accordion.motion.panel.opacity",
                );
            }
        };

        let set_y = {
            let style = style.clone();
            move |y_px: f64| {
                set_css_property_observed(
                    &style,
                    "--ui-accordion-panel-y",
                    &format!("{y_px}px"),
                    "accordion.motion.panel.y",
                );
            }
        };

        let measure_height_px = {
            let element = element.clone();
            move || element.scroll_height() as f64
        };

        let driver_instance = Rc::new(RefCell::new(PanelMotionDriver::new(
            motion,
            set_hidden,
            measure_height_px,
            set_height,
            set_opacity,
            set_y,
        )));

        driver_instance.borrow_mut().sync_initial_state(open_now);
        driver.set_value(Some(driver_instance));
        if last_hidden.get_value() != Some(!open_now) {
            last_hidden.set_value(Some(!open_now));
            on_panel_lifecycle.run(panel_lifecycle_event_from_hidden(!open_now));
        }

        if resize_observer.get_value().is_none() {
            let surface_element: leptos::web_sys::Element = surface.unchecked_into();
            observed_surface.set_value(Some(surface_element.clone()));
            let driver_for_resize = driver.get_value();
            if let Some(driver_for_resize) = driver_for_resize {
                let closure = Closure::wrap(Box::new(
                    move |_: js_sys::Array, _: leptos::web_sys::ResizeObserver| {
                        driver_for_resize.borrow_mut().sync_open_height();
                    },
                )
                    as Box<dyn FnMut(js_sys::Array, leptos::web_sys::ResizeObserver)>);

                if let Ok(observer) =
                    leptos::web_sys::ResizeObserver::new(closure.as_ref().unchecked_ref())
                {
                    let should_observe = !(slot_projection
                        == logic::AccordionSlotProjection::KeepAlive
                        && !open_now);
                    if should_observe {
                        observer.observe(&surface_element);
                        is_observing.set_value(true);
                    }
                    resize_observer.set_value(Some(observer));
                    resize_closure.set_value(Some(closure));
                }
            }
        }

        let driver_for_cleanup = driver;
        let resize_observer_for_cleanup = resize_observer;
        let resize_closure_for_cleanup = resize_closure;
        on_cleanup(move || {
            if let Some(observer) = resize_observer_for_cleanup.get_value() {
                observer.disconnect();
            }
            is_observing.set_value(false);
            resize_observer_for_cleanup.set_value(None);
            observed_surface.set_value(None);
            resize_closure_for_cleanup.set_value(None);

            if let Some(driver) = driver_for_cleanup.get_value() {
                driver.borrow_mut().stop();
            }
        });
    });

    Effect::new(move |_| {
        let open = is_open.get();
        let Some(driver) = driver.get_value() else {
            return;
        };
        driver.borrow_mut().set_open(open);
    });

    if slot_projection == logic::AccordionSlotProjection::KeepAlive {
        Effect::new(move |_| {
            let hidden = is_hidden.get();
            let Some(observer) = resize_observer.get_value() else {
                return;
            };
            let Some(surface) = observed_surface.get_value() else {
                return;
            };

            let observing = is_observing.get_value();
            if hidden && observing {
                observer.disconnect();
                is_observing.set_value(false);
                return;
            }
            if !hidden && !observing {
                observer.observe(&surface);
                is_observing.set_value(true);
            }
        });
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn attach_panel_motion(
    _node_ref: leptos::prelude::NodeRef<leptos::html::Div>,
    _surface_ref: leptos::prelude::NodeRef<leptos::html::Div>,
    is_open: leptos::prelude::Signal<bool>,
    is_hidden: leptos::prelude::RwSignal<bool>,
    motion: AccordionMotion,
    _slot_projection: logic::AccordionSlotProjection,
    on_panel_lifecycle: leptos::prelude::Callback<logic::AccordionPanelLifecycleEvent>,
) {
    use leptos::prelude::*;

    sanitize_motion(motion);
    let last_hidden = StoredValue::new_local(None::<bool>);

    Effect::new(move |_| {
        let hidden = !is_open.get();
        is_hidden.set(hidden);
        if last_hidden.get_value() != Some(hidden) {
            last_hidden.set_value(Some(hidden));
            on_panel_lifecycle.run(panel_lifecycle_event_from_hidden(hidden));
        }
    });
}

#[cfg(test)]
#[path = "../test/motion.rs"]
mod tests;
