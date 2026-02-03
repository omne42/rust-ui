#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum SpinnerSize {
    Sm,
    #[default]
    Md,
    Lg,
}

impl SpinnerSize {
    pub fn class_name(self) -> &'static str {
        match self {
            SpinnerSize::Sm => "ui-spinner--size-sm",
            SpinnerSize::Md => "ui-spinner--size-md",
            SpinnerSize::Lg => "ui-spinner--size-lg",
        }
    }
}
