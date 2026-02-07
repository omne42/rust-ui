#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum AvatarSize {
    Sm,
    #[default]
    Md,
    Lg,
}

impl AvatarSize {
    pub fn class_name(self) -> &'static str {
        match self {
            AvatarSize::Sm => "ui-avatar--sm",
            AvatarSize::Md => "ui-avatar--md",
            AvatarSize::Lg => "ui-avatar--lg",
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            AvatarSize::Sm => "sm",
            AvatarSize::Md => "md",
            AvatarSize::Lg => "lg",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AvatarLabelSource {
    Alt,
    Name,
    Fallback,
}

impl AvatarLabelSource {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Alt => "alt",
            Self::Name => "name",
            Self::Fallback => "fallback",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AvatarStateInput {
    pub size: AvatarSize,
    pub has_name: bool,
    pub has_src: bool,
    pub has_alt: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AvatarState {
    pub size: AvatarSize,
    pub size_class: &'static str,
    pub size_attr: &'static str,
    pub has_name: bool,
    pub has_src: bool,
    pub has_alt: bool,
    pub has_custom_class_name: bool,
    pub label_source: AvatarLabelSource,
    pub label_source_attr: &'static str,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AvatarAccessibility {
    pub aria_label: String,
    pub img_alt: String,
    pub title: Option<String>,
    pub label_source: AvatarLabelSource,
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.to_string())
    })
}

pub(super) fn initials_from_name(name: &str) -> Option<String> {
    let name = name.trim();
    if name.is_empty() {
        return None;
    }

    let mut words = name.split_whitespace().filter(|w| !w.is_empty());
    let first = words.next()?;
    let last = words.next_back().unwrap_or(first);

    let first_char = first.chars().next()?;
    let last_char = (last != first).then(|| last.chars().next()).flatten();

    let mut initials = String::new();
    initials.push(first_char);
    if let Some(last_char) = last_char {
        initials.push(last_char);
    }

    Some(initials.to_uppercase())
}

pub fn resolve_initials(name: Option<&str>) -> String {
    name.and_then(initials_from_name)
        .unwrap_or_else(|| "?".to_string())
}

pub fn resolve_accessibility(name: Option<&str>, alt: Option<&str>) -> AvatarAccessibility {
    let title = name.map(|value| value.to_string());

    if let Some(alt) = alt {
        return AvatarAccessibility {
            aria_label: alt.to_string(),
            img_alt: alt.to_string(),
            title,
            label_source: AvatarLabelSource::Alt,
        };
    }

    if let Some(name) = name {
        return AvatarAccessibility {
            aria_label: name.to_string(),
            img_alt: name.to_string(),
            title,
            label_source: AvatarLabelSource::Name,
        };
    }

    AvatarAccessibility {
        aria_label: "Avatar".to_string(),
        img_alt: String::new(),
        title,
        label_source: AvatarLabelSource::Fallback,
    }
}

pub fn resolve_state(input: AvatarStateInput) -> AvatarState {
    let label_source = if input.has_alt {
        AvatarLabelSource::Alt
    } else if input.has_name {
        AvatarLabelSource::Name
    } else {
        AvatarLabelSource::Fallback
    };

    AvatarState {
        size: input.size,
        size_class: input.size.class_name(),
        size_attr: input.size.as_str(),
        has_name: input.has_name,
        has_src: input.has_src,
        has_alt: input.has_alt,
        has_custom_class_name: input.has_custom_class_name,
        label_source,
        label_source_attr: label_source.as_str(),
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: AvatarState) -> String {
    let mut classes = vec!["ui-avatar".to_string(), state.size_class.to_string()];

    if state.has_name {
        classes.push("ui-avatar--has-name".to_string());
    }
    if state.has_src {
        classes.push("ui-avatar--has-src".to_string());
    }
    if state.has_alt {
        classes.push("ui-avatar--has-alt".to_string());
    }

    classes.push(format!("ui-avatar--label-{}", state.label_source_attr));

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
    fn size_class_names_are_stable() {
        assert_eq!(AvatarSize::Sm.class_name(), "ui-avatar--sm");
        assert_eq!(AvatarSize::Md.class_name(), "ui-avatar--md");
        assert_eq!(AvatarSize::Lg.class_name(), "ui-avatar--lg");
    }

    #[test]
    fn label_source_attrs_are_stable() {
        assert_eq!(AvatarLabelSource::Alt.as_str(), "alt");
        assert_eq!(AvatarLabelSource::Name.as_str(), "name");
        assert_eq!(AvatarLabelSource::Fallback.as_str(), "fallback");
    }

    #[test]
    fn normalize_optional_text_filters_blank_values() {
        assert_eq!(normalize_optional_text(None), None);
        assert_eq!(normalize_optional_text(Some("   ".to_string())), None);
        assert_eq!(
            normalize_optional_text(Some("  Ada Lovelace  ".to_string())),
            Some("Ada Lovelace".to_string())
        );
    }

    #[test]
    fn initials_from_name_uses_first_and_last_words() {
        assert_eq!(initials_from_name("Ada Lovelace"), Some("AL".to_string()));
        assert_eq!(initials_from_name("grace"), Some("G".to_string()));
        assert_eq!(initials_from_name("   "), None);
    }

    #[test]
    fn resolve_accessibility_prefers_alt_then_name_then_fallback() {
        let alt = resolve_accessibility(Some("Ada Lovelace"), Some("Profile photo"));
        assert_eq!(alt.aria_label, "Profile photo");
        assert_eq!(alt.img_alt, "Profile photo");
        assert_eq!(alt.label_source, AvatarLabelSource::Alt);

        let name = resolve_accessibility(Some("Ada Lovelace"), None);
        assert_eq!(name.aria_label, "Ada Lovelace");
        assert_eq!(name.img_alt, "Ada Lovelace");
        assert_eq!(name.label_source, AvatarLabelSource::Name);

        let fallback = resolve_accessibility(None, None);
        assert_eq!(fallback.aria_label, "Avatar");
        assert_eq!(fallback.img_alt, "");
        assert_eq!(fallback.label_source, AvatarLabelSource::Fallback);
    }

    #[test]
    fn resolve_state_tracks_size_source_and_flags() {
        let state = resolve_state(AvatarStateInput {
            size: AvatarSize::Lg,
            has_name: true,
            has_src: true,
            has_alt: false,
            has_custom_class_name: true,
        });

        assert_eq!(state.size, AvatarSize::Lg);
        assert_eq!(state.size_class, "ui-avatar--lg");
        assert_eq!(state.size_attr, "lg");
        assert!(state.has_name);
        assert!(state.has_src);
        assert!(!state.has_alt);
        assert!(state.has_custom_class_name);
        assert_eq!(state.label_source, AvatarLabelSource::Name);
        assert_eq!(state.label_source_attr, "name");
    }

    #[test]
    fn compose_class_name_includes_state_markers() {
        let class_name = compose_class_name(
            Some("custom".to_string()),
            resolve_state(AvatarStateInput {
                size: AvatarSize::Md,
                has_name: true,
                has_src: true,
                has_alt: true,
                has_custom_class_name: true,
            }),
        );

        for token in [
            "ui-avatar",
            "ui-avatar--md",
            "ui-avatar--has-name",
            "ui-avatar--has-src",
            "ui-avatar--has-alt",
            "ui-avatar--label-alt",
            "custom",
        ] {
            assert!(
                class_name.contains(token),
                "composed class name should include `{token}`"
            );
        }
    }
}
