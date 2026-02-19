pub use ui_state_primitives::separator::{
    SeparatorElementType, SeparatorOrientation, SeparatorStateInput, compose_class_name,
    normalize_optional_text, resolve_state,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SeparatorNormalizeInput {
    pub orientation: Option<SeparatorOrientation>,
    pub is_decorative: Option<bool>,
    pub element_type: Option<SeparatorElementType>,
    pub class_name: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SeparatorNormalizedProps {
    pub state_input: SeparatorStateInput,
    pub class_name: Option<String>,
}

pub fn normalize_orientation(value: Option<SeparatorOrientation>) -> SeparatorOrientation {
    value.unwrap_or_default()
}

pub fn normalize_is_decorative(value: Option<bool>) -> bool {
    value.unwrap_or(false)
}

pub fn normalize_element_type(value: Option<SeparatorElementType>) -> SeparatorElementType {
    value.unwrap_or_default()
}

pub fn normalize_props(input: SeparatorNormalizeInput) -> SeparatorNormalizedProps {
    let class_name = normalize_optional_text(input.class_name);
    let orientation = normalize_orientation(input.orientation);
    let element_type = normalize_element_type(input.element_type);
    let is_decorative = normalize_is_decorative(input.is_decorative);

    SeparatorNormalizedProps {
        state_input: SeparatorStateInput {
            orientation,
            element_type,
            decorative: is_decorative,
            has_custom_class_name: class_name.is_some(),
        },
        class_name,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_defaults_are_explicit_and_stable() {
        assert_eq!(
            normalize_orientation(None),
            SeparatorOrientation::Horizontal
        );
        assert_eq!(
            normalize_orientation(Some(SeparatorOrientation::Vertical)),
            SeparatorOrientation::Vertical
        );

        assert!(!normalize_is_decorative(None));
        assert!(normalize_is_decorative(Some(true)));

        assert_eq!(normalize_element_type(None), SeparatorElementType::Div);
        assert_eq!(
            normalize_element_type(Some(SeparatorElementType::Hr)),
            SeparatorElementType::Hr
        );
    }

    #[test]
    fn normalize_props_centralizes_state_input_and_class_source() {
        let normalized = normalize_props(SeparatorNormalizeInput {
            orientation: Some(SeparatorOrientation::Vertical),
            is_decorative: None,
            element_type: None,
            class_name: Some(" docs-separator ".to_string()),
        });

        assert_eq!(
            normalized.state_input.orientation,
            SeparatorOrientation::Vertical
        );
        assert_eq!(
            normalized.state_input.element_type,
            SeparatorElementType::Div
        );
        assert!(!normalized.state_input.decorative);
        assert!(normalized.state_input.has_custom_class_name);
        assert_eq!(normalized.class_name, Some("docs-separator".to_string()));
    }
}
