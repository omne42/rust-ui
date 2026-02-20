use super::*;

#[test]
fn sanitize_motion_clamps_infield_button_duration() {
    assert_eq!(
        sanitize_motion(InfieldButtonMotion {
            transition_duration_ms: 1,
        }),
        InfieldButtonMotion {
            transition_duration_ms: 40,
        }
    );
    assert_eq!(
        sanitize_motion(InfieldButtonMotion {
            transition_duration_ms: 3200,
        }),
        InfieldButtonMotion {
            transition_duration_ms: 1200,
        }
    );
}

#[test]
fn attach_motion_exports_button_motion_variable() {
    let style = attach_motion(
        None,
        InfieldButtonMotion {
            transition_duration_ms: 180,
        },
    );

    assert!(style.contains("--ui-infield-button-motion-duration: 180ms;"));
}
