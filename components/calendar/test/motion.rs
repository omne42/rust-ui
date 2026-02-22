use super::*;
use ui_theme::default_text_field_motion_tokens;

#[test]
fn default_motion_comes_from_theme_tokens() {
    let motion = CalendarMotion::default();
    let tokens = default_text_field_motion_tokens();
    assert_eq!(motion.duration_ms, f64::from(tokens.duration_ms));
    assert_eq!(motion.spring, ui_motion::presets::spring_soft());
    assert!(motion.enabled);
}

#[test]
fn sanitize_motion_clamps_values() {
    assert_eq!(
        sanitize_motion(CalendarMotion {
            enabled: true,
            duration_ms: f64::NAN,
            spring: ui_motion::spring::SpringConfig {
                stiffness: f64::NAN,
                damping: -1.0,
                mass: f64::INFINITY,
                precision: 0.0,
            },
        }),
        CalendarMotion::default()
    );
    assert_eq!(
        sanitize_motion(CalendarMotion {
            enabled: true,
            duration_ms: 0.0,
            spring: ui_motion::presets::spring_soft(),
        }),
        CalendarMotion {
            enabled: true,
            duration_ms: 1.0,
            spring: ui_motion::presets::spring_soft(),
        }
    );
    assert_eq!(
        sanitize_motion(CalendarMotion {
            enabled: true,
            duration_ms: 5000.0,
            spring: ui_motion::presets::spring_soft(),
        }),
        CalendarMotion {
            enabled: true,
            duration_ms: 1000.0,
            spring: ui_motion::presets::spring_soft(),
        }
    );
}

#[test]
fn source_attr_reflects_default_vs_custom_motion() {
    let default = CalendarMotion::default();
    let custom_duration = default.duration_ms + 60.0;
    assert_eq!(source_attr(default), "default");
    assert_eq!(
        source_attr(CalendarMotion {
            enabled: true,
            duration_ms: custom_duration,
            spring: default.spring,
        }),
        "custom"
    );
}

#[test]
fn resolve_effective_motion_respects_disabled_and_reduced_paths() {
    let default = CalendarMotion::default();
    let spring = ui_motion::spring::SpringConfig {
        stiffness: default.spring.stiffness + 20.0,
        damping: default.spring.damping + 2.0,
        mass: default.spring.mass + 0.2,
        precision: default.spring.precision * 2.0,
    };
    let custom_duration = default.duration_ms + 60.0;
    let motion = CalendarMotion {
        enabled: true,
        duration_ms: custom_duration,
        spring,
    };

    assert_eq!(
        resolve_effective_motion(motion, false),
        EffectiveCalendarMotion {
            duration_ms: custom_duration,
            spring,
            reduced: false,
        }
    );

    assert_eq!(
        resolve_effective_motion(motion, true),
        EffectiveCalendarMotion {
            duration_ms: 1.0,
            spring,
            reduced: true,
        }
    );

    assert_eq!(
        resolve_effective_motion(
            CalendarMotion {
                enabled: false,
                duration_ms: custom_duration,
                spring,
            },
            false
        ),
        EffectiveCalendarMotion {
            duration_ms: 1.0,
            spring,
            reduced: true,
        }
    );
}

#[test]
fn attach_motion_outputs_contract_css_variables() {
    let default = CalendarMotion::default();
    let source = attach_motion(
        None,
        CalendarMotion {
            enabled: true,
            duration_ms: default.duration_ms + 60.0,
            spring: ui_motion::spring::SpringConfig {
                stiffness: default.spring.stiffness + 20.0,
                damping: default.spring.damping + 4.0,
                mass: default.spring.mass + 0.1,
                precision: default.spring.precision * 3.0,
            },
        },
    );

    for needle in [
        "--ui-calendar-motion-duration:",
        "--ui-calendar-motion-stiffness:",
        "--ui-calendar-motion-damping:",
        "--ui-calendar-motion-mass:",
        "--ui-calendar-motion-precision:",
        "--ui-calendar-motion-reduced:",
    ] {
        assert!(
            source.contains(needle),
            "calendar motion style vars should include `{needle}`."
        );
    }
}
