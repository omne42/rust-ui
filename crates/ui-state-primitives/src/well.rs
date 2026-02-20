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
        (!trimmed.is_empty()).then(|| trimmed.into())
    })
}

pub fn normalize_aria_label(value: Option<String>) -> (String, bool) {
    if let Some(label) = normalize_optional_text(value) {
        return (label, true);
    }

    (DEFAULT_ARIA_LABEL.into(), false)
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
#[path = "test/well.rs"]
mod tests;
