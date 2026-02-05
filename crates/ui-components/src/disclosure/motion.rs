use ui_motion::spring::SpringConfig;

#[cfg(any(test, target_arch = "wasm32"))]
use std::{cell::RefCell, rc::Rc};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DisclosureMotion {
    pub spring: SpringConfig,
    pub closed_rotation_deg: f64,
    pub open_rotation_deg: f64,
    pub panel_offset_y_px: f64,
}

impl Default for DisclosureMotion {
    fn default() -> Self {
        Self {
            spring: SpringConfig {
                stiffness: 260.0,
                damping: 18.0,
                mass: 1.0,
                ..Default::default()
            },
            closed_rotation_deg: 0.0,
            open_rotation_deg: 90.0,
            panel_offset_y_px: 4.0,
        }
    }
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
    motion: DisclosureMotion,
    set_hidden: SetHidden,
    measure_height_px: MeasureHeight,
    set_height: SetPanelHeight,
    set_opacity: SetPanelOpacity,
    set_y: SetPanelY,
    height_spring: Option<ui_motion::spring::SpringAnimator>,
    opacity_spring: Option<ui_motion::spring::SpringAnimator>,
    y_spring: Option<ui_motion::spring::SpringAnimator>,
    last_open: Option<bool>,
}

#[cfg(any(test, target_arch = "wasm32"))]
impl PanelMotionDriver {
    fn new(
        motion: DisclosureMotion,
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
            last_open: None,
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

    fn sync_initial_state(&mut self, open: bool) {
        self.stop();
        self.last_open = Some(open);
        if open {
            (self.set_hidden.borrow_mut())(false);

            let height_px = (self.measure_height_px)().max(0.0);

            (self.set_height.borrow_mut())(height_px);
            (self.set_opacity.borrow_mut())(1.0);
            (self.set_y.borrow_mut())(0.0);

            let config = self.motion.spring;
            let set_height = Rc::clone(&self.set_height);
            let height = ui_motion::spring::SpringAnimator::new(height_px, config, move |v| {
                let v = v.clamp(0.0, 100000.0);
                (set_height.borrow_mut())(v);
            });

            let set_opacity = Rc::clone(&self.set_opacity);
            let opacity = ui_motion::spring::SpringAnimator::new(1.0, config, move |v| {
                let v = v.clamp(0.0, 1.0);
                (set_opacity.borrow_mut())(v);
            });

            let set_y = Rc::clone(&self.set_y);
            let y = ui_motion::spring::SpringAnimator::new(0.0, config, move |v| {
                let v = v.clamp(-1000.0, 1000.0);
                (set_y.borrow_mut())(v);
            });

            self.height_spring = Some(height);
            self.opacity_spring = Some(opacity);
            self.y_spring = Some(y);
        } else {
            (self.set_hidden.borrow_mut())(true);
            (self.set_height.borrow_mut())(0.0);
            (self.set_opacity.borrow_mut())(0.0);
            (self.set_y.borrow_mut())(self.motion.panel_offset_y_px);

            let config = self.motion.spring;
            let set_height = Rc::clone(&self.set_height);
            let height = ui_motion::spring::SpringAnimator::new(0.0, config, move |v| {
                let v = v.clamp(0.0, 100000.0);
                (set_height.borrow_mut())(v);
            });

            let set_opacity = Rc::clone(&self.set_opacity);
            let opacity = ui_motion::spring::SpringAnimator::new(0.0, config, move |v| {
                let v = v.clamp(0.0, 1.0);
                (set_opacity.borrow_mut())(v);
            });

            let set_y = Rc::clone(&self.set_y);
            let y = ui_motion::spring::SpringAnimator::new(
                self.motion.panel_offset_y_px,
                config,
                move |v| {
                    let v = v.clamp(-1000.0, 1000.0);
                    (set_y.borrow_mut())(v);
                },
            );

            self.height_spring = Some(height);
            self.opacity_spring = Some(opacity);
            self.y_spring = Some(y);
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

        height.clear_on_rest();
        height.set_target(target_height);

        opacity.clear_on_rest();
        opacity.set_target(1.0);

        y.clear_on_rest();
        y.set_target(0.0);
    }

    fn close_panel(&mut self) {
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
        opacity.set_target(0.0);

        y.clear_on_rest();
        y.set_target(self.motion.panel_offset_y_px);

        height.clear_on_rest();
        let set_hidden = Rc::clone(&self.set_hidden);
        height.set_on_rest(move || {
            (set_hidden.borrow_mut())(true);
        });
        height.set_target(0.0);
    }
}

#[cfg(target_arch = "wasm32")]
pub fn attach_indicator_motion(
    node_ref: leptos::prelude::NodeRef<leptos::html::Span>,
    is_open: leptos::prelude::Signal<bool>,
    motion: DisclosureMotion,
) {
    use leptos::prelude::*;
    use leptos::wasm_bindgen::JsCast;

    let motion = StoredValue::new(motion);
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
        let initial = motion.get_value().closed_rotation_deg;

        let animator = ui_motion::spring::SpringAnimator::new(initial, config, move |deg| {
            let _ = style.set_property("--ui-disclosure-indicator-rotation", &format!("{deg}deg"));
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
            motion.open_rotation_deg
        } else {
            motion.closed_rotation_deg
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
    _motion: DisclosureMotion,
) {
}

#[cfg(target_arch = "wasm32")]
pub fn attach_panel_motion(
    node_ref: leptos::prelude::NodeRef<leptos::html::Div>,
    is_open: leptos::prelude::Signal<bool>,
    is_hidden: leptos::prelude::RwSignal<bool>,
    motion: DisclosureMotion,
) {
    use leptos::prelude::*;
    use leptos::wasm_bindgen::JsCast;

    let motion = StoredValue::new(motion);
    let driver = StoredValue::new_local(None::<Rc<RefCell<PanelMotionDriver>>>);

    Effect::new(move |_| {
        let motion = motion.get_value();

        let Some(panel) = node_ref.get() else {
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
            }
        };

        let set_height = {
            let style = style.clone();
            move |height_px: f64| {
                let height_px = height_px.clamp(0.0, 100000.0);
                let _ =
                    style.set_property("--ui-disclosure-panel-height", &format!("{height_px}px"));
            }
        };

        let set_opacity = {
            let style = style.clone();
            move |opacity: f64| {
                let _ = style.set_property("--ui-disclosure-panel-opacity", &format!("{opacity}"));
            }
        };

        let set_y = {
            let style = style.clone();
            move |y_px: f64| {
                let _ = style.set_property("--ui-disclosure-panel-y", &format!("{y_px}px"));
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

        let driver_for_cleanup = driver;
        on_cleanup(move || {
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
}

#[cfg(not(target_arch = "wasm32"))]
pub fn attach_panel_motion(
    _node_ref: leptos::prelude::NodeRef<leptos::html::Div>,
    is_open: leptos::prelude::Signal<bool>,
    is_hidden: leptos::prelude::RwSignal<bool>,
    _motion: DisclosureMotion,
) {
    use leptos::prelude::*;

    Effect::new(move |_| {
        is_hidden.set(!is_open.get());
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;

    #[derive(Debug, PartialEq)]
    enum Event {
        Hidden(bool),
        Height(f64),
        Opacity(f64),
        Y(f64),
    }

    fn record(events: &Rc<RefCell<Vec<Event>>>, event: Event) {
        events.borrow_mut().push(event);
    }

    #[test]
    fn panel_motion_initial_state_tracks_open_flag() {
        let events: Rc<RefCell<Vec<Event>>> = Rc::new(RefCell::new(Vec::new()));
        let measured_height = Rc::new(Cell::new(120.0));

        let mut driver = PanelMotionDriver::new(
            DisclosureMotion::default(),
            {
                let events = Rc::clone(&events);
                move |hidden| record(&events, Event::Hidden(hidden))
            },
            {
                let measured_height = Rc::clone(&measured_height);
                move || measured_height.get()
            },
            {
                let events = Rc::clone(&events);
                move |height| record(&events, Event::Height(height))
            },
            {
                let events = Rc::clone(&events);
                move |opacity| record(&events, Event::Opacity(opacity))
            },
            {
                let events = Rc::clone(&events);
                move |y| record(&events, Event::Y(y))
            },
        );

        driver.sync_initial_state(false);
        assert_eq!(
            &*events.borrow(),
            &[
                Event::Hidden(true),
                Event::Height(0.0),
                Event::Opacity(0.0),
                Event::Y(DisclosureMotion::default().panel_offset_y_px),
            ]
        );

        events.borrow_mut().clear();
        driver.sync_initial_state(true);
        assert_eq!(
            &*events.borrow(),
            &[
                Event::Hidden(false),
                Event::Height(120.0),
                Event::Opacity(1.0),
                Event::Y(0.0),
            ]
        );
    }

    #[test]
    fn panel_motion_open_then_close_records_expected_sequence() {
        let events: Rc<RefCell<Vec<Event>>> = Rc::new(RefCell::new(Vec::new()));
        let measured_height = Rc::new(Cell::new(200.0));

        let mut driver = PanelMotionDriver::new(
            DisclosureMotion::default(),
            {
                let events = Rc::clone(&events);
                move |hidden| record(&events, Event::Hidden(hidden))
            },
            {
                let measured_height = Rc::clone(&measured_height);
                move || measured_height.get()
            },
            {
                let events = Rc::clone(&events);
                move |height| record(&events, Event::Height(height))
            },
            {
                let events = Rc::clone(&events);
                move |opacity| record(&events, Event::Opacity(opacity))
            },
            {
                let events = Rc::clone(&events);
                move |y| record(&events, Event::Y(y))
            },
        );

        driver.sync_initial_state(false);
        events.borrow_mut().clear();

        driver.set_open(true);
        assert_eq!(
            &*events.borrow(),
            &[
                Event::Hidden(false),
                Event::Height(200.0),
                Event::Opacity(1.0),
                Event::Y(0.0),
            ]
        );

        events.borrow_mut().clear();
        driver.set_open(false);
        assert_eq!(
            &*events.borrow(),
            &[
                Event::Opacity(0.0),
                Event::Y(DisclosureMotion::default().panel_offset_y_px),
                Event::Height(0.0),
                Event::Hidden(true),
            ]
        );
    }

    #[test]
    fn panel_motion_noops_when_state_is_unchanged() {
        let events: Rc<RefCell<Vec<Event>>> = Rc::new(RefCell::new(Vec::new()));

        let mut driver = PanelMotionDriver::new(
            DisclosureMotion::default(),
            {
                let events = Rc::clone(&events);
                move |hidden| record(&events, Event::Hidden(hidden))
            },
            move || 80.0,
            {
                let events = Rc::clone(&events);
                move |height| record(&events, Event::Height(height))
            },
            {
                let events = Rc::clone(&events);
                move |opacity| record(&events, Event::Opacity(opacity))
            },
            {
                let events = Rc::clone(&events);
                move |y| record(&events, Event::Y(y))
            },
        );

        driver.sync_initial_state(false);
        events.borrow_mut().clear();

        driver.set_open(false);
        assert!(events.borrow().is_empty());

        driver.set_open(true);
        assert!(!events.borrow().is_empty());

        events.borrow_mut().clear();
        driver.set_open(true);
        assert!(events.borrow().is_empty());
    }
}
