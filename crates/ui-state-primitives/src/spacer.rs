#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum SpacerAxis {
    #[default]
    Vertical,
    Horizontal,
}

impl SpacerAxis {
    pub fn class_name(self) -> &'static str {
        match self {
            SpacerAxis::Vertical => "ui-spacer--axis-vertical",
            SpacerAxis::Horizontal => "ui-spacer--axis-horizontal",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            SpacerAxis::Vertical => "vertical",
            SpacerAxis::Horizontal => "horizontal",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum SpacerSize {
    Xs,
    Sm,
    #[default]
    Md,
    Lg,
    Xl,
}

impl SpacerSize {
    pub fn class_name(self) -> &'static str {
        match self {
            SpacerSize::Xs => "ui-spacer--size-xs",
            SpacerSize::Sm => "ui-spacer--size-sm",
            SpacerSize::Md => "ui-spacer--size-md",
            SpacerSize::Lg => "ui-spacer--size-lg",
            SpacerSize::Xl => "ui-spacer--size-xl",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            SpacerSize::Xs => "xs",
            SpacerSize::Sm => "sm",
            SpacerSize::Md => "md",
            SpacerSize::Lg => "lg",
            SpacerSize::Xl => "xl",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SpacerStateInput {
    pub axis: SpacerAxis,
    pub size: SpacerSize,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SpacerState {
    pub axis: SpacerAxis,
    pub size: SpacerSize,
    pub axis_class: &'static str,
    pub size_class: &'static str,
    pub axis_attr: &'static str,
    pub size_attr: &'static str,
    pub is_vertical: bool,
    pub is_horizontal: bool,
    pub has_custom_class_name: bool,
}

pub fn resolve_state(input: SpacerStateInput) -> SpacerState {
    SpacerState {
        axis: input.axis,
        size: input.size,
        axis_class: input.axis.class_name(),
        size_class: input.size.class_name(),
        axis_attr: input.axis.as_attr(),
        size_attr: input.size.as_attr(),
        is_vertical: matches!(input.axis, SpacerAxis::Vertical),
        is_horizontal: matches!(input.axis, SpacerAxis::Horizontal),
        has_custom_class_name: input.has_custom_class_name,
    }
}

#[cfg(test)]
#[path = "test/spacer.rs"]
mod tests;
