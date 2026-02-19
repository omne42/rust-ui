use crate::button::{ButtonSize, ButtonVariant};
use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LinkButtonState {
    pub is_disabled: bool,
    pub is_enabled: bool,
    pub has_href: bool,
    pub target_kind: &'static str,
    pub opens_new_context: bool,
    pub has_explicit_rel: bool,
    pub has_aria_label: bool,
    pub has_custom_class_name: bool,
}

pub fn normalize_href(href: String) -> Option<String> {
    let trimmed = href.trim();
    (!trimmed.is_empty()).then(|| trimmed.into())
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.into())
    })
}

pub fn resolve_rel(target: Option<&'static str>, rel: Option<String>) -> Option<String> {
    let mut tokens: BTreeSet<String> = BTreeSet::new();

    if let Some(rel) = rel {
        for token in rel.split_whitespace() {
            let token = token.trim();
            if !token.is_empty() {
                tokens.insert(token.into());
            }
        }
    }

    if matches!(target, Some("_blank")) {
        tokens.insert("noopener".to_string());
        tokens.insert("noreferrer".to_string());
    }

    if tokens.is_empty() {
        None
    } else {
        Some(tokens.into_iter().collect::<Vec<_>>().join(" "))
    }
}

pub fn resolve_state(
    disabled: bool,
    href: Option<&str>,
    target: Option<&'static str>,
    has_explicit_rel: bool,
    has_aria_label: bool,
    has_custom_class_name: bool,
) -> LinkButtonState {
    let target_kind = match target {
        Some("_blank") => "blank",
        Some(_) => "custom",
        None => "self",
    };

    LinkButtonState {
        is_disabled: disabled,
        is_enabled: !disabled,
        has_href: href.is_some(),
        target_kind,
        opens_new_context: target_kind == "blank",
        has_explicit_rel,
        has_aria_label,
        has_custom_class_name,
    }
}

pub fn compose_class_name(
    variant: ButtonVariant,
    size: ButtonSize,
    base_class_name: Option<String>,
    state: LinkButtonState,
) -> String {
    let mut classes = vec![
        "ui-link-button".to_string(),
        "ui-button".to_string(),
        variant.class_name().to_string(),
        size.class_name().to_string(),
    ];

    if state.is_enabled {
        classes.push("ui-link-button--enabled".to_string());
    }
    if state.is_disabled {
        classes.push("ui-link-button--disabled".to_string());
    }
    if state.opens_new_context {
        classes.push("ui-link-button--external".to_string());
    }
    if !state.has_href {
        classes.push("ui-link-button--missing-href".to_string());
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
    fn normalize_href_trims_and_rejects_blank_values() {
        assert_eq!(
            normalize_href(" https://example.com/docs ".to_string()),
            Some("https://example.com/docs".to_string())
        );
        assert_eq!(normalize_href("   ".to_string()), None);
    }

    #[test]
    fn normalize_optional_text_trims_and_rejects_blank_values() {
        assert_eq!(
            normalize_optional_text(Some(" external ".to_string())),
            Some("external".to_string())
        );
        assert_eq!(normalize_optional_text(Some("  ".to_string())), None);
        assert_eq!(normalize_optional_text(None), None);
    }

    #[test]
    fn resolve_rel_adds_security_tokens_for_blank_targets() {
        assert_eq!(
            resolve_rel(Some("_blank"), Some("noopener custom".to_string())),
            Some("custom noopener noreferrer".to_string())
        );
        assert_eq!(
            resolve_rel(Some("_self"), Some("  sponsored   sponsored  ".to_string())),
            Some("sponsored".to_string())
        );
        assert_eq!(resolve_rel(None, None), None);
    }

    #[test]
    fn resolve_state_tracks_enablement_target_and_metadata() {
        let enabled_state = resolve_state(
            false,
            Some("https://example.com"),
            Some("_blank"),
            true,
            true,
            true,
        );

        assert!(enabled_state.is_enabled);
        assert!(!enabled_state.is_disabled);
        assert!(enabled_state.has_href);
        assert_eq!(enabled_state.target_kind, "blank");
        assert!(enabled_state.opens_new_context);
        assert!(enabled_state.has_explicit_rel);
        assert!(enabled_state.has_aria_label);
        assert!(enabled_state.has_custom_class_name);

        let disabled_state = resolve_state(false, None, None, false, false, false);
        assert!(disabled_state.is_enabled);
        assert!(!disabled_state.has_href);
        assert_eq!(disabled_state.target_kind, "self");
        assert!(!disabled_state.opens_new_context);
        assert!(!disabled_state.has_explicit_rel);
    }

    #[test]
    fn compose_class_name_includes_state_tokens() {
        let class_name = compose_class_name(
            ButtonVariant::Secondary,
            ButtonSize::Lg,
            Some("custom".to_string()),
            resolve_state(
                true,
                Some("https://example.com"),
                Some("_blank"),
                false,
                false,
                true,
            ),
        );

        for token in [
            "ui-link-button",
            "ui-button",
            "ui-link-button--disabled",
            "ui-link-button--external",
            "custom",
        ] {
            assert!(
                class_name.contains(token),
                "composed class name should include `{token}`"
            );
        }
    }
}
