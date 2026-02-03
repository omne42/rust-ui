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
}
