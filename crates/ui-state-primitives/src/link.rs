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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LinkTargetKind {
    SelfContext,
    Blank,
    Custom,
}

impl LinkTargetKind {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::SelfContext => "self",
            Self::Blank => "blank",
            Self::Custom => "custom",
        }
    }

    pub const fn opens_new_context(self) -> bool {
        matches!(self, Self::Blank)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LinkVisualState {
    Enabled,
    Disabled,
    MissingHref,
}

impl LinkVisualState {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::Enabled => "enabled",
            Self::Disabled => "disabled",
            Self::MissingHref => "missing-href",
        }
    }

    pub const fn as_class(self) -> &'static str {
        match self {
            Self::Enabled => "ui-link--enabled",
            Self::Disabled => "ui-link--disabled",
            Self::MissingHref => "ui-link--missing-href",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LinkRelSource {
    Provided,
    Auto,
}

impl LinkRelSource {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::Provided => "provided",
            Self::Auto => "auto",
        }
    }

    pub const fn as_class(self) -> &'static str {
        match self {
            Self::Provided => "ui-link--rel-provided",
            Self::Auto => "ui-link--rel-auto",
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
    pub target_kind: LinkTargetKind,
    pub has_explicit_rel: bool,
    pub has_aria_label: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LinkState {
    pub is_disabled: bool,
    pub is_enabled: bool,
    pub has_href: bool,
    pub target_kind: LinkTargetKind,
    pub opens_new_context: bool,
    pub has_explicit_rel: bool,
    pub has_aria_label: bool,
    pub has_custom_class_name: bool,
    pub state: LinkVisualState,
    pub rel_source: LinkRelSource,
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

pub fn resolve_target_kind(target: Option<&str>) -> LinkTargetKind {
    match target {
        Some("_blank") => LinkTargetKind::Blank,
        Some(_) => LinkTargetKind::Custom,
        None => LinkTargetKind::SelfContext,
    }
}

pub fn resolve_rel(target_kind: LinkTargetKind, rel: Option<String>) -> Option<String> {
    let mut tokens: BTreeSet<String> = BTreeSet::new();

    if let Some(rel) = rel {
        for token in rel.split_whitespace() {
            let token = token.trim();
            if !token.is_empty() {
                tokens.insert(token.into());
            }
        }
    }

    if target_kind.opens_new_context() {
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
    let state = if input.is_disabled {
        LinkVisualState::Disabled
    } else if input.has_href {
        LinkVisualState::Enabled
    } else {
        LinkVisualState::MissingHref
    };

    let rel_source = if input.has_explicit_rel {
        LinkRelSource::Provided
    } else {
        LinkRelSource::Auto
    };

    LinkState {
        is_disabled: input.is_disabled,
        is_enabled: !input.is_disabled,
        has_href: input.has_href,
        target_kind: input.target_kind,
        opens_new_context: input.target_kind.opens_new_context(),
        has_explicit_rel: input.has_explicit_rel,
        has_aria_label: input.has_aria_label,
        has_custom_class_name: input.has_custom_class_name,
        state,
        rel_source,
    }
}

#[cfg(test)]
#[path = "test/link.rs"]
mod tests;
