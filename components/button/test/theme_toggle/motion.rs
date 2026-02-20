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
