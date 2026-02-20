use super::*;

#[test]
fn default_motion_uses_expected_drop_zone_contract() {
    let motion = DropZoneMotion::default();

    assert_eq!(motion.spring.stiffness, 260.0);
    assert_eq!(motion.spring.damping, 18.0);
    assert_eq!(motion.spring.mass, 1.0);
    assert_eq!(motion.hover_scale, 1.01);
    assert_eq!(motion.drop_scale, 1.02);
    assert_eq!(motion.hover_highlight, 0.35);
}

#[test]
fn sanitize_motion_falls_back_for_invalid_values() {
    let default = DropZoneMotion::default();

    let motion = sanitize_motion(DropZoneMotion {
        spring: ui_motion::spring::SpringConfig {
            stiffness: f64::NAN,
            damping: -1.0,
            mass: 0.0,
            precision: f64::INFINITY,
        },
        hover_scale: f64::NAN,
        drop_scale: f64::INFINITY,
        hover_highlight: -2.0,
    });

    assert_eq!(motion.spring.stiffness, default.spring.stiffness);
    assert_eq!(motion.spring.damping, default.spring.damping);
    assert_eq!(motion.spring.mass, default.spring.mass);
    assert_eq!(motion.spring.precision, default.spring.precision);
    assert_eq!(motion.hover_scale, default.hover_scale);
    assert_eq!(motion.drop_scale, default.drop_scale);
    assert_eq!(motion.hover_highlight, 0.0);
}

#[test]
fn supports_custom_drop_zone_motion_contract() {
    let motion = DropZoneMotion {
        spring: ui_motion::spring::SpringConfig {
            stiffness: 220.0,
            damping: 20.0,
            mass: 1.0,
            precision: 0.002,
        },
        hover_scale: 1.015,
        drop_scale: 1.03,
        hover_highlight: 0.42,
    };

    assert_eq!(motion.spring.stiffness, 220.0);
    assert_eq!(motion.spring.damping, 20.0);
    assert_eq!(motion.spring.mass, 1.0);
    assert_eq!(motion.spring.precision, 0.002);
    assert_eq!(motion.hover_scale, 1.015);
    assert_eq!(motion.drop_scale, 1.03);
    assert_eq!(motion.hover_highlight, 0.42);
}
