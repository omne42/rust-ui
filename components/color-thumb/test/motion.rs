use super::*;

#[test]
fn sanitize_motion_clamps_contract_values() {
    assert_eq!(
        sanitize_motion(ColorThumbMotion {
            handle_duration_ms: 12,
            loupe_duration_ms: 3000,
            spring: ui_motion::spring::SpringConfig {
                stiffness: f64::NAN,
                damping: -1.0,
                mass: 0.0,
                precision: 0.0,
            },
        }),
        ColorThumbMotion {
            handle_duration_ms: 60,
            loupe_duration_ms: 1200,
            spring: ColorThumbMotion::default().spring,
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
            spring: ui_motion::spring::SpringConfig {
                stiffness: 240.0,
                damping: 18.0,
                mass: 1.1,
                precision: 0.002,
            },
        },
    );

    if ui_motion::web::prefers_reduced_motion() {
        assert!(style.contains("--ui-color-thumb-handle-duration: 1ms;"));
        assert!(style.contains("--ui-color-thumb-loupe-duration: 1ms;"));
    } else {
        assert!(style.contains("--ui-color-thumb-handle-duration: 220ms;"));
        assert!(style.contains("--ui-color-thumb-loupe-duration: 200ms;"));
    }

    for required in [
        "--ui-color-thumb-motion-stiffness: 240",
        "--ui-color-thumb-motion-damping: 18",
        "--ui-color-thumb-motion-mass: 1.1",
        "--ui-color-thumb-motion-precision: 0.002",
    ] {
        assert!(
            style.contains(required),
            "attach_motion should export spring contract variable `{required}`."
        );
    }
}
