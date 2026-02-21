use super::*;
use ui_theme::default_text_field_motion_tokens;

#[test]
fn default_motion_is_stable() {
    let tokens = default_text_field_motion_tokens();
    assert_eq!(
        FieldMotion::default(),
        FieldMotion {
            duration_ms: f64::from(tokens.duration_ms),
            spring: ui_motion::presets::spring_soft(),
        }
    );
}

#[test]
fn sanitize_motion_clamps_values() {
    assert_eq!(
        sanitize_motion(FieldMotion {
            duration_ms: f64::NAN,
            spring: ui_motion::spring::SpringConfig::default(),
        }),
        FieldMotion::default()
    );
    assert_eq!(
        sanitize_motion(FieldMotion {
            duration_ms: -20.0,
            spring: ui_motion::spring::SpringConfig {
                stiffness: f64::NAN,
                damping: -1.0,
                mass: 0.0,
                precision: 0.0,
            },
        }),
        FieldMotion {
            duration_ms: 1.0,
            spring: FieldMotion::default().spring,
        }
    );
    assert_eq!(
        sanitize_motion(FieldMotion {
            duration_ms: 9999.0,
            spring: ui_motion::spring::SpringConfig {
                stiffness: 410.0,
                damping: 31.0,
                mass: 1.4,
                precision: 0.004,
            },
        }),
        FieldMotion {
            duration_ms: 800.0,
            spring: ui_motion::spring::SpringConfig {
                stiffness: 410.0,
                damping: 31.0,
                mass: 1.4,
                precision: 0.004,
            },
        }
    );
}

#[test]
fn attach_motion_outputs_css_variable() {
    let expected_suffix = "--ui-field-motion-stiffness: 200;--ui-field-motion-damping: 16;--ui-field-motion-mass: 1;--ui-field-motion-precision: 0.001;";
    let expected = if cfg!(target_arch = "wasm32") {
        format!("--ui-field-motion-duration: 200ms;{expected_suffix}")
    } else {
        format!("--ui-field-motion-duration: 1ms;{expected_suffix}")
    };
    assert_eq!(
        attach_motion(FieldMotion {
            duration_ms: 200.0,
            spring: ui_motion::presets::spring_fast(),
        }),
        expected
    );
}

#[test]
fn source_attr_tracks_default_vs_custom() {
    assert_eq!(source_attr(FieldMotion::default()), "default");
    assert_eq!(
        source_attr(FieldMotion {
            duration_ms: 420.0,
            spring: FieldMotion::default().spring,
        }),
        "custom"
    );
    assert_eq!(
        source_attr(FieldMotion {
            duration_ms: FieldMotion::default().duration_ms,
            spring: ui_motion::presets::spring_fast(),
        }),
        "custom"
    );
}
