#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ItemVariant {
    #[default]
    Default,
    Outline,
    Muted,
}

impl ItemVariant {
    pub fn as_attr(self) -> &'static str {
        match self {
            ItemVariant::Default => "default",
            ItemVariant::Outline => "outline",
            ItemVariant::Muted => "muted",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ItemSize {
    #[default]
    Default,
    Sm,
}

impl ItemSize {
    pub fn as_attr(self) -> &'static str {
        match self {
            ItemSize::Default => "default",
            ItemSize::Sm => "sm",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ItemMediaVariant {
    #[default]
    Default,
    Icon,
    Image,
}

impl ItemMediaVariant {
    pub fn as_attr(self) -> &'static str {
        match self {
            ItemMediaVariant::Default => "default",
            ItemMediaVariant::Icon => "icon",
            ItemMediaVariant::Image => "image",
        }
    }
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.to_string())
    })
}

pub fn compose_class(base: &'static str, class_name: Option<String>) -> String {
    normalize_optional_text(class_name)
        .map(|class_name| format!("{base} {class_name}"))
        .unwrap_or_else(|| base.to_string())
}
