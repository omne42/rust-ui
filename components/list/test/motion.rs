use super::*;

#[test]
fn sanitize_motion_uses_default_for_invalid_values() {
    let motion = sanitize_motion(ActiveHighlightMotion {
        spring: ui_motion::spring::SpringConfig {
            stiffness: f64::NAN,
            damping: -10.0,
            mass: 0.0,
            precision: f64::INFINITY,
        },
    });

    let default = ActiveHighlightMotion::default();
    assert_eq!(motion.spring.stiffness, default.spring.stiffness);
    assert_eq!(motion.spring.damping, default.spring.damping);
    assert_eq!(motion.spring.mass, default.spring.mass);
    assert_eq!(motion.spring.precision, default.spring.precision);
}

#[test]
fn sanitize_section_motion_preserves_default_contract() {
    let motion = sanitize_section_motion(ListSectionMotion::default());
    assert_eq!(motion, ListSectionMotion::default());
}
