use super::*;

#[test]
fn default_motion_uses_default_sheet_motion_contract() {
    let motion = BottomSheetMotion::default();

    assert_eq!(motion.sheet, crate::sheet::SheetMotion::default());
}

#[test]
fn supports_custom_sheet_motion_contract() {
    let motion = BottomSheetMotion {
        sheet: crate::sheet::SheetMotion {
            spring: ui_motion::spring::SpringConfig {
                stiffness: 268.0,
                damping: 24.0,
                mass: 1.0,
                precision: 0.002,
            },
            initial_offset_px: 44.0,
        },
    };

    assert_eq!(motion.sheet.spring.stiffness, 268.0);
    assert_eq!(motion.sheet.spring.damping, 24.0);
    assert_eq!(motion.sheet.spring.mass, 1.0);
    assert_eq!(motion.sheet.spring.precision, 0.002);
    assert_eq!(motion.sheet.initial_offset_px, 44.0);
}

#[test]
fn sanitize_motion_delegates_to_sheet_contract() {
    let motion = sanitize_motion(BottomSheetMotion {
        sheet: crate::sheet::SheetMotion {
            spring: ui_motion::spring::SpringConfig {
                stiffness: f64::NAN,
                damping: -1.0,
                mass: 0.0,
                precision: f64::INFINITY,
            },
            initial_offset_px: -9999.0,
        },
    });

    assert_eq!(
        motion.sheet,
        crate::sheet::motion::sanitize_motion(motion.sheet)
    );
    assert_eq!(motion.sheet.initial_offset_px, 640.0);
    assert_eq!(
        motion.sheet.spring,
        crate::sheet::motion::sanitize_motion(crate::sheet::SheetMotion {
            spring: ui_motion::spring::SpringConfig {
                stiffness: f64::NAN,
                damping: -1.0,
                mass: 0.0,
                precision: f64::INFINITY,
            },
            initial_offset_px: -9999.0,
        })
        .spring
    );
}
