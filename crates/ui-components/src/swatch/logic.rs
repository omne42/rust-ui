pub use ui_state_primitives::swatch::{
    SwatchAriaLabelFallbacks, SwatchBorder, SwatchRounding, SwatchShape, SwatchSize, SwatchState,
    SwatchStateInput, normalize_optional_text, resolve_aria_label_with_fallbacks, resolve_state,
    sanitize_color_value,
};

pub fn normalize_default_selected(default_selected: Option<bool>) -> bool {
    default_selected.unwrap_or(false)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwatchAgentSchemaVersion {
    V1,
}

impl SwatchAgentSchemaVersion {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::V1 => "1",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwatchAgentIntent {
    ColorSelection,
}

impl SwatchAgentIntent {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ColorSelection => "color-selection",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwatchAgentAction {
    Initialize,
    TogglePress,
}

impl SwatchAgentAction {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Initialize => "initialize",
            Self::TogglePress => "toggle-press",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwatchAgentStateAxis {
    Selected,
    Unselected,
    Disabled,
    Mixed,
    Nothing,
}

impl SwatchAgentStateAxis {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Selected => "selected",
            Self::Unselected => "unselected",
            Self::Disabled => "disabled",
            Self::Mixed => "mixed",
            Self::Nothing => "nothing",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwatchAgentSource {
    ControlledExternal,
    UncontrolledDefault,
    UncontrolledImplicitDefault,
    TogglePress,
}

impl SwatchAgentSource {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ControlledExternal => "controlled-external",
            Self::UncontrolledDefault => "uncontrolled-default",
            Self::UncontrolledImplicitDefault => "uncontrolled-implicit-default",
            Self::TogglePress => "toggle-press",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwatchAgentOutputStatus {
    Verified,
    Submittable,
}

impl SwatchAgentOutputStatus {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Verified => "verified",
            Self::Submittable => "submittable",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwatchAgentStreamSupport {
    Unsupported,
}

impl SwatchAgentStreamSupport {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Unsupported => "unsupported",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwatchAgentStreamFallback {
    FullSnapshot,
}

impl SwatchAgentStreamFallback {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullSnapshot => "full-snapshot",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SwatchAgentCapabilities {
    pub can_toggle: bool,
    pub can_disable: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SwatchAgentContract {
    pub schema_name: &'static str,
    pub schema_version: SwatchAgentSchemaVersion,
    pub intent: SwatchAgentIntent,
    pub action: SwatchAgentAction,
    pub state: SwatchAgentStateAxis,
    pub source: SwatchAgentSource,
    pub output_status: SwatchAgentOutputStatus,
    pub stream_support: SwatchAgentStreamSupport,
    pub stream_fallback: SwatchAgentStreamFallback,
    pub capabilities: SwatchAgentCapabilities,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SwatchSelectionControlInput {
    pub has_controlled_selected: bool,
    pub default_selected: Option<bool>,
    pub has_on_selected_change: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SwatchSelectionControlState {
    pub default_selected: bool,
    pub is_controlled_selected: bool,
    pub is_uncontrolled_selected: bool,
    pub control_mode_attr: &'static str,
    pub default_selected_source_attr: &'static str,
    pub selected_change_source_attr: &'static str,
}

pub fn resolve_selection_control_state(
    input: SwatchSelectionControlInput,
) -> SwatchSelectionControlState {
    let default_selected = normalize_default_selected(input.default_selected);
    let is_controlled_selected = input.has_controlled_selected;
    let is_uncontrolled_selected = !is_controlled_selected;
    let control_mode_attr = if is_controlled_selected {
        "controlled"
    } else {
        "uncontrolled"
    };
    let default_selected_source_attr = if input.default_selected.is_some() {
        "custom"
    } else {
        "default"
    };
    let selected_change_source_attr = if input.has_on_selected_change {
        "custom"
    } else {
        "none"
    };

    SwatchSelectionControlState {
        default_selected,
        is_controlled_selected,
        is_uncontrolled_selected,
        control_mode_attr,
        default_selected_source_attr,
        selected_change_source_attr,
    }
}

pub fn resolve_agent_source(selection: SwatchSelectionControlState) -> SwatchAgentSource {
    if selection.is_controlled_selected {
        SwatchAgentSource::ControlledExternal
    } else if selection.default_selected_source_attr == "custom" {
        SwatchAgentSource::UncontrolledDefault
    } else {
        SwatchAgentSource::UncontrolledImplicitDefault
    }
}

pub fn resolve_agent_action(source: SwatchAgentSource) -> SwatchAgentAction {
    match source {
        SwatchAgentSource::TogglePress => SwatchAgentAction::TogglePress,
        SwatchAgentSource::ControlledExternal
        | SwatchAgentSource::UncontrolledDefault
        | SwatchAgentSource::UncontrolledImplicitDefault => SwatchAgentAction::Initialize,
    }
}

pub fn resolve_agent_state_axis(state: SwatchState, selected: bool) -> SwatchAgentStateAxis {
    if state.disabled {
        SwatchAgentStateAxis::Disabled
    } else if state.show_mixed_value {
        SwatchAgentStateAxis::Mixed
    } else if state.show_nothing {
        SwatchAgentStateAxis::Nothing
    } else if selected {
        SwatchAgentStateAxis::Selected
    } else {
        SwatchAgentStateAxis::Unselected
    }
}

pub fn resolve_agent_output_status(source: SwatchAgentSource) -> SwatchAgentOutputStatus {
    match source {
        SwatchAgentSource::TogglePress => SwatchAgentOutputStatus::Submittable,
        SwatchAgentSource::ControlledExternal
        | SwatchAgentSource::UncontrolledDefault
        | SwatchAgentSource::UncontrolledImplicitDefault => SwatchAgentOutputStatus::Verified,
    }
}

pub fn resolve_agent_contract(
    state: SwatchState,
    selected: bool,
    source: SwatchAgentSource,
) -> SwatchAgentContract {
    SwatchAgentContract {
        schema_name: "ui.swatch.agent-contract",
        schema_version: SwatchAgentSchemaVersion::V1,
        intent: SwatchAgentIntent::ColorSelection,
        action: resolve_agent_action(source),
        state: resolve_agent_state_axis(state, selected),
        source,
        output_status: resolve_agent_output_status(source),
        stream_support: SwatchAgentStreamSupport::Unsupported,
        stream_fallback: SwatchAgentStreamFallback::FullSnapshot,
        capabilities: SwatchAgentCapabilities {
            can_toggle: state.is_interactive,
            can_disable: true,
        },
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: SwatchState) -> String {
    let mut classes = vec![
        "ui-swatch".to_string(),
        state.size_class.to_string(),
        state.border_class.to_string(),
        state.rounding_class.to_string(),
        state.shape_class.to_string(),
    ];

    if state.show_mixed_value {
        classes.push("ui-swatch--mixed".to_string());
    }

    if state.show_nothing {
        classes.push("ui-swatch--nothing".to_string());
    }

    if state.disabled {
        classes.push("ui-swatch--disabled".to_string());
    }

    if !state.is_interactive {
        classes.push("ui-swatch--static".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-swatch--custom-class".to_string());
        if let Some(base_class_name) = base_class_name {
            classes.push(base_class_name);
        }
    }

    classes.join(" ")
}

pub fn compose_inline_style(color: Option<&str>) -> Option<String> {
    color.map(|color| format!("--ui-swatch-color: {color};"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compose_class_name_includes_expected_tokens() {
        let state = resolve_state(SwatchStateInput {
            size: SwatchSize::L,
            border: SwatchBorder::Light,
            rounding: SwatchRounding::Full,
            shape: SwatchShape::Rectangle,
            has_color: false,
            nothing: true,
            mixed_value: false,
            disabled: true,
            decorative: true,
            has_custom_class_name: true,
        });

        let class_name = compose_class_name(Some("docs-swatch".to_string()), state);
        for token in [
            "ui-swatch",
            "ui-swatch--size-l",
            "ui-swatch--border-light",
            "ui-swatch--rounding-full",
            "ui-swatch--shape-rectangle",
            "ui-swatch--nothing",
            "ui-swatch--disabled",
            "ui-swatch--static",
            "ui-swatch--custom-class",
            "docs-swatch",
        ] {
            assert!(
                class_name.contains(token),
                "composed class name should include `{token}`"
            );
        }
    }

    #[test]
    fn compose_inline_style_includes_css_variable() {
        assert_eq!(
            compose_inline_style(Some("#ff0000")),
            Some("--ui-swatch-color: #ff0000;".to_string())
        );
    }

    #[test]
    fn normalize_default_selected_falls_back_to_false() {
        assert!(!normalize_default_selected(None));
        assert!(normalize_default_selected(Some(true)));
        assert!(!normalize_default_selected(Some(false)));
    }

    #[test]
    fn resolve_selection_control_state_maps_control_and_source_markers() {
        let controlled = resolve_selection_control_state(SwatchSelectionControlInput {
            has_controlled_selected: true,
            default_selected: Some(true),
            has_on_selected_change: true,
        });
        assert!(controlled.default_selected);
        assert!(controlled.is_controlled_selected);
        assert!(!controlled.is_uncontrolled_selected);
        assert_eq!(controlled.control_mode_attr, "controlled");
        assert_eq!(controlled.default_selected_source_attr, "custom");
        assert_eq!(controlled.selected_change_source_attr, "custom");

        let uncontrolled = resolve_selection_control_state(SwatchSelectionControlInput {
            has_controlled_selected: false,
            default_selected: None,
            has_on_selected_change: false,
        });
        assert!(!uncontrolled.default_selected);
        assert!(!uncontrolled.is_controlled_selected);
        assert!(uncontrolled.is_uncontrolled_selected);
        assert_eq!(uncontrolled.control_mode_attr, "uncontrolled");
        assert_eq!(uncontrolled.default_selected_source_attr, "default");
        assert_eq!(uncontrolled.selected_change_source_attr, "none");
    }

    #[test]
    fn resolve_agent_contract_is_schema_typed_and_snapshot_fallback_explicit() {
        let state = resolve_state(SwatchStateInput {
            size: SwatchSize::M,
            border: SwatchBorder::Default,
            rounding: SwatchRounding::Default,
            shape: SwatchShape::Square,
            has_color: true,
            nothing: false,
            mixed_value: false,
            disabled: false,
            decorative: false,
            has_custom_class_name: false,
        });
        let contract = resolve_agent_contract(state, true, SwatchAgentSource::TogglePress);

        assert_eq!(contract.schema_name, "ui.swatch.agent-contract");
        assert_eq!(contract.schema_version.as_str(), "1");
        assert_eq!(contract.intent.as_str(), "color-selection");
        assert_eq!(contract.action.as_str(), "toggle-press");
        assert_eq!(contract.state.as_str(), "selected");
        assert_eq!(contract.source.as_str(), "toggle-press");
        assert_eq!(contract.output_status.as_str(), "submittable");
        assert_eq!(contract.stream_support.as_str(), "unsupported");
        assert_eq!(contract.stream_fallback.as_str(), "full-snapshot");
        assert!(contract.capabilities.can_toggle);
        assert!(contract.capabilities.can_disable);
    }
}
