use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};

use ui_motion::spring::{SpringAnimator, SpringConfig};

#[derive(Debug, PartialEq)]
enum Event {
    Apply(f64),
    Rest,
}

#[test]
fn reduced_motion_set_target_applies_immediately() {
    let applied: Rc<RefCell<Vec<f64>>> = Rc::new(RefCell::new(Vec::new()));
    let applied_for_cb = Rc::clone(&applied);

    let animator = SpringAnimator::new(0.0, SpringConfig::default(), move |value| {
        applied_for_cb.borrow_mut().push(value);
    });

    animator.set_target(42.0);

    assert_eq!(&*applied.borrow(), &[42.0]);
}

#[test]
fn reduced_motion_set_target_triggers_on_rest_synchronously() {
    let events: Rc<RefCell<Vec<Event>>> = Rc::new(RefCell::new(Vec::new()));

    let events_for_apply = Rc::clone(&events);
    let animator = SpringAnimator::new(0.0, SpringConfig::default(), move |value| {
        events_for_apply.borrow_mut().push(Event::Apply(value));
    });

    let events_for_rest = Rc::clone(&events);
    animator.set_on_rest(move || {
        events_for_rest.borrow_mut().push(Event::Rest);
    });

    animator.set_target(1.0);

    assert_eq!(&*events.borrow(), &[Event::Apply(1.0), Event::Rest]);
}

#[test]
fn reduced_motion_clear_on_rest_stops_triggering() {
    let rest_count: Rc<Cell<u32>> = Rc::new(Cell::new(0));
    let rest_count_for_cb = Rc::clone(&rest_count);

    let animator = SpringAnimator::new(0.0, SpringConfig::default(), |_| {});
    animator.set_on_rest(move || {
        rest_count_for_cb.set(rest_count_for_cb.get() + 1);
    });

    animator.set_target(1.0);
    assert_eq!(rest_count.get(), 1);

    animator.clear_on_rest();
    animator.set_target(2.0);
    assert_eq!(rest_count.get(), 1);
}
