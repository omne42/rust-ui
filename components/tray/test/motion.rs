use super::*;

#[test]
fn default_motion_uses_default_sheet_motion_contract() {
    let motion = TrayMotion::default();

    assert_eq!(motion.sheet, crate::sheet::SheetMotion::default());
}

#[test]
fn supports_custom_sheet_motion_contract() {
    let motion = TrayMotion {
        sheet: crate::sheet::SheetMotion {
            spring: ui_motion::spring::SpringConfig {
                stiffness: 255.0,
                damping: 23.0,
                mass: 1.0,
                precision: 0.002,
            },
            initial_offset_px: 40.0,
        },
    };

    assert_eq!(motion.sheet.spring.stiffness, 255.0);
    assert_eq!(motion.sheet.spring.damping, 23.0);
    assert_eq!(motion.sheet.spring.mass, 1.0);
    assert_eq!(motion.sheet.spring.precision, 0.002);
    assert_eq!(motion.sheet.initial_offset_px, 40.0);
}

#[test]
fn sanitize_motion_falls_back_for_invalid_values() {
    let motion = sanitize_motion(TrayMotion {
        sheet: crate::sheet::SheetMotion {
            spring: ui_motion::spring::SpringConfig {
                stiffness: f64::NAN,
                damping: -1.0,
                mass: 0.0,
                precision: f64::INFINITY,
            },
            initial_offset_px: f64::NAN,
        },
    });

    let default = TrayMotion::default();
    assert_eq!(
        motion.sheet.spring.stiffness,
        default.sheet.spring.stiffness
    );
    assert_eq!(motion.sheet.spring.damping, default.sheet.spring.damping);
    assert_eq!(motion.sheet.spring.mass, default.sheet.spring.mass);
    assert_eq!(
        motion.sheet.spring.precision,
        default.sheet.spring.precision
    );
    assert_eq!(
        motion.sheet.initial_offset_px,
        default.sheet.initial_offset_px
    );
}

#[test]
fn sanitize_motion_clamps_offset_range() {
    let motion = sanitize_motion(TrayMotion {
        sheet: crate::sheet::SheetMotion {
            spring: ui_motion::spring::SpringConfig {
                stiffness: 220.0,
                damping: 20.0,
                mass: 1.05,
                precision: 0.003,
            },
            initial_offset_px: -9999.0,
        },
    });

    assert_eq!(motion.sheet.initial_offset_px, 640.0);
    assert_eq!(motion.sheet.spring.stiffness, 220.0);
    assert_eq!(motion.sheet.spring.damping, 20.0);
    assert_eq!(motion.sheet.spring.mass, 1.05);
    assert_eq!(motion.sheet.spring.precision, 0.003);
}

#[test]
fn sanitize_motion_delegates_to_sheet_contract() {
    let input = crate::sheet::SheetMotion {
        spring: ui_motion::spring::SpringConfig {
            stiffness: f64::NAN,
            damping: -1.0,
            mass: 0.0,
            precision: f64::INFINITY,
        },
        initial_offset_px: -9999.0,
    };

    let motion = sanitize_motion(TrayMotion { sheet: input });
    let expected = crate::sheet::motion::sanitize_motion(input);

    assert_eq!(motion.sheet, expected);
}
