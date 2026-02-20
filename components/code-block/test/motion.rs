use super::*;
use std::{cell::RefCell, rc::Rc};

#[test]
fn default_motion_uses_soft_spring() {
    let motion = CodeBlockMotion::default();
    assert_eq!(motion.spring, ui_motion::presets::spring_soft());
    assert!(motion.flash_hold_ms > 0);
}

#[test]
fn sanitize_motion_falls_back_for_invalid_values() {
    let default = CodeBlockMotion::default();

    let motion = sanitize_motion(CodeBlockMotion {
        spring: ui_motion::spring::SpringConfig {
            stiffness: f64::NAN,
            damping: -1.0,
            mass: 0.0,
            precision: f64::INFINITY,
        },
        flash_hold_ms: 0,
    });

    assert_eq!(motion.spring.stiffness, default.spring.stiffness);
    assert_eq!(motion.spring.damping, default.spring.damping);
    assert_eq!(motion.spring.mass, default.spring.mass);
    assert_eq!(motion.spring.precision, default.spring.precision);
    assert_eq!(motion.flash_hold_ms, default.flash_hold_ms);

    let capped = sanitize_motion(CodeBlockMotion {
        flash_hold_ms: 99_999,
        ..CodeBlockMotion::default()
    });
    assert_eq!(capped.flash_hold_ms, 10_000);
}

#[test]
fn flash_driver_triggers_peak_and_reset() {
    let values: Rc<RefCell<Vec<f64>>> = Rc::new(RefCell::new(Vec::new()));

    let driver = CopyFlashDriver::new(ui_motion::presets::spring_soft(), {
        let values = Rc::clone(&values);
        move |v| values.borrow_mut().push(v)
    });

    driver.flash_immediate();

    assert_eq!(&*values.borrow(), &[1.0, 0.0]);

    driver.stop();
}
