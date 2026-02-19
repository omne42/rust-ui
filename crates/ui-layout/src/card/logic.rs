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
mod tests {
    use super::*;

    #[test]
    fn variant_mappings_are_stable() {
        assert_eq!(
            CardVariant::Default.class_name(),
            "ui-card--variant-default"
        );
        assert_eq!(CardVariant::Muted.class_name(), "ui-card--variant-muted");
        assert_eq!(
            CardVariant::Outline.class_name(),
            "ui-card--variant-outline"
        );

        assert_eq!(CardVariant::Default.as_str(), "default");
        assert_eq!(CardVariant::Muted.as_str(), "muted");
        assert_eq!(CardVariant::Outline.as_str(), "outline");
    }

    #[test]
    fn normalize_optional_text_filters_blank_values() {
        assert_eq!(normalize_optional_text(None), None);
        assert_eq!(normalize_optional_text(Some("\n\t".to_string())), None);
        assert_eq!(
            normalize_optional_text(Some("  docs-card  ".to_string())),
            Some("docs-card".to_string())
        );
    }

    #[test]
    fn resolve_state_tracks_variant_padding_and_custom_class() {
        let state = resolve_state(CardStateInput {
            variant: CardVariant::Outline,
            padded: false,
            has_custom_class_name: true,
        });

        assert_eq!(state.variant, CardVariant::Outline);
        assert_eq!(state.variant_class, "ui-card--variant-outline");
        assert_eq!(state.variant_attr, "outline");
        assert!(!state.is_padded);
        assert!(state.is_flush);
        assert!(state.has_custom_class_name);
    }

    #[test]
    fn compose_class_name_includes_state_markers() {
        let class_name = compose_class_name(
            Some("custom".to_string()),
            resolve_state(CardStateInput {
                variant: CardVariant::Muted,
                padded: true,
                has_custom_class_name: true,
            }),
        );

        for token in [
            "ui-card",
            "ui-card--variant-muted",
            "ui-card--padded",
            "custom",
        ] {
            assert!(
                class_name.contains(token),
                "composed class name should include `{token}`"
            );
        }
    }
}
