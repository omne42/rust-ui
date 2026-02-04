#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ContextualHelpVariant {
    #[default]
    Help,
    Info,
}

impl ContextualHelpVariant {
    pub fn default_label(self) -> &'static str {
        match self {
            ContextualHelpVariant::Help => "Help",
            ContextualHelpVariant::Info => "Info",
        }
    }

    pub fn class_name(self) -> &'static str {
        match self {
            ContextualHelpVariant::Help => "ui-contextual-help--variant-help",
            ContextualHelpVariant::Info => "ui-contextual-help--variant-info",
        }
    }
}

pub fn resolve_trigger_aria_label(
    variant: ContextualHelpVariant,
    aria_label: Option<&str>,
) -> String {
    aria_label
        .map(str::trim)
        .filter(|v| !v.is_empty())
        .map(|v| v.to_string())
        .unwrap_or_else(|| variant.default_label().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_label_from_variant() {
        assert_eq!(
            resolve_trigger_aria_label(ContextualHelpVariant::Help, None),
            "Help"
        );
        assert_eq!(
            resolve_trigger_aria_label(ContextualHelpVariant::Info, None),
            "Info"
        );
    }

    #[test]
    fn aria_label_overrides_default() {
        assert_eq!(
            resolve_trigger_aria_label(ContextualHelpVariant::Help, Some("Custom")),
            "Custom"
        );
    }

    #[test]
    fn ignores_blank_label() {
        assert_eq!(
            resolve_trigger_aria_label(ContextualHelpVariant::Info, Some("   ")),
            "Info"
        );
    }
}
