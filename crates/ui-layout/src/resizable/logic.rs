use crate::resizable::DEFAULT_ARIA_LABEL;
use leptos::prelude::*;

pub use ui_state_primitives::resizable::{
    ResizableOrientation, ResizableState, normalize_bounds, normalize_split,
};

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.into())
    })
}

pub fn normalize_aria_label(value: Option<String>) -> String {
    normalize_optional_text(value).unwrap_or_else(|| DEFAULT_ARIA_LABEL.into())
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ResizableControlMode {
    Controlled,
    Uncontrolled,
}

impl ResizableControlMode {
    pub fn as_attr(self) -> &'static str {
        match self {
            Self::Controlled => "controlled",
            Self::Uncontrolled => "uncontrolled",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ResizableValueSource {
    Value,
    SplitPercent,
    DefaultValue,
}

impl ResizableValueSource {
    pub fn as_attr(self) -> &'static str {
        match self {
            Self::Value => "value",
            Self::SplitPercent => "split_percent",
            Self::DefaultValue => "default_value",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ResizableDefaultValueSource {
    DefaultValue,
    DefaultSplitPercent,
    ImplicitDefault,
}

impl ResizableDefaultValueSource {
    pub fn as_attr(self) -> &'static str {
        match self {
            Self::DefaultValue => "default_value",
            Self::DefaultSplitPercent => "default_split_percent",
            Self::ImplicitDefault => "implicit_default",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ResizableValueChangeSource {
    OnValueChange,
    OnSplitPercentChange,
    None,
}

impl ResizableValueChangeSource {
    pub fn as_attr(self) -> &'static str {
        match self {
            Self::OnValueChange => "on_value_change",
            Self::OnSplitPercentChange => "on_split_percent_change",
            Self::None => "none",
        }
    }
}

pub struct ResizableValueAxisInput {
    pub value: Option<Signal<f64>>,
    pub split_percent: Option<Signal<f64>>,
    pub default_value: Option<f64>,
    pub default_split_percent: Option<f64>,
    pub on_value_change: Option<Callback<f64>>,
    pub on_split_percent_change: Option<Callback<f64>>,
    pub bounds: ui_state_primitives::resizable::SplitBounds,
}

pub struct ResizableValueAxisState {
    pub value: Option<Signal<f64>>,
    pub default_value: f64,
    pub on_value_change: Option<Callback<f64>>,
    pub value_change_source: ResizableValueChangeSource,
    pub control_mode_attr: &'static str,
    pub value_source_attr: &'static str,
    pub default_value_source_attr: &'static str,
    pub value_change_source_attr: &'static str,
}

pub fn normalize_value_axis(input: ResizableValueAxisInput) -> ResizableValueAxisState {
    let value = input.value.or(input.split_percent);
    let has_controlled_value = value.is_some();
    let value_source = if input.value.is_some() {
        ResizableValueSource::Value
    } else if input.split_percent.is_some() {
        ResizableValueSource::SplitPercent
    } else {
        ResizableValueSource::DefaultValue
    };

    let default_value_source = if input.default_value.is_some() {
        ResizableDefaultValueSource::DefaultValue
    } else if input.default_split_percent.is_some() {
        ResizableDefaultValueSource::DefaultSplitPercent
    } else {
        ResizableDefaultValueSource::ImplicitDefault
    };
    let normalized_default_value = normalize_split(
        input.default_value.or(input.default_split_percent),
        input.bounds,
    );

    let value_change_source = if input.on_value_change.is_some() {
        ResizableValueChangeSource::OnValueChange
    } else if input.on_split_percent_change.is_some() {
        ResizableValueChangeSource::OnSplitPercentChange
    } else {
        ResizableValueChangeSource::None
    };
    let on_value_change = input.on_value_change.or(input.on_split_percent_change);

    let control_mode = if has_controlled_value {
        ResizableControlMode::Controlled
    } else {
        ResizableControlMode::Uncontrolled
    };

    ResizableValueAxisState {
        value,
        default_value: normalized_default_value,
        on_value_change,
        value_change_source,
        control_mode_attr: control_mode.as_attr(),
        value_source_attr: value_source.as_attr(),
        default_value_source_attr: default_value_source.as_attr(),
        value_change_source_attr: value_change_source.as_attr(),
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ResizableDisabledSource {
    IsDisabled,
    Disabled,
    Default,
}

impl ResizableDisabledSource {
    pub fn as_attr(self) -> &'static str {
        match self {
            Self::IsDisabled => "is_disabled",
            Self::Disabled => "disabled",
            Self::Default => "default",
        }
    }
}

pub struct ResizableDisabledInput {
    pub is_disabled: Option<bool>,
    pub disabled: bool,
}

pub struct ResizableDisabledState {
    pub is_disabled: bool,
    pub disabled_source_attr: &'static str,
}

pub fn normalize_disabled(input: ResizableDisabledInput) -> ResizableDisabledState {
    let is_disabled = input.is_disabled.unwrap_or(input.disabled);
    let source = if input.is_disabled.is_some() {
        ResizableDisabledSource::IsDisabled
    } else if input.disabled {
        ResizableDisabledSource::Disabled
    } else {
        ResizableDisabledSource::Default
    };

    ResizableDisabledState {
        is_disabled,
        disabled_source_attr: source.as_attr(),
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ResizableHandleSource {
    IsWithHandle,
    WithHandle,
    Default,
}

impl ResizableHandleSource {
    pub fn as_attr(self) -> &'static str {
        match self {
            Self::IsWithHandle => "is_with_handle",
            Self::WithHandle => "with_handle",
            Self::Default => "default",
        }
    }
}

pub struct ResizableHandleInput {
    pub is_with_handle: Option<bool>,
    pub with_handle: bool,
}

pub struct ResizableHandleState {
    pub with_handle: bool,
    pub with_handle_source_attr: &'static str,
}

pub fn normalize_handle(input: ResizableHandleInput) -> ResizableHandleState {
    let with_handle = input.is_with_handle.unwrap_or(input.with_handle);
    let source = if input.is_with_handle.is_some() {
        ResizableHandleSource::IsWithHandle
    } else if input.with_handle {
        ResizableHandleSource::WithHandle
    } else {
        ResizableHandleSource::Default
    };

    ResizableHandleState {
        with_handle,
        with_handle_source_attr: source.as_attr(),
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ResizableAgentSchema {
    V1,
}

impl ResizableAgentSchema {
    pub fn as_attr(self) -> &'static str {
        match self {
            Self::V1 => "ui.resizable.agent-contract.v1",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ResizableAgentIntent {
    AdjustSplit,
}

impl ResizableAgentIntent {
    pub fn as_attr(self) -> &'static str {
        match self {
            Self::AdjustSplit => "adjust-split",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ResizableAgentActionModel {
    PointerKeyboard,
}

impl ResizableAgentActionModel {
    pub fn as_attr(self) -> &'static str {
        match self {
            Self::PointerKeyboard => "pointer+keyboard",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ResizableStreamSupport {
    Unsupported,
}

impl ResizableStreamSupport {
    pub fn as_attr(self) -> &'static str {
        match self {
            Self::Unsupported => "unsupported",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ResizableStreamFallback {
    Snapshot,
}

impl ResizableStreamFallback {
    pub fn as_attr(self) -> &'static str {
        match self {
            Self::Snapshot => "snapshot",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ResizableStreamMode {
    Snapshot,
}

impl ResizableStreamMode {
    pub fn as_attr(self) -> &'static str {
        match self {
            Self::Snapshot => "snapshot",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ResizableOutputStatus {
    Verified,
    Submittable,
}

impl ResizableOutputStatus {
    pub fn as_attr(self) -> &'static str {
        match self {
            Self::Verified => "verified",
            Self::Submittable => "submittable",
        }
    }
}

pub struct ResizableAgentContract {
    pub schema_attr: &'static str,
    pub intent_attr: &'static str,
    pub action_model_attr: &'static str,
    pub state_axis_attr: &'static str,
    pub source_axis_attr: &'static str,
    pub stream_support_attr: &'static str,
    pub stream_fallback_attr: &'static str,
    pub stream_mode_attr: &'static str,
    pub output_status_attr: &'static str,
}

pub fn resolve_agent_contract(
    value_change_source: ResizableValueChangeSource,
) -> ResizableAgentContract {
    let output_status = if matches!(value_change_source, ResizableValueChangeSource::None) {
        ResizableOutputStatus::Verified
    } else {
        ResizableOutputStatus::Submittable
    };

    ResizableAgentContract {
        schema_attr: ResizableAgentSchema::V1.as_attr(),
        intent_attr: ResizableAgentIntent::AdjustSplit.as_attr(),
        action_model_attr: ResizableAgentActionModel::PointerKeyboard.as_attr(),
        state_axis_attr: "orientation:split:dragging:disabled:control:handle",
        source_axis_attr: "value:default:value_change:disabled:handle:class",
        stream_support_attr: ResizableStreamSupport::Unsupported.as_attr(),
        stream_fallback_attr: ResizableStreamFallback::Snapshot.as_attr(),
        stream_mode_attr: ResizableStreamMode::Snapshot.as_attr(),
        output_status_attr: output_status.as_attr(),
    }
}

pub fn compose_class_name(class_name: Option<String>, state: ResizableState) -> String {
    let mut classes = vec!["ui-resizable".to_string()];

    match state.orientation {
        ResizableOrientation::Horizontal => classes.push("ui-resizable--horizontal".to_string()),
        ResizableOrientation::Vertical => classes.push("ui-resizable--vertical".to_string()),
    }

    if state.dragging {
        classes.push("ui-resizable--dragging".to_string());
    }

    if state.disabled {
        classes.push("ui-resizable--disabled".to_string());
    }

    if state.with_handle {
        classes.push("ui-resizable--with-handle".to_string());
    }

    if state.is_controlled {
        classes.push("ui-resizable--controlled".to_string());
    } else {
        classes.push("ui-resizable--uncontrolled".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-resizable--custom-class".to_string());
        if let Some(class_name) = class_name {
            classes.push(class_name);
        }
    }

    classes.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_helpers_trim_and_fallback() {
        assert_eq!(normalize_optional_text(None), None);
        assert_eq!(normalize_optional_text(Some("  ".to_string())), None);
        assert_eq!(
            normalize_optional_text(Some("  layout splitter  ".to_string())),
            Some("layout splitter".to_string())
        );

        assert_eq!(normalize_aria_label(None), DEFAULT_ARIA_LABEL);
        assert_eq!(
            normalize_aria_label(Some("  Pane split control  ".to_string())),
            "Pane split control"
        );
    }

    #[test]
    fn normalize_value_axis_supports_canonical_and_legacy_names() {
        let bounds = normalize_bounds(0.0, 100.0);
        let (_value, set_value) = signal(0.0_f64);

        let normalized = normalize_value_axis(ResizableValueAxisInput {
            value: None,
            split_percent: None,
            default_value: Some(64.0),
            default_split_percent: Some(36.0),
            on_value_change: None,
            on_split_percent_change: Some(Callback::new(move |next| set_value.set(next))),
            bounds,
        });

        assert_eq!(normalized.control_mode_attr, "uncontrolled");
        assert_eq!(normalized.value_source_attr, "default_value");
        assert_eq!(normalized.default_value_source_attr, "default_value");
        assert_eq!(
            normalized.value_change_source_attr,
            "on_split_percent_change"
        );
        assert_eq!(normalized.default_value, 64.0);
        assert!(normalized.on_value_change.is_some());
    }

    #[test]
    fn normalize_disabled_and_handle_track_sources() {
        let disabled = normalize_disabled(ResizableDisabledInput {
            is_disabled: Some(true),
            disabled: false,
        });
        assert!(disabled.is_disabled);
        assert_eq!(disabled.disabled_source_attr, "is_disabled");

        let handle = normalize_handle(ResizableHandleInput {
            is_with_handle: None,
            with_handle: true,
        });
        assert!(handle.with_handle);
        assert_eq!(handle.with_handle_source_attr, "with_handle");
    }

    #[test]
    fn compose_class_name_surface_all_markers() {
        let state = ui_state_primitives::resizable::resolve_state(
            ui_state_primitives::resizable::ResizableStateInput {
                orientation: ResizableOrientation::Vertical,
                split_percent: 88.0,
                bounds: ui_state_primitives::resizable::SplitBounds {
                    min: 20.0,
                    max: 80.0,
                },
                disabled: false,
                dragging: true,
                is_controlled: true,
                with_handle: true,
                has_custom_class_name: true,
            },
        );

        let class_name = compose_class_name(Some("docs-resizable".to_string()), state);
        for token in [
            "ui-resizable",
            "ui-resizable--vertical",
            "ui-resizable--dragging",
            "ui-resizable--with-handle",
            "ui-resizable--controlled",
            "ui-resizable--custom-class",
            "docs-resizable",
        ] {
            assert!(
                class_name.contains(token),
                "composed class should include `{token}`"
            );
        }
    }

    #[test]
    fn resolve_agent_contract_tracks_change_handler_presence() {
        let contract = resolve_agent_contract(ResizableValueChangeSource::OnValueChange);
        assert_eq!(contract.schema_attr, "ui.resizable.agent-contract.v1");
        assert_eq!(contract.intent_attr, "adjust-split");
        assert_eq!(contract.action_model_attr, "pointer+keyboard");
        assert_eq!(
            contract.state_axis_attr,
            "orientation:split:dragging:disabled:control:handle"
        );
        assert_eq!(
            contract.source_axis_attr,
            "value:default:value_change:disabled:handle:class"
        );
        assert_eq!(contract.stream_support_attr, "unsupported");
        assert_eq!(contract.stream_fallback_attr, "snapshot");
        assert_eq!(contract.stream_mode_attr, "snapshot");
        assert_eq!(contract.output_status_attr, "submittable");

        let readonly_contract = resolve_agent_contract(ResizableValueChangeSource::None);
        assert_eq!(readonly_contract.output_status_attr, "verified");
    }
}
