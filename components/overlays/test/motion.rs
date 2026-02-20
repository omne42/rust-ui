use super::*;

#[test]
fn default_motion_matches_underlaying_contracts() {
    let motion = OverlaysMotion::default();

    assert_eq!(motion.overlay, crate::overlay::OverlayMotion::default());
    assert_eq!(motion.popover, crate::popover::PopoverMotion::default());
    assert_eq!(motion.tray, crate::tray::TrayMotion::default());
}

#[test]
fn sanitize_motion_delegates_to_overlay_family() {
    let input = OverlaysMotion {
        overlay: crate::overlay::OverlayMotion {
            spring: ui_motion::spring::SpringConfig {
                stiffness: f64::NAN,
                damping: -1.0,
                mass: 0.0,
                precision: f64::INFINITY,
            },
            initial_scale: f64::NAN,
            initial_y_px: f64::NAN,
        },
        popover: crate::popover::PopoverMotion {
            spring: ui_motion::spring::SpringConfig {
                stiffness: f64::NAN,
                damping: -1.0,
                mass: 0.0,
                precision: f64::INFINITY,
            },
            initial_scale: f64::NAN,
            offset_y_px: f64::NAN,
        },
        tray: crate::tray::TrayMotion {
            sheet: crate::sheet::SheetMotion {
                spring: ui_motion::spring::SpringConfig {
                    stiffness: f64::NAN,
                    damping: -1.0,
                    mass: 0.0,
                    precision: f64::INFINITY,
                },
                initial_offset_px: f64::NAN,
            },
        },
    };

    let motion = sanitize_motion(input);

    assert_eq!(
        motion.overlay,
        crate::overlay::motion::sanitize_motion(input.overlay)
    );
    assert_eq!(
        motion.popover,
        crate::popover::motion::sanitize_motion(input.popover)
    );
    assert_eq!(
        motion.tray,
        crate::tray::motion::sanitize_motion(input.tray)
    );
}
