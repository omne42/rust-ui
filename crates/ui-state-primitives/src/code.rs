pub use crate::button::normalize_optional_text;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum CodeVariant {
    #[default]
    Inline,
    Block,
}

impl CodeVariant {
    pub const fn class_name(self) -> &'static str {
        match self {
            Self::Inline => "ui-code--variant-inline",
            Self::Block => "ui-code--variant-block",
        }
    }

    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::Inline => "inline",
            Self::Block => "block",
        }
    }

    pub const fn is_block(self) -> bool {
        matches!(self, Self::Block)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CodeStateInput {
    pub variant: CodeVariant,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CodeState {
    pub variant: CodeVariant,
    pub variant_class: &'static str,
    pub variant_attr: &'static str,
    pub state_class: &'static str,
    pub state_attr: &'static str,
    pub is_inline: bool,
    pub is_block: bool,
    pub has_custom_class_name: bool,
}

pub fn resolve_state(input: CodeStateInput) -> CodeState {
    let is_block = input.variant.is_block();
    let (state_class, state_attr) = if is_block {
        ("ui-code--state-block", "block")
    } else {
        ("ui-code--state-inline", "inline")
    };

    CodeState {
        variant: input.variant,
        variant_class: input.variant.class_name(),
        variant_attr: input.variant.as_attr(),
        state_class,
        state_attr,
        is_inline: !is_block,
        is_block,
        has_custom_class_name: input.has_custom_class_name,
    }
}

#[cfg(test)]
#[path = "test/code.rs"]
mod tests;
