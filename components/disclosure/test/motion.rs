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

#[test]
fn panel_motion_sync_open_height_updates_height_only() {
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

    driver.sync_initial_state(true);
    events.borrow_mut().clear();

    measured_height.set(200.0);
    driver.sync_open_height();

    assert_eq!(&*events.borrow(), &[Event::Height(200.0)]);
}

#[test]
fn panel_motion_sync_open_height_noops_when_closed() {
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
    events.borrow_mut().clear();

    measured_height.set(200.0);
    driver.sync_open_height();

    assert!(events.borrow().is_empty());
}

#[test]
fn sanitize_motion_falls_back_for_invalid_values() {
    let motion = sanitize_motion(DisclosureMotion {
        spring: SpringConfig {
            stiffness: f64::NAN,
            damping: -1.0,
            mass: 0.0,
            precision: f64::INFINITY,
        },
        closed_rotation_deg: f64::NAN,
        open_rotation_deg: f64::INFINITY,
        panel_offset_y_px: f64::NAN,
    });

    let default = DisclosureMotion::default();
    assert_eq!(motion.spring.stiffness, default.spring.stiffness);
    assert_eq!(motion.spring.damping, default.spring.damping);
    assert_eq!(motion.spring.mass, default.spring.mass);
    assert_eq!(motion.spring.precision, default.spring.precision);
    assert_eq!(motion.closed_rotation_deg, default.closed_rotation_deg);
    assert_eq!(motion.open_rotation_deg, default.open_rotation_deg);
    assert_eq!(motion.panel_offset_y_px, default.panel_offset_y_px);
}

#[test]
fn sanitize_motion_clamps_rotation_and_offset_ranges() {
    let motion = sanitize_motion(DisclosureMotion {
        spring: SpringConfig {
            stiffness: 320.0,
            damping: 24.0,
            mass: 1.2,
            precision: 0.002,
        },
        closed_rotation_deg: -720.0,
        open_rotation_deg: 1080.0,
        panel_offset_y_px: -1000.0,
    });

    assert_eq!(motion.closed_rotation_deg, -360.0);
    assert_eq!(motion.open_rotation_deg, 360.0);
    assert_eq!(motion.panel_offset_y_px, 240.0);
    assert_eq!(motion.spring.stiffness, 320.0);
    assert_eq!(motion.spring.damping, 24.0);
    assert_eq!(motion.spring.mass, 1.2);
    assert_eq!(motion.spring.precision, 0.002);
}
