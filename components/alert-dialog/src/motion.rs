#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct AlertDialogMotion {
    pub overlay: crate::overlay::OverlayMotion,
}

pub fn sanitize_motion(motion: AlertDialogMotion) -> AlertDialogMotion {
    AlertDialogMotion {
        overlay: crate::overlay::motion::sanitize_motion(motion.overlay),
    }
}

#[cfg(test)]
#[path = "test/motion.rs"]
mod tests;
