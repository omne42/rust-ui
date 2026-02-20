use super::*;

#[test]
fn default_motion_uses_soft_spring_contract() {
    let motion = TooltipMotion::default();

    assert_eq!(motion.spring, ui_motion::presets::spring_soft());
    assert_eq!(motion.initial_scale, 0.98);
    assert_eq!(motion.offset_y_px, 6.0);
}

#[test]
fn placement_offset_y_follows_vertical_direction_contract() {
    assert_eq!(placement_offset_y(TooltipPlacement::Bottom, 10.0), 10.0);
    assert_eq!(placement_offset_y(TooltipPlacement::Bottom, -4.0), 4.0);
    assert_eq!(placement_offset_y(TooltipPlacement::Top, 10.0), -10.0);
    assert_eq!(placement_offset_y(TooltipPlacement::Top, -4.0), -4.0);
}

#[test]
fn supports_custom_motion_contract() {
    let motion = TooltipMotion {
        spring: ui_motion::spring::SpringConfig {
            stiffness: 220.0,
            damping: 20.0,
            mass: 1.0,
            precision: 0.003,
        },
        initial_scale: 0.94,
        offset_y_px: 11.0,
    };

    assert_eq!(motion.spring.stiffness, 220.0);
    assert_eq!(motion.spring.damping, 20.0);
    assert_eq!(motion.spring.mass, 1.0);
    assert_eq!(motion.spring.precision, 0.003);
    assert_eq!(motion.initial_scale, 0.94);
    assert_eq!(motion.offset_y_px, 11.0);
}

#[test]
fn sanitize_motion_falls_back_for_invalid_values() {
    let motion = sanitize_motion(TooltipMotion {
        spring: ui_motion::spring::SpringConfig {
            stiffness: f64::NAN,
            damping: -1.0,
            mass: 0.0,
            precision: f64::INFINITY,
        },
        initial_scale: f64::NAN,
        offset_y_px: f64::NAN,
    });

    let default = TooltipMotion::default();
    assert_eq!(motion.spring.stiffness, default.spring.stiffness);
    assert_eq!(motion.spring.damping, default.spring.damping);
    assert_eq!(motion.spring.mass, default.spring.mass);
    assert_eq!(motion.spring.precision, default.spring.precision);
    assert_eq!(motion.initial_scale, default.initial_scale);
    assert_eq!(motion.offset_y_px, default.offset_y_px);
}

#[test]
fn sanitize_motion_clamps_scale_and_offset_ranges() {
    let motion = sanitize_motion(TooltipMotion {
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
