use crate::popover::PopoverMotion;

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct DropdownMenuMotion {
    pub popover: PopoverMotion,
}

pub fn sanitize_motion(motion: DropdownMenuMotion) -> DropdownMenuMotion {
    DropdownMenuMotion {
        popover: crate::popover::motion::sanitize_motion(motion.popover),
    }
}

#[cfg(test)]
#[path = "../../test/dropdown_menu/motion.rs"]
mod tests;
