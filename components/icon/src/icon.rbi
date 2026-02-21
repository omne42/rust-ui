pub type A11yDirection = ui_headless::A11yDirection;

pub type IconSize = crate::IconSize;
pub type IconTone = crate::IconTone;
pub type IconSlotKind = crate::IconSlotKind;
pub type IconStateInput = crate::IconStateInput;
pub type IconState = crate::IconState;

pub type IconsSet = crate::icons::IconsSet;
pub type IconsScale = crate::icons::IconsScale;
pub type IconsGlyph = crate::icons::IconsGlyph;
pub type IconsTone = crate::icons::IconsTone;
pub type IconsStateInput = crate::icons::IconsStateInput;
pub type IconsState = crate::icons::IconsState;

pub type IconsetSize = crate::iconset::IconsetSize;
pub type IconsetTone = crate::iconset::IconsetTone;
pub type IconsetGlyph = crate::iconset::IconsetGlyph;
pub type IconsetStateInput = crate::iconset::IconsetStateInput;
pub type IconsetState = crate::iconset::IconsetState;

pub type IconsUiSize = crate::icons_ui::IconsUiSize;
pub type IconsUiTone = crate::icons_ui::IconsUiTone;
pub type IconsUiStateInput = crate::icons_ui::IconsUiStateInput;
pub type IconsUiState = crate::icons_ui::IconsUiState;

pub type IconsWorkflowSize = crate::icons_workflow::IconsWorkflowSize;
pub type IconsWorkflowTone = crate::icons_workflow::IconsWorkflowTone;
pub type IconsWorkflowStateInput = crate::icons_workflow::IconsWorkflowStateInput;
pub type IconsWorkflowState = crate::icons_workflow::IconsWorkflowState;

pub const DEFAULT_ARIA_LABEL: &str;
pub const ICON_AGENT_SCHEMA: &str;

pub type IconAgentSchemaVersion = crate::protocol::IconAgentSchemaVersion;
pub type IconAgentIntent = crate::protocol::IconAgentIntent;
pub type IconAgentAction = crate::protocol::IconAgentAction;
pub type IconAgentState = crate::protocol::IconAgentState;
pub type IconAgentSource = crate::protocol::IconAgentSource;
pub type IconAgentInput = crate::protocol::IconAgentInput;
pub type IconAgentDataAttrs = crate::protocol::IconAgentDataAttrs;
pub type IconStreamingRequirement = crate::protocol::IconStreamingRequirement;
pub type IconOutputMode = crate::protocol::IconOutputMode;
pub type IconOutputStatus = crate::protocol::IconOutputStatus;
pub type IconOutputDataAttrs = crate::protocol::IconOutputDataAttrs;

pub fn resolve_agent_data_attrs(input: IconAgentInput) -> IconAgentDataAttrs;
pub fn resolve_output_data_attrs() -> IconOutputDataAttrs;

pub fn Icon(
    size: IconSize,
    tone: IconTone,
    is_disabled: bool,
    is_decorative: bool,
    aria_label: Option<String>,
    class_name: Option<String>,
    slot: Option<String>,
    lang: Option<String>,
    dir: Option<A11yDirection>,
    children: leptos::children::Children,
) -> impl leptos::prelude::IntoView;

pub fn Icons(
    name: String,
    set: IconsSet,
    scale: IconsScale,
    tone: IconsTone,
    is_disabled: bool,
    is_decorative: bool,
    aria_label: Option<String>,
    class_name: Option<String>,
    lang: Option<String>,
    dir: Option<A11yDirection>,
    glyphs: Vec<IconsGlyph>,
) -> impl leptos::prelude::IntoView;

pub fn Iconset(
    icon: String,
    iconset: Option<String>,
    glyphs: Vec<IconsetGlyph>,
    size: IconsetSize,
    tone: IconsetTone,
    is_disabled: bool,
    is_decorative: bool,
    aria_label: Option<String>,
    class_name: Option<String>,
    lang: Option<String>,
    dir: Option<A11yDirection>,
) -> impl leptos::prelude::IntoView;

pub fn IconsUi(
    icon: String,
    size: IconsUiSize,
    tone: IconsUiTone,
    is_disabled: bool,
    is_decorative: bool,
    aria_label: Option<String>,
    class_name: Option<String>,
    lang: Option<String>,
    dir: Option<A11yDirection>,
    glyphs: Vec<IconsetGlyph>,
) -> impl leptos::prelude::IntoView;

pub fn IconsWorkflow(
    icon: String,
    size: IconsWorkflowSize,
    tone: IconsWorkflowTone,
    is_disabled: bool,
    is_decorative: bool,
    aria_label: Option<String>,
    class_name: Option<String>,
    lang: Option<String>,
    dir: Option<A11yDirection>,
    glyphs: Vec<IconsetGlyph>,
) -> impl leptos::prelude::IntoView;
