pub use crate::button::normalize_optional_text;

pub const SEPARATOR_UI_SCHEMA: &str = "ui.separator.v1";

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
    pub state_source_attr: &'static str,
    pub ui_schema_attr: &'static str,
    pub intent_attr: &'static str,
    pub action_attr: &'static str,
    pub output_mode_attr: &'static str,
    pub streaming_fallback_attr: &'static str,
    pub output_status_attr: &'static str,
    pub has_custom_class_name: bool,
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
        state_source_attr: "props-static",
        ui_schema_attr: SEPARATOR_UI_SCHEMA,
        intent_attr: "separate-content",
        action_attr: "none",
        output_mode_attr: "snapshot",
        streaming_fallback_attr: "snapshot",
        output_status_attr: "verified",
        has_custom_class_name: input.has_custom_class_name,
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: SeparatorState) -> String {
    let mut classes = vec![
        "ui-separator".to_string(),
        state.orientation_class.into(),
        state.element_class.into(),
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
#[path = "test/separator.rs"]
mod tests;
