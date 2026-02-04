use crate::ActiveHighlightMotion;
use crate::popover::PopoverMotion;

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct AutocompleteMotion {
    pub popover: PopoverMotion,
    pub highlight: ActiveHighlightMotion,
}
