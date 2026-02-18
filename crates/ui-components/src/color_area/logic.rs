pub use ui_state_primitives::color_area::{
    ColorAreaState, ColorAreaStateInput, DEFAULT_GRID_SIZE, DEFAULT_LABEL, DEFAULT_STEP,
    DEFAULT_X_AXIS_LABEL, DEFAULT_Y_AXIS_LABEL, clamp_value, normalize_aria_label,
    normalize_optional_text, resolve_state, sanitize_preview_color,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColorAreaDisabledSourceAttr {
    IsProp,
    LegacyProp,
}

impl ColorAreaDisabledSourceAttr {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::IsProp => "is-prop",
            Self::LegacyProp => "legacy-prop",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColorAreaValueControlMode {
    Controlled,
    Uncontrolled,
}

impl ColorAreaValueControlMode {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::Controlled => "controlled",
            Self::Uncontrolled => "uncontrolled",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColorAreaValueSourceAttr {
    External,
    Default,
}

impl ColorAreaValueSourceAttr {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::External => "external",
            Self::Default => "default",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColorAreaAgentSchema {
    V1,
}

impl ColorAreaAgentSchema {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::V1 => "ui.color-area.agent-contract.v1",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColorAreaStreamSupport {
    Optional,
}

impl ColorAreaStreamSupport {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::Optional => "optional",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColorAreaStreamFallback {
    Snapshot,
}

impl ColorAreaStreamFallback {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::Snapshot => "snapshot",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColorAreaStreamMode {
    Snapshot,
}

impl ColorAreaStreamMode {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::Snapshot => "snapshot",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColorAreaOutputStatus {
    Verified,
}

impl ColorAreaOutputStatus {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::Verified => "verified",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColorAreaAgentIntent {
    SelectColorPoint,
}

impl ColorAreaAgentIntent {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::SelectColorPoint => "select-color-point",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColorAreaAgentAction {
    Select,
    Disabled,
}

impl ColorAreaAgentAction {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::Select => "select",
            Self::Disabled => "disabled",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColorAreaAgentState {
    Active,
    Disabled,
}

impl ColorAreaAgentState {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Disabled => "disabled",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ColorAreaAgentContract {
    pub schema_attr: &'static str,
    pub stream_support_attr: &'static str,
    pub stream_fallback_attr: &'static str,
    pub stream_mode_attr: &'static str,
    pub output_status_attr: &'static str,
    pub intent_attr: &'static str,
    pub action_attr: &'static str,
    pub state_attr: &'static str,
    pub source_attr: &'static str,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ColorAreaDisableInput {
    pub is_disabled: Option<bool>,
    pub disabled: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ColorAreaDisableState {
    pub is_disabled: bool,
    pub disabled_source_attr: ColorAreaDisabledSourceAttr,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ColorAreaValueAxis {
    pub control_mode: ColorAreaValueControlMode,
    pub value_source: ColorAreaValueSourceAttr,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ColorAreaRootInput {
    pub class_name: Option<String>,
    pub label: Option<String>,
    pub fallback_label: String,
    pub aria_label: Option<String>,
    pub fallback_aria_label: String,
    pub x_axis_label: Option<String>,
    pub fallback_x_axis_label: String,
    pub y_axis_label: Option<String>,
    pub fallback_y_axis_label: String,
    pub preview_color: Option<String>,
    pub value: (f32, f32),
    pub step: f32,
    pub grid_size: usize,
    pub disabled: ColorAreaDisableInput,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ColorAreaRootState {
    pub class_name: Option<String>,
    pub label: String,
    pub aria_label: String,
    pub x_axis_label: String,
    pub y_axis_label: String,
    pub preview_color: Option<String>,
    pub state: ColorAreaState,
    pub disabled_source_attr: ColorAreaDisabledSourceAttr,
}

pub fn normalize_disable_state(input: ColorAreaDisableInput) -> ColorAreaDisableState {
    if let Some(is_disabled) = input.is_disabled {
        ColorAreaDisableState {
            is_disabled,
            disabled_source_attr: ColorAreaDisabledSourceAttr::IsProp,
        }
    } else {
        ColorAreaDisableState {
            is_disabled: input.disabled,
            disabled_source_attr: ColorAreaDisabledSourceAttr::LegacyProp,
        }
    }
}

pub fn normalize_value_axis(is_controlled: bool) -> ColorAreaValueAxis {
    if is_controlled {
        ColorAreaValueAxis {
            control_mode: ColorAreaValueControlMode::Controlled,
            value_source: ColorAreaValueSourceAttr::External,
        }
    } else {
        ColorAreaValueAxis {
            control_mode: ColorAreaValueControlMode::Uncontrolled,
            value_source: ColorAreaValueSourceAttr::Default,
        }
    }
}

pub fn normalize_default_value(default_value: Option<(f32, f32)>) -> (f32, f32) {
    clamp_value(default_value.unwrap_or((1.0, 1.0)))
}

pub fn normalize_label_with_fallback(label: Option<String>, fallback: &str) -> (String, bool) {
    if let Some(value) = normalize_optional_text(label) {
        return (value, true);
    }

    let fallback = normalize_optional_text(Some(fallback.to_string()))
        .unwrap_or_else(|| DEFAULT_LABEL.to_string());
    (fallback, false)
}

pub fn normalize_aria_label_with_fallback(
    aria_label: Option<String>,
    fallback: &str,
) -> (String, bool) {
    if let Some(value) = normalize_optional_text(aria_label) {
        return (value, true);
    }

    let fallback = normalize_optional_text(Some(fallback.to_string()))
        .unwrap_or_else(|| normalize_aria_label(None).0);
    (fallback, false)
}

pub fn normalize_axis_label_with_fallback(
    axis_label: Option<String>,
    fallback: &str,
    default_value: &str,
) -> (String, bool) {
    if let Some(value) = normalize_optional_text(axis_label) {
        return (value, true);
    }

    let fallback = normalize_optional_text(Some(fallback.to_string()))
        .unwrap_or_else(|| default_value.to_string());
    (fallback, false)
}

pub fn normalize_root_state(input: ColorAreaRootInput) -> ColorAreaRootState {
    let class_name = normalize_optional_text(input.class_name);
    let (label, has_custom_label) =
        normalize_label_with_fallback(input.label, input.fallback_label.as_str());
    let (aria_label, has_custom_aria_label) =
        normalize_aria_label_with_fallback(input.aria_label, input.fallback_aria_label.as_str());
    let (x_axis_label, has_custom_x_axis_label) = normalize_axis_label_with_fallback(
        input.x_axis_label,
        input.fallback_x_axis_label.as_str(),
        DEFAULT_X_AXIS_LABEL,
    );
    let (y_axis_label, has_custom_y_axis_label) = normalize_axis_label_with_fallback(
        input.y_axis_label,
        input.fallback_y_axis_label.as_str(),
        DEFAULT_Y_AXIS_LABEL,
    );
    let preview_color = sanitize_preview_color(input.preview_color);
    let disable = normalize_disable_state(input.disabled);

    let state = resolve_state(ColorAreaStateInput {
        disabled: disable.is_disabled,
        step: input.step,
        value: input.value,
        grid_size: input.grid_size,
        has_preview_color: preview_color.is_some(),
        has_custom_label,
        has_custom_aria_label,
        has_custom_class_name: class_name.is_some(),
        has_custom_x_axis_label,
        has_custom_y_axis_label,
    });

    ColorAreaRootState {
        class_name,
        label,
        aria_label,
        x_axis_label,
        y_axis_label,
        preview_color,
        state,
        disabled_source_attr: disable.disabled_source_attr,
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: ColorAreaState) -> String {
    let mut classes = vec!["ui-color-area".to_string()];

    if state.is_disabled {
        classes.push("ui-color-area--disabled".to_string());
    }

    if state.has_preview_color {
        classes.push("ui-color-area--with-preview".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-color-area--custom-class".to_string());
    }

    if let Some(base_class_name) = base_class_name {
        classes.push(base_class_name);
    }

    classes.join(" ")
}

pub fn resolve_agent_contract(
    state: ColorAreaState,
    value_axis: ColorAreaValueAxis,
) -> ColorAreaAgentContract {
    let action = if state.is_disabled {
        ColorAreaAgentAction::Disabled
    } else {
        ColorAreaAgentAction::Select
    };
    let state_axis = if state.is_disabled {
        ColorAreaAgentState::Disabled
    } else {
        ColorAreaAgentState::Active
    };

    ColorAreaAgentContract {
        schema_attr: ColorAreaAgentSchema::V1.as_attr(),
        stream_support_attr: ColorAreaStreamSupport::Optional.as_attr(),
        stream_fallback_attr: ColorAreaStreamFallback::Snapshot.as_attr(),
        stream_mode_attr: ColorAreaStreamMode::Snapshot.as_attr(),
        output_status_attr: ColorAreaOutputStatus::Verified.as_attr(),
        intent_attr: ColorAreaAgentIntent::SelectColorPoint.as_attr(),
        action_attr: action.as_attr(),
        state_attr: state_axis.as_attr(),
        source_attr: value_axis.value_source.as_attr(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_disable_state_uses_is_prefix_first() {
        let from_is = normalize_disable_state(ColorAreaDisableInput {
            is_disabled: Some(true),
            disabled: false,
        });
        assert!(from_is.is_disabled);
        assert_eq!(
            from_is.disabled_source_attr,
            ColorAreaDisabledSourceAttr::IsProp
        );

        let from_legacy = normalize_disable_state(ColorAreaDisableInput {
            is_disabled: None,
            disabled: true,
        });
        assert!(from_legacy.is_disabled);
        assert_eq!(
            from_legacy.disabled_source_attr,
            ColorAreaDisabledSourceAttr::LegacyProp
        );
    }

    #[test]
    fn normalize_value_axis_tracks_control_mode() {
        let controlled = normalize_value_axis(true);
        assert_eq!(
            controlled.control_mode,
            ColorAreaValueControlMode::Controlled
        );
        assert_eq!(controlled.value_source, ColorAreaValueSourceAttr::External);

        let uncontrolled = normalize_value_axis(false);
        assert_eq!(
            uncontrolled.control_mode,
            ColorAreaValueControlMode::Uncontrolled
        );
        assert_eq!(uncontrolled.value_source, ColorAreaValueSourceAttr::Default);
    }

    #[test]
    fn normalize_default_value_uses_single_fallback_source() {
        assert_eq!(normalize_default_value(None), (1.0, 1.0));
        assert_eq!(normalize_default_value(Some((1.3, -0.1))), (1.0, 0.0));
    }

    #[test]
    fn normalize_root_state_uses_i18n_fallback_and_tracks_sources() {
        let root = normalize_root_state(ColorAreaRootInput {
            class_name: Some(" docs-color-area ".to_string()),
            label: None,
            fallback_label: "Color area i18n".to_string(),
            aria_label: None,
            fallback_aria_label: "Color area aria i18n".to_string(),
            x_axis_label: None,
            fallback_x_axis_label: "Saturation i18n".to_string(),
            y_axis_label: None,
            fallback_y_axis_label: "Lightness i18n".to_string(),
            preview_color: Some("#09f".to_string()),
            value: (0.4, 0.6),
            step: 0.1,
            grid_size: 11,
            disabled: ColorAreaDisableInput {
                is_disabled: Some(false),
                disabled: true,
            },
        });

        assert_eq!(root.label, "Color area i18n");
        assert_eq!(root.aria_label, "Color area aria i18n");
        assert_eq!(root.x_axis_label, "Saturation i18n");
        assert_eq!(root.y_axis_label, "Lightness i18n");
        assert_eq!(root.preview_color.as_deref(), Some("#09f"));
        assert_eq!(
            root.state.label_source_attr,
            ui_state_primitives::color_area::ColorAreaSourceAttr::Default
        );
        assert_eq!(
            root.state.aria_source_attr,
            ui_state_primitives::color_area::ColorAreaSourceAttr::Default
        );
        assert_eq!(
            root.state.x_axis_source_attr,
            ui_state_primitives::color_area::ColorAreaSourceAttr::Default
        );
        assert_eq!(
            root.state.y_axis_source_attr,
            ui_state_primitives::color_area::ColorAreaSourceAttr::Default
        );
        assert_eq!(
            root.disabled_source_attr,
            ColorAreaDisabledSourceAttr::IsProp
        );
    }

    #[test]
    fn compose_class_name_supports_stable_markers() {
        let state = resolve_state(ColorAreaStateInput {
            disabled: true,
            step: 0.1,
            value: (0.2, 0.8),
            grid_size: 11,
            has_preview_color: true,
            has_custom_label: false,
            has_custom_aria_label: false,
            has_custom_class_name: true,
            has_custom_x_axis_label: false,
            has_custom_y_axis_label: false,
        });

        let class_name = compose_class_name(Some("docs-color-area".to_string()), state);
        assert!(class_name.contains("ui-color-area"));
        assert!(class_name.contains("ui-color-area--disabled"));
        assert!(class_name.contains("ui-color-area--with-preview"));
        assert!(class_name.contains("ui-color-area--custom-class"));
        assert!(class_name.contains("docs-color-area"));
    }

    #[test]
    fn resolve_agent_contract_uses_closed_schema_markers() {
        let state = resolve_state(ColorAreaStateInput {
            disabled: false,
            step: 0.1,
            value: (0.2, 0.8),
            grid_size: 11,
            has_preview_color: false,
            has_custom_label: false,
            has_custom_aria_label: false,
            has_custom_class_name: false,
            has_custom_x_axis_label: false,
            has_custom_y_axis_label: false,
        });
        let value_axis = normalize_value_axis(true);
        let agent = resolve_agent_contract(state, value_axis);

        assert_eq!(agent.schema_attr, "ui.color-area.agent-contract.v1");
        assert_eq!(agent.stream_support_attr, "optional");
        assert_eq!(agent.stream_fallback_attr, "snapshot");
        assert_eq!(agent.stream_mode_attr, "snapshot");
        assert_eq!(agent.output_status_attr, "verified");
        assert_eq!(agent.intent_attr, "select-color-point");
        assert_eq!(agent.action_attr, "select");
        assert_eq!(agent.state_attr, "active");
        assert_eq!(agent.source_attr, "external");
    }
}
