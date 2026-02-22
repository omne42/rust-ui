use super::*;
use ui_theme::default_text_field_motion_tokens;

#[test]
fn default_motion_is_stable() {
    let tokens = default_text_field_motion_tokens();
    assert_eq!(
        ColorAreaMotion::default(),
        ColorAreaMotion {
            duration_ms: f64::from(tokens.duration_ms),
        }
    );
}

#[test]
fn sanitize_motion_clamps_values() {
    assert_eq!(
        sanitize_motion(ColorAreaMotion {
            duration_ms: f64::NAN
        }),
        ColorAreaMotion::default()
    );
    assert_eq!(
        sanitize_motion(ColorAreaMotion { duration_ms: 0.0 }),
        ColorAreaMotion { duration_ms: 1.0 }
    );
    assert_eq!(
        sanitize_motion(ColorAreaMotion {
            duration_ms: 9999.0
        }),
        ColorAreaMotion {
            duration_ms: 1000.0
        }
    );
}

#[test]
fn source_attr_distinguishes_default_and_custom_motion() {
    let custom_duration = ColorAreaMotion::default().duration_ms + 40.0;
    assert_eq!(source_attr(ColorAreaMotion::default()), "default");
    assert_eq!(
        source_attr(ColorAreaMotion {
            duration_ms: custom_duration
        }),
        "custom"
    );
}

#[test]
fn attach_motion_outputs_css_variable() {
    let custom_duration = ColorAreaMotion::default().duration_ms + 40.0;
    let expected = format!("--ui-color-area-motion-duration: {custom_duration}ms;");
    let expected_with_base = format!(
        "--ui-color-area-preview-color: #09f; --ui-color-area-motion-duration: {custom_duration}ms;"
    );

    assert_eq!(attach_motion(None, ColorAreaMotion::default()), "");
    assert_eq!(
        attach_motion(
            Some("--ui-color-area-preview-color: #09f;".to_string()),
            ColorAreaMotion::default()
        ),
        "--ui-color-area-preview-color: #09f;"
    );
    assert_eq!(
        attach_motion(
            None,
            ColorAreaMotion {
                duration_ms: custom_duration
            }
        ),
        expected
    );
    assert_eq!(
        attach_motion(
            Some("--ui-color-area-preview-color: #09f;".to_string()),
            ColorAreaMotion {
                duration_ms: custom_duration
            }
        ),
        expected_with_base
    );
}
