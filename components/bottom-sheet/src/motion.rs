#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct BottomSheetMotion {
    pub sheet: crate::sheet::SheetMotion,
}

pub fn sanitize_motion(motion: BottomSheetMotion) -> BottomSheetMotion {
    BottomSheetMotion {
        sheet: crate::sheet::motion::sanitize_motion(motion.sheet),
    }
}

#[cfg(test)]
#[path = "../test/motion.rs"]
mod tests;
