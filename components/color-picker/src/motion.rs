use crate::popover::PopoverMotion;

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct ColorPickerMotion {
    pub popover: PopoverMotion,
}

pub fn sanitize_motion(motion: ColorPickerMotion) -> ColorPickerMotion {
    ColorPickerMotion {
        popover: crate::popover::motion::sanitize_motion(motion.popover),
    }
}

#[cfg(test)]
#[path = "../test/motion.rs"]
mod tests;
