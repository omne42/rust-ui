use super::*;

#[test]
fn sanitize_motion_falls_back_for_non_finite_or_non_positive_spring_fields() {
    let default = CommandMotion::default();
    let mut invalid = CommandMotion::default();
    invalid.spring.stiffness = f64::NAN;
    invalid.spring.damping = -1.0;
    invalid.spring.mass = 0.0;
    invalid.spring.precision = f64::INFINITY;

    let sanitized = sanitize_motion(invalid);

    assert_eq!(sanitized, default);
}

#[test]
fn is_custom_motion_uses_sanitized_contract() {
    let mut invalid = CommandMotion::default();
    invalid.spring.stiffness = f64::NAN;
    invalid.spring.damping = f64::NAN;
    invalid.spring.mass = f64::NAN;
    invalid.spring.precision = f64::NAN;
    assert!(!is_custom_motion(invalid));

    let mut custom = CommandMotion::default();
    custom.spring.stiffness += 10.0;
    assert!(is_custom_motion(custom));
}
