#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct DrawerMotion {
    pub sheet: crate::sheet::SheetMotion,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_motion_uses_default_sheet_motion_contract() {
        let motion = DrawerMotion::default();

        assert_eq!(motion.sheet, crate::sheet::SheetMotion::default());
    }

    #[test]
    fn supports_custom_sheet_motion_contract() {
        let motion = DrawerMotion {
            sheet: crate::sheet::SheetMotion {
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
}
