use super::*;

#[test]
fn sanitize_motion_clamps_transition_range() {
    assert_eq!(
        sanitize_motion(ErrorMessageMotion { transition_ms: 0 }).transition_ms,
        1
    );
    assert_eq!(
        sanitize_motion(ErrorMessageMotion {
            transition_ms: 4_000
        })
        .transition_ms,
        MAX_TRANSITION_MS
    );
}
