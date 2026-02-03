#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum KbdSize {
    Sm,
    #[default]
    Md,
}

impl KbdSize {
    pub fn class_name(self) -> &'static str {
        match self {
            KbdSize::Sm => "ui-kbd--size-sm",
            KbdSize::Md => "ui-kbd--size-md",
        }
    }
}
