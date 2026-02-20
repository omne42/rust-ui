#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum DirectionMode {
    #[default]
    Ltr,
    Rtl,
}

impl DirectionMode {
    pub fn as_attr(self) -> &'static str {
        match self {
            DirectionMode::Ltr => "ltr",
            DirectionMode::Rtl => "rtl",
        }
    }
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.into())
    })
}

pub fn compose_class_name(class_name: Option<String>) -> String {
    normalize_optional_text(class_name)
        .map(|class_name| format!("ui-direction-provider {class_name}"))
        .unwrap_or_else(|| "ui-direction-provider".to_string())
}

#[cfg(test)]
#[path = "../test/logic.rs"]
mod tests;
