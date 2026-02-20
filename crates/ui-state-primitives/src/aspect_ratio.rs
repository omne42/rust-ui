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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AspectRatioStateInput {
    pub ratio: AspectRatioPreset,
    pub radius: AspectRatioRadius,
    pub bordered: bool,
    pub fill: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AspectRatioState {
    pub ratio: AspectRatioPreset,
    pub ratio_class: &'static str,
    pub ratio_attr: &'static str,
    pub radius: AspectRatioRadius,
    pub radius_class: &'static str,
    pub radius_attr: &'static str,
    pub is_bordered: bool,
    pub bordered_class: &'static str,
    pub is_fill: bool,
    pub fill_class: &'static str,
    pub data_state_attr: &'static str,
    pub aria_source_attr: &'static str,
    pub class_source_attr: &'static str,
    pub has_custom_class_name: bool,
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

#[cfg(test)]
#[path = "test/aspect_ratio.rs"]
mod tests;
