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

#[test]
fn resolve_motion_reports_custom_flag_after_sanitization() {
    let (motion, is_custom) = resolve_motion(ActiveHighlightMotion {
        spring: ui_motion::spring::SpringConfig {
            stiffness: f64::NAN,
            damping: f64::NEG_INFINITY,
            mass: 0.0,
            precision: -1.0,
        },
    });

    assert_eq!(motion, ListMotion::default());
    assert!(!is_custom);

    let (custom, is_custom) = resolve_motion(ActiveHighlightMotion {
        spring: ui_motion::spring::SpringConfig {
            stiffness: 420.0,
            damping: 36.0,
            mass: 1.0,
            precision: 0.005,
        },
    });
    assert!(is_custom);
    assert_eq!(custom.spring.stiffness, 420.0);
}
