#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SlidingNumberMotion {
    pub spring: ui_motion::spring::SpringConfig,
    pub animate: bool,
}

impl Default for SlidingNumberMotion {
    fn default() -> Self {
        Self {
            spring: ui_motion::presets::spring_slide(),
            animate: true,
        }
    }
}

fn sanitize_spring(value: ui_motion::spring::SpringConfig) -> ui_motion::spring::SpringConfig {
    let default = SlidingNumberMotion::default().spring;

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

pub fn sanitize_motion(motion: SlidingNumberMotion) -> SlidingNumberMotion {
    SlidingNumberMotion {
        spring: sanitize_spring(motion.spring),
        animate: motion.animate && !ui_motion::web::prefers_reduced_motion(),
    }
}

#[cfg(any(test, target_arch = "wasm32"))]
use std::{cell::Cell, cell::RefCell, rc::Rc};

#[cfg(any(test, target_arch = "wasm32"))]
const DIGIT_CYCLE: i32 = 10;

#[cfg(any(test, target_arch = "wasm32"))]
type ApplyFn = Rc<RefCell<Box<dyn FnMut(f64)>>>;

#[cfg(any(test, target_arch = "wasm32"))]
fn normalized_offset(digit: u8) -> f64 {
    // Keep the resting state in the middle cycle so wrap-around moves stay within bounds.
    10.0 + f64::from(digit)
}

#[cfg(any(test, target_arch = "wasm32"))]
fn current_digit(offset: f64) -> u8 {
    let digit = (offset.round() as i32).rem_euclid(DIGIT_CYCLE);
    digit.clamp(0, 9) as u8
}

#[cfg(any(test, target_arch = "wasm32"))]
fn shortest_delta(from: u8, to: u8) -> i32 {
    let from = i32::from(from);
    let to = i32::from(to);
    let diff = to - from;
    let forward = diff.rem_euclid(DIGIT_CYCLE);
    let backward = forward - DIGIT_CYCLE;

    let abs_forward = forward.abs();
    let abs_backward = backward.abs();

    if abs_forward < abs_backward {
        forward
    } else if abs_backward < abs_forward {
        backward
    } else if diff >= 0 {
        forward
    } else {
        backward
    }
}

#[cfg(any(test, target_arch = "wasm32"))]
struct SlidingNumberRollerDriver {
    offset: ui_motion::spring::SpringAnimator,
    latest_offset: Rc<Cell<f64>>,
    apply: ApplyFn,
}

#[cfg(any(test, target_arch = "wasm32"))]
impl SlidingNumberRollerDriver {
    fn new(
        initial_digit: u8,
        config: ui_motion::spring::SpringConfig,
        apply: impl FnMut(f64) + 'static,
    ) -> Self {
        let initial_offset = normalized_offset(initial_digit);
        let latest_offset = Rc::new(Cell::new(initial_offset));
        let apply: ApplyFn = Rc::new(RefCell::new(Box::new(apply)));

        let latest_for_spring = Rc::clone(&latest_offset);
        let apply_for_spring = Rc::clone(&apply);
        let offset = ui_motion::spring::SpringAnimator::new(initial_offset, config, move |v| {
            latest_for_spring.set(v);
            (apply_for_spring.borrow_mut())(v);
        });

        Self {
            offset,
            latest_offset,
            apply,
        }
    }

    fn stop(&self) {
        self.offset.stop();
    }

    fn set_digit(&mut self, digit: u8) {
        let current_offset = self.latest_offset.get();
        let from = current_digit(current_offset);
        let delta = shortest_delta(from, digit);
        let target = current_offset + f64::from(delta);
        let normalized = normalized_offset(digit);

        let latest_for_rest = Rc::clone(&self.latest_offset);
        let apply_for_rest = Rc::clone(&self.apply);
        self.offset.clear_on_rest();
        self.offset.set_on_rest(move || {
            let current = latest_for_rest.get();
            if (current - normalized).abs() < 0.0001 {
                return;
            }
            latest_for_rest.set(normalized);
            (apply_for_rest.borrow_mut())(normalized);
        });

        self.offset.set_target(target);
    }
}

#[cfg(target_arch = "wasm32")]
pub fn attach_motion(
    roller_ref: leptos::prelude::NodeRef<leptos::html::Span>,
    digit: leptos::prelude::Signal<u8>,
    motion: SlidingNumberMotion,
) {
    use leptos::prelude::*;
    use leptos::wasm_bindgen::JsCast;

    let motion = sanitize_motion(motion);

    if !motion.animate {
        return;
    }

    let motion = StoredValue::new(motion);
    let driver = StoredValue::new_local(None::<Rc<RefCell<SlidingNumberRollerDriver>>>);

    Effect::new(move |_| {
        let config = motion.get_value().spring;
        let Some(span) = roller_ref.get() else {
            return;
        };
        if driver.get_value().is_some() {
            return;
        }

        let element: leptos::web_sys::HtmlElement = span.unchecked_into();
        let style = element.style();

        let initial_digit = digit.get_untracked();
        let initial_offset = normalized_offset(initial_digit);
        drop(style.set_property("--ui-sliding-number-offset", &format!("{initial_offset}")));
        let style_for_offset = style.clone();
        let driver_instance = Rc::new(RefCell::new(SlidingNumberRollerDriver::new(
            initial_digit,
            config,
            move |v| {
                let v = v.clamp(-1000.0, 1000.0);
                drop(style_for_offset.set_property("--ui-sliding-number-offset", &format!("{v}")));
            },
        )));

        driver.set_value(Some(Rc::clone(&driver_instance)));

        let driver_for_cleanup = driver;
        on_cleanup(move || {
            if let Some(driver) = driver_for_cleanup.get_value() {
                driver.borrow().stop();
            }
        });
    });

    Effect::new(move |_| {
        let digit_value = digit.get();
        drop(roller_ref.get());
        let Some(driver) = driver.get_value() else {
            return;
        };
        driver.borrow_mut().set_digit(digit_value);
    });
}

#[cfg(not(target_arch = "wasm32"))]
pub fn attach_motion(
    _roller_ref: leptos::prelude::NodeRef<leptos::html::Span>,
    _digit: leptos::prelude::Signal<u8>,
    motion: SlidingNumberMotion,
) {
    std::hint::black_box(sanitize_motion(motion)); // drop(sanitize_motion(motion));
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{cell::RefCell, rc::Rc};

    #[test]
    fn default_motion_animates_with_slide_spring() {
        let motion = SlidingNumberMotion::default();
        assert!(motion.animate);
        assert_eq!(motion.spring, ui_motion::presets::spring_slide());
    }

    #[test]
    fn sanitize_motion_falls_back_for_invalid_values() {
        let default = SlidingNumberMotion::default();

        let motion = sanitize_motion(SlidingNumberMotion {
            spring: ui_motion::spring::SpringConfig {
                stiffness: f64::NAN,
                damping: -1.0,
                mass: 0.0,
                precision: f64::INFINITY,
            },
            animate: true,
        });

        assert_eq!(motion.spring.stiffness, default.spring.stiffness);
        assert_eq!(motion.spring.damping, default.spring.damping);
        assert_eq!(motion.spring.mass, default.spring.mass);
        assert_eq!(motion.spring.precision, default.spring.precision);
        assert_eq!(motion.animate, !ui_motion::web::prefers_reduced_motion());
    }

    #[test]
    fn supports_custom_spring_motion_contract() {
        let motion = sanitize_motion(SlidingNumberMotion {
            spring: ui_motion::spring::SpringConfig {
                stiffness: 330.0,
                damping: 26.0,
                mass: 0.9,
                precision: 0.002,
            },
            animate: false,
        });

        assert_eq!(motion.spring.stiffness, 330.0);
        assert_eq!(motion.spring.damping, 26.0);
        assert_eq!(motion.spring.mass, 0.9);
        assert_eq!(motion.spring.precision, 0.002);
        assert!(!motion.animate);
    }

    #[test]
    fn shortest_delta_prefers_wraparound() {
        assert_eq!(shortest_delta(9, 0), 1);
        assert_eq!(shortest_delta(0, 9), -1);
        assert_eq!(shortest_delta(1, 6), 5);
        assert_eq!(shortest_delta(6, 1), -5);
    }

    #[test]
    fn driver_wraps_and_recenters_after_rest() {
        let values: Rc<RefCell<Vec<f64>>> = Rc::new(RefCell::new(Vec::new()));
        let mut driver = SlidingNumberRollerDriver::new(9, ui_motion::presets::spring_slide(), {
            let values = Rc::clone(&values);
            move |v| values.borrow_mut().push(v)
        });

        driver.set_digit(0);

        assert_eq!(&*values.borrow(), &[20.0, 10.0]);

        driver.stop();
    }
}
