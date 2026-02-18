pub const DEFAULT_ARIA_LABEL: &str = "Content well";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum WellTone {
    #[default]
    Default,
    Quiet,
    Strong,
}

impl WellTone {
    pub fn class_name(self) -> &'static str {
        match self {
            WellTone::Default => "ui-well--tone-default",
            WellTone::Quiet => "ui-well--tone-quiet",
            WellTone::Strong => "ui-well--tone-strong",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            WellTone::Default => "default",
            WellTone::Quiet => "quiet",
            WellTone::Strong => "strong",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum WellDensity {
    #[default]
    Comfortable,
    Compact,
}

impl WellDensity {
    pub fn class_name(self) -> &'static str {
        match self {
            WellDensity::Comfortable => "ui-well--density-comfortable",
            WellDensity::Compact => "ui-well--density-compact",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            WellDensity::Comfortable => "comfortable",
            WellDensity::Compact => "compact",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WellStateInput {
    pub tone: WellTone,
    pub density: WellDensity,
    pub inset: bool,
    pub has_custom_label: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WellState {
    pub tone: WellTone,
    pub tone_class: &'static str,
    pub tone_attr: &'static str,
    pub density: WellDensity,
    pub density_class: &'static str,
    pub density_attr: &'static str,
    pub is_inset: bool,
    pub is_not_inset: bool,
    pub has_custom_label: bool,
    pub has_custom_class_name: bool,
    pub label_source_attr: &'static str,
    pub class_source_attr: &'static str,
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

pub fn resolve_state(input: WellStateInput) -> WellState {
    let label_source_attr = if input.has_custom_label {
        "custom"
    } else {
        "default"
    };
    let class_source_attr = if input.has_custom_class_name {
        "custom"
    } else {
        "default"
    };

    WellState {
        tone: input.tone,
        tone_class: input.tone.class_name(),
        tone_attr: input.tone.as_attr(),
        density: input.density,
        density_class: input.density.class_name(),
        density_attr: input.density.as_attr(),
        is_inset: input.inset,
        is_not_inset: !input.inset,
        has_custom_label: input.has_custom_label,
        has_custom_class_name: input.has_custom_class_name,
        label_source_attr,
        class_source_attr,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tone_class_names_and_attrs_are_stable() {
        assert_eq!(WellTone::Default.class_name(), "ui-well--tone-default");
        assert_eq!(WellTone::Quiet.class_name(), "ui-well--tone-quiet");
        assert_eq!(WellTone::Strong.class_name(), "ui-well--tone-strong");

        assert_eq!(WellTone::Default.as_attr(), "default");
        assert_eq!(WellTone::Quiet.as_attr(), "quiet");
        assert_eq!(WellTone::Strong.as_attr(), "strong");
    }

    #[test]
    fn density_class_names_and_attrs_are_stable() {
        assert_eq!(
            WellDensity::Comfortable.class_name(),
            "ui-well--density-comfortable"
        );
        assert_eq!(
            WellDensity::Compact.class_name(),
            "ui-well--density-compact"
        );

        assert_eq!(WellDensity::Comfortable.as_attr(), "comfortable");
        assert_eq!(WellDensity::Compact.as_attr(), "compact");
    }

    #[test]
    fn normalize_optional_text_filters_blank_values() {
        assert_eq!(normalize_optional_text(None), None);
        assert_eq!(normalize_optional_text(Some("\n\t".to_string())), None);
        assert_eq!(
            normalize_optional_text(Some(" docs-well ".to_string())),
            Some("docs-well".to_string())
        );
    }

    #[test]
    fn normalize_aria_label_uses_trimmed_text_or_fallback() {
        let (label, custom) = normalize_aria_label(Some("  Selection summary  ".to_string()));
        assert_eq!(label, "Selection summary");
        assert!(custom);

        let (label, custom) = normalize_aria_label(Some("  ".to_string()));
        assert_eq!(label, DEFAULT_ARIA_LABEL);
        assert!(!custom);
    }

    #[test]
    fn resolve_state_tracks_density_inset_and_sources() {
        let state = resolve_state(WellStateInput {
            tone: WellTone::Strong,
            density: WellDensity::Compact,
            inset: true,
            has_custom_label: true,
            has_custom_class_name: false,
        });

        assert_eq!(state.tone_attr, "strong");
        assert_eq!(state.density_attr, "compact");
        assert!(state.is_inset);
        assert!(!state.is_not_inset);
        assert_eq!(state.label_source_attr, "custom");
        assert_eq!(state.class_source_attr, "default");
    }
}
