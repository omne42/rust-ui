use super::*;

#[test]
fn sanitize_motion_falls_back_for_invalid_values() {
    let motion = sanitize_motion(MenuMotion {
        highlight: ActiveHighlightMotion {
            spring: ui_motion::spring::SpringConfig {
                stiffness: f64::NAN,
                damping: -1.0,
                mass: 0.0,
                precision: f64::INFINITY,
            },
        },
    });

    let default = ActiveHighlightMotion::default();
    assert_eq!(motion.highlight.spring.stiffness, default.spring.stiffness);
    assert_eq!(motion.highlight.spring.damping, default.spring.damping);
    assert_eq!(motion.highlight.spring.mass, default.spring.mass);
    assert_eq!(motion.highlight.spring.precision, default.spring.precision);
}

#[test]
fn attach_motion_returns_sanitized_highlight_motion() {
    let attached = attach_motion(MenuMotion {
        highlight: ActiveHighlightMotion {
            spring: ui_motion::spring::SpringConfig {
                stiffness: 280.0,
                damping: 24.0,
                mass: 1.0,
                precision: 0.002,
            },
        },
    });

    assert_eq!(attached.spring.stiffness, 280.0);
    assert_eq!(attached.spring.damping, 24.0);
}
