#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct TrayMotion {
    pub sheet: crate::sheet::SheetMotion,
}

fn sanitize_spring(value: ui_motion::spring::SpringConfig) -> ui_motion::spring::SpringConfig {
    let default = TrayMotion::default().sheet.spring;

    ui_motion::spring::SpringConfig {
        stiffness: if value.stiffness.is_finite() && value.stiffness > 0.0 {
            value.stiffness
        } else {
            default.stiffness
        },
        damping: if value.damping.is_finite() && value.damping > 0.0 {
            value.damping
        } else {
            default.damping
        },
        mass: if value.mass.is_finite() && value.mass > 0.0 {
            value.mass
        } else {
            default.mass
        },
        precision: if value.precision.is_finite() && value.precision > 0.0 {
            value.precision
        } else {
            default.precision
        },
    }
}

pub fn sanitize_motion(motion: TrayMotion) -> TrayMotion {
    let default = TrayMotion::default();
    let initial_offset_px = if motion.sheet.initial_offset_px.is_finite() {
        motion.sheet.initial_offset_px.abs().clamp(0.0, 640.0)
    } else {
        default.sheet.initial_offset_px
    };

    TrayMotion {
        sheet: crate::sheet::SheetMotion {
            spring: sanitize_spring(motion.sheet.spring),
            initial_offset_px,
        },
    }
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
}
