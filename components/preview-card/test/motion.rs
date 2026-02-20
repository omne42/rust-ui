use super::*;

#[test]
fn default_motion_uses_slide_spring_contract() {
    let motion = PreviewCardMotion::default();

    assert_eq!(motion.spring, ui_motion::presets::spring_slide());
    assert_eq!(motion.initial_scale, 0.98);
    assert_eq!(motion.offset_y_px, 8.0);
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
fn supports_custom_motion_contract() {
    let motion = PreviewCardMotion {
        spring: ui_motion::spring::SpringConfig {
            stiffness: 260.0,
            damping: 24.0,
            mass: 1.0,
            precision: 0.002,
        },
        initial_scale: 0.95,
        offset_y_px: 12.0,
    };

    assert_eq!(motion.spring.stiffness, 260.0);
    assert_eq!(motion.spring.damping, 24.0);
    assert_eq!(motion.spring.mass, 1.0);
    assert_eq!(motion.spring.precision, 0.002);
    assert_eq!(motion.initial_scale, 0.95);
    assert_eq!(motion.offset_y_px, 12.0);
}

#[test]
fn sanitize_motion_falls_back_for_invalid_values() {
    let motion = sanitize_motion(PreviewCardMotion {
        spring: ui_motion::spring::SpringConfig {
            stiffness: f64::NAN,
            damping: -1.0,
            mass: 0.0,
            precision: f64::INFINITY,
        },
        initial_scale: f64::NAN,
        offset_y_px: f64::NAN,
    });

    let default = PreviewCardMotion::default();
    assert_eq!(motion.spring.stiffness, default.spring.stiffness);
    assert_eq!(motion.spring.damping, default.spring.damping);
    assert_eq!(motion.spring.mass, default.spring.mass);
    assert_eq!(motion.spring.precision, default.spring.precision);
    assert_eq!(motion.initial_scale, default.initial_scale);
    assert_eq!(motion.offset_y_px, default.offset_y_px);
}

#[test]
fn sanitize_motion_clamps_scale_and_offset_ranges() {
    let motion = sanitize_motion(PreviewCardMotion {
        spring: ui_motion::spring::SpringConfig {
            stiffness: 220.0,
            damping: 20.0,
            mass: 1.05,
            precision: 0.003,
        },
        initial_scale: 12.0,
        offset_y_px: -9999.0,
    });

    assert_eq!(motion.initial_scale, 3.0);
    assert_eq!(motion.offset_y_px, 320.0);
    assert_eq!(motion.spring.stiffness, 220.0);
    assert_eq!(motion.spring.damping, 20.0);
    assert_eq!(motion.spring.mass, 1.05);
    assert_eq!(motion.spring.precision, 0.003);
}
