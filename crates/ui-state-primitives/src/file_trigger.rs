#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FileTriggerStateInput {
    pub disabled: bool,
    pub has_custom_motion: bool,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct FileTriggerPropsInput {
    pub is_disabled: Option<bool>,
    pub disabled: Option<bool>,
    pub is_multiple: Option<bool>,
    pub multiple: Option<bool>,
    pub is_accept_directory: Option<bool>,
    pub accept_directory: Option<bool>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FileTriggerSelectionMode {
    SingleFile,
    MultipleFiles,
    Directory,
}

impl FileTriggerSelectionMode {
    pub const fn is_multiple(self) -> bool {
        match self {
            Self::SingleFile => false,
            Self::MultipleFiles | Self::Directory => true,
        }
    }

    pub const fn is_accept_directory(self) -> bool {
        matches!(self, Self::Directory)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FileTriggerProps {
    pub is_disabled: bool,
    pub selection_mode: FileTriggerSelectionMode,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FileTriggerState {
    pub is_disabled: bool,
    pub is_enabled: bool,
    pub state_attr: &'static str,
    pub motion_source_attr: &'static str,
    pub has_custom_motion: bool,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct FileTriggerRenderStateInput {
    pub props: FileTriggerPropsInput,
    pub has_custom_motion: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FileTriggerRenderState {
    pub props: FileTriggerProps,
    pub state: FileTriggerState,
}

pub fn resolve_props(input: FileTriggerPropsInput) -> FileTriggerProps {
    let is_multiple = input.is_multiple.or(input.multiple).unwrap_or(false);
    let is_accept_directory = input
        .is_accept_directory
        .or(input.accept_directory)
        .unwrap_or(false);
    let selection_mode = match (is_accept_directory, is_multiple) {
        (true, _) => FileTriggerSelectionMode::Directory,
        (false, true) => FileTriggerSelectionMode::MultipleFiles,
        (false, false) => FileTriggerSelectionMode::SingleFile,
    };

    FileTriggerProps {
        is_disabled: input.is_disabled.or(input.disabled).unwrap_or(false),
        selection_mode,
    }
}

pub fn resolve_state(input: FileTriggerStateInput) -> FileTriggerState {
    FileTriggerState {
        is_disabled: input.disabled,
        is_enabled: !input.disabled,
        state_attr: if input.disabled { "disabled" } else { "ready" },
        motion_source_attr: if input.has_custom_motion {
            "custom"
        } else {
            "default"
        },
        has_custom_motion: input.has_custom_motion,
    }
}

pub fn resolve_render_state(input: FileTriggerRenderStateInput) -> FileTriggerRenderState {
    let props = resolve_props(input.props);
    let state = resolve_state(FileTriggerStateInput {
        disabled: props.is_disabled,
        has_custom_motion: input.has_custom_motion,
    });

    FileTriggerRenderState { props, state }
}

#[cfg(test)]
#[path = "test/file_trigger.rs"]
mod tests;
