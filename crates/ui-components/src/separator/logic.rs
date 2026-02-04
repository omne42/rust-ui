#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum SeparatorOrientation {
    #[default]
    Horizontal,
    Vertical,
}

impl SeparatorOrientation {
    pub fn class_name(self) -> &'static str {
        match self {
            SeparatorOrientation::Horizontal => "ui-separator--horizontal",
            SeparatorOrientation::Vertical => "ui-separator--vertical",
        }
    }

    pub fn aria_orientation(self) -> Option<&'static str> {
        match self {
            SeparatorOrientation::Horizontal => None,
            SeparatorOrientation::Vertical => Some("vertical"),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum SeparatorElementType {
    #[default]
    Div,
    Hr,
}

impl SeparatorElementType {
    pub fn as_attr(self) -> &'static str {
        match self {
            SeparatorElementType::Div => "div",
            SeparatorElementType::Hr => "hr",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SeparatorState {
    pub orientation: SeparatorOrientation,
    pub element_type: SeparatorElementType,
    pub is_decorative: bool,
}

pub fn resolve_state(
    orientation: SeparatorOrientation,
    element_type: SeparatorElementType,
    decorative: bool,
) -> SeparatorState {
    SeparatorState {
        orientation,
        element_type,
        is_decorative: decorative,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vertical_sets_aria_orientation() {
        assert_eq!(
            SeparatorOrientation::Vertical.aria_orientation(),
            Some("vertical")
        );
        assert_eq!(SeparatorOrientation::Horizontal.aria_orientation(), None);
    }

    #[test]
    fn element_type_attr_mapping_is_stable() {
        assert_eq!(SeparatorElementType::Div.as_attr(), "div");
        assert_eq!(SeparatorElementType::Hr.as_attr(), "hr");
    }

    #[test]
    fn resolve_state_preserves_fields() {
        let state = resolve_state(
            SeparatorOrientation::Vertical,
            SeparatorElementType::Hr,
            true,
        );
        assert_eq!(state.orientation, SeparatorOrientation::Vertical);
        assert_eq!(state.element_type, SeparatorElementType::Hr);
        assert!(state.is_decorative);
    }
}
