use super::*;

#[test]
fn default_motion_is_stable() {
    assert_eq!(FieldMotion::default(), FieldMotion { duration_ms: 160.0 });
}

#[test]
fn sanitize_motion_clamps_values() {
    assert_eq!(
        sanitize_motion(FieldMotion {
            duration_ms: f64::NAN
        }),
        FieldMotion::default()
    );
    assert_eq!(
        sanitize_motion(FieldMotion { duration_ms: -20.0 }),
        FieldMotion { duration_ms: 1.0 }
    );
    assert_eq!(
        sanitize_motion(FieldMotion {
            duration_ms: 9999.0
        }),
        FieldMotion { duration_ms: 800.0 }
    );
}

#[test]
fn attach_motion_outputs_css_variable() {
    assert_eq!(
        attach_motion(FieldMotion { duration_ms: 200.0 }),
        "--ui-field-motion-duration: 200ms;"
    );
}
