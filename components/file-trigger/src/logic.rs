#[cfg(test)]
pub use ui_state_primitives::file_trigger::{
    FileTriggerProps, FileTriggerSelectionMode, FileTriggerStateInput, resolve_props, resolve_state,
};
pub use ui_state_primitives::file_trigger::{
    FileTriggerPropsInput, FileTriggerRenderState, FileTriggerRenderStateInput, FileTriggerState,
    resolve_render_state,
};

pub fn compose_class_name(state: FileTriggerState) -> String {
    let mut classes = Vec::with_capacity(3);
    classes.push("ui-file-trigger");

    if state.is_disabled {
        classes.push("ui-file-trigger--disabled");
    }

    if state.has_custom_motion {
        classes.push("ui-file-trigger--custom-motion");
    }

    classes.join(" ")
}

pub fn compose_class_name_from_render_state(render_state: FileTriggerRenderState) -> String {
    compose_class_name(render_state.state)
}

pub const FILE_TRIGGER_COMPONENT_SCHEMA_NAME: &str = "ui.file_trigger";
pub const FILE_TRIGGER_COMPONENT_SCHEMA_VERSION: &str = "1";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FileTriggerAgentIntent {
    FilePick,
}

impl FileTriggerAgentIntent {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::FilePick => "file-pick",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FileTriggerAgentAction {
    RenderSnapshot,
}

impl FileTriggerAgentAction {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::RenderSnapshot => "render-snapshot",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FileTriggerAgentSource {
    Default,
    Custom,
}

impl FileTriggerAgentSource {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Custom => "custom",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FileTriggerAgentStreamSupport {
    Optional,
}

impl FileTriggerAgentStreamSupport {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::Optional => "optional",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FileTriggerAgentStreamFallback {
    Snapshot,
}

impl FileTriggerAgentStreamFallback {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::Snapshot => "snapshot",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FileTriggerAgentOutputStatus {
    Verified,
}

impl FileTriggerAgentOutputStatus {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::Verified => "verified",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FileTriggerAgentContract {
    pub schema_name: &'static str,
    pub schema_version: &'static str,
    pub intent: FileTriggerAgentIntent,
    pub action: FileTriggerAgentAction,
    pub state: &'static str,
    pub source: FileTriggerAgentSource,
    pub stream_support: FileTriggerAgentStreamSupport,
    pub stream_fallback: FileTriggerAgentStreamFallback,
    pub output_status: FileTriggerAgentOutputStatus,
}

pub fn resolve_agent_contract(state: FileTriggerState) -> FileTriggerAgentContract {
    FileTriggerAgentContract {
        schema_name: FILE_TRIGGER_COMPONENT_SCHEMA_NAME,
        schema_version: FILE_TRIGGER_COMPONENT_SCHEMA_VERSION,
        intent: FileTriggerAgentIntent::FilePick,
        action: FileTriggerAgentAction::RenderSnapshot,
        state: state.state_attr,
        source: if state.has_custom_motion {
            FileTriggerAgentSource::Custom
        } else {
            FileTriggerAgentSource::Default
        },
        stream_support: FileTriggerAgentStreamSupport::Optional,
        stream_fallback: FileTriggerAgentStreamFallback::Snapshot,
        output_status: FileTriggerAgentOutputStatus::Verified,
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FileTriggerFile {
    pub name: String,
    pub size: u64,
    pub mime: String,
}

#[cfg(target_arch = "wasm32")]
pub(crate) fn collect_files_from_input(
    input: &leptos::web_sys::HtmlInputElement,
) -> Vec<FileTriggerFile> {
    let Some(files) = input.files() else {
        return Vec::new();
    };

    let mut out = Vec::new();
    for idx in 0..files.length() {
        let Some(file) = files.get(idx) else {
            continue;
        };
        out.push(FileTriggerFile {
            name: file.name(),
            size: file.size().max(0.0) as u64,
            mime: file.type_(),
        });
    }
    out
}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn collect_files_from_input(
    _input: &leptos::web_sys::HtmlInputElement,
) -> Vec<FileTriggerFile> {
    Vec::new()
}

#[cfg(test)]
#[path = "../test/logic.rs"]
mod tests;
