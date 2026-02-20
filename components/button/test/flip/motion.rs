use super::*;

#[test]
fn default_motion_matches_flip_button_spring_contract() {
    let motion = FlipButtonMotion::default();

    assert_eq!(
        motion.spring,
        crate::button::motion::ButtonMotion::default().spring
    );
}

#[test]
fn sanitize_motion_falls_back_for_invalid_values() {
    let default = FlipButtonMotion::default();

    let motion = sanitize_motion(FlipButtonMotion {
        spring: ui_motion::spring::SpringConfig {
            stiffness: f64::NAN,
            damping: -1.0,
            mass: 0.0,
            precision: f64::INFINITY,
        },
    });

    assert_eq!(motion.spring.stiffness, default.spring.stiffness);
    assert_eq!(motion.spring.damping, default.spring.damping);
    assert_eq!(motion.spring.mass, default.spring.mass);
    assert_eq!(motion.spring.precision, default.spring.precision);
}

#[test]
fn supports_custom_flip_motion_contract() {
    let motion = FlipButtonMotion {
        spring: ui_motion::spring::SpringConfig {
            stiffness: 292.0,
            damping: 20.0,
            mass: 1.0,
            precision: 0.002,
        },
    };

    assert_eq!(motion.spring.stiffness, 292.0);
    assert_eq!(motion.spring.damping, 20.0);
    assert_eq!(motion.spring.mass, 1.0);
    assert_eq!(motion.spring.precision, 0.002);
}
