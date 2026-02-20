use super::*;

#[test]
fn default_motion_uses_flip3d_spring_contract() {
    let motion = OverlayMotion::default();

    assert_eq!(motion.spring, ui_motion::presets::spring_flip_3d());
    assert_eq!(motion.initial_scale, 0.96);
    assert_eq!(motion.initial_y_px, 8.0);
}

#[test]
fn supports_custom_overlay_motion_contract() {
    let motion = OverlayMotion {
        spring: ui_motion::spring::SpringConfig {
            stiffness: 210.0,
            damping: 19.0,
            mass: 1.1,
            precision: 0.002,
        },
        initial_scale: 0.94,
        initial_y_px: 14.0,
    };

    assert_eq!(motion.spring.stiffness, 210.0);
    assert_eq!(motion.spring.damping, 19.0);
    assert_eq!(motion.spring.mass, 1.1);
    assert_eq!(motion.spring.precision, 0.002);
    assert_eq!(motion.initial_scale, 0.94);
    assert_eq!(motion.initial_y_px, 14.0);
}

#[test]
fn sanitize_motion_falls_back_for_invalid_values() {
    let motion = sanitize_motion(OverlayMotion {
        spring: ui_motion::spring::SpringConfig {
            stiffness: f64::NAN,
            damping: -1.0,
            mass: 0.0,
            precision: f64::INFINITY,
        },
        initial_scale: f64::NAN,
        initial_y_px: f64::NAN,
    });

    let default = OverlayMotion::default();
    assert_eq!(motion.spring.stiffness, default.spring.stiffness);
    assert_eq!(motion.spring.damping, default.spring.damping);
    assert_eq!(motion.spring.mass, default.spring.mass);
    assert_eq!(motion.spring.precision, default.spring.precision);
    assert_eq!(motion.initial_scale, default.initial_scale);
    assert_eq!(motion.initial_y_px, default.initial_y_px);
}

#[test]
fn sanitize_motion_clamps_scale_and_y_offset_ranges() {
    let motion = sanitize_motion(OverlayMotion {
        spring: ui_motion::spring::SpringConfig {
            stiffness: 220.0,
            damping: 20.0,
            mass: 1.05,
            precision: 0.003,
        },
        initial_scale: 8.0,
        initial_y_px: -9999.0,
    });

    assert_eq!(motion.initial_scale, 3.0);
    assert_eq!(motion.initial_y_px, 320.0);
    assert_eq!(motion.spring.stiffness, 220.0);
    assert_eq!(motion.spring.damping, 20.0);
    assert_eq!(motion.spring.mass, 1.05);
    assert_eq!(motion.spring.precision, 0.003);
}
