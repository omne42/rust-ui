#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct TrayMotion {
    pub sheet: crate::sheet::SheetMotion,
}

#[cfg(test)]
mod tests {
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
}
