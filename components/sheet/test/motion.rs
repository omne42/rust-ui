use super::*;

#[test]
fn default_motion_uses_slide_spring_contract() {
    let motion = SheetMotion::default();

    assert_eq!(motion.spring, ui_motion::presets::spring_slide());
    assert_eq!(motion.initial_offset_px, 32.0);
}

#[test]
fn placement_offset_maps_to_sheet_direction_contract() {
    assert_eq!(placement_offset(SheetPlacement::Bottom, 12.0), (0.0, 12.0));
    assert_eq!(placement_offset(SheetPlacement::Bottom, -4.0), (0.0, 4.0));
    assert_eq!(placement_offset(SheetPlacement::Left, 12.0), (-12.0, 0.0));
    assert_eq!(placement_offset(SheetPlacement::Left, -4.0), (-4.0, 0.0));
    assert_eq!(placement_offset(SheetPlacement::Right, 12.0), (12.0, 0.0));
    assert_eq!(placement_offset(SheetPlacement::Right, -4.0), (4.0, 0.0));
}

#[test]
fn supports_custom_motion_contract() {
    let motion = SheetMotion {
        spring: ui_motion::spring::SpringConfig {
            stiffness: 260.0,
            damping: 24.0,
            mass: 1.0,
            precision: 0.002,
        },
        initial_offset_px: 48.0,
    };

    assert_eq!(motion.spring.stiffness, 260.0);
    assert_eq!(motion.spring.damping, 24.0);
    assert_eq!(motion.spring.mass, 1.0);
    assert_eq!(motion.spring.precision, 0.002);
    assert_eq!(motion.initial_offset_px, 48.0);
}

#[test]
fn sanitize_motion_falls_back_for_invalid_values() {
    let motion = sanitize_motion(SheetMotion {
        spring: ui_motion::spring::SpringConfig {
            stiffness: f64::NAN,
            damping: -1.0,
            mass: 0.0,
            precision: f64::INFINITY,
        },
        initial_offset_px: f64::NAN,
    });

    let default = SheetMotion::default();
    assert_eq!(motion.spring.stiffness, default.spring.stiffness);
    assert_eq!(motion.spring.damping, default.spring.damping);
    assert_eq!(motion.spring.mass, default.spring.mass);
    assert_eq!(motion.spring.precision, default.spring.precision);
    assert_eq!(motion.initial_offset_px, default.initial_offset_px);
}

#[test]
fn sanitize_motion_clamps_offset_range() {
    let motion = sanitize_motion(SheetMotion {
        spring: ui_motion::spring::SpringConfig {
            stiffness: 220.0,
            damping: 20.0,
            mass: 1.05,
            precision: 0.003,
        },
        initial_offset_px: -9999.0,
    });

    assert_eq!(motion.initial_offset_px, 640.0);
    assert_eq!(motion.spring.stiffness, 220.0);
    assert_eq!(motion.spring.damping, 20.0);
    assert_eq!(motion.spring.mass, 1.05);
    assert_eq!(motion.spring.precision, 0.003);
}
