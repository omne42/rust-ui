use super::*;

#[test]
fn default_motion_matches_auto_height_contract() {
    let motion = AutoHeightMotion::default();

    assert_eq!(motion.spring, ui_motion::presets::spring_soft());
    assert!(motion.animate_height);
}

#[test]
fn sanitize_motion_falls_back_for_invalid_values() {
    let default = AutoHeightMotion::default();

    let motion = sanitize_motion(AutoHeightMotion {
        spring: ui_motion::spring::SpringConfig {
            stiffness: f64::NAN,
            damping: -1.0,
            mass: 0.0,
            precision: f64::INFINITY,
        },
        animate_height: false,
    });

    assert_eq!(motion.spring.stiffness, default.spring.stiffness);
    assert_eq!(motion.spring.damping, default.spring.damping);
    assert_eq!(motion.spring.mass, default.spring.mass);
    assert_eq!(motion.spring.precision, default.spring.precision);
    assert!(!motion.animate_height);
}

#[test]
fn supports_custom_motion_contract_values() {
    let motion = AutoHeightMotion {
        spring: ui_motion::spring::SpringConfig {
            stiffness: 286.0,
            damping: 22.0,
            mass: 1.0,
            precision: 0.002,
        },
        animate_height: false,
    };

    assert_eq!(motion.spring.stiffness, 286.0);
    assert_eq!(motion.spring.damping, 22.0);
    assert_eq!(motion.spring.mass, 1.0);
    assert_eq!(motion.spring.precision, 0.002);
    assert!(!motion.animate_height);
}
