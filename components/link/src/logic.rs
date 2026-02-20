use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LinkDisabledSource {
    IsProp,
    Default,
}

impl LinkDisabledSource {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::IsProp => "is-prop",
            Self::Default => "default",
        }
    }
}

pub fn normalize_is_disabled(is_disabled: Option<bool>) -> (bool, LinkDisabledSource) {
    if let Some(value) = is_disabled {
        return (value, LinkDisabledSource::IsProp);
    }

    (false, LinkDisabledSource::Default)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LinkStateInput {
    pub is_disabled: bool,
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

pub fn resolve_state(input: LinkStateInput) -> LinkState {
    let target_kind = match input.target {
        Some("_blank") => "blank",
        Some(_) => "custom",
        None => "self",
    };

    let (state_class, state_attr) = if input.is_disabled {
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
        is_disabled: input.is_disabled,
        is_enabled: !input.is_disabled,
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
        state.state_class.into(),
        state.rel_source_class.into(),
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
#[path = "../test/logic.rs"]
mod tests;
