#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct DrawerMotion {
    pub sheet: ui_sheet::SheetMotion,
}

pub fn sanitize_motion(motion: DrawerMotion) -> DrawerMotion {
    DrawerMotion {
        sheet: ui_sheet::motion::sanitize_motion(motion.sheet),
    }
}

#[cfg(test)]
#[path = "test/motion.rs"]
mod tests;
