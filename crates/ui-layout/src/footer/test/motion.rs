use super::*;

#[test]
fn sanitize_motion_clamps_footer_duration() {
    assert_eq!(
        sanitize_motion(FooterMotion {
            transition_duration_ms: 0,
        }),
        FooterMotion {
            transition_duration_ms: 60,
        }
    );
    assert_eq!(
        sanitize_motion(FooterMotion {
            transition_duration_ms: 9999,
        }),
        FooterMotion {
            transition_duration_ms: 1200,
        }
    );
}

#[test]
fn attach_motion_adds_footer_motion_variable() {
    let style = attach_motion(
        Some("--ui-initial: 1;".to_string()),
        FooterMotion {
            transition_duration_ms: 240,
        },
    );

    assert!(style.contains("--ui-footer-motion-duration: 240ms;"));
}
