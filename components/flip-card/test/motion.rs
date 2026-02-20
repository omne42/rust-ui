use super::*;

#[test]
fn default_motion_uses_soft_spring_contract() {
    let motion = FlipCardMotion::default();

    assert_eq!(motion.spring, ui_motion::presets::spring_soft());
    assert_eq!(motion.hover_scale, 1.015);
    assert_eq!(motion.hover_tilt_deg, 3.0);
}

#[test]
fn sanitize_motion_falls_back_for_invalid_values() {
    let default = FlipCardMotion::default();

    let motion = sanitize_motion(FlipCardMotion {
        spring: ui_motion::spring::SpringConfig {
            stiffness: f64::NAN,
            damping: -1.0,
            mass: 0.0,
            precision: f64::INFINITY,
        },
        hover_scale: f64::NAN,
        hover_tilt_deg: f64::INFINITY,
    });

    assert_eq!(motion.spring.stiffness, default.spring.stiffness);
    assert_eq!(motion.spring.damping, default.spring.damping);
    assert_eq!(motion.spring.mass, default.spring.mass);
    assert_eq!(motion.spring.precision, default.spring.precision);
    assert_eq!(motion.hover_scale, default.hover_scale);
    assert_eq!(motion.hover_tilt_deg, default.hover_tilt_deg);
}

#[test]
fn supports_custom_motion_contract() {
    let motion = FlipCardMotion {
        spring: ui_motion::spring::SpringConfig {
            stiffness: 240.0,
            damping: 22.0,
            mass: 1.0,
            precision: 0.002,
        },
        hover_scale: 1.03,
        hover_tilt_deg: 4.5,
    };

    assert_eq!(motion.spring.stiffness, 240.0);
    assert_eq!(motion.spring.damping, 22.0);
    assert_eq!(motion.spring.mass, 1.0);
    assert_eq!(motion.spring.precision, 0.002);
    assert_eq!(motion.hover_scale, 1.03);
    assert_eq!(motion.hover_tilt_deg, 4.5);
}
