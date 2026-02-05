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
        let _ = style.set_property("--ui-sliding-number-offset", &format!("{initial_offset}"));

        let style_for_offset = style.clone();
        let driver_instance = Rc::new(RefCell::new(SlidingNumberRollerDriver::new(
            initial_digit,
            config,
            move |v| {
                let v = v.clamp(-1000.0, 1000.0);
                let _ =
                    style_for_offset.set_property("--ui-sliding-number-offset", &format!("{v}"));
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
        let _ = roller_ref.get();

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
    _motion: SlidingNumberMotion,
) {
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
