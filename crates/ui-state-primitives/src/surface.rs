pub const DEFAULT_ARIA_LABEL: &str = "Surface";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum SurfaceTone {
    #[default]
    Default,
    Subtle,
    Strong,
}

impl SurfaceTone {
    pub fn class_name(self) -> &'static str {
        match self {
            SurfaceTone::Default => "ui-surface--tone-default",
            SurfaceTone::Subtle => "ui-surface--tone-subtle",
            SurfaceTone::Strong => "ui-surface--tone-strong",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            SurfaceTone::Default => "default",
            SurfaceTone::Subtle => "subtle",
            SurfaceTone::Strong => "strong",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum SurfaceElevation {
    Flat,
    #[default]
    Raised,
    Floating,
}

impl SurfaceElevation {
    pub fn class_name(self) -> &'static str {
        match self {
            SurfaceElevation::Flat => "ui-surface--elevation-flat",
            SurfaceElevation::Raised => "ui-surface--elevation-raised",
            SurfaceElevation::Floating => "ui-surface--elevation-floating",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            SurfaceElevation::Flat => "flat",
            SurfaceElevation::Raised => "raised",
            SurfaceElevation::Floating => "floating",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SurfaceStateInput {
    pub tone: SurfaceTone,
    pub elevation: SurfaceElevation,
    pub bordered: bool,
    pub padded: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SurfaceState {
    pub tone: SurfaceTone,
    pub tone_class: &'static str,
    pub tone_attr: &'static str,
    pub elevation: SurfaceElevation,
    pub elevation_class: &'static str,
    pub elevation_attr: &'static str,
    pub is_bordered: bool,
    pub is_padded: bool,
    pub is_plain: bool,
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

pub fn resolve_state(input: SurfaceStateInput) -> SurfaceState {
    let data_state_attr = if input.bordered && input.padded {
        "framed"
    } else if input.bordered {
        "bordered"
    } else if input.padded {
        "padded"
    } else {
        "plain"
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

    SurfaceState {
        tone: input.tone,
        tone_class: input.tone.class_name(),
        tone_attr: input.tone.as_attr(),
        elevation: input.elevation,
        elevation_class: input.elevation.class_name(),
        elevation_attr: input.elevation.as_attr(),
        is_bordered: input.bordered,
        is_padded: input.padded,
        is_plain: !input.bordered && !input.padded,
        data_state_attr,
        aria_source_attr,
        class_source_attr,
        has_custom_class_name: input.has_custom_class_name,
    }
}

#[cfg(test)]
#[path = "test/surface.rs"]
mod tests;
