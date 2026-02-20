use super::*;

#[test]
fn default_motion_is_stable() {
    assert_eq!(
        SidebarTriggerMotion::default(),
        SidebarTriggerMotion { duration_ms: 160.0 }
    );
}

#[test]
fn sanitize_motion_clamps_values() {
    assert_eq!(
        sanitize_motion(SidebarTriggerMotion {
            duration_ms: f64::NAN
        }),
        SidebarTriggerMotion::default()
    );
    assert_eq!(
        sanitize_motion(SidebarTriggerMotion { duration_ms: 0.0 }),
        SidebarTriggerMotion { duration_ms: 1.0 }
    );
    assert_eq!(
        sanitize_motion(SidebarTriggerMotion {
            duration_ms: 5000.0
        }),
        SidebarTriggerMotion {
            duration_ms: 1000.0
        }
    );
}

#[test]
fn attach_motion_outputs_css_variable() {
    assert_eq!(
        attach_motion(SidebarTriggerMotion { duration_ms: 300.0 }),
        "--ui-sidebar-trigger-motion-duration: 300ms;"
    );
}
