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
#[path = "../../test/link_button/logic.rs"]
mod tests;
