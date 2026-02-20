use super::*;

#[test]
fn default_motion_is_stable() {
    assert_eq!(
        ColorAreaMotion::default(),
        ColorAreaMotion { duration_ms: 180.0 }
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
    assert_eq!(source_attr(ColorAreaMotion::default()), "default");
    assert_eq!(
        source_attr(ColorAreaMotion { duration_ms: 220.0 }),
        "custom"
    );
}

#[test]
fn attach_motion_outputs_css_variable() {
    assert_eq!(
        attach_motion(None, ColorAreaMotion { duration_ms: 220.0 }),
        "--ui-color-area-motion-duration: 220ms;"
    );
    assert_eq!(
        attach_motion(
            Some("--ui-color-area-preview-color: #09f;".to_string()),
            ColorAreaMotion { duration_ms: 220.0 }
        ),
        "--ui-color-area-preview-color: #09f; --ui-color-area-motion-duration: 220ms;"
    );
}
