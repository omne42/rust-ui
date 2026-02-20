#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct OverlaysMotion {
    pub overlay: crate::overlay::OverlayMotion,
    pub popover: crate::popover::PopoverMotion,
    pub tray: crate::tray::TrayMotion,
}

pub fn sanitize_motion(motion: OverlaysMotion) -> OverlaysMotion {
    OverlaysMotion {
        overlay: crate::overlay::motion::sanitize_motion(motion.overlay),
        popover: crate::popover::motion::sanitize_motion(motion.popover),
        tray: crate::tray::motion::sanitize_motion(motion.tray),
    }
}

#[cfg(test)]
#[path = "../test/motion.rs"]
mod tests;
