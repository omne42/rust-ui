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
        (!trimmed.is_empty()).then(|| trimmed.into())
    })
}

pub fn compose_class(base: &'static str, class_name: Option<String>) -> String {
    normalize_optional_text(class_name)
        .map(|class_name| format!("{base} {class_name}"))
        .unwrap_or_else(|| base.into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_optional_text_trims_and_filters_blank_values() {
        assert_eq!(normalize_optional_text(None), None);
        assert_eq!(normalize_optional_text(Some("".to_string())), None);
        assert_eq!(normalize_optional_text(Some("   ".to_string())), None);
        assert_eq!(
            normalize_optional_text(Some("  hello  ".to_string())),
            Some("hello".to_string())
        );
    }

    #[test]
    fn compose_class_appends_custom_class_when_present() {
        assert_eq!(compose_class("base", None), "base");
        assert_eq!(compose_class("base", Some("".to_string())), "base");
        assert_eq!(
            compose_class("base", Some("  extra  ".to_string())),
            "base extra"
        );
    }

    #[test]
    fn item_attrs_match_variants() {
        assert_eq!(ItemVariant::Default.as_attr(), "default");
        assert_eq!(ItemVariant::Outline.as_attr(), "outline");
        assert_eq!(ItemVariant::Muted.as_attr(), "muted");

        assert_eq!(ItemSize::Default.as_attr(), "default");
        assert_eq!(ItemSize::Sm.as_attr(), "sm");

        assert_eq!(ItemMediaVariant::Default.as_attr(), "default");
        assert_eq!(ItemMediaVariant::Icon.as_attr(), "icon");
        assert_eq!(ItemMediaVariant::Image.as_attr(), "image");
    }
}
