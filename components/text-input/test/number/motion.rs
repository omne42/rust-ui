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
