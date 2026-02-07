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
        (!trimmed.is_empty()).then(|| trimmed.to_string())
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

pub fn compose_class_name(base_class_name: Option<String>, state: DividerState) -> String {
    let mut classes = vec![
        "ui-divider".to_string(),
        state.orientation_class.to_string(),
    ];

    if state.has_custom_class_name
        && let Some(base_class_name) = base_class_name
    {
        classes.push(base_class_name);
    }

    classes.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn orientation_mappings_are_stable() {
        assert_eq!(
            DividerOrientation::Horizontal.class_name(),
            "ui-divider--horizontal"
        );
        assert_eq!(
            DividerOrientation::Vertical.class_name(),
            "ui-divider--vertical"
        );

        assert_eq!(DividerOrientation::Horizontal.as_str(), "horizontal");
        assert_eq!(DividerOrientation::Vertical.as_str(), "vertical");

        assert_eq!(DividerOrientation::Horizontal.aria_orientation(), None);
        assert_eq!(
            DividerOrientation::Vertical.aria_orientation(),
            Some("vertical")
        );
    }

    #[test]
    fn normalize_optional_text_filters_blank_values() {
        assert_eq!(normalize_optional_text(None), None);
        assert_eq!(normalize_optional_text(Some("  \n\t".to_string())), None);
        assert_eq!(
            normalize_optional_text(Some("  docs-divider  ".to_string())),
            Some("docs-divider".to_string())
        );
    }

    #[test]
    fn resolve_state_tracks_orientation_flags() {
        let state = resolve_state(DividerStateInput {
            orientation: DividerOrientation::Vertical,
            has_custom_class_name: true,
        });

        assert_eq!(state.orientation, DividerOrientation::Vertical);
        assert_eq!(state.orientation_class, "ui-divider--vertical");
        assert_eq!(state.orientation_attr, "vertical");
        assert_eq!(state.aria_orientation, Some("vertical"));
        assert!(!state.is_horizontal);
        assert!(state.is_vertical);
        assert!(state.has_custom_class_name);
    }

    #[test]
    fn compose_class_name_includes_state_markers() {
        let class_name = compose_class_name(
            Some("custom".to_string()),
            resolve_state(DividerStateInput {
                orientation: DividerOrientation::Horizontal,
                has_custom_class_name: true,
            }),
        );

        for token in ["ui-divider", "ui-divider--horizontal", "custom"] {
            assert!(
                class_name.contains(token),
                "composed class name should include `{token}`"
            );
        }
    }
}
