use super::*;

#[test]
fn default_motion_matches_upstream_style_spring_contract() {
    let motion = PopoverMotion::default();

    assert_eq!(motion.spring.stiffness, 300.0);
    assert_eq!(motion.spring.damping, 25.0);
    assert_eq!(motion.spring.mass, 1.0);
    assert_eq!(motion.spring.precision, 0.001);
    assert_eq!(motion.initial_scale, 0.98);
    assert_eq!(motion.offset_y_px, 6.0);
}

#[test]
fn placement_offset_y_follows_vertical_direction_contract() {
    assert_eq!(
        placement_offset_y(PopoverPlacement::BottomStart, 10.0),
        10.0
    );
    assert_eq!(placement_offset_y(PopoverPlacement::BottomEnd, -4.0), 4.0);
    assert_eq!(placement_offset_y(PopoverPlacement::TopStart, 10.0), -10.0);
    assert_eq!(placement_offset_y(PopoverPlacement::TopEnd, -4.0), -4.0);
}

#[test]
fn values_for_state_matches_open_and_closed_contract() {
    let motion = PopoverMotion::default();

    assert_eq!(
        values_for_state(true, PopoverPlacement::BottomStart, motion),
        (1.0, 1.0, 0.0)
    );
    assert_eq!(
        values_for_state(false, PopoverPlacement::BottomStart, motion),
        (0.0, motion.initial_scale, motion.offset_y_px)
    );
    assert_eq!(
        values_for_state(false, PopoverPlacement::TopStart, motion),
        (0.0, motion.initial_scale, -motion.offset_y_px)
    );
}

#[test]
fn sanitize_motion_falls_back_for_invalid_values() {
    let motion = sanitize_motion(PopoverMotion {
        spring: ui_motion::spring::SpringConfig {
            stiffness: f64::NAN,
            damping: -1.0,
            mass: 0.0,
            precision: f64::INFINITY,
        },
        initial_scale: f64::NAN,
        offset_y_px: f64::NAN,
    });

    let default = PopoverMotion::default();
    assert_eq!(motion.spring.stiffness, default.spring.stiffness);
    assert_eq!(motion.spring.damping, default.spring.damping);
    assert_eq!(motion.spring.mass, default.spring.mass);
    assert_eq!(motion.spring.precision, default.spring.precision);
    assert_eq!(motion.initial_scale, default.initial_scale);
    assert_eq!(motion.offset_y_px, default.offset_y_px);
}

#[test]
fn sanitize_motion_clamps_scale_and_offset() {
    let motion = sanitize_motion(PopoverMotion {
        spring: ui_motion::spring::SpringConfig {
            stiffness: 260.0,
            damping: 18.0,
            mass: 1.1,
            precision: 0.002,
        },
        initial_scale: -5.0,
        offset_y_px: -1000.0,
    });

    assert_eq!(motion.initial_scale, 0.0);
    assert_eq!(motion.offset_y_px, 240.0);
    assert_eq!(motion.spring.stiffness, 260.0);
    assert_eq!(motion.spring.damping, 18.0);
    assert_eq!(motion.spring.mass, 1.1);
    assert_eq!(motion.spring.precision, 0.002);
}
