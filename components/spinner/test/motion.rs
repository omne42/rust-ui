use super::*;

#[test]
fn sanitize_motion_clamps_spinner_rotation_duration() {
    assert_eq!(
        sanitize_motion(SpinnerMotion {
            rotation_duration_ms: 120,
        }),
        SpinnerMotion {
            rotation_duration_ms: 240,
        }
    );
    assert_eq!(
        sanitize_motion(SpinnerMotion {
            rotation_duration_ms: 9000,
        }),
        SpinnerMotion {
            rotation_duration_ms: 4000,
        }
    );
}

#[test]
fn attach_motion_adds_spinner_motion_var() {
    let style = attach_motion(
        None,
        SpinnerMotion {
            rotation_duration_ms: 1200,
        },
    );

    assert!(style.contains("--ui-spinner-rotation-duration: 1200ms;"));
}
