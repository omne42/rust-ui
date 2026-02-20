use super::*;

#[test]
fn sanitize_config_falls_back_for_non_finite_or_non_positive_values() {
    let fallback = SpringConfig::default();
    let sanitized = sanitize_config(
        SpringConfig {
            stiffness: f64::NAN,
            damping: -1.0,
            mass: 0.0,
            precision: f64::INFINITY,
        },
        fallback,
    );

    assert_eq!(sanitized.stiffness, fallback.stiffness);
    assert_eq!(sanitized.damping, fallback.damping);
    assert_eq!(sanitized.mass, fallback.mass);
    assert_eq!(sanitized.precision, fallback.precision);
}

#[test]
fn sanitize_config_keeps_valid_values() {
    let fallback = SpringConfig::default();
    let sanitized = sanitize_config(
        SpringConfig {
            stiffness: 240.0,
            damping: 22.0,
            mass: 1.1,
            precision: 0.002,
        },
        fallback,
    );

    assert_eq!(sanitized.stiffness, 240.0);
    assert_eq!(sanitized.damping, 22.0);
    assert_eq!(sanitized.mass, 1.1);
    assert_eq!(sanitized.precision, 0.002);
}
