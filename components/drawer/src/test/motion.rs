use super::*;

#[test]
fn default_motion_uses_default_sheet_motion_contract() {
    let motion = DrawerMotion::default();

    assert_eq!(motion.sheet, ui_sheet::SheetMotion::default());
}

#[test]
fn supports_custom_sheet_motion_contract() {
    let motion = DrawerMotion {
        sheet: ui_sheet::SheetMotion {
            spring: ui_motion::spring::SpringConfig {
                stiffness: 250.0,
                damping: 22.0,
                mass: 1.0,
                precision: 0.002,
            },
            initial_offset_px: 44.0,
        },
    };

    assert_eq!(motion.sheet.spring.stiffness, 250.0);
    assert_eq!(motion.sheet.spring.damping, 22.0);
    assert_eq!(motion.sheet.spring.mass, 1.0);
    assert_eq!(motion.sheet.spring.precision, 0.002);
    assert_eq!(motion.sheet.initial_offset_px, 44.0);
}

#[test]
fn sanitize_motion_delegates_to_sheet_contract() {
    let input = ui_sheet::SheetMotion {
        spring: ui_motion::spring::SpringConfig {
            stiffness: f64::NAN,
            damping: -1.0,
            mass: 0.0,
            precision: f64::INFINITY,
        },
        initial_offset_px: -9999.0,
    };
    let motion = sanitize_motion(DrawerMotion { sheet: input });
    let expected = ui_sheet::motion::sanitize_motion(input);

    assert_eq!(motion.sheet, expected);
    assert_eq!(motion.sheet.initial_offset_px, 640.0);
}
