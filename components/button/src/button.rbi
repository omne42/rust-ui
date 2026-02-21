pub enum ButtonVariant {
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

pub enum ButtonColor {
    Default,
    Primary,
    Secondary,
    Success,
    Warning,
    Danger,
}

pub enum ButtonRadius {
    None,
    Sm,
    Md,
    Lg,
    Full,
}

pub enum ButtonSize {
    Xs,
    S,
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

pub enum ButtonLoadingPlacement {
    Start,
    End,
    Center,
}

pub enum ButtonType {
    Button,
    Submit,
    Reset,
}

pub struct ButtonMotion {
    pub spring: ui_motion::spring::SpringConfig,
    pub hover_scale: f64,
    pub tap_scale: f64,
}

pub enum ButtonIntent {
    Primary,
    Accent,
    Destructive,
    Outline,
    Secondary,
    Ghost,
    Link,
}

pub struct ButtonSchema {
    pub schema_version: u16,
    pub element_id: String,
    pub intent: ButtonIntent,
    pub action_signature: String,
    pub requires_confirmation: bool,
}

pub struct ButtonSchemaError {
    pub code: &'static str,
    pub message: String,
    pub schema_version: Option<u16>,
    pub supported_schema_version: u16,
}

impl ButtonSchema {
    pub fn to_json_result(&self) -> Result<String, ButtonSchemaError>;
    pub fn from_json(raw: &str) -> Result<Self, ButtonSchemaError>;
}

pub struct ButtonSpec;

impl ButtonSpec {
    pub fn new() -> Self;
    pub fn intent(self, value: ButtonIntent) -> Self;
    pub fn size(self, value: ButtonSize) -> Self;
    pub fn motion(self, value: ButtonMotion) -> Self;
    pub fn schema(self, value: ButtonSchema) -> Self;
    pub fn render(self) -> impl leptos::prelude::IntoView;
}

pub const BUTTON_AGENT_SCHEMA: &str;

pub enum ButtonAgentSchemaVersion {
    V1,
}

impl ButtonAgentSchemaVersion {
    pub const fn as_str(self) -> &'static str;
}

pub enum ButtonAgentIntent {
    Trigger,
}

impl ButtonAgentIntent {
    pub const fn as_str(self) -> &'static str;
}

pub enum ButtonAgentAction {
    Press,
}

impl ButtonAgentAction {
    pub const fn as_str(self) -> &'static str;
}

pub enum ButtonAgentStateAxis {
    Disabled,
    Loading,
    Ready,
}

impl ButtonAgentStateAxis {
    pub const fn as_str(self) -> &'static str;
}

pub enum ButtonAgentSource {
    StatePrimitives,
}

impl ButtonAgentSource {
    pub const fn as_str(self) -> &'static str;
}

pub struct ButtonAgentCapabilities {
    pub can_press: bool,
    pub can_focus: bool,
    pub can_hover: bool,
    pub can_popup_trigger: bool,
}

pub struct ButtonAgentContract {
    pub schema_name: &'static str,
    pub schema_version: ButtonAgentSchemaVersion,
    pub intent: ButtonAgentIntent,
    pub action: ButtonAgentAction,
    pub state: ButtonAgentStateAxis,
    pub source: ButtonAgentSource,
    pub capabilities: ButtonAgentCapabilities,
}

pub enum ButtonSchemaInputSource {
    Missing,
    PropValidated,
    PropRejected,
}

impl ButtonSchemaInputSource {
    pub const fn as_attr(self) -> &'static str;
}

pub struct ButtonSchemaInputNormalization {
    pub schema_json: Option<String>,
    pub source: ButtonSchemaInputSource,
}

pub fn normalize_schema_json_input(
    schema_json: Option<String>,
) -> ButtonSchemaInputNormalization;

pub fn Button(
    id: Option<String>,
    is_disabled: bool,
    is_loading: bool,
    variant: ButtonVariant,
    color: ButtonColor,
    radius: ButtonRadius,
    size: ButtonSize,
    is_full_width: bool,
    motion: ButtonMotion,
    loading_placement: ButtonLoadingPlacement,
    class_name: Option<String>,
    schema_json: Option<String>,
    button_type: ButtonType,
    aria_label: Option<String>,
    lang: Option<String>,
    dir: Option<ui_headless::A11yDirection>,
    on_press: Option<ui_headless::OnPress>,
    children: leptos::children::Children,
) -> impl leptos::prelude::IntoView;
