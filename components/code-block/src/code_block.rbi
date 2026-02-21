pub struct CodeBlockStrings {
    pub copy_to_clipboard_aria_label: std::sync::Arc<str>,
    pub copied_status_text: std::sync::Arc<str>,
}

pub struct CodeBlockMotion {
    pub spring: ui_motion::spring::SpringConfig,
    pub flash_hold_ms: u64,
}

pub enum CodeBlockComponentSchemaVersion {
    V1,
}

pub struct CodeBlockComponentSpec {
    pub schema_version: CodeBlockComponentSchemaVersion,
}

pub const CODE_BLOCK_AGENT_SCHEMA: &str;

pub enum CodeBlockAgentSchema {
    V1,
}

pub enum CodeBlockAgentIntent {
    DisplayCode,
}

pub enum CodeBlockAgentAction {
    CopyCode,
}

pub enum CodeBlockAgentState {
    Idle,
    Copied,
    CopyLoading,
    CopyError,
}

pub enum CodeBlockAgentSource {
    Controlled,
    Uncontrolled,
}

pub enum CodeBlockAgentCopyableSource {
    Default,
    IsCopyable,
    CopyableLegacy,
}

pub enum CodeBlockAgentCopiedSource {
    Controlled,
    Uncontrolled,
}

pub enum CodeBlockAgentMotionSource {
    Default,
    Custom,
}

pub enum CodeBlockAgentOutputMode {
    Streaming,
    Snapshot,
}

pub enum CodeBlockAgentOutputStatus {
    Draft,
    Validated,
    ReadyToSubmit,
}

pub struct CodeBlockRenderPolicy {
    pub allow_inner_html: bool,
    pub allow_script_injection: bool,
    pub output_status: CodeBlockAgentOutputStatus,
}

pub struct CodeBlockAgentInput {
    pub copied: bool,
    pub is_loading: bool,
    pub has_error: bool,
    pub output_mode: CodeBlockAgentOutputMode,
    pub output_status: CodeBlockAgentOutputStatus,
    pub copyable_source: CodeBlockAgentCopyableSource,
    pub copied_source: CodeBlockAgentCopiedSource,
    pub motion_source: CodeBlockAgentMotionSource,
}

pub struct CodeBlockAgentDataAttrs {
    pub schema: CodeBlockAgentSchema,
    pub intent: CodeBlockAgentIntent,
    pub action: CodeBlockAgentAction,
    pub state: CodeBlockAgentState,
    pub source: CodeBlockAgentSource,
    pub source_copyable: CodeBlockAgentCopyableSource,
    pub source_copied: CodeBlockAgentCopiedSource,
    pub source_motion: CodeBlockAgentMotionSource,
    pub output_mode: CodeBlockAgentOutputMode,
    pub output_status: CodeBlockAgentOutputStatus,
}

pub fn resolve_agent_data_attrs(input: CodeBlockAgentInput) -> CodeBlockAgentDataAttrs;

pub fn render_policy() -> CodeBlockRenderPolicy;

pub fn CodeBlock(
    code: String,
    label: Option<String>,
    language: Option<String>,
    lang: Option<String>,
    dir: Option<ui_headless::a11y::A11yDirection>,
    is_copyable: Option<bool>,
    copyable: Option<bool>,
    is_copied: Option<leptos::prelude::Signal<bool>>,
    copied: Option<leptos::prelude::Signal<bool>>,
    default_copied: Option<bool>,
    on_copied_change: Option<leptos::prelude::Callback<bool>>,
    output_mode: Option<CodeBlockAgentOutputMode>,
    output_status: Option<CodeBlockAgentOutputStatus>,
    motion: CodeBlockMotion,
    class_name: Option<String>,
) -> impl leptos::prelude::IntoView;
