use crate::empty_state::{EmptyStateState, EmptyStateStateInput};

pub const DEFAULT_TITLE: &str = "Nothing to show";
pub const DEFAULT_DESCRIPTION: &str = "Try adjusting filters or refreshing data.";
pub const DEFAULT_ARIA_LABEL: &str = "Empty state";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum EmptyStateTone {
    #[default]
    Default,
    Muted,
    Accent,
}

impl EmptyStateTone {
    pub fn class_name(self) -> &'static str {
        match self {
            EmptyStateTone::Default => "ui-empty-state--tone-default",
            EmptyStateTone::Muted => "ui-empty-state--tone-muted",
            EmptyStateTone::Accent => "ui-empty-state--tone-accent",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            EmptyStateTone::Default => "default",
            EmptyStateTone::Muted => "muted",
            EmptyStateTone::Accent => "accent",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum EmptyStateAlign {
    #[default]
    Start,
    Center,
}

impl EmptyStateAlign {
    pub fn class_name(self) -> &'static str {
        match self {
            EmptyStateAlign::Start => "ui-empty-state--align-start",
            EmptyStateAlign::Center => "ui-empty-state--align-center",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            EmptyStateAlign::Start => "start",
            EmptyStateAlign::Center => "center",
        }
    }
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.to_string())
    })
}

pub fn normalize_title(value: Option<String>, default: &str) -> (String, bool) {
    normalize_optional_text(value)
        .map(|title| (title, true))
        .unwrap_or_else(|| (default.to_string(), false))
}

pub fn normalize_description(value: Option<String>, default: &str) -> (String, bool) {
    normalize_optional_text(value)
        .map(|description| (description, true))
        .unwrap_or_else(|| (default.to_string(), false))
}

pub fn normalize_aria_label(value: Option<String>, default: &str) -> (String, bool) {
    normalize_optional_text(value)
        .map(|label| (label, true))
        .unwrap_or_else(|| (default.to_string(), false))
}

pub fn resolve_state(input: EmptyStateStateInput) -> EmptyStateState {
    let data_state_attr = if input.has_icon && input.has_actions {
        "rich"
    } else if input.has_actions {
        "actions"
    } else if input.has_icon {
        "icon"
    } else {
        "plain"
    };

    let title_source_attr = if input.has_custom_title {
        "custom"
    } else {
        "default"
    };

    let description_source_attr = if input.has_custom_description {
        "custom"
    } else {
        "default"
    };

    let aria_source_attr = if input.has_custom_aria_label {
        "custom"
    } else {
        "default"
    };

    let class_source_attr = if input.has_custom_class_name {
        "custom"
    } else {
        "default"
    };

    EmptyStateState {
        tone: input.tone,
        tone_class: input.tone.class_name(),
        tone_attr: input.tone.as_attr(),
        align: input.align,
        align_class: input.align.class_name(),
        align_attr: input.align.as_attr(),
        is_compact: input.compact,
        is_bordered: input.bordered,
        has_icon: input.has_icon,
        has_actions: input.has_actions,
        data_state_attr,
        title_source_attr,
        description_source_attr,
        aria_source_attr,
        class_source_attr,
        has_custom_class_name: input.has_custom_class_name,
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: EmptyStateState) -> String {
    let mut classes = vec![
        "ui-empty-state".to_string(),
        state.tone_class.to_string(),
        state.align_class.to_string(),
    ];

    if state.is_compact {
        classes.push("ui-empty-state--compact".to_string());
    }

    if state.is_bordered {
        classes.push("ui-empty-state--bordered".to_string());
    }

    if state.has_icon {
        classes.push("ui-empty-state--with-icon".to_string());
    }

    if state.has_actions {
        classes.push("ui-empty-state--with-actions".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-empty-state--custom-class".to_string());
        if let Some(base_class_name) = base_class_name {
            classes.push(base_class_name);
        }
    }

    classes.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::empty_state::EmptyStateStateInput;

    #[test]
    fn tone_and_align_contracts_are_stable() {
        assert_eq!(
            EmptyStateTone::Default.class_name(),
            "ui-empty-state--tone-default"
        );
        assert_eq!(
            EmptyStateTone::Muted.class_name(),
            "ui-empty-state--tone-muted"
        );
        assert_eq!(
            EmptyStateTone::Accent.class_name(),
            "ui-empty-state--tone-accent"
        );

        assert_eq!(EmptyStateTone::Default.as_attr(), "default");
        assert_eq!(EmptyStateTone::Muted.as_attr(), "muted");
        assert_eq!(EmptyStateTone::Accent.as_attr(), "accent");

        assert_eq!(
            EmptyStateAlign::Start.class_name(),
            "ui-empty-state--align-start"
        );
        assert_eq!(
            EmptyStateAlign::Center.class_name(),
            "ui-empty-state--align-center"
        );

        assert_eq!(EmptyStateAlign::Start.as_attr(), "start");
        assert_eq!(EmptyStateAlign::Center.as_attr(), "center");
    }

    #[test]
    fn normalize_helpers_trim_and_fallback() {
        assert_eq!(normalize_optional_text(None), None);
        assert_eq!(normalize_optional_text(Some(" \n\t ".to_string())), None);
        assert_eq!(
            normalize_optional_text(Some("  add filters  ".to_string())),
            Some("add filters".to_string())
        );

        let (title, custom_title) = normalize_title(Some("  No matches  ".to_string()), "Default");
        assert_eq!(title, "No matches");
        assert!(custom_title);

        let (title, custom_title) = normalize_title(None, "Default title");
        assert_eq!(title, "Default title");
        assert!(!custom_title);

        let (description, custom_description) = normalize_description(
            Some("  Try another keyword  ".to_string()),
            "Default description",
        );
        assert_eq!(description, "Try another keyword");
        assert!(custom_description);

        let (description, custom_description) = normalize_description(None, "Default description");
        assert_eq!(description, "Default description");
        assert!(!custom_description);

        let (label, custom_label) =
            normalize_aria_label(Some("  Project state  ".to_string()), "Default label");
        assert_eq!(label, "Project state");
        assert!(custom_label);

        let (label, custom_label) = normalize_aria_label(None, "Default label");
        assert_eq!(label, "Default label");
        assert!(!custom_label);
    }

    #[test]
    fn resolve_state_tracks_markers_and_sources() {
        let state = resolve_state(EmptyStateStateInput {
            tone: EmptyStateTone::Accent,
            align: EmptyStateAlign::Center,
            compact: true,
            bordered: true,
            has_icon: true,
            has_actions: false,
            has_custom_title: true,
            has_custom_description: false,
            has_custom_aria_label: true,
            has_custom_class_name: true,
        });

        assert_eq!(state.tone_attr, "accent");
        assert_eq!(state.align_attr, "center");
        assert_eq!(state.data_state_attr, "icon");
        assert!(state.is_compact);
        assert!(state.is_bordered);
        assert_eq!(state.title_source_attr, "custom");
        assert_eq!(state.description_source_attr, "default");
        assert_eq!(state.aria_source_attr, "custom");
        assert_eq!(state.class_source_attr, "custom");
    }

    #[test]
    fn compose_class_name_includes_state_markers() {
        let class_name = compose_class_name(
            Some("docs-empty-state".to_string()),
            resolve_state(EmptyStateStateInput {
                tone: EmptyStateTone::Muted,
                align: EmptyStateAlign::Center,
                compact: true,
                bordered: true,
                has_icon: true,
                has_actions: true,
                has_custom_title: false,
                has_custom_description: false,
                has_custom_aria_label: false,
                has_custom_class_name: true,
            }),
        );

        for token in [
            "ui-empty-state",
            "ui-empty-state--tone-muted",
            "ui-empty-state--align-center",
            "ui-empty-state--compact",
            "ui-empty-state--bordered",
            "ui-empty-state--with-icon",
            "ui-empty-state--with-actions",
            "ui-empty-state--custom-class",
            "docs-empty-state",
        ] {
            assert!(
                class_name.contains(token),
                "composed class should include `{token}`"
            );
        }
    }
}
