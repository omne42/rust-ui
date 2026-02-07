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

    pub fn as_str(self) -> &'static str {
        match self {
            SeparatorOrientation::Horizontal => "horizontal",
            SeparatorOrientation::Vertical => "vertical",
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
    pub fn class_name(self) -> &'static str {
        match self {
            SeparatorElementType::Div => "ui-separator--element-div",
            SeparatorElementType::Hr => "ui-separator--element-hr",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            SeparatorElementType::Div => "div",
            SeparatorElementType::Hr => "hr",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SeparatorStateInput {
    pub orientation: SeparatorOrientation,
    pub element_type: SeparatorElementType,
    pub decorative: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SeparatorState {
    pub orientation: SeparatorOrientation,
    pub orientation_class: &'static str,
    pub orientation_attr: &'static str,
    pub aria_orientation: Option<&'static str>,
    pub element_type: SeparatorElementType,
    pub element_class: &'static str,
    pub element_attr: &'static str,
    pub is_decorative: bool,
    pub is_semantic: bool,
    pub state_attr: &'static str,
    pub has_custom_class_name: bool,
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.to_string())
    })
}

pub fn resolve_state(input: SeparatorStateInput) -> SeparatorState {
    let is_semantic = !input.decorative;

    SeparatorState {
        orientation: input.orientation,
        orientation_class: input.orientation.class_name(),
        orientation_attr: input.orientation.as_str(),
        aria_orientation: input.orientation.aria_orientation(),
        element_type: input.element_type,
        element_class: input.element_type.class_name(),
        element_attr: input.element_type.as_attr(),
        is_decorative: input.decorative,
        is_semantic,
        state_attr: if input.decorative {
            "decorative"
        } else {
            "semantic"
        },
        has_custom_class_name: input.has_custom_class_name,
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: SeparatorState) -> String {
    let mut classes = vec![
        "ui-separator".to_string(),
        state.orientation_class.to_string(),
        state.element_class.to_string(),
    ];

    if state.is_semantic {
        classes.push("ui-separator--semantic".to_string());
    }
    if state.is_decorative {
        classes.push("ui-separator--decorative".to_string());
    }

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
    fn vertical_sets_aria_orientation() {
        assert_eq!(
            SeparatorOrientation::Vertical.aria_orientation(),
            Some("vertical")
        );
        assert_eq!(SeparatorOrientation::Horizontal.aria_orientation(), None);

        assert_eq!(SeparatorOrientation::Vertical.as_str(), "vertical");
        assert_eq!(SeparatorOrientation::Horizontal.as_str(), "horizontal");
    }

    #[test]
    fn element_type_mapping_is_stable() {
        assert_eq!(SeparatorElementType::Div.as_attr(), "div");
        assert_eq!(SeparatorElementType::Hr.as_attr(), "hr");

        assert_eq!(
            SeparatorElementType::Div.class_name(),
            "ui-separator--element-div"
        );
        assert_eq!(
            SeparatorElementType::Hr.class_name(),
            "ui-separator--element-hr"
        );
    }

    #[test]
    fn normalize_optional_text_filters_blank_values() {
        assert_eq!(normalize_optional_text(None), None);
        assert_eq!(normalize_optional_text(Some("  \n\t".to_string())), None);
        assert_eq!(
            normalize_optional_text(Some("  docs-separator  ".to_string())),
            Some("docs-separator".to_string())
        );
    }

    #[test]
    fn resolve_state_preserves_fields_and_flags() {
        let state = resolve_state(SeparatorStateInput {
            orientation: SeparatorOrientation::Vertical,
            element_type: SeparatorElementType::Hr,
            decorative: true,
            has_custom_class_name: true,
        });

        assert_eq!(state.orientation, SeparatorOrientation::Vertical);
        assert_eq!(state.orientation_class, "ui-separator--vertical");
        assert_eq!(state.orientation_attr, "vertical");
        assert_eq!(state.aria_orientation, Some("vertical"));

        assert_eq!(state.element_type, SeparatorElementType::Hr);
        assert_eq!(state.element_class, "ui-separator--element-hr");
        assert_eq!(state.element_attr, "hr");

        assert!(state.is_decorative);
        assert!(!state.is_semantic);
        assert_eq!(state.state_attr, "decorative");
        assert!(state.has_custom_class_name);
    }

    #[test]
    fn compose_class_name_includes_state_markers() {
        let class_name = compose_class_name(
            Some("custom".to_string()),
            resolve_state(SeparatorStateInput {
                orientation: SeparatorOrientation::Horizontal,
                element_type: SeparatorElementType::Div,
                decorative: false,
                has_custom_class_name: true,
            }),
        );

        for token in [
            "ui-separator",
            "ui-separator--horizontal",
            "ui-separator--element-div",
            "ui-separator--semantic",
            "custom",
        ] {
            assert!(
                class_name.contains(token),
                "composed class name should include `{token}`"
            );
        }
    }
}
