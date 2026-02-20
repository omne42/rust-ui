use super::*;

#[test]
fn sanitize_motion_clamps_view_duration() {
    assert_eq!(
        sanitize_motion(ViewMotion {
            transition_duration_ms: 0,
        }),
        ViewMotion {
            transition_duration_ms: 60,
        }
    );

    assert_eq!(
        sanitize_motion(ViewMotion {
            transition_duration_ms: 5000,
        }),
        ViewMotion {
            transition_duration_ms: 1200,
        }
    );
}

#[test]
fn attach_motion_adds_view_motion_var() {
    let style = attach_motion(
        Some("--ui-view-local: 1;".to_string()),
        ViewMotion {
            transition_duration_ms: 210,
        },
    );

    assert!(style.contains("--ui-view-local: 1;"));
    assert!(style.contains("--ui-view-motion-duration: 210ms;"));
}
