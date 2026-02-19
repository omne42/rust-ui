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
    pub state: ButtonAgentStateAxis,
    pub capabilities: ButtonAgentCapabilities,
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
        schema_name: "ui.button.agent-contract",
        schema_version: ButtonAgentSchemaVersion::V1,
        intent: ButtonAgentIntent::Trigger,
        state,
        capabilities: resolve_agent_capabilities_for_state_axis(state, has_popup_trigger),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn variant_class_names_are_stable() {
        assert_eq!(
            ButtonVariant::Default.class_name(),
            "ui-button--variant-default"
        );
        assert_eq!(
            ButtonVariant::Solid.class_name(),
            "ui-button--variant-solid"
        );
        assert_eq!(
            ButtonVariant::Faded.class_name(),
            "ui-button--variant-faded"
        );
        assert_eq!(
            ButtonVariant::Bordered.class_name(),
            "ui-button--variant-bordered"
        );
        assert_eq!(
            ButtonVariant::Light.class_name(),
            "ui-button--variant-light"
        );
        assert_eq!(ButtonVariant::Flat.class_name(), "ui-button--variant-flat");
        assert_eq!(
            ButtonVariant::Shadow.class_name(),
            "ui-button--variant-shadow"
        );
        assert_eq!(
            ButtonVariant::Accent.class_name(),
            "ui-button--variant-accent"
        );
        assert_eq!(
            ButtonVariant::Destructive.class_name(),
            "ui-button--variant-destructive"
        );
        assert_eq!(
            ButtonVariant::Outline.class_name(),
            "ui-button--variant-outline"
        );
        assert_eq!(
            ButtonVariant::Secondary.class_name(),
            "ui-button--variant-secondary"
        );
        assert_eq!(
            ButtonVariant::Ghost.class_name(),
            "ui-button--variant-ghost"
        );
        assert_eq!(ButtonVariant::Link.class_name(), "ui-button--variant-link");
    }

    #[test]
    fn color_class_and_attr_names_are_stable() {
        assert_eq!(
            ButtonColor::Default.class_name(),
            "ui-button--color-default"
        );
        assert_eq!(
            ButtonColor::Primary.class_name(),
            "ui-button--color-primary"
        );
        assert_eq!(
            ButtonColor::Secondary.class_name(),
            "ui-button--color-secondary"
        );
        assert_eq!(
            ButtonColor::Success.class_name(),
            "ui-button--color-success"
        );
        assert_eq!(
            ButtonColor::Warning.class_name(),
            "ui-button--color-warning"
        );
        assert_eq!(ButtonColor::Danger.class_name(), "ui-button--color-danger");

        assert_eq!(ButtonColor::Default.as_attr(), "default");
        assert_eq!(ButtonColor::Primary.as_attr(), "primary");
        assert_eq!(ButtonColor::Secondary.as_attr(), "secondary");
        assert_eq!(ButtonColor::Success.as_attr(), "success");
        assert_eq!(ButtonColor::Warning.as_attr(), "warning");
        assert_eq!(ButtonColor::Danger.as_attr(), "danger");
    }

    #[test]
    fn radius_class_and_attr_names_are_stable() {
        assert_eq!(ButtonRadius::None.class_name(), "ui-button--radius-none");
        assert_eq!(ButtonRadius::Sm.class_name(), "ui-button--radius-sm");
        assert_eq!(ButtonRadius::Md.class_name(), "ui-button--radius-md");
        assert_eq!(ButtonRadius::Lg.class_name(), "ui-button--radius-lg");
        assert_eq!(ButtonRadius::Full.class_name(), "ui-button--radius-full");

        assert_eq!(ButtonRadius::None.as_attr(), "none");
        assert_eq!(ButtonRadius::Sm.as_attr(), "sm");
        assert_eq!(ButtonRadius::Md.as_attr(), "md");
        assert_eq!(ButtonRadius::Lg.as_attr(), "lg");
        assert_eq!(ButtonRadius::Full.as_attr(), "full");
    }

    #[test]
    fn string_conversions_cover_aligned_parameter_tokens() {
        assert_eq!(ButtonVariant::from("solid"), ButtonVariant::Solid);
        assert_eq!(ButtonVariant::from("faded"), ButtonVariant::Faded);
        assert_eq!(ButtonVariant::from("bordered"), ButtonVariant::Bordered);
        assert_eq!(ButtonVariant::from("light"), ButtonVariant::Light);
        assert_eq!(ButtonVariant::from("flat"), ButtonVariant::Flat);
        assert_eq!(ButtonVariant::from("ghost"), ButtonVariant::Ghost);
        assert_eq!(ButtonVariant::from("shadow"), ButtonVariant::Shadow);

        assert_eq!(ButtonColor::from("primary"), ButtonColor::Primary);
        assert_eq!(ButtonColor::from("danger"), ButtonColor::Danger);
        assert_eq!(ButtonRadius::from("full"), ButtonRadius::Full);
        assert_eq!(ButtonRadius::from("none"), ButtonRadius::None);
    }

    #[test]
    fn size_class_names_are_stable() {
        assert_eq!(ButtonSize::Xs.class_name(), "ui-button--size-xs");
        assert_eq!(ButtonSize::S.class_name(), "ui-button--size-s");
        assert_eq!(ButtonSize::M.class_name(), "ui-button--size-m");
        assert_eq!(ButtonSize::L.class_name(), "ui-button--size-l");
        assert_eq!(ButtonSize::Xl.class_name(), "ui-button--size-xl");
        assert_eq!(ButtonSize::IconXs.class_name(), "ui-button--size-icon-xs");
        assert_eq!(ButtonSize::IconS.class_name(), "ui-button--size-icon-s");
        assert_eq!(ButtonSize::IconM.class_name(), "ui-button--size-icon-m");
        assert_eq!(ButtonSize::IconL.class_name(), "ui-button--size-icon-l");
        assert_eq!(ButtonSize::IconXl.class_name(), "ui-button--size-icon-xl");

        assert_eq!(ButtonSize::Default.class_name(), "ui-button--size-m");
        assert_eq!(ButtonSize::Sm.class_name(), "ui-button--size-s");
        assert_eq!(ButtonSize::Lg.class_name(), "ui-button--size-l");
        assert_eq!(ButtonSize::Icon.class_name(), "ui-button--size-icon-m");
        assert_eq!(ButtonSize::IconSm.class_name(), "ui-button--size-icon-s");
        assert_eq!(ButtonSize::IconLg.class_name(), "ui-button--size-icon-l");
    }

    #[test]
    fn size_string_conversions_cover_xs_to_xl_contract() {
        assert_eq!(ButtonSize::from("xs"), ButtonSize::Xs);
        assert_eq!(ButtonSize::from("s"), ButtonSize::S);
        assert_eq!(ButtonSize::from("m"), ButtonSize::M);
        assert_eq!(ButtonSize::from("l"), ButtonSize::L);
        assert_eq!(ButtonSize::from("xl"), ButtonSize::Xl);
    }

    #[test]
    fn loading_placement_attrs_match_variants() {
        assert_eq!(ButtonLoadingPlacement::Start.as_attr(), "start");
        assert_eq!(ButtonLoadingPlacement::End.as_attr(), "end");
        assert_eq!(ButtonLoadingPlacement::Center.as_attr(), "center");
    }

    #[test]
    fn button_type_attrs_and_string_conversions_are_stable() {
        assert_eq!(ButtonType::Button.as_attr(), "button");
        assert_eq!(ButtonType::Submit.as_attr(), "submit");
        assert_eq!(ButtonType::Reset.as_attr(), "reset");

        assert_eq!(ButtonType::from("button"), ButtonType::Button);
        assert_eq!(ButtonType::from("submit"), ButtonType::Submit);
        assert_eq!(ButtonType::from("reset"), ButtonType::Reset);
    }

    #[test]
    fn boolean_input_source_attrs_are_stable() {
        assert_eq!(ButtonBooleanInputSource::IsProp.as_attr(), "is-prop");
        assert_eq!(ButtonBooleanInputSource::Default.as_attr(), "default");
    }

    #[test]
    fn label_source_attrs_are_stable() {
        assert_eq!(
            ui_state_primitives::button::ButtonLabelSource::Explicit.as_attr(),
            "explicit"
        );
        assert_eq!(
            ui_state_primitives::button::ButtonLabelSource::Fallback.as_attr(),
            "fallback"
        );
        assert_eq!(
            ui_state_primitives::button::ButtonLabelSource::None.as_attr(),
            "none"
        );
    }

    #[test]
    fn normalize_optional_text_trims_and_filters_blank_values() {
        assert_eq!(normalize_optional_text(None), None);
        assert_eq!(normalize_optional_text(Some("  \n\t".to_string())), None);
        assert_eq!(
            normalize_optional_text(Some("  Button  ".to_string())),
            Some("Button".to_string())
        );
    }

    #[test]
    fn resolve_aria_label_prefers_explicit_then_fallback() {
        assert_eq!(
            resolve_aria_label(Some(" Save ".to_string()), true, None),
            (
                Some("Save".to_string()),
                ui_state_primitives::button::ButtonLabelSource::Explicit
            )
        );
        assert_eq!(
            resolve_aria_label(None, true, None),
            (
                Some(ui_state_primitives::button::BUTTON_ICON_ONLY_FALLBACK_ARIA_LABEL.to_string()),
                ui_state_primitives::button::ButtonLabelSource::Fallback,
            )
        );
        assert_eq!(
            resolve_aria_label(None, false, None),
            (None, ui_state_primitives::button::ButtonLabelSource::None)
        );
    }

    #[test]
    fn resolve_state_tracks_visual_markers() {
        let state = resolve_state(ButtonStateInput {
            is_disabled: false,
            is_loading: true,
            variant: ButtonVariant::Secondary,
            color: ButtonColor::Success,
            radius: ButtonRadius::Full,
            size: ButtonSize::Icon,
            loading_placement: ButtonLoadingPlacement::End,
            is_icon_only: true,
            is_full_width: true,
            has_start_content: true,
            has_end_content: false,
            has_custom_class_name: true,
            has_custom_motion: true,
        });

        assert!(state.is_disabled);
        assert_eq!(state.state_attr, "loading");
        assert_eq!(state.color_attr, "success");
        assert_eq!(state.radius_attr, "full");
        assert_eq!(state.loading_placement_attr, "end");
        assert!(state.is_icon_only);
        assert!(state.is_full_width);
        assert!(state.has_start_content);
        assert!(!state.has_end_content);
        assert!(state.has_custom_class_name);
        assert!(state.has_custom_motion);
    }

    #[test]
    fn resolve_agent_contract_exposes_machine_readable_capabilities() {
        let interactive = resolve_state(ButtonStateInput {
            is_disabled: false,
            is_loading: false,
            variant: ButtonVariant::Default,
            color: ButtonColor::Primary,
            radius: ButtonRadius::Md,
            size: ButtonSize::M,
            loading_placement: ButtonLoadingPlacement::Start,
            is_icon_only: false,
            is_full_width: false,
            has_start_content: false,
            has_end_content: false,
            has_custom_class_name: false,
            has_custom_motion: false,
        });
        let contract = resolve_agent_contract(interactive, true);
        assert_eq!(contract.schema_name, "ui.button.agent-contract");
        assert_eq!(contract.schema_version.as_str(), "1");
        assert_eq!(contract.intent.as_str(), "trigger");
        assert_eq!(contract.state.as_str(), "ready");
        assert!(contract.capabilities.can_press);
        assert!(contract.capabilities.can_focus);
        assert!(contract.capabilities.can_hover);
        assert!(contract.capabilities.can_popup_trigger);

        let disabled = resolve_state(ButtonStateInput {
            is_disabled: true,
            ..ButtonStateInput {
                is_disabled: false,
                is_loading: false,
                variant: ButtonVariant::Default,
                color: ButtonColor::Primary,
                radius: ButtonRadius::Md,
                size: ButtonSize::M,
                loading_placement: ButtonLoadingPlacement::Start,
                is_icon_only: false,
                is_full_width: false,
                has_start_content: false,
                has_end_content: false,
                has_custom_class_name: false,
                has_custom_motion: false,
            }
        });
        let contract = resolve_agent_contract(disabled, false);
        assert_eq!(contract.state.as_str(), "disabled");
        assert!(!contract.capabilities.can_press);
        assert!(!contract.capabilities.can_focus);
        assert!(!contract.capabilities.can_hover);
        assert!(!contract.capabilities.can_popup_trigger);
    }

    #[test]
    fn normalize_input_prefers_is_prefix_aliases_and_applies_defaults() {
        let normalized = normalize_input(ButtonInputNormalizationInput {
            is_disabled: true,
            is_full_width: false,
            class_name: Some("  docs-btn  ".to_string()),
            aria_label: None,
            icon_only_fallback_aria_label: None,
            is_icon_only: true,
            button_type: ButtonType::default(),
        });

        assert!(normalized.is_disabled);
        assert!(!normalized.is_full_width);
        assert_eq!(
            normalized.disabled_input_source,
            ButtonBooleanInputSource::IsProp
        );
        assert_eq!(
            normalized.full_width_input_source,
            ButtonBooleanInputSource::Default
        );
        assert_eq!(normalized.class_name, Some("docs-btn".to_string()));
        assert!(normalized.has_custom_class_name);
        assert_eq!(normalized.button_type, ButtonType::Button);
        assert_eq!(
            normalized.aria_label,
            Some(ui_state_primitives::button::BUTTON_ICON_ONLY_FALLBACK_ARIA_LABEL.to_string())
        );
        assert_eq!(normalized.aria_label_source, ButtonLabelSource::Fallback);
    }

    #[test]
    fn normalize_input_uses_is_flags_without_legacy_aliases() {
        let normalized = normalize_input(ButtonInputNormalizationInput {
            is_disabled: false,
            is_full_width: true,
            class_name: Some("   ".to_string()),
            aria_label: Some("  Save  ".to_string()),
            icon_only_fallback_aria_label: None,
            is_icon_only: false,
            button_type: ButtonType::Submit,
        });

        assert!(!normalized.is_disabled);
        assert!(normalized.is_full_width);
        assert_eq!(
            normalized.disabled_input_source,
            ButtonBooleanInputSource::Default
        );
        assert_eq!(
            normalized.full_width_input_source,
            ButtonBooleanInputSource::IsProp
        );
        assert_eq!(normalized.class_name, None);
        assert!(!normalized.has_custom_class_name);
        assert_eq!(normalized.aria_label, Some("Save".to_string()));
        assert_eq!(normalized.aria_label_source, ButtonLabelSource::Explicit);
        assert_eq!(normalized.button_type, ButtonType::Submit);
    }

    #[test]
    fn loading_forces_disabled() {
        assert!(
            !resolve_state(ButtonStateInput {
                is_disabled: false,
                is_loading: false,
                variant: ButtonVariant::Default,
                color: ButtonColor::Primary,
                radius: ButtonRadius::Md,
                size: ButtonSize::M,
                loading_placement: ButtonLoadingPlacement::Start,
                is_icon_only: false,
                is_full_width: false,
                has_start_content: false,
                has_end_content: false,
                has_custom_class_name: false,
                has_custom_motion: false,
            })
            .is_disabled
        );

        assert!(
            resolve_state(ButtonStateInput {
                is_disabled: false,
                is_loading: true,
                variant: ButtonVariant::Default,
                color: ButtonColor::Primary,
                radius: ButtonRadius::Md,
                size: ButtonSize::M,
                loading_placement: ButtonLoadingPlacement::Start,
                is_icon_only: false,
                is_full_width: false,
                has_start_content: false,
                has_end_content: false,
                has_custom_class_name: false,
                has_custom_motion: false,
            })
            .is_disabled
        );

        assert!(
            resolve_state(ButtonStateInput {
                is_disabled: true,
                is_loading: false,
                variant: ButtonVariant::Default,
                color: ButtonColor::Primary,
                radius: ButtonRadius::Md,
                size: ButtonSize::M,
                loading_placement: ButtonLoadingPlacement::Start,
                is_icon_only: false,
                is_full_width: false,
                has_start_content: false,
                has_end_content: false,
                has_custom_class_name: false,
                has_custom_motion: false,
            })
            .is_disabled
        );
    }

    #[test]
    fn compose_class_name_includes_state_markers() {
        let class_name = compose_class_name(
            Some("docs-button".to_string()),
            resolve_state(ButtonStateInput {
                is_disabled: false,
                is_loading: true,
                variant: ButtonVariant::Outline,
                color: ButtonColor::Danger,
                radius: ButtonRadius::Sm,
                size: ButtonSize::S,
                loading_placement: ButtonLoadingPlacement::Center,
                is_icon_only: true,
                is_full_width: true,
                has_start_content: true,
                has_end_content: true,
                has_custom_class_name: true,
                has_custom_motion: true,
            }),
        );

        for needle in [
            "ui-button",
            "ui-button--variant-outline",
            "ui-button--color-danger",
            "ui-button--radius-sm",
            "ui-button--size-s",
            "ui-button--loading-center",
            "ui-button--icon-only",
            "ui-button--full-width",
            "ui-button--loading",
            "ui-button--has-start",
            "ui-button--has-end",
            "ui-button--custom-motion",
            "docs-button",
        ] {
            assert!(
                class_name.contains(needle),
                "composed class name should contain `{needle}`"
            );
        }
    }

    #[test]
    fn derive_render_state_maps_loading_placement_to_spinner_slots() {
        let start_with_slot = derive_render_state(resolve_state(ButtonStateInput {
            is_disabled: false,
            is_loading: true,
            variant: ButtonVariant::Default,
            color: ButtonColor::Primary,
            radius: ButtonRadius::Md,
            size: ButtonSize::M,
            loading_placement: ButtonLoadingPlacement::Start,
            is_icon_only: false,
            is_full_width: false,
            has_start_content: true,
            has_end_content: false,
            has_custom_class_name: false,
            has_custom_motion: false,
        }));
        assert!(!start_with_slot.show_start_inline_spinner);
        assert!(start_with_slot.show_start_overlay_spinner);
        assert_eq!(start_with_slot.start_loading_attr, Some("true"));

        let end = derive_render_state(resolve_state(ButtonStateInput {
            is_disabled: false,
            is_loading: true,
            variant: ButtonVariant::Default,
            color: ButtonColor::Primary,
            radius: ButtonRadius::Md,
            size: ButtonSize::M,
            loading_placement: ButtonLoadingPlacement::End,
            is_icon_only: false,
            is_full_width: false,
            has_start_content: false,
            has_end_content: false,
            has_custom_class_name: false,
            has_custom_motion: false,
        }));
        assert!(end.show_end_spinner);
        assert!(!end.show_start_inline_spinner);
        assert!(!end.show_center_spinner);
    }

    #[test]
    fn resolve_view_state_centralizes_state_and_class_derivation() {
        let view_state = resolve_view_state(ButtonLogicInput {
            normalized: normalize_input(ButtonInputNormalizationInput {
                is_disabled: true,
                is_full_width: true,
                class_name: Some("  docs-btn  ".to_string()),
                aria_label: None,
                icon_only_fallback_aria_label: None,
                is_icon_only: false,
                button_type: ButtonType::Button,
            }),
            is_loading: true,
            variant: ButtonVariant::Outline,
            color: ButtonColor::Danger,
            radius: ButtonRadius::Sm,
            size: ButtonSize::S,
            loading_placement: ButtonLoadingPlacement::Center,
            is_icon_only: true,
            has_start_content: false,
            has_end_content: true,
            has_custom_motion: true,
        });

        assert!(view_state.state.is_disabled);
        assert!(view_state.state.is_full_width);
        assert!(view_state.state.has_end_content);
        assert!(view_state.state.has_custom_motion);
        assert_eq!(view_state.source.disabled_source_attr, "loading");
        assert_eq!(view_state.source.loading_source_attr, "prop");
        assert_eq!(view_state.source.disabled_input_source_attr, "is-prop");
        assert_eq!(view_state.source.full_width_input_source_attr, "is-prop");
        assert!(view_state.class_name.contains("ui-button--variant-outline"));
        assert!(view_state.class_name.contains("docs-btn"));
        assert!(view_state.render.show_center_spinner);
    }

    #[cfg(feature = "component-button_group")]
    #[test]
    fn button_group_orientation_class_and_data_values_are_stable() {
        assert_eq!(
            ButtonGroupOrientation::Horizontal.class_name(),
            "ui-button-group--horizontal"
        );
        assert_eq!(
            ButtonGroupOrientation::Vertical.class_name(),
            "ui-button-group--vertical"
        );
        assert_eq!(
            ButtonGroupOrientation::Horizontal.data_orientation(),
            "horizontal"
        );
        assert_eq!(
            ButtonGroupOrientation::Vertical.data_orientation(),
            "vertical"
        );
    }

    #[cfg(feature = "component-button_group")]
    #[test]
    fn button_group_aria_label_uses_trimmed_label_or_fallback() {
        let (label, explicit) =
            normalize_button_group_aria_label(Some("  Text align  ".to_string()));
        assert_eq!(label, "Text align");
        assert!(explicit);

        let (label, explicit) = normalize_button_group_aria_label(Some("   ".to_string()));
        assert_eq!(label, "Button group");
        assert!(!explicit);

        let (label, explicit) = normalize_button_group_aria_label(None);
        assert_eq!(label, "Button group");
        assert!(!explicit);
    }

    #[cfg(feature = "component-button_group")]
    #[test]
    fn resolve_button_group_state_tracks_orientation_attachment_and_label_source() {
        let state = resolve_button_group_state(ButtonGroupOrientation::Vertical, true, true);

        assert!(!state.is_horizontal);
        assert!(state.is_vertical);
        assert!(state.is_attached);
        assert!(!state.is_detached);
        assert!(state.has_explicit_label);
        assert!(!state.has_fallback_label);

        let state = resolve_button_group_state(ButtonGroupOrientation::Horizontal, false, false);

        assert!(state.is_horizontal);
        assert!(!state.is_vertical);
        assert!(!state.is_attached);
        assert!(state.is_detached);
        assert!(!state.has_explicit_label);
        assert!(state.has_fallback_label);
    }
}
