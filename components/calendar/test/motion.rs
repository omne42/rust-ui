use super::*;
use ui_theme::default_text_field_motion_tokens;

#[test]
fn default_motion_comes_from_theme_tokens() {
    let motion = CalendarMotion::default();
    let tokens = default_text_field_motion_tokens();
    assert_eq!(motion.duration_ms, f64::from(tokens.duration_ms));
    assert!(motion.enabled);
}

#[test]
fn sanitize_motion_clamps_values() {
    assert_eq!(
        sanitize_motion(CalendarMotion {
            enabled: true,
            duration_ms: f64::NAN,
        }),
        CalendarMotion::default()
    );
    assert_eq!(
        sanitize_motion(CalendarMotion {
            enabled: true,
            duration_ms: 0.0,
        }),
        CalendarMotion {
            enabled: true,
            duration_ms: 1.0,
        }
    );
    assert_eq!(
        sanitize_motion(CalendarMotion {
            enabled: true,
            duration_ms: 5000.0,
        }),
        CalendarMotion {
            enabled: true,
            duration_ms: 1000.0,
        }
    );
}

#[test]
fn source_attr_reflects_default_vs_custom_motion() {
    assert_eq!(source_attr(CalendarMotion::default()), "default");
    assert_eq!(
        source_attr(CalendarMotion {
            enabled: true,
            duration_ms: 240.0,
        }),
        "custom"
    );
}

#[test]
fn attach_motion_outputs_css_variable() {
    assert_eq!(
        attach_motion(
            None,
            CalendarMotion {
                enabled: true,
                duration_ms: 240.0,
            }
        ),
        " --ui-calendar-motion-duration: 240ms;"
    );

    assert_eq!(
        attach_motion(
            None,
            CalendarMotion {
                enabled: false,
                duration_ms: 240.0,
            }
        ),
        " --ui-calendar-motion-duration: 1ms;"
    );
}
