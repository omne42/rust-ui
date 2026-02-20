use crate::popover::PopoverMotion;

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct DatePickerMotion {
    pub popover: PopoverMotion,
}

pub fn sanitize_motion(motion: DatePickerMotion) -> DatePickerMotion {
    DatePickerMotion {
        popover: crate::popover::motion::sanitize_motion(motion.popover),
    }
}

#[cfg(test)]
#[path = "../../test/date_picker/motion.rs"]
mod tests;
