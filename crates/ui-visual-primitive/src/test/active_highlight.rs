use super::*;
use std::cell::Cell;

#[derive(Debug, PartialEq)]
enum Event {
    Y(f64),
    H(f64),
    O(f64),
}

fn record(events: &Rc<RefCell<Vec<Event>>>, event: Event) {
    events.borrow_mut().push(event);
}

#[test]
fn driver_sync_layout_updates_all_css_vars() {
    let events: Rc<RefCell<Vec<Event>>> = Rc::new(RefCell::new(Vec::new()));

    let mut driver = ActiveHighlightMotionDriver::new(
        ui_motion::presets::spring_slide(),
        || None,
        {
            let events = Rc::clone(&events);
            move |v| record(&events, Event::Y(v))
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

    driver.sync_layout(Some(HighlightLayout {
        y_px: 12.0,
        height_px: 40.0,
    }));

    assert_eq!(
        &*events.borrow(),
        &[Event::Y(12.0), Event::H(40.0), Event::O(1.0)]
    );
}

#[test]
fn driver_sync_layout_noops_when_geometry_is_unchanged() {
    let events: Rc<RefCell<Vec<Event>>> = Rc::new(RefCell::new(Vec::new()));

    let mut driver = ActiveHighlightMotionDriver::new(
        ui_motion::presets::spring_slide(),
        || None,
        {
            let events = Rc::clone(&events);
            move |v| record(&events, Event::Y(v))
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

    let layout = HighlightLayout {
        y_px: 12.0,
        height_px: 40.0,
    };

    driver.sync_layout(Some(layout));
    events.borrow_mut().clear();

    driver.sync_layout(Some(layout));
    assert_eq!(&*events.borrow(), &[Event::O(1.0)]);
}

#[test]
fn driver_sync_layout_hides_when_layout_is_missing() {
    let events: Rc<RefCell<Vec<Event>>> = Rc::new(RefCell::new(Vec::new()));

    let mut driver = ActiveHighlightMotionDriver::new(
        ui_motion::presets::spring_slide(),
        || None,
        {
            let events = Rc::clone(&events);
            move |v| record(&events, Event::Y(v))
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
    let layout = Rc::new(Cell::new(HighlightLayout {
        y_px: 0.0,
        height_px: 34.0,
    }));

    let mut driver = ActiveHighlightMotionDriver::new(
        ui_motion::presets::spring_slide(),
        {
            let layout = Rc::clone(&layout);
            move || Some(layout.get())
        },
        {
            let events = Rc::clone(&events);
            move |v| record(&events, Event::Y(v))
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

    layout.set(HighlightLayout {
        y_px: 12.0,
        height_px: 42.0,
    });
    driver.sync_measured_layout();

    assert_eq!(
        &*events.borrow(),
        &[Event::Y(12.0), Event::H(42.0), Event::O(1.0)]
    );
}
