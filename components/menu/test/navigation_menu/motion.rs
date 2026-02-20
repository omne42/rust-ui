use super::*;

#[test]
fn sanitize_motion_clamps_invalid_spring_values() {
    let default = ui_visual_primitive::active_highlight::ActiveHighlightMotion::default();
    let motion = sanitize_motion(
        ui_visual_primitive::active_highlight::ActiveHighlightMotion {
            spring: ui_motion::spring::SpringConfig {
                stiffness: f64::NAN,
                damping: -1.0,
                mass: 0.0,
                precision: f64::INFINITY,
            },
        },
    );

    assert_eq!(motion.spring.stiffness, default.spring.stiffness);
    assert_eq!(motion.spring.damping, default.spring.damping);
    assert_eq!(motion.spring.mass, default.spring.mass);
    assert_eq!(motion.spring.precision, default.spring.precision);
}
