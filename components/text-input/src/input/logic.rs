#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum InputLabelPlacement {
    #[default]
    Outside,
    Inside,
}

impl InputLabelPlacement {
    pub fn class_name(self) -> &'static str {
        match self {
            InputLabelPlacement::Outside => "ui-input--label-outside",
            InputLabelPlacement::Inside => "ui-input--label-inside",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum InputSize {
    Sm,
    #[default]
    Md,
    Lg,
}

impl InputSize {
    pub fn class_name(self) -> &'static str {
        match self {
            InputSize::Sm => "ui-input--size-sm",
            InputSize::Md => "ui-input--size-md",
            InputSize::Lg => "ui-input--size-lg",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum InputVariant {
    #[default]
    Bordered,
    Flat,
    Underlined,
}

impl InputVariant {
    pub fn class_name(self) -> &'static str {
        match self {
            InputVariant::Bordered => "ui-input--variant-bordered",
            InputVariant::Flat => "ui-input--variant-flat",
            InputVariant::Underlined => "ui-input--variant-underlined",
        }
    }
}

pub use ui_state_primitives::input::{
    InputLogicState, resolve_clear_aria_label, resolve_view_state,
};

#[cfg(test)]
#[path = "../../test/input/logic.rs"]
mod tests;
