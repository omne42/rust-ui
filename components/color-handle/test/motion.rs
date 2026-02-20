use super::*;

#[test]
fn sanitize_motion_clamps_contract_values() {
    assert_eq!(
        sanitize_motion(ColorHandleMotion { duration_ms: 12 }),
        ColorHandleMotion { duration_ms: 60 }
    );
    assert_eq!(
        sanitize_motion(ColorHandleMotion { duration_ms: 3000 }),
        ColorHandleMotion { duration_ms: 1200 }
    );
}

#[test]
fn attach_motion_exports_css_variables() {
    let style = attach_motion(None, ColorHandleMotion { duration_ms: 220 });
    assert!(style.contains("--ui-color-handle-motion-duration: 220ms;"));
}
