use super::*;

#[test]
fn default_motion_is_stable() {
    assert_eq!(LegendMotion::default(), LegendMotion { duration_ms: 140.0 });
}

#[test]
fn sanitize_motion_clamps_values() {
    assert_eq!(
        sanitize_motion(LegendMotion {
            duration_ms: f64::NAN
        }),
        LegendMotion::default()
    );
    assert_eq!(
        sanitize_motion(LegendMotion { duration_ms: -10.0 }),
        LegendMotion { duration_ms: 1.0 }
    );
    assert_eq!(
        sanitize_motion(LegendMotion {
            duration_ms: 9999.0
        }),
        LegendMotion { duration_ms: 800.0 }
    );
}

#[test]
fn attach_motion_outputs_css_variable() {
    assert_eq!(
        attach_motion(LegendMotion { duration_ms: 220.0 }),
        "--ui-legend-motion-duration: 220ms;"
    );
}
