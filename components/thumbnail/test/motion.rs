use super::*;

#[test]
fn sanitize_motion_falls_back_for_invalid_values() {
    let motion = sanitize_motion(ThumbnailMotion {
        enabled: true,
        spring: ui_motion::spring::SpringConfig {
            stiffness: f64::NAN,
            damping: -1.0,
            mass: 0.0,
            precision: f64::INFINITY,
        },
        active_scale: f64::NAN,
        active_ring_opacity: f64::INFINITY,
    });

    let default = ThumbnailMotion::default();
    assert_eq!(motion.spring.stiffness, default.spring.stiffness);
    assert_eq!(motion.spring.damping, default.spring.damping);
    assert_eq!(motion.spring.mass, default.spring.mass);
    assert_eq!(motion.spring.precision, default.spring.precision);
    assert_eq!(motion.active_scale, default.active_scale);
    assert_eq!(motion.active_ring_opacity, 1.0);
}

#[test]
fn disabled_constructor_turns_motion_off() {
    let motion = ThumbnailMotion::disabled();
    assert!(!motion.enabled);
}
