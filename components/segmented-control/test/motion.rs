use super::*;
use std::cell::Cell;

#[derive(Debug, PartialEq)]
enum Event {
    X(f64),
    Y(f64),
    W(f64),
    H(f64),
    O(f64),
}

fn record(events: &Rc<RefCell<Vec<Event>>>, event: Event) {
    events.borrow_mut().push(event);
}

#[test]
fn sanitize_motion_falls_back_for_invalid_values() {
    let default = SegmentedControlMotion::default();

    let motion = sanitize_motion(SegmentedControlMotion {
        spring: ui_motion::spring::SpringConfig {
            stiffness: f64::NAN,
            damping: -1.0,
            mass: 0.0,
            precision: f64::INFINITY,
        },
    });

    assert_eq!(motion.spring.stiffness, default.spring.stiffness);
    assert_eq!(motion.spring.damping, default.spring.damping);
    assert_eq!(motion.spring.mass, default.spring.mass);
    assert_eq!(motion.spring.precision, default.spring.precision);
}

#[test]
fn supports_custom_spring_motion_contract() {
    let motion = sanitize_motion(SegmentedControlMotion {
        spring: ui_motion::spring::SpringConfig {
            stiffness: 330.0,
            damping: 26.0,
            mass: 0.9,
            precision: 0.002,
        },
    });

    assert_eq!(motion.spring.stiffness, 330.0);
    assert_eq!(motion.spring.damping, 26.0);
    assert_eq!(motion.spring.mass, 0.9);
    assert_eq!(motion.spring.precision, 0.002);
}

#[test]
fn indicator_layout_subtracts_inset() {
    let container = Rect::new(0.0, 0.0, 240.0, 42.0);
    let option = Rect::new(4.0, 4.0, 80.0, 34.0);

    assert_eq!(
        compute_indicator_layout(container, option),
        IndicatorLayout {
            x_px: 0.0,
            y_px: 0.0,
            width_px: 80.0,
            height_px: 34.0
        }
    );
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
            move |v| record(&events, Event::Y(v))
        },
        {
            let events = Rc::clone(&events);
            move |v| record(&events, Event::W(v))
        },
        {
            let events = Rc::clone(&events);
            move |v| record(&events, Event::H(v))
        },
        {
            let events = Rc::clone(&events);
            move |v| record(&events, Event::O(v))
        },
    );

    driver.sync_layout(Some(IndicatorLayout {
        x_px: 12.0,
        y_px: 0.0,
        width_px: 100.0,
        height_px: 34.0,
    }));

    assert_eq!(
        &*events.borrow(),
        &[
            Event::X(12.0),
            Event::Y(0.0),
            Event::W(100.0),
            Event::H(34.0),
            Event::O(1.0),
        ]
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
            move |v| record(&events, Event::Y(v))
        },
        {
            let events = Rc::clone(&events);
            move |v| record(&events, Event::W(v))
        },
        {
            let events = Rc::clone(&events);
            move |v| record(&events, Event::H(v))
        },
        {
            let events = Rc::clone(&events);
            move |v| record(&events, Event::O(v))
        },
    );

    let layout = IndicatorLayout {
        x_px: 0.0,
        y_px: 0.0,
        width_px: 88.0,
        height_px: 34.0,
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
            move |v| record(&events, Event::Y(v))
        },
        {
            let events = Rc::clone(&events);
            move |v| record(&events, Event::W(v))
        },
        {
            let events = Rc::clone(&events);
            move |v| record(&events, Event::H(v))
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
        y_px: 0.0,
        width_px: 80.0,
        height_px: 34.0,
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
            move |v| record(&events, Event::Y(v))
        },
        {
            let events = Rc::clone(&events);
            move |v| record(&events, Event::W(v))
        },
        {
            let events = Rc::clone(&events);
            move |v| record(&events, Event::H(v))
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
        y_px: 0.0,
        width_px: 100.0,
        height_px: 34.0,
    });
    driver.sync_measured_layout();

    assert_eq!(
        &*events.borrow(),
        &[
            Event::X(12.0),
            Event::Y(0.0),
            Event::W(100.0),
            Event::H(34.0),
            Event::O(1.0),
        ]
    );
}
