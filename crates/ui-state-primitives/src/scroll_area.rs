pub const DEFAULT_ARIA_LABEL: &str = "Scrollable region";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ScrollAreaOrientation {
    #[default]
    Vertical,
    Horizontal,
    Both,
}

impl ScrollAreaOrientation {
    pub const fn as_attr(self) -> &'static str {
        match self {
            ScrollAreaOrientation::Vertical => "vertical",
            ScrollAreaOrientation::Horizontal => "horizontal",
            ScrollAreaOrientation::Both => "both",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ScrollAreaMaxHeightAttr {
    Default,
    Custom,
}

impl ScrollAreaMaxHeightAttr {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Custom => "custom",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ScrollAreaSourceAttr {
    Default,
    Custom,
}

impl ScrollAreaSourceAttr {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Custom => "custom",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ScrollAreaStateInput {
    pub orientation: ScrollAreaOrientation,
    pub disabled: bool,
    pub max_height_px: Option<u32>,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ScrollAreaState {
    pub orientation: ScrollAreaOrientation,
    pub orientation_attr: &'static str,
    pub disabled: bool,
    pub max_height_px: Option<u32>,
    pub has_custom_max_height: bool,
    pub max_height_attr: ScrollAreaMaxHeightAttr,
    pub has_custom_aria_label: bool,
    pub aria_source_attr: ScrollAreaSourceAttr,
    pub has_custom_class_name: bool,
    pub class_source_attr: ScrollAreaSourceAttr,
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.into())
    })
}

pub fn normalize_aria_label(value: Option<String>) -> (String, bool) {
    if let Some(value) = normalize_optional_text(value) {
        (value, true)
    } else {
        (DEFAULT_ARIA_LABEL.into(), false)
    }
}

pub fn normalize_max_height(max_height_px: Option<u32>) -> Option<u32> {
    max_height_px.filter(|px| *px > 0)
}

pub fn resolve_state(input: ScrollAreaStateInput) -> ScrollAreaState {
    let max_height_px = normalize_max_height(input.max_height_px);

    ScrollAreaState {
        orientation: input.orientation,
        orientation_attr: input.orientation.as_attr(),
        disabled: input.disabled,
        max_height_px,
        has_custom_max_height: max_height_px.is_some(),
        max_height_attr: if max_height_px.is_some() {
            ScrollAreaMaxHeightAttr::Custom
        } else {
            ScrollAreaMaxHeightAttr::Default
        },
        has_custom_aria_label: input.has_custom_aria_label,
        aria_source_attr: if input.has_custom_aria_label {
            ScrollAreaSourceAttr::Custom
        } else {
            ScrollAreaSourceAttr::Default
        },
        has_custom_class_name: input.has_custom_class_name,
        class_source_attr: if input.has_custom_class_name {
            ScrollAreaSourceAttr::Custom
        } else {
            ScrollAreaSourceAttr::Default
        },
    }
}

#[cfg(test)]
#[path = "test/scroll_area.rs"]
mod tests;
