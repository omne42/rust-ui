use super::*;
use std::cell::Cell;

#[derive(Debug, PartialEq)]
enum Event {
    X(f64),
    W(f64),
    O(f64),
}

fn record(events: &Rc<RefCell<Vec<Event>>>, event: Event) {
    events.borrow_mut().push(event);
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
            move |v| record(&events, Event::W(v))
        },
        {
            let events = Rc::clone(&events);
            move |v| record(&events, Event::O(v))
        },
    );

    driver.sync_layout(Some(IndicatorLayout {
        x_px: 12.0,
        width_px: 88.0,
    }));

    assert_eq!(
        &*events.borrow(),
        &[Event::X(12.0), Event::W(88.0), Event::O(1.0)]
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
            move |v| record(&events, Event::W(v))
        },
        {
            let events = Rc::clone(&events);
            move |v| record(&events, Event::O(v))
        },
    );

    let layout = IndicatorLayout {
        x_px: 0.0,
        width_px: 88.0,
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
            move |v| record(&events, Event::W(v))
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
        width_px: 80.0,
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
            move |v| record(&events, Event::W(v))
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
        width_px: 100.0,
    });
    driver.sync_measured_layout();

    assert_eq!(
        &*events.borrow(),
        &[Event::X(12.0), Event::W(100.0), Event::O(1.0)]
    );
}

#[test]
fn sanitize_motion_falls_back_for_invalid_spring_values() {
    let motion = sanitize_motion(TabsMotion {
        spring: ui_motion::spring::SpringConfig {
            stiffness: f64::NAN,
            damping: -1.0,
            mass: 0.0,
            precision: f64::INFINITY,
        },
    });

    let default = TabsMotion::default().spring;
    assert_eq!(motion.spring.stiffness, default.stiffness);
    assert_eq!(motion.spring.damping, default.damping);
    assert_eq!(motion.spring.mass, default.mass);
    assert_eq!(motion.spring.precision, default.precision);
}

#[test]
fn sanitize_motion_keeps_valid_custom_spring_values() {
    let motion = sanitize_motion(TabsMotion {
        spring: ui_motion::spring::SpringConfig {
            stiffness: 280.0,
            damping: 22.0,
            mass: 1.1,
            precision: 0.002,
        },
    });

    assert_eq!(motion.spring.stiffness, 280.0);
    assert_eq!(motion.spring.damping, 22.0);
    assert_eq!(motion.spring.mass, 1.1);
    assert_eq!(motion.spring.precision, 0.002);
}
