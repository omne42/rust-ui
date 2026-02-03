#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum CodeVariant {
    #[default]
    Inline,
    Block,
}

impl CodeVariant {
    pub fn class_name(self) -> &'static str {
        match self {
            CodeVariant::Inline => "ui-code--variant-inline",
            CodeVariant::Block => "ui-code--variant-block",
        }
    }
}
