use super::*;

#[test]
fn source_attr_tracks_default_vs_custom_motion() {
    assert_eq!(source_attr(ColorEditorMotion::default()), "default");
    assert_eq!(source_attr(ColorEditorMotion::disabled()), "custom");
}

#[test]
fn default_motion_keeps_positive_spring_contract_values() {
    let motion = ColorEditorMotion::default();

    assert!(motion.spring.stiffness.is_finite() && motion.spring.stiffness > 0.0);
    assert!(motion.spring.damping.is_finite() && motion.spring.damping > 0.0);
    assert!(motion.spring.mass.is_finite() && motion.spring.mass > 0.0);
    assert!(motion.spring.precision.is_finite() && motion.spring.precision > 0.0);
}

#[test]
fn attach_motion_returns_sanitized_motion_contract() {
    let invalid = ColorEditorMotion {
        enabled: true,
        spring: ui_motion::spring::SpringConfig {
            stiffness: f64::NAN,
            damping: f64::INFINITY,
            mass: -1.0,
            precision: 0.0,
        },
    };

    let attached = attach_motion(invalid);
    let expected = sanitize_motion(invalid);
    assert_eq!(attached, expected);
}
