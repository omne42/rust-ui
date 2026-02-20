use super::*;

#[test]
fn default_motion_is_stable() {
    assert_eq!(
        ActionButtonGroupMotion::default(),
        ActionButtonGroupMotion { duration_ms: 160.0 }
    );
}

#[test]
fn sanitize_motion_clamps_and_falls_back() {
    assert_eq!(
        sanitize_motion(ActionButtonGroupMotion {
            duration_ms: f64::NAN
        }),
        ActionButtonGroupMotion::default()
    );
    assert_eq!(
        sanitize_motion(ActionButtonGroupMotion { duration_ms: -10.0 }),
        ActionButtonGroupMotion { duration_ms: 1.0 }
    );
    assert_eq!(
        sanitize_motion(ActionButtonGroupMotion {
            duration_ms: 9999.0
        }),
        ActionButtonGroupMotion { duration_ms: 800.0 }
    );
}

#[test]
fn attach_motion_only_outputs_css_variable() {
    assert_eq!(
        attach_motion(ActionButtonGroupMotion { duration_ms: 240.0 }),
        "--ui-action-button-group-motion-duration: 240ms;"
    );
}
