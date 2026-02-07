use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LinkStateInput {
    pub disabled: bool,
    pub has_href: bool,
    pub target: Option<&'static str>,
    pub has_explicit_rel: bool,
    pub has_aria_label: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LinkState {
    pub is_disabled: bool,
    pub is_enabled: bool,
    pub has_href: bool,
    pub target_kind: &'static str,
    pub opens_new_context: bool,
    pub has_explicit_rel: bool,
    pub has_aria_label: bool,
    pub has_custom_class_name: bool,
    pub state_class: &'static str,
    pub state_attr: &'static str,
    pub rel_source_class: &'static str,
    pub rel_source_attr: &'static str,
}

pub fn normalize_href(href: String) -> Option<String> {
    let trimmed = href.trim();
    (!trimmed.is_empty()).then(|| trimmed.to_string())
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.to_string())
    })
}

pub fn resolve_rel(target: Option<&'static str>, rel: Option<String>) -> Option<String> {
    let mut tokens: BTreeSet<String> = BTreeSet::new();

    if let Some(rel) = rel {
        for token in rel.split_whitespace() {
            let token = token.trim();
            if !token.is_empty() {
                tokens.insert(token.to_string());
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

pub fn resolve_state(input: LinkStateInput) -> LinkState {
    let target_kind = match input.target {
        Some("_blank") => "blank",
        Some(_) => "custom",
        None => "self",
    };

    let (state_class, state_attr) = if input.disabled {
        ("ui-link--disabled", "disabled")
    } else if input.has_href {
        ("ui-link--enabled", "enabled")
    } else {
        ("ui-link--missing-href", "missing-href")
    };

    let (rel_source_class, rel_source_attr) = if input.has_explicit_rel {
        ("ui-link--rel-provided", "provided")
    } else {
        ("ui-link--rel-auto", "auto")
    };

    LinkState {
        is_disabled: input.disabled,
        is_enabled: !input.disabled,
        has_href: input.has_href,
        target_kind,
        opens_new_context: target_kind == "blank",
        has_explicit_rel: input.has_explicit_rel,
        has_aria_label: input.has_aria_label,
        has_custom_class_name: input.has_custom_class_name,
        state_class,
        state_attr,
        rel_source_class,
        rel_source_attr,
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: LinkState) -> String {
    let mut classes = vec![
        "ui-link".to_string(),
        state.state_class.to_string(),
        state.rel_source_class.to_string(),
    ];

    if state.opens_new_context {
        classes.push("ui-link--external".to_string());
    }

    if state.has_aria_label {
        classes.push("ui-link--with-aria-label".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-link--custom-class".to_string());
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
        let enabled_state = resolve_state(LinkStateInput {
            disabled: false,
            has_href: true,
            target: Some("_blank"),
            has_explicit_rel: true,
            has_aria_label: true,
            has_custom_class_name: true,
        });

        assert!(enabled_state.is_enabled);
        assert!(!enabled_state.is_disabled);
        assert!(enabled_state.has_href);
        assert_eq!(enabled_state.target_kind, "blank");
        assert!(enabled_state.opens_new_context);
        assert!(enabled_state.has_explicit_rel);
        assert!(enabled_state.has_aria_label);
        assert!(enabled_state.has_custom_class_name);
        assert_eq!(enabled_state.state_attr, "enabled");
        assert_eq!(enabled_state.rel_source_attr, "provided");

        let missing_state = resolve_state(LinkStateInput {
            disabled: false,
            has_href: false,
            target: None,
            has_explicit_rel: false,
            has_aria_label: false,
            has_custom_class_name: false,
        });
        assert_eq!(missing_state.state_attr, "missing-href");
        assert_eq!(missing_state.target_kind, "self");
        assert_eq!(missing_state.rel_source_attr, "auto");
    }

    #[test]
    fn compose_class_name_includes_state_tokens() {
        let class_name = compose_class_name(
            Some("custom".to_string()),
            resolve_state(LinkStateInput {
                disabled: false,
                has_href: true,
                target: Some("_blank"),
                has_explicit_rel: false,
                has_aria_label: true,
                has_custom_class_name: true,
            }),
        );

        for token in [
            "ui-link",
            "ui-link--enabled",
            "ui-link--rel-auto",
            "ui-link--external",
            "ui-link--with-aria-label",
            "ui-link--custom-class",
            "custom",
        ] {
            assert!(
                class_name.contains(token),
                "composed class name should include `{token}`"
            );
        }
    }
}
