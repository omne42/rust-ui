use super::*;
use crate::popover::PopoverMotion;

#[test]
fn default_motion_uses_default_popover_motion() {
    let motion = ContextMenuMotion::default();
    assert_eq!(motion.popover, PopoverMotion::default());
}

#[test]
fn sanitize_motion_delegates_to_popover_contract() {
    let input = PopoverMotion {
        spring: ui_motion::spring::SpringConfig {
            stiffness: f64::NAN,
            damping: -1.0,
            mass: 0.0,
            precision: f64::INFINITY,
        },
        initial_scale: f64::NAN,
        offset_y_px: -9999.0,
    };

    let motion = sanitize_motion(ContextMenuMotion { popover: input });
    let expected = crate::popover::motion::sanitize_motion(input);

    assert_eq!(motion.popover, expected);
    assert_eq!(motion.popover.initial_scale, 0.98);
    assert_eq!(motion.popover.offset_y_px, 240.0);
}
