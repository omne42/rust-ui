use super::ThemeMode;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ThemeToggleMotion {
    pub spring: ui_motion::spring::SpringConfig,
    pub rotate_deg: f64,
    pub scale_down: f64,
    pub scale_settle_delay_ms: u64,
}

impl Default for ThemeToggleMotion {
    fn default() -> Self {
        Self {
            spring: ui_motion::presets::spring_soft(),
            rotate_deg: 180.0,
            scale_down: 0.92,
            scale_settle_delay_ms: 40,
        }
    }
}

#[cfg(any(test, target_arch = "wasm32"))]
use std::{cell::Cell, rc::Rc};

#[cfg(target_arch = "wasm32")]
use std::cell::RefCell;

#[cfg(any(test, target_arch = "wasm32"))]
struct ThemeToggleMotionDriver {
    rotate: ui_motion::spring::SpringAnimator,
    scale: ui_motion::spring::SpringAnimator,
    latest_rotate_deg: Rc<Cell<f64>>,
}

#[cfg(any(test, target_arch = "wasm32"))]
impl ThemeToggleMotionDriver {
    fn new(
        config: ui_motion::spring::SpringConfig,
        mut set_rotate_deg: impl FnMut(f64) + 'static,
        mut set_scale: impl FnMut(f64) + 'static,
    ) -> Self {
        let latest_rotate_deg = Rc::new(Cell::new(0.0));

        let latest_for_rotate = Rc::clone(&latest_rotate_deg);
        let rotate = ui_motion::spring::SpringAnimator::new(0.0, config, move |v| {
            latest_for_rotate.set(v);
            set_rotate_deg(v);
        });

        let scale = ui_motion::spring::SpringAnimator::new(1.0, config, move |v| {
            set_scale(v);
        });

        Self {
            rotate,
            scale,
            latest_rotate_deg,
        }
    }

    fn stop(&self) {
        self.rotate.stop();
        self.scale.stop();
    }

    fn kick_rotate(&self, motion: ThemeToggleMotion) {
        let current = self.latest_rotate_deg.get();
        self.rotate.set_target(current + motion.rotate_deg);
    }

    #[cfg(test)]
    fn kick_scale_immediate(&self, motion: ThemeToggleMotion) {
        self.scale.set_target(motion.scale_down);
        self.scale.set_target(1.0);
    }
}

#[cfg(target_arch = "wasm32")]
pub fn attach_motion(
    icon_ref: leptos::prelude::NodeRef<leptos::html::Span>,
    mode: leptos::prelude::Signal<ThemeMode>,
    motion: ThemeToggleMotion,
) {
    use leptos::prelude::*;
    use leptos::wasm_bindgen::JsCast;

    let motion = StoredValue::new(motion);
    let driver = StoredValue::new_local(None::<Rc<RefCell<ThemeToggleMotionDriver>>>);
    let last_mode = StoredValue::new(None::<ThemeMode>);
    let scale_timeout = StoredValue::new_local(None::<TimeoutHandle>);

    Effect::new(move |_| {
        let config = motion.get_value().spring;
        let Some(span) = icon_ref.get() else {
            return;
        };
        if driver.get_value().is_some() {
            return;
        }

        let element: leptos::web_sys::HtmlElement = span.unchecked_into();
        let style = element.style();

        let _ = style.set_property("--ui-theme-toggle-rotate", "0deg");
        let _ = style.set_property("--ui-theme-toggle-scale", "1");

        let style_for_rotate = style.clone();
        let style_for_scale = style.clone();
        let driver_instance = Rc::new(RefCell::new(ThemeToggleMotionDriver::new(
            config,
            move |deg| {
                let deg = deg.clamp(-100000.0, 100000.0);
                let _ =
                    style_for_rotate.set_property("--ui-theme-toggle-rotate", &format!("{deg}deg"));
            },
            move |scale| {
                let scale = scale.clamp(0.0, 10.0);
                let _ =
                    style_for_scale.set_property("--ui-theme-toggle-scale", &format!("{scale}"));
            },
        )));

        driver.set_value(Some(Rc::clone(&driver_instance)));

        let driver_for_cleanup = driver;
        let scale_timeout_for_cleanup = scale_timeout;
        on_cleanup(move || {
            if let Some(handle) = scale_timeout_for_cleanup.get_value() {
                handle.clear();
            }
            scale_timeout_for_cleanup.set_value(None);

            if let Some(driver) = driver_for_cleanup.get_value() {
                driver.borrow().stop();
            }
        });
    });

    Effect::new(move |_| {
        let current = mode.get();
        let Some(prev) = last_mode.get_value() else {
            last_mode.set_value(Some(current));
            return;
        };

        if prev == current {
            return;
        }
        last_mode.set_value(Some(current));

        let Some(driver) = driver.get_value() else {
            return;
        };

        let motion = motion.get_value();
        driver.borrow().kick_rotate(motion);

        if let Some(handle) = scale_timeout.get_value() {
            handle.clear();
        }
        scale_timeout.set_value(None);

        driver.borrow().scale.set_target(motion.scale_down);

        let driver_for_timeout = Rc::clone(&driver);
        if let Ok(handle) = set_timeout_with_handle(
            move || driver_for_timeout.borrow().scale.set_target(1.0),
            std::time::Duration::from_millis(motion.scale_settle_delay_ms),
        ) {
            scale_timeout.set_value(Some(handle));
        }
    });
}

#[cfg(not(target_arch = "wasm32"))]
pub fn attach_motion(
    _icon_ref: leptos::prelude::NodeRef<leptos::html::Span>,
    _mode: leptos::prelude::Signal<ThemeMode>,
    _motion: ThemeToggleMotion,
) {
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{cell::RefCell, rc::Rc};

    #[test]
    fn default_motion_has_reasonable_params() {
        let motion = ThemeToggleMotion::default();
        assert_eq!(motion.spring, ui_motion::presets::spring_soft());
        assert!(motion.rotate_deg.abs() > 0.0);
        assert!(motion.scale_down > 0.0);
    }

    #[test]
    fn driver_kick_scale_applies_down_then_up() {
        let events: Rc<RefCell<Vec<f64>>> = Rc::new(RefCell::new(Vec::new()));
        let driver = ThemeToggleMotionDriver::new(ui_motion::presets::spring_soft(), |_| {}, {
            let events = Rc::clone(&events);
            move |scale| events.borrow_mut().push(scale)
        });

        driver.kick_scale_immediate(ThemeToggleMotion::default());

        assert_eq!(&*events.borrow(), &[0.92, 1.0]);

        driver.stop();
    }

    #[test]
    fn driver_kick_rotate_accumulates_degrees() {
        let events: Rc<RefCell<Vec<f64>>> = Rc::new(RefCell::new(Vec::new()));

        let driver = ThemeToggleMotionDriver::new(
            ui_motion::presets::spring_soft(),
            {
                let events = Rc::clone(&events);
                move |deg| events.borrow_mut().push(deg)
            },
            |_| {},
        );

        driver.kick_rotate(ThemeToggleMotion::default());
        driver.kick_rotate(ThemeToggleMotion::default());

        assert_eq!(&*events.borrow(), &[180.0, 360.0]);

        driver.stop();
    }
}
