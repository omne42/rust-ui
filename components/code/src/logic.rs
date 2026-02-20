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

    pub fn as_attr(self) -> &'static str {
        match self {
            CodeVariant::Inline => "inline",
            CodeVariant::Block => "block",
        }
    }

    pub fn is_block(self) -> bool {
        matches!(self, CodeVariant::Block)
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

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.into())
    })
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

pub fn compose_class_name(base_class_name: Option<String>, state: CodeState) -> String {
    let mut classes = vec![
        "ui-code".to_string(),
        state.variant_class.into(),
        state.state_class.into(),
    ];

    if state.has_custom_class_name {
        classes.push("ui-code--custom-class".to_string());
        if let Some(base_class_name) = base_class_name {
            classes.push(base_class_name);
        }
    }

    classes.join(" ")
}

#[cfg(test)]
#[path = "../test/logic.rs"]
mod tests;
