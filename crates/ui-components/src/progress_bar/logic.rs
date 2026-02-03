#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ProgressBarVariant {
    #[default]
    Default,
    Accent,
    Danger,
}

impl ProgressBarVariant {
    pub fn class_name(self) -> &'static str {
        match self {
            ProgressBarVariant::Default => "ui-progress-bar--variant-default",
            ProgressBarVariant::Accent => "ui-progress-bar--variant-accent",
            ProgressBarVariant::Danger => "ui-progress-bar--variant-danger",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ProgressBarSize {
    Sm,
    #[default]
    Md,
    Lg,
}

impl ProgressBarSize {
    pub fn class_name(self) -> &'static str {
        match self {
            ProgressBarSize::Sm => "ui-progress-bar--size-sm",
            ProgressBarSize::Md => "ui-progress-bar--size-md",
            ProgressBarSize::Lg => "ui-progress-bar--size-lg",
        }
    }
}
