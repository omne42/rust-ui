use crate::aspect_ratio::{AspectRatioState, AspectRatioStateInput};

pub const DEFAULT_ARIA_LABEL: &str = "Aspect ratio frame";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum AspectRatioPreset {
    Square,
    Standard,
    #[default]
    Video,
    Portrait,
    UltraWide,
}

impl AspectRatioPreset {
    pub fn class_name(self) -> &'static str {
        match self {
            AspectRatioPreset::Square => "ui-aspect-ratio--ratio-square",
            AspectRatioPreset::Standard => "ui-aspect-ratio--ratio-standard",
            AspectRatioPreset::Video => "ui-aspect-ratio--ratio-video",
            AspectRatioPreset::Portrait => "ui-aspect-ratio--ratio-portrait",
            AspectRatioPreset::UltraWide => "ui-aspect-ratio--ratio-ultra-wide",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            AspectRatioPreset::Square => "square",
            AspectRatioPreset::Standard => "standard",
            AspectRatioPreset::Video => "video",
            AspectRatioPreset::Portrait => "portrait",
            AspectRatioPreset::UltraWide => "ultra-wide",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum AspectRatioRadius {
    #[default]
    None,
    Sm,
    Md,
    Lg,
    Full,
}

impl AspectRatioRadius {
    pub fn class_name(self) -> &'static str {
        match self {
            AspectRatioRadius::None => "ui-aspect-ratio--radius-none",
            AspectRatioRadius::Sm => "ui-aspect-ratio--radius-sm",
            AspectRatioRadius::Md => "ui-aspect-ratio--radius-md",
            AspectRatioRadius::Lg => "ui-aspect-ratio--radius-lg",
            AspectRatioRadius::Full => "ui-aspect-ratio--radius-full",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            AspectRatioRadius::None => "none",
            AspectRatioRadius::Sm => "sm",
            AspectRatioRadius::Md => "md",
            AspectRatioRadius::Lg => "lg",
            AspectRatioRadius::Full => "full",
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

pub fn resolve_state(input: AspectRatioStateInput) -> AspectRatioState {
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

    let data_state_attr = if input.bordered && input.fill {
        "media"
    } else if input.bordered {
        "framed"
    } else if input.fill {
        "fill"
    } else {
        "plain"
    };

    AspectRatioState {
        ratio: input.ratio,
        ratio_class: input.ratio.class_name(),
        ratio_attr: input.ratio.as_attr(),
        radius: input.radius,
        radius_class: input.radius.class_name(),
        radius_attr: input.radius.as_attr(),
        is_bordered: input.bordered,
        bordered_class: "ui-aspect-ratio--bordered",
        is_fill: input.fill,
        fill_class: "ui-aspect-ratio--fill",
        data_state_attr,
        aria_source_attr,
        class_source_attr,
        has_custom_class_name: input.has_custom_class_name,
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: AspectRatioState) -> String {
    let mut classes = vec![
        "ui-aspect-ratio".to_string(),
        state.ratio_class.to_string(),
        state.radius_class.to_string(),
    ];

    if state.is_bordered {
        classes.push(state.bordered_class.to_string());
    }

    if state.is_fill {
        classes.push(state.fill_class.to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-aspect-ratio--custom-class".to_string());
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
    fn class_and_attr_contracts_are_stable() {
        assert_eq!(
            AspectRatioPreset::Square.class_name(),
            "ui-aspect-ratio--ratio-square"
        );
        assert_eq!(AspectRatioPreset::UltraWide.as_attr(), "ultra-wide");
        assert_eq!(
            AspectRatioRadius::Md.class_name(),
            "ui-aspect-ratio--radius-md"
        );
        assert_eq!(AspectRatioRadius::Full.as_attr(), "full");
    }

    #[test]
    fn normalize_helpers_trim_and_fallback() {
        assert_eq!(normalize_optional_text(None), None);
        assert_eq!(normalize_optional_text(Some("  \n\t".to_string())), None);
        assert_eq!(
            normalize_optional_text(Some("  docs-aspect-ratio  ".to_string())),
            Some("docs-aspect-ratio".to_string())
        );

        let (custom_label, is_custom) = normalize_aria_label(Some("  Featured  ".to_string()));
        assert_eq!(custom_label, "Featured");
        assert!(is_custom);

        let (fallback_label, is_custom) = normalize_aria_label(Some(" ".to_string()));
        assert_eq!(fallback_label, DEFAULT_ARIA_LABEL);
        assert!(!is_custom);
    }

    #[test]
    fn resolve_state_tracks_sources_and_priority_state() {
        let state = resolve_state(AspectRatioStateInput {
            ratio: AspectRatioPreset::Portrait,
            radius: AspectRatioRadius::Lg,
            bordered: true,
            fill: true,
            has_custom_aria_label: true,
            has_custom_class_name: false,
        });

        assert_eq!(state.ratio_attr, "portrait");
        assert_eq!(state.radius_attr, "lg");
        assert!(state.is_bordered);
        assert!(state.is_fill);
        assert_eq!(state.data_state_attr, "media");
        assert_eq!(state.aria_source_attr, "custom");
        assert_eq!(state.class_source_attr, "default");
    }

    #[test]
    fn compose_class_name_merges_custom_class_and_flags() {
        let state = resolve_state(AspectRatioStateInput {
            ratio: AspectRatioPreset::Video,
            radius: AspectRatioRadius::Sm,
            bordered: true,
            fill: false,
            has_custom_aria_label: false,
            has_custom_class_name: true,
        });

        let class = compose_class_name(Some("docs-aspect".to_string()), state);

        for class_name in [
            "ui-aspect-ratio",
            "ui-aspect-ratio--ratio-video",
            "ui-aspect-ratio--radius-sm",
            "ui-aspect-ratio--bordered",
            "ui-aspect-ratio--custom-class",
            "docs-aspect",
        ] {
            assert!(
                class.contains(class_name),
                "class list should include `{class_name}`; got: {class}"
            );
        }
    }
}
