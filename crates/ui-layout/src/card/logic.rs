#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum CardVariant {
    #[default]
    Default,
    Muted,
    Outline,
}

impl CardVariant {
    pub fn class_name(self) -> &'static str {
        match self {
            CardVariant::Default => "ui-card--variant-default",
            CardVariant::Muted => "ui-card--variant-muted",
            CardVariant::Outline => "ui-card--variant-outline",
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            CardVariant::Default => "default",
            CardVariant::Muted => "muted",
            CardVariant::Outline => "outline",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CardStateInput {
    pub variant: CardVariant,
    pub padded: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CardState {
    pub variant: CardVariant,
    pub variant_class: &'static str,
    pub variant_attr: &'static str,
    pub is_padded: bool,
    pub is_flush: bool,
    pub has_custom_class_name: bool,
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.into())
    })
}

pub fn resolve_state(input: CardStateInput) -> CardState {
    CardState {
        variant: input.variant,
        variant_class: input.variant.class_name(),
        variant_attr: input.variant.as_str(),
        is_padded: input.padded,
        is_flush: !input.padded,
        has_custom_class_name: input.has_custom_class_name,
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: CardState) -> String {
    let mut classes = vec!["ui-card".to_string(), state.variant_class.into()];

    if state.is_padded {
        classes.push("ui-card--padded".to_string());
    }
    if state.is_flush {
        classes.push("ui-card--no-padding".to_string());
    }

    if state.has_custom_class_name
        && let Some(base_class_name) = base_class_name
    {
        classes.push(base_class_name);
    }

    classes.join(" ")
}

#[cfg(test)]
#[path = "test/logic.rs"]
mod tests;
