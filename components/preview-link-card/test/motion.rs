use super::*;

#[test]
fn default_motion_uses_slide_spring_contract() {
    let motion = PreviewLinkCardMotion::default();

    assert_eq!(motion.spring, ui_motion::presets::spring_slide());
    assert!(motion.initial_scale > 0.0);
    assert!(motion.initial_scale <= 1.0);
    assert!(motion.offset_y_px > 0.0);
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
    let default = PreviewLinkCardMotion::default();
    let custom_spring = ui_motion::spring::SpringConfig {
        stiffness: default.spring.stiffness - 40.0,
        damping: default.spring.damping - 6.0,
        mass: default.spring.mass,
        precision: default.spring.precision * 2.0,
    };
    let custom_scale = (default.initial_scale - 0.03).clamp(0.0, 1.0);
    let custom_offset = default.offset_y_px + 4.0;
    let motion = PreviewLinkCardMotion {
        spring: custom_spring,
        initial_scale: custom_scale,
        offset_y_px: custom_offset,
    };

    assert_eq!(motion.spring.stiffness, custom_spring.stiffness);
    assert_eq!(motion.spring.damping, custom_spring.damping);
    assert_eq!(motion.spring.mass, custom_spring.mass);
    assert_eq!(motion.spring.precision, custom_spring.precision);
    assert_eq!(motion.initial_scale, custom_scale);
    assert_eq!(motion.offset_y_px, custom_offset);
}

#[test]
fn sanitize_motion_falls_back_for_invalid_values() {
    let motion = sanitize_motion(PreviewLinkCardMotion {
        spring: ui_motion::spring::SpringConfig {
            stiffness: f64::NAN,
            damping: -1.0,
            mass: 0.0,
            precision: f64::INFINITY,
        },
        initial_scale: f64::NAN,
        offset_y_px: f64::NAN,
    });

    let default = PreviewLinkCardMotion::default();
    assert_eq!(motion.spring.stiffness, default.spring.stiffness);
    assert_eq!(motion.spring.damping, default.spring.damping);
    assert_eq!(motion.spring.mass, default.spring.mass);
    assert_eq!(motion.spring.precision, default.spring.precision);
    assert_eq!(motion.initial_scale, default.initial_scale);
    assert_eq!(motion.offset_y_px, default.offset_y_px);
}

#[test]
fn sanitize_motion_clamps_scale_and_offset_ranges() {
    let default = PreviewLinkCardMotion::default();
    let custom_spring = ui_motion::spring::SpringConfig {
        stiffness: default.spring.stiffness - 80.0,
        damping: default.spring.damping - 10.0,
        mass: default.spring.mass + 0.05,
        precision: default.spring.precision * 3.0,
    };
    let motion = sanitize_motion(PreviewLinkCardMotion {
        spring: custom_spring,
        initial_scale: 12.0,
        offset_y_px: -9999.0,
    });

    assert_eq!(motion.initial_scale, 3.0);
    assert_eq!(motion.offset_y_px, 320.0);
    assert_eq!(motion.spring.stiffness, custom_spring.stiffness);
    assert_eq!(motion.spring.damping, custom_spring.damping);
    assert_eq!(motion.spring.mass, custom_spring.mass);
    assert_eq!(motion.spring.precision, custom_spring.precision);
}
