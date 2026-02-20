use super::*;

#[test]
fn sanitize_motion_clamps_contract_values() {
    assert_eq!(
        sanitize_motion(ColorThumbMotion {
            handle_duration_ms: 12,
            loupe_duration_ms: 3000,
        }),
        ColorThumbMotion {
            handle_duration_ms: 60,
            loupe_duration_ms: 1200,
        }
    );
}

#[test]
fn attach_motion_exports_css_variables() {
    let style = attach_motion(
        None,
        ColorThumbMotion {
            handle_duration_ms: 220,
            loupe_duration_ms: 200,
        },
    );

    assert!(style.contains("--ui-color-thumb-handle-duration: 220ms;"));
    assert!(style.contains("--ui-color-thumb-loupe-duration: 200ms;"));
}
