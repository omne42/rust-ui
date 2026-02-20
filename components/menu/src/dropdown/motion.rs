use crate::popover::PopoverMotion;

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct DropdownMotion {
    pub popover: PopoverMotion,
}

pub fn sanitize_motion(motion: DropdownMotion) -> DropdownMotion {
    DropdownMotion {
        popover: crate::popover::motion::sanitize_motion(motion.popover),
    }
}

#[cfg(test)]
#[path = "../../test/dropdown/motion.rs"]
mod tests;
