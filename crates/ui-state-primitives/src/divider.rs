#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum DividerOrientation {
    #[default]
    Horizontal,
    Vertical,
}

impl DividerOrientation {
    pub fn class_name(self) -> &'static str {
        match self {
            DividerOrientation::Horizontal => "ui-divider--horizontal",
            DividerOrientation::Vertical => "ui-divider--vertical",
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            DividerOrientation::Horizontal => "horizontal",
            DividerOrientation::Vertical => "vertical",
        }
    }

    pub fn aria_orientation(self) -> Option<&'static str> {
        match self {
            DividerOrientation::Horizontal => None,
            DividerOrientation::Vertical => Some("vertical"),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DividerStateInput {
    pub orientation: DividerOrientation,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DividerState {
    pub orientation: DividerOrientation,
    pub orientation_class: &'static str,
    pub orientation_attr: &'static str,
    pub aria_orientation: Option<&'static str>,
    pub is_horizontal: bool,
    pub is_vertical: bool,
    pub has_custom_class_name: bool,
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.into())
    })
}

pub fn resolve_state(input: DividerStateInput) -> DividerState {
    DividerState {
        orientation: input.orientation,
        orientation_class: input.orientation.class_name(),
        orientation_attr: input.orientation.as_str(),
        aria_orientation: input.orientation.aria_orientation(),
        is_horizontal: matches!(input.orientation, DividerOrientation::Horizontal),
        is_vertical: matches!(input.orientation, DividerOrientation::Vertical),
        has_custom_class_name: input.has_custom_class_name,
    }
}

#[cfg(test)]
#[path = "test/divider.rs"]
mod tests;
