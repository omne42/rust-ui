use super::*;

#[test]
fn default_motion_uses_slide_spring_contract() {
    let motion = SheetMotion::default();

    assert_eq!(motion.spring, ui_motion::presets::spring_slide());
    assert!(motion.initial_offset_px > 0.0);
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
    let default = SheetMotion::default();
    let custom_spring = ui_motion::spring::SpringConfig {
        stiffness: default.spring.stiffness - 40.0,
        damping: default.spring.damping - 6.0,
        mass: default.spring.mass,
        precision: default.spring.precision * 2.0,
    };
    let custom_offset = default.initial_offset_px + 16.0;
    let motion = SheetMotion {
        spring: custom_spring,
        initial_offset_px: custom_offset,
    };

    assert_eq!(motion.spring.stiffness, custom_spring.stiffness);
    assert_eq!(motion.spring.damping, custom_spring.damping);
    assert_eq!(motion.spring.mass, custom_spring.mass);
    assert_eq!(motion.spring.precision, custom_spring.precision);
    assert_eq!(motion.initial_offset_px, custom_offset);
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
    let default = SheetMotion::default();
    let custom_spring = ui_motion::spring::SpringConfig {
        stiffness: default.spring.stiffness - 80.0,
        damping: default.spring.damping - 10.0,
        mass: default.spring.mass + 0.05,
        precision: default.spring.precision * 3.0,
    };
    let motion = sanitize_motion(SheetMotion {
        spring: custom_spring,
        initial_offset_px: -9999.0,
    });

    assert_eq!(motion.initial_offset_px, 640.0);
    assert_eq!(motion.spring.stiffness, custom_spring.stiffness);
    assert_eq!(motion.spring.damping, custom_spring.damping);
    assert_eq!(motion.spring.mass, custom_spring.mass);
    assert_eq!(motion.spring.precision, custom_spring.precision);
}
