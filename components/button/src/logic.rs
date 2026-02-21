use super::spec::ButtonSchema;
use ui_state_primitives::button::{ButtonStateCoreInput, resolve_state_core};

pub use ui_state_primitives::button::ButtonLabelSource;
pub use ui_state_primitives::button::{normalize_optional_text, resolve_aria_label};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ButtonVariant {
    #[default]
    Default,
    Solid,
    Faded,
    Bordered,
    Light,
    Flat,
    Shadow,
    Accent,
    Destructive,
    Outline,
    Secondary,
    Ghost,
    Link,
}

impl ButtonVariant {
    pub fn class_name(self) -> &'static str {
        match self {
            ButtonVariant::Default => "ui-button--variant-default",
            ButtonVariant::Solid => "ui-button--variant-solid",
            ButtonVariant::Faded => "ui-button--variant-faded",
            ButtonVariant::Bordered => "ui-button--variant-bordered",
            ButtonVariant::Light => "ui-button--variant-light",
            ButtonVariant::Flat => "ui-button--variant-flat",
            ButtonVariant::Shadow => "ui-button--variant-shadow",
            ButtonVariant::Accent => "ui-button--variant-accent",
            ButtonVariant::Destructive => "ui-button--variant-destructive",
            ButtonVariant::Outline => "ui-button--variant-outline",
            ButtonVariant::Secondary => "ui-button--variant-secondary",
            ButtonVariant::Ghost => "ui-button--variant-ghost",
            ButtonVariant::Link => "ui-button--variant-link",
        }
    }
}

impl From<&str> for ButtonVariant {
    fn from(value: &str) -> Self {
        match value.trim().to_ascii_lowercase().as_str() {
            "default" => ButtonVariant::Default,
            "solid" => ButtonVariant::Solid,
            "faded" => ButtonVariant::Faded,
            "bordered" => ButtonVariant::Bordered,
            "light" => ButtonVariant::Light,
            "flat" => ButtonVariant::Flat,
            "shadow" => ButtonVariant::Shadow,
            "accent" => ButtonVariant::Accent,
            "destructive" | "danger" => ButtonVariant::Destructive,
            "outline" => ButtonVariant::Outline,
            "secondary" => ButtonVariant::Secondary,
            "ghost" => ButtonVariant::Ghost,
            "link" => ButtonVariant::Link,
            other => panic!("unsupported ButtonVariant `{other}`"),
        }
    }
}

impl From<String> for ButtonVariant {
    fn from(value: String) -> Self {
        Self::from(value.as_str())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ButtonColor {
    Default,
    #[default]
    Primary,
    Secondary,
    Success,
    Warning,
    Danger,
}

impl ButtonColor {
    pub fn class_name(self) -> &'static str {
        match self {
            ButtonColor::Default => "ui-button--color-default",
            ButtonColor::Primary => "ui-button--color-primary",
            ButtonColor::Secondary => "ui-button--color-secondary",
            ButtonColor::Success => "ui-button--color-success",
            ButtonColor::Warning => "ui-button--color-warning",
            ButtonColor::Danger => "ui-button--color-danger",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            ButtonColor::Default => "default",
            ButtonColor::Primary => "primary",
            ButtonColor::Secondary => "secondary",
            ButtonColor::Success => "success",
            ButtonColor::Warning => "warning",
            ButtonColor::Danger => "danger",
        }
    }
}

impl From<&str> for ButtonColor {
    fn from(value: &str) -> Self {
        match value.trim().to_ascii_lowercase().as_str() {
            "default" => ButtonColor::Default,
            "primary" => ButtonColor::Primary,
            "secondary" => ButtonColor::Secondary,
            "success" => ButtonColor::Success,
            "warning" => ButtonColor::Warning,
            "danger" | "destructive" => ButtonColor::Danger,
            other => panic!("unsupported ButtonColor `{other}`"),
        }
    }
}

impl From<String> for ButtonColor {
    fn from(value: String) -> Self {
        Self::from(value.as_str())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ButtonRadius {
    None,
    Sm,
    #[default]
    Md,
    Lg,
    Full,
}

impl ButtonRadius {
    pub fn class_name(self) -> &'static str {
        match self {
            ButtonRadius::None => "ui-button--radius-none",
            ButtonRadius::Sm => "ui-button--radius-sm",
            ButtonRadius::Md => "ui-button--radius-md",
            ButtonRadius::Lg => "ui-button--radius-lg",
            ButtonRadius::Full => "ui-button--radius-full",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            ButtonRadius::None => "none",
            ButtonRadius::Sm => "sm",
            ButtonRadius::Md => "md",
            ButtonRadius::Lg => "lg",
            ButtonRadius::Full => "full",
        }
    }
}

impl From<&str> for ButtonRadius {
    fn from(value: &str) -> Self {
        match value.trim().to_ascii_lowercase().as_str() {
            "none" => ButtonRadius::None,
            "sm" | "small" => ButtonRadius::Sm,
            "md" | "medium" => ButtonRadius::Md,
            "lg" | "large" => ButtonRadius::Lg,
            "full" => ButtonRadius::Full,
            other => panic!("unsupported ButtonRadius `{other}`"),
        }
    }
}

impl From<String> for ButtonRadius {
    fn from(value: String) -> Self {
        Self::from(value.as_str())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ButtonSize {
    Xs,
    S,
    #[default]
    M,
    L,
    Xl,
    IconXs,
    IconS,
    IconM,
    IconL,
    IconXl,
    Default,
    Sm,
    Lg,
    Icon,
    IconSm,
    IconLg,
}

impl ButtonSize {
    pub fn class_name(self) -> &'static str {
        match self {
            ButtonSize::Xs => "ui-button--size-xs",
            ButtonSize::S => "ui-button--size-s",
            ButtonSize::M => "ui-button--size-m",
            ButtonSize::L => "ui-button--size-l",
            ButtonSize::Xl => "ui-button--size-xl",
            ButtonSize::IconXs => "ui-button--size-icon-xs",
            ButtonSize::IconS => "ui-button--size-icon-s",
            ButtonSize::IconM => "ui-button--size-icon-m",
            ButtonSize::IconL => "ui-button--size-icon-l",
            ButtonSize::IconXl => "ui-button--size-icon-xl",
            ButtonSize::Default => "ui-button--size-m",
            ButtonSize::Sm => "ui-button--size-s",
            ButtonSize::Lg => "ui-button--size-l",
            ButtonSize::Icon => "ui-button--size-icon-m",
            ButtonSize::IconSm => "ui-button--size-icon-s",
            ButtonSize::IconLg => "ui-button--size-icon-l",
        }
    }
}

impl From<&str> for ButtonSize {
    fn from(value: &str) -> Self {
        match value.trim().to_ascii_lowercase().as_str() {
            "xs" => ButtonSize::Xs,
            "s" | "small" | "sm" => ButtonSize::S,
            "m" | "medium" | "md" | "default" => ButtonSize::M,
            "l" | "large" | "lg" => ButtonSize::L,
            "xl" => ButtonSize::Xl,
            "icon-xs" => ButtonSize::IconXs,
            "icon-s" | "icon-sm" => ButtonSize::IconS,
            "icon-m" | "icon-md" | "icon" => ButtonSize::IconM,
            "icon-l" | "icon-lg" => ButtonSize::IconL,
            "icon-xl" => ButtonSize::IconXl,
            other => panic!("unsupported ButtonSize `{other}`"),
        }
    }
}

impl From<String> for ButtonSize {
    fn from(value: String) -> Self {
        Self::from(value.as_str())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ButtonLoadingPlacement {
    #[default]
    Start,
    End,
    Center,
}

impl ButtonLoadingPlacement {
    pub fn as_attr(self) -> &'static str {
        match self {
            ButtonLoadingPlacement::Start => "start",
            ButtonLoadingPlacement::End => "end",
            ButtonLoadingPlacement::Center => "center",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ButtonType {
    #[default]
    Button,
    Submit,
    Reset,
}

impl ButtonType {
    pub fn as_attr(self) -> &'static str {
        match self {
            ButtonType::Button => "button",
            ButtonType::Submit => "submit",
            ButtonType::Reset => "reset",
        }
    }
}

impl From<&str> for ButtonType {
    fn from(value: &str) -> Self {
        match value.trim().to_ascii_lowercase().as_str() {
            "button" => ButtonType::Button,
            "submit" => ButtonType::Submit,
            "reset" => ButtonType::Reset,
            other => panic!("unsupported ButtonType `{other}`"),
        }
    }
}

impl From<String> for ButtonType {
    fn from(value: String) -> Self {
        Self::from(value.as_str())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ButtonStateInput {
    pub is_disabled: bool,
    pub is_loading: bool,
    pub variant: ButtonVariant,
    pub color: ButtonColor,
    pub radius: ButtonRadius,
    pub size: ButtonSize,
    pub loading_placement: ButtonLoadingPlacement,
    pub is_icon_only: bool,
    pub is_full_width: bool,
    pub has_start_content: bool,
    pub has_end_content: bool,
    pub has_custom_class_name: bool,
    pub has_custom_motion: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ButtonInputNormalizationInput {
    pub is_disabled: bool,
    pub is_full_width: bool,
    pub class_name: Option<String>,
    pub aria_label: Option<String>,
    pub icon_only_fallback_aria_label: Option<String>,
    pub is_icon_only: bool,
    pub button_type: ButtonType,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ButtonInputNormalization {
    pub is_disabled: bool,
    pub is_full_width: bool,
    pub disabled_input_source: ButtonBooleanInputSource,
    pub full_width_input_source: ButtonBooleanInputSource,
    pub class_name: Option<String>,
    pub has_custom_class_name: bool,
    pub aria_label: Option<String>,
    pub aria_label_source: ButtonLabelSource,
    pub button_type: ButtonType,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ButtonBooleanInputSource {
    IsProp,
    Default,
}

impl ButtonBooleanInputSource {
    pub fn as_attr(self) -> &'static str {
        match self {
            ButtonBooleanInputSource::IsProp => "is-prop",
            ButtonBooleanInputSource::Default => "default",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ButtonStateSource {
    pub disabled_source_attr: &'static str,
    pub loading_source_attr: &'static str,
    pub disabled_input_source_attr: &'static str,
    pub full_width_input_source_attr: &'static str,
}

pub const BUTTON_AGENT_SCHEMA: &str = "ui.button.agent-contract";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ButtonAgentSchemaVersion {
    V1,
}

impl ButtonAgentSchemaVersion {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::V1 => "1",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ButtonAgentIntent {
    Trigger,
}

impl ButtonAgentIntent {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Trigger => "trigger",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ButtonAgentAction {
    Press,
}

impl ButtonAgentAction {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Press => "press",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ButtonAgentStateAxis {
    Disabled,
    Loading,
    Ready,
}

impl ButtonAgentStateAxis {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Disabled => "disabled",
            Self::Loading => "loading",
            Self::Ready => "ready",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ButtonAgentSource {
    StatePrimitives,
}

impl ButtonAgentSource {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StatePrimitives => "state-primitives",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ButtonAgentCapabilities {
    pub can_press: bool,
    pub can_focus: bool,
    pub can_hover: bool,
    pub can_popup_trigger: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ButtonAgentContract {
    pub schema_name: &'static str,
    pub schema_version: ButtonAgentSchemaVersion,
    pub intent: ButtonAgentIntent,
    pub action: ButtonAgentAction,
    pub state: ButtonAgentStateAxis,
    pub source: ButtonAgentSource,
    pub capabilities: ButtonAgentCapabilities,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ButtonSchemaInputSource {
    Missing,
    PropValidated,
    PropRejected,
}

impl ButtonSchemaInputSource {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::Missing => "missing",
            Self::PropValidated => "prop-validated",
            Self::PropRejected => "prop-rejected",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ButtonSchemaInputNormalization {
    pub schema_json: Option<String>,
    pub source: ButtonSchemaInputSource,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ButtonOutputStatus {
    Draft,
    Verified,
    Submittable,
}

impl ButtonOutputStatus {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::Draft => "draft",
            Self::Verified => "verified",
            Self::Submittable => "submittable",
        }
    }
}

pub fn normalize_schema_json_input(schema_json: Option<String>) -> ButtonSchemaInputNormalization {
    let normalized = normalize_optional_text(schema_json);
    let Some(raw) = normalized else {
        return ButtonSchemaInputNormalization {
            schema_json: None,
            source: ButtonSchemaInputSource::Missing,
        };
    };

    match ButtonSchema::from_json(&raw).and_then(|schema| schema.to_json_result()) {
        Ok(canonical_json) => ButtonSchemaInputNormalization {
            schema_json: Some(canonical_json),
            source: ButtonSchemaInputSource::PropValidated,
        },
        Err(_) => ButtonSchemaInputNormalization {
            schema_json: None,
            source: ButtonSchemaInputSource::PropRejected,
        },
    }
}

pub fn resolve_agent_state_axis(state: ButtonState) -> ButtonAgentStateAxis {
    if state.is_disabled {
        ButtonAgentStateAxis::Disabled
    } else if state.is_loading {
        ButtonAgentStateAxis::Loading
    } else {
        ButtonAgentStateAxis::Ready
    }
}

pub fn resolve_agent_capabilities_for_state_axis(
    state: ButtonAgentStateAxis,
    has_popup_trigger: bool,
) -> ButtonAgentCapabilities {
    let can_interact = !matches!(state, ButtonAgentStateAxis::Disabled);
    ButtonAgentCapabilities {
        can_press: can_interact,
        can_focus: can_interact,
        can_hover: can_interact,
        can_popup_trigger: has_popup_trigger,
    }
}

pub fn resolve_agent_contract(state: ButtonState, has_popup_trigger: bool) -> ButtonAgentContract {
    resolve_agent_contract_for_state_axis(resolve_agent_state_axis(state), has_popup_trigger)
}

pub fn resolve_agent_contract_for_state_axis(
    state: ButtonAgentStateAxis,
    has_popup_trigger: bool,
) -> ButtonAgentContract {
    ButtonAgentContract {
        schema_name: BUTTON_AGENT_SCHEMA,
        schema_version: ButtonAgentSchemaVersion::V1,
        intent: ButtonAgentIntent::Trigger,
        action: ButtonAgentAction::Press,
        state,
        source: ButtonAgentSource::StatePrimitives,
        capabilities: resolve_agent_capabilities_for_state_axis(state, has_popup_trigger),
    }
}

pub fn resolve_output_status(state: ButtonState, button_type: ButtonType) -> ButtonOutputStatus {
    if state.is_loading {
        ButtonOutputStatus::Draft
    } else if !state.is_disabled && matches!(button_type, ButtonType::Submit) {
        ButtonOutputStatus::Submittable
    } else {
        ButtonOutputStatus::Verified
    }
}

pub fn normalize_input(input: ButtonInputNormalizationInput) -> ButtonInputNormalization {
    let (is_disabled, disabled_input_source) = if input.is_disabled {
        (true, ButtonBooleanInputSource::IsProp)
    } else {
        (false, ButtonBooleanInputSource::Default)
    };
    let (is_full_width, full_width_input_source) = if input.is_full_width {
        (true, ButtonBooleanInputSource::IsProp)
    } else {
        (false, ButtonBooleanInputSource::Default)
    };
    let class_name = normalize_optional_text(input.class_name);
    let has_custom_class_name = class_name.is_some();
    let (aria_label, aria_label_source) = resolve_aria_label(
        input.aria_label,
        input.is_icon_only,
        input.icon_only_fallback_aria_label,
    );
    let button_type = input.button_type;

    ButtonInputNormalization {
        is_disabled,
        is_full_width,
        disabled_input_source,
        full_width_input_source,
        class_name,
        has_custom_class_name,
        aria_label,
        aria_label_source,
        button_type,
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ButtonLogicInput {
    pub normalized: ButtonInputNormalization,
    pub is_loading: bool,
    pub variant: ButtonVariant,
    pub color: ButtonColor,
    pub radius: ButtonRadius,
    pub size: ButtonSize,
    pub loading_placement: ButtonLoadingPlacement,
    pub is_icon_only: bool,
    pub has_start_content: bool,
    pub has_end_content: bool,
    pub has_custom_motion: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ButtonState {
    pub is_disabled: bool,
    pub is_loading: bool,
    pub variant: ButtonVariant,
    pub color: ButtonColor,
    pub color_attr: &'static str,
    pub radius: ButtonRadius,
    pub radius_attr: &'static str,
    pub size: ButtonSize,
    pub loading_placement: ButtonLoadingPlacement,
    pub loading_placement_attr: &'static str,
    pub is_icon_only: bool,
    pub is_full_width: bool,
    pub has_start_content: bool,
    pub has_end_content: bool,
    pub has_custom_class_name: bool,
    pub has_custom_motion: bool,
    pub state_attr: &'static str,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ButtonRenderState {
    pub show_start_inline_spinner: bool,
    pub show_start_overlay_spinner: bool,
    pub show_end_spinner: bool,
    pub show_center_spinner: bool,
    pub start_loading_attr: Option<&'static str>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ButtonViewState {
    pub state: ButtonState,
    pub source: ButtonStateSource,
    pub class_name: String,
    pub render: ButtonRenderState,
}

pub fn resolve_state(input: ButtonStateInput) -> ButtonState {
    let core = resolve_state_core(ButtonStateCoreInput {
        is_disabled: input.is_disabled,
        is_loading: input.is_loading,
        is_icon_only: input.is_icon_only,
        is_full_width: input.is_full_width,
        has_start_content: input.has_start_content,
        has_end_content: input.has_end_content,
        has_custom_class_name: input.has_custom_class_name,
        has_custom_motion: input.has_custom_motion,
    });

    ButtonState {
        is_disabled: core.is_disabled,
        is_loading: core.is_loading,
        variant: input.variant,
        color: input.color,
        color_attr: input.color.as_attr(),
        radius: input.radius,
        radius_attr: input.radius.as_attr(),
        size: input.size,
        loading_placement: input.loading_placement,
        loading_placement_attr: input.loading_placement.as_attr(),
        is_icon_only: core.is_icon_only,
        is_full_width: core.is_full_width,
        has_start_content: core.has_start_content,
        has_end_content: core.has_end_content,
        has_custom_class_name: core.has_custom_class_name,
        has_custom_motion: core.has_custom_motion,
        state_attr: core.state_attr,
    }
}

pub fn derive_render_state(state: ButtonState) -> ButtonRenderState {
    let is_start_loading =
        state.is_loading && matches!(state.loading_placement, ButtonLoadingPlacement::Start);

    ButtonRenderState {
        show_start_inline_spinner: is_start_loading && !state.has_start_content,
        show_start_overlay_spinner: is_start_loading && state.has_start_content,
        show_end_spinner: state.is_loading
            && matches!(state.loading_placement, ButtonLoadingPlacement::End),
        show_center_spinner: state.is_loading
            && matches!(state.loading_placement, ButtonLoadingPlacement::Center),
        start_loading_attr: (is_start_loading && state.has_start_content).then_some("true"),
    }
}

pub fn resolve_view_state(input: ButtonLogicInput) -> ButtonViewState {
    let normalized = input.normalized;
    let state = resolve_state(ButtonStateInput {
        is_disabled: normalized.is_disabled,
        is_loading: input.is_loading,
        variant: input.variant,
        color: input.color,
        radius: input.radius,
        size: input.size,
        loading_placement: input.loading_placement,
        is_icon_only: input.is_icon_only,
        is_full_width: normalized.is_full_width,
        has_start_content: input.has_start_content,
        has_end_content: input.has_end_content,
        has_custom_class_name: normalized.has_custom_class_name,
        has_custom_motion: input.has_custom_motion,
    });
    let source = ButtonStateSource {
        disabled_source_attr: if state.is_loading {
            "loading"
        } else if normalized.is_disabled {
            "prop"
        } else {
            "default"
        },
        loading_source_attr: if state.is_loading { "prop" } else { "default" },
        disabled_input_source_attr: normalized.disabled_input_source.as_attr(),
        full_width_input_source_attr: normalized.full_width_input_source.as_attr(),
    };
    let class_name = compose_class_name(normalized.class_name, state);
    let render = derive_render_state(state);

    ButtonViewState {
        state,
        source,
        class_name,
        render,
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: ButtonState) -> String {
    let mut classes = vec![
        "ui-button".to_string(),
        state.variant.class_name().into(),
        state.color.class_name().into(),
        state.radius.class_name().into(),
        state.size.class_name().into(),
        format!("ui-button--loading-{}", state.loading_placement_attr),
    ];

    if state.is_icon_only {
        classes.push("ui-button--icon-only".to_string());
    }
    if state.is_full_width {
        classes.push("ui-button--full-width".to_string());
    }
    if state.is_loading {
        classes.push("ui-button--loading".to_string());
    }
    if state.has_start_content {
        classes.push("ui-button--has-start".to_string());
    }
    if state.has_end_content {
        classes.push("ui-button--has-end".to_string());
    }
    if state.has_custom_motion {
        classes.push("ui-button--custom-motion".to_string());
    }

    if state.has_custom_class_name
        && let Some(base_class_name) = base_class_name
    {
        classes.push(base_class_name);
    }

    classes.join(" ")
}

#[cfg(feature = "component-button_group")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ButtonGroupOrientation {
    #[default]
    Horizontal,
    Vertical,
}

#[cfg(feature = "component-button_group")]
impl ButtonGroupOrientation {
    pub fn class_name(self) -> &'static str {
        match self {
            ButtonGroupOrientation::Horizontal => "ui-button-group--horizontal",
            ButtonGroupOrientation::Vertical => "ui-button-group--vertical",
        }
    }

    pub fn data_orientation(self) -> &'static str {
        match self {
            ButtonGroupOrientation::Horizontal => "horizontal",
            ButtonGroupOrientation::Vertical => "vertical",
        }
    }
}

#[cfg(feature = "component-button_group")]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ButtonGroupState {
    pub is_horizontal: bool,
    pub is_vertical: bool,
    pub is_attached: bool,
    pub is_detached: bool,
    pub has_explicit_label: bool,
    pub has_fallback_label: bool,
}

#[cfg(feature = "component-button_group")]
pub fn normalize_button_group_aria_label(aria_label: Option<String>) -> (String, bool) {
    if let Some(label) = aria_label {
        let trimmed = label.trim();
        if !trimmed.is_empty() {
            return (trimmed.into(), true);
        }
    }

    ("Button group".to_string(), false)
}

#[cfg(feature = "component-button_group")]
pub fn resolve_button_group_state(
    orientation: ButtonGroupOrientation,
    attached: bool,
    has_explicit_label: bool,
) -> ButtonGroupState {
    ButtonGroupState {
        is_horizontal: matches!(orientation, ButtonGroupOrientation::Horizontal),
        is_vertical: matches!(orientation, ButtonGroupOrientation::Vertical),
        is_attached: attached,
        is_detached: !attached,
        has_explicit_label,
        has_fallback_label: !has_explicit_label,
    }
}

#[cfg(feature = "component-button_group")]
pub fn compose_button_group_class_name(
    base_class_name: Option<String>,
    orientation: ButtonGroupOrientation,
    is_attached: bool,
) -> String {
    let mut classes = vec![
        "ui-button-group".to_string(),
        orientation.class_name().to_string(),
    ];

    if is_attached {
        classes.push("ui-button-group--attached".to_string());
    }

    if let Some(base_class_name) = base_class_name {
        classes.push(base_class_name);
    }

    classes.join(" ")
}

#[cfg(test)]
#[path = "../test/logic.rs"]
mod tests;
