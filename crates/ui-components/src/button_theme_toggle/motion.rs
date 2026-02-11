use super::ThemeMode;

const MAX_ROTATE_DEG: f64 = 3600.0;
const MIN_SCALE_DOWN: f64 = 0.1;
const MAX_SCALE_DOWN: f64 = 1.0;
const MAX_SETTLE_DELAY_MS: u64 = 2_000;

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

fn sanitize_number(value: f64, fallback: f64) -> f64 {
    if value.is_finite() { value } else { fallback }
}

fn sanitize_spring(value: ui_motion::spring::SpringConfig) -> ui_motion::spring::SpringConfig {
    let default = ThemeToggleMotion::default().spring;

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

pub fn sanitize_motion(motion: ThemeToggleMotion) -> ThemeToggleMotion {
    let default = ThemeToggleMotion::default();

    ThemeToggleMotion {
        spring: sanitize_spring(motion.spring),
        rotate_deg: sanitize_number(motion.rotate_deg, default.rotate_deg)
            .clamp(-MAX_ROTATE_DEG, MAX_ROTATE_DEG),
        scale_down: sanitize_number(motion.scale_down, default.scale_down)
            .clamp(MIN_SCALE_DOWN, MAX_SCALE_DOWN),
        scale_settle_delay_ms: motion.scale_settle_delay_ms.min(MAX_SETTLE_DELAY_MS),
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

    let motion = StoredValue::new(sanitize_motion(motion));
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
    motion: ThemeToggleMotion,
) {
    let _ = sanitize_motion(motion);
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

    #[test]
    fn sanitize_motion_falls_back_and_clamps_values() {
        let motion = sanitize_motion(ThemeToggleMotion {
            spring: ui_motion::spring::SpringConfig {
                stiffness: f64::NAN,
                damping: -1.0,
                mass: 0.0,
                precision: f64::INFINITY,
            },
            rotate_deg: f64::NAN,
            scale_down: f64::NAN,
            scale_settle_delay_ms: u64::MAX,
        });

        let default = ThemeToggleMotion::default();
        assert_eq!(motion.spring.stiffness, default.spring.stiffness);
        assert_eq!(motion.spring.damping, default.spring.damping);
        assert_eq!(motion.spring.mass, default.spring.mass);
        assert_eq!(motion.spring.precision, default.spring.precision);
        assert_eq!(motion.rotate_deg, default.rotate_deg);
        assert_eq!(motion.scale_down, default.scale_down);
        assert_eq!(motion.scale_settle_delay_ms, MAX_SETTLE_DELAY_MS);
    }

    #[test]
    fn sanitize_motion_keeps_valid_values() {
        let motion = sanitize_motion(ThemeToggleMotion {
            spring: ui_motion::spring::SpringConfig {
                stiffness: 300.0,
                damping: 20.0,
                mass: 1.2,
                precision: 0.003,
            },
            rotate_deg: -720.0,
            scale_down: 0.88,
            scale_settle_delay_ms: 120,
        });

        assert_eq!(motion.spring.stiffness, 300.0);
        assert_eq!(motion.spring.damping, 20.0);
        assert_eq!(motion.spring.mass, 1.2);
        assert_eq!(motion.spring.precision, 0.003);
        assert_eq!(motion.rotate_deg, -720.0);
        assert_eq!(motion.scale_down, 0.88);
        assert_eq!(motion.scale_settle_delay_ms, 120);
    }
}
