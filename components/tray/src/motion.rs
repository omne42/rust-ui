#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct TrayMotion {
    pub sheet: crate::sheet::SheetMotion,
}

pub fn sanitize_motion(motion: TrayMotion) -> TrayMotion {
    TrayMotion {
        sheet: crate::sheet::motion::sanitize_motion(motion.sheet),
    }
}

#[cfg(test)]
#[path = "../test/motion.rs"]
mod tests;
