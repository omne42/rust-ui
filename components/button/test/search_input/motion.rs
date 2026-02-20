use super::*;

#[test]
fn default_motion_matches_search_input_button_spring_contract() {
    let motion = SearchInputButtonMotion::default();

    assert_eq!(
        motion.spring,
        ui_motion::spring::SpringConfig {
            stiffness: 260.0,
            damping: 16.0,
            mass: 1.0,
            ..Default::default()
        }
    );
    assert_eq!(motion.hover_scale, 1.0);
    assert_eq!(motion.tap_scale, 0.98);
}

#[test]
fn supports_custom_search_input_button_motion_contract() {
    let motion = SearchInputButtonMotion {
        spring: ui_motion::spring::SpringConfig {
            stiffness: 284.0,
            damping: 18.0,
            mass: 1.0,
            precision: 0.002,
        },
        hover_scale: 1.03,
        tap_scale: 0.95,
    };

    assert_eq!(motion.spring.stiffness, 284.0);
    assert_eq!(motion.spring.damping, 18.0);
    assert_eq!(motion.spring.mass, 1.0);
    assert_eq!(motion.spring.precision, 0.002);
    assert_eq!(motion.hover_scale, 1.03);
    assert_eq!(motion.tap_scale, 0.95);
}

#[test]
fn sanitize_motion_falls_back_for_invalid_values() {
    let motion = sanitize_motion(SearchInputButtonMotion {
        spring: ui_motion::spring::SpringConfig {
            stiffness: f64::NAN,
            damping: -1.0,
            mass: 0.0,
            precision: f64::INFINITY,
        },
        hover_scale: f64::NAN,
        tap_scale: f64::NAN,
    });

    let default = SearchInputButtonMotion::default();
    assert_eq!(motion.spring.stiffness, default.spring.stiffness);
    assert_eq!(motion.spring.damping, default.spring.damping);
    assert_eq!(motion.spring.mass, default.spring.mass);
    assert_eq!(motion.spring.precision, default.spring.precision);
    assert_eq!(motion.hover_scale, default.hover_scale);
    assert_eq!(motion.tap_scale, default.tap_scale);
}

#[test]
fn sanitize_motion_clamps_scale_values() {
    let motion = sanitize_motion(SearchInputButtonMotion {
        spring: ui_motion::spring::SpringConfig {
            stiffness: 320.0,
            damping: 20.0,
            mass: 1.1,
            precision: 0.002,
        },
        hover_scale: 5.0,
        tap_scale: -2.0,
    });

    assert_eq!(motion.spring.stiffness, 320.0);
    assert_eq!(motion.spring.damping, 20.0);
    assert_eq!(motion.spring.mass, 1.1);
    assert_eq!(motion.spring.precision, 0.002);
    assert_eq!(motion.hover_scale, 2.0);
    assert_eq!(motion.tap_scale, 0.5);
}
