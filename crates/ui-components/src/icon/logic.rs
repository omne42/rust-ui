use crate::icon::{IconState, IconStateInput};

pub const DEFAULT_ARIA_LABEL: &str = "Icon";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum IconSize {
    Sm,
    #[default]
    Md,
    Lg,
}

impl IconSize {
    pub fn class_name(self) -> &'static str {
        match self {
            IconSize::Sm => "ui-icon--size-sm",
            IconSize::Md => "ui-icon--size-md",
            IconSize::Lg => "ui-icon--size-lg",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            IconSize::Sm => "sm",
            IconSize::Md => "md",
            IconSize::Lg => "lg",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum IconTone {
    #[default]
    Default,
    Muted,
    Accent,
    Danger,
}

impl IconTone {
    pub fn class_name(self) -> &'static str {
        match self {
            IconTone::Default => "ui-icon--tone-default",
            IconTone::Muted => "ui-icon--tone-muted",
            IconTone::Accent => "ui-icon--tone-accent",
            IconTone::Danger => "ui-icon--tone-danger",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            IconTone::Default => "default",
            IconTone::Muted => "muted",
            IconTone::Accent => "accent",
            IconTone::Danger => "danger",
        }
    }
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.to_string())
    })
}

pub fn normalize_aria_label(value: Option<String>) -> (String, bool) {
    if let Some(label) = normalize_optional_text(value) {
        return (label, true);
    }

    (DEFAULT_ARIA_LABEL.to_string(), false)
}

pub fn resolve_state(input: IconStateInput) -> IconState {
    let has_accessible_name = input.has_custom_aria_label && !input.decorative;

    let data_state_attr = if input.disabled {
        "disabled"
    } else if input.decorative {
        "decorative"
    } else if has_accessible_name {
        "labeled"
    } else {
        "default"
    };

    IconState {
        size: input.size,
        tone: input.tone,
        size_class: input.size.class_name(),
        tone_class: input.tone.class_name(),
        size_attr: input.size.as_attr(),
        tone_attr: input.tone.as_attr(),
        is_disabled: input.disabled,
        is_decorative: input.decorative,
        has_accessible_name,
        has_custom_aria_label: input.has_custom_aria_label,
        has_custom_class_name: input.has_custom_class_name,
        data_state_attr,
        aria_source_attr: if input.has_custom_aria_label {
            "custom"
        } else {
            "default"
        },
        class_source_attr: if input.has_custom_class_name {
            "custom"
        } else {
            "default"
        },
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: IconState) -> String {
    let mut classes = vec![
        "ui-icon".to_string(),
        state.size_class.to_string(),
        state.tone_class.to_string(),
    ];

    if state.is_disabled {
        classes.push("ui-icon--disabled".to_string());
    }

    if state.is_decorative {
        classes.push("ui-icon--decorative".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-icon--custom-class".to_string());
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
    fn size_and_tone_contracts_are_stable() {
        assert_eq!(IconSize::Sm.class_name(), "ui-icon--size-sm");
        assert_eq!(IconSize::Md.class_name(), "ui-icon--size-md");
        assert_eq!(IconSize::Lg.class_name(), "ui-icon--size-lg");
        assert_eq!(IconSize::Sm.as_attr(), "sm");
        assert_eq!(IconSize::Md.as_attr(), "md");
        assert_eq!(IconSize::Lg.as_attr(), "lg");

        assert_eq!(IconTone::Default.class_name(), "ui-icon--tone-default");
        assert_eq!(IconTone::Muted.class_name(), "ui-icon--tone-muted");
        assert_eq!(IconTone::Accent.class_name(), "ui-icon--tone-accent");
        assert_eq!(IconTone::Danger.class_name(), "ui-icon--tone-danger");
        assert_eq!(IconTone::Default.as_attr(), "default");
        assert_eq!(IconTone::Muted.as_attr(), "muted");
        assert_eq!(IconTone::Accent.as_attr(), "accent");
        assert_eq!(IconTone::Danger.as_attr(), "danger");
    }

    #[test]
    fn normalize_helpers_trim_and_fallback() {
        assert_eq!(normalize_optional_text(None), None);
        assert_eq!(normalize_optional_text(Some("  \n\t ".to_string())), None);
        assert_eq!(
            normalize_optional_text(Some("  Completion state  ".to_string())),
            Some("Completion state".to_string())
        );

        assert_eq!(
            normalize_aria_label(Some("  Save success  ".to_string())),
            ("Save success".to_string(), true)
        );
        assert_eq!(
            normalize_aria_label(None),
            (DEFAULT_ARIA_LABEL.to_string(), false)
        );
    }

    #[test]
    fn resolve_state_tracks_accessibility_and_sources() {
        let state = resolve_state(IconStateInput {
            size: IconSize::Lg,
            tone: IconTone::Accent,
            disabled: false,
            decorative: false,
            has_custom_aria_label: true,
            has_custom_class_name: true,
        });

        assert_eq!(state.data_state_attr, "labeled");
        assert!(state.has_accessible_name);
        assert_eq!(state.aria_source_attr, "custom");
        assert_eq!(state.class_source_attr, "custom");

        let decorative = resolve_state(IconStateInput {
            size: IconSize::Md,
            tone: IconTone::Default,
            disabled: false,
            decorative: true,
            has_custom_aria_label: false,
            has_custom_class_name: false,
        });

        assert_eq!(decorative.data_state_attr, "decorative");
        assert!(decorative.is_decorative);
        assert!(!decorative.has_accessible_name);
    }

    #[test]
    fn compose_class_name_includes_state_markers() {
        let state = resolve_state(IconStateInput {
            size: IconSize::Sm,
            tone: IconTone::Danger,
            disabled: true,
            decorative: true,
            has_custom_aria_label: false,
            has_custom_class_name: true,
        });

        let class_name = compose_class_name(Some("docs-icon-custom".to_string()), state);

        for token in [
            "ui-icon",
            "ui-icon--size-sm",
            "ui-icon--tone-danger",
            "ui-icon--disabled",
            "ui-icon--decorative",
            "ui-icon--custom-class",
            "docs-icon-custom",
        ] {
            assert!(
                class_name.contains(token),
                "composed class name should include `{token}`"
            );
        }
    }
}
