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
        (!trimmed.is_empty()).then(|| trimmed.to_string())
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
        state.variant_class.to_string(),
        state.state_class.to_string(),
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
mod tests {
    use super::*;

    #[test]
    fn variant_class_names_and_attrs_are_stable() {
        assert_eq!(CodeVariant::Inline.class_name(), "ui-code--variant-inline");
        assert_eq!(CodeVariant::Block.class_name(), "ui-code--variant-block");

        assert_eq!(CodeVariant::Inline.as_attr(), "inline");
        assert_eq!(CodeVariant::Block.as_attr(), "block");
    }

    #[test]
    fn normalize_optional_text_filters_blank_values() {
        assert_eq!(normalize_optional_text(None), None);
        assert_eq!(normalize_optional_text(Some("  \n\t".to_string())), None);
        assert_eq!(
            normalize_optional_text(Some("  docs-code  ".to_string())),
            Some("docs-code".to_string())
        );
    }

    #[test]
    fn resolve_state_tracks_variant_and_class_source() {
        let state = resolve_state(CodeStateInput {
            variant: CodeVariant::Block,
            has_custom_class_name: true,
        });

        assert_eq!(state.variant, CodeVariant::Block);
        assert_eq!(state.variant_class, "ui-code--variant-block");
        assert_eq!(state.variant_attr, "block");
        assert_eq!(state.state_class, "ui-code--state-block");
        assert_eq!(state.state_attr, "block");
        assert!(!state.is_inline);
        assert!(state.is_block);
        assert!(state.has_custom_class_name);
    }

    #[test]
    fn compose_class_name_includes_state_markers() {
        let class_name = compose_class_name(
            Some("docs-code".to_string()),
            resolve_state(CodeStateInput {
                variant: CodeVariant::Inline,
                has_custom_class_name: true,
            }),
        );

        for token in [
            "ui-code",
            "ui-code--variant-inline",
            "ui-code--state-inline",
            "ui-code--custom-class",
            "docs-code",
        ] {
            assert!(
                class_name.contains(token),
                "composed class name should include `{token}`"
            );
        }
    }
}
