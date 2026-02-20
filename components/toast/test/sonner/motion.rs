use super::*;

#[test]
fn sanitize_motion_delegates_to_toast_contract() {
    let input = crate::toast::ToastMotion {
        spring: ui_motion::spring::SpringConfig {
            stiffness: f64::NAN,
            damping: -1.0,
            mass: 0.0,
            precision: f64::INFINITY,
        },
        initial_y_px: f64::INFINITY,
        initial_scale: 0.0,
    };
    let expected = crate::toast::motion::sanitize_motion(input);

    assert_eq!(sanitize_motion(input), expected);
}
