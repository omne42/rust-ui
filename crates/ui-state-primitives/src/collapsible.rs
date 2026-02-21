use crate::controlled::{ControlledState, ControlledStateOptions, use_controlled_state};

pub const DEFAULT_ID_BASE: &str = "collapsible";
pub const DEFAULT_TITLE: &str = "Collapsible";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CollapsibleStatus {
    Open,
    Closed,
    Disabled,
}

impl CollapsibleStatus {
    pub const fn from_parts(is_open: bool, is_disabled: bool) -> Self {
        if is_disabled {
            Self::Disabled
        } else if is_open {
            Self::Open
        } else {
            Self::Closed
        }
    }

    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::Open => "open",
            Self::Closed => "closed",
            Self::Disabled => "disabled",
        }
    }

    pub const fn is_open(self) -> bool {
        matches!(self, Self::Open)
    }

    pub const fn is_closed(self) -> bool {
        matches!(self, Self::Closed)
    }

    pub const fn is_disabled(self) -> bool {
        matches!(self, Self::Disabled)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CollapsibleOpenMode {
    Controlled,
    Uncontrolled,
}

impl CollapsibleOpenMode {
    pub const fn from_is_controlled(is_controlled: bool) -> Self {
        if is_controlled {
            Self::Controlled
        } else {
            Self::Uncontrolled
        }
    }

    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::Controlled => "controlled",
            Self::Uncontrolled => "uncontrolled",
        }
    }

    pub const fn is_controlled(self) -> bool {
        matches!(self, Self::Controlled)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CollapsibleLabelSource {
    Title,
    Custom,
}

impl CollapsibleLabelSource {
    pub const fn from_has_custom(has_custom: bool) -> Self {
        if has_custom {
            Self::Custom
        } else {
            Self::Title
        }
    }

    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::Title => "title",
            Self::Custom => "custom",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CollapsibleClassSource {
    Default,
    Custom,
}

impl CollapsibleClassSource {
    pub const fn from_has_custom(has_custom: bool) -> Self {
        if has_custom {
            Self::Custom
        } else {
            Self::Default
        }
    }

    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Custom => "custom",
        }
    }

    pub const fn is_custom(self) -> bool {
        matches!(self, Self::Custom)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CollapsibleMotionSource {
    Default,
    Custom,
}

impl CollapsibleMotionSource {
    pub const fn from_has_custom(has_custom: bool) -> Self {
        if has_custom {
            Self::Custom
        } else {
            Self::Default
        }
    }

    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Custom => "custom",
        }
    }

    pub const fn is_custom(self) -> bool {
        matches!(self, Self::Custom)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CollapsibleOpenValueSource {
    External,
    Default,
    Primitive,
}

impl CollapsibleOpenValueSource {
    pub const fn from_input(open: Option<bool>, default_open: Option<bool>) -> Self {
        if open.is_some() {
            Self::External
        } else if default_open.is_some() {
            Self::Default
        } else {
            Self::Primitive
        }
    }

    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::External => "external",
            Self::Default => "default",
            Self::Primitive => "primitive",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CollapsibleOpenChangeSource {
    Initial,
    Interaction,
    ExternalSync,
}

impl CollapsibleOpenChangeSource {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::Initial => "initial",
            Self::Interaction => "interaction",
            Self::ExternalSync => "external-sync",
        }
    }
}

#[derive(Clone)]
pub struct CollapsibleOpenState {
    open: ControlledState<bool>,
}

#[derive(Clone, Default)]
pub struct CollapsibleOpenStateOptions {
    pub open: Option<bool>,
    pub default_open: Option<bool>,
}

pub fn use_collapsible_open_state(options: CollapsibleOpenStateOptions) -> CollapsibleOpenState {
    CollapsibleOpenState {
        open: use_controlled_state(
            false,
            ControlledStateOptions {
                value: options.open,
                default_value: options.default_open,
                on_change: None,
            },
        ),
    }
}

impl CollapsibleOpenState {
    pub fn is_open(&self) -> bool {
        *self.open.value()
    }

    pub fn default_open(&self) -> bool {
        *self.open.default_value()
    }

    pub fn is_controlled(&self) -> bool {
        self.open.is_controlled()
    }

    pub fn sync_controlled(&mut self, open: Option<bool>) {
        self.open.sync_controlled(open);
    }

    pub fn set_open(&mut self, open: bool) {
        self.open.set_value(open);
    }

    pub fn toggle(&mut self) {
        self.set_open(!self.is_open());
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CollapsibleStateInput {
    pub status: CollapsibleStatus,
    pub open_mode: CollapsibleOpenMode,
    pub label_source: CollapsibleLabelSource,
    pub class_source: CollapsibleClassSource,
    pub motion_source: CollapsibleMotionSource,
    pub open_value_source: CollapsibleOpenValueSource,
    pub open_change_source: CollapsibleOpenChangeSource,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CollapsibleState {
    pub status: CollapsibleStatus,
    pub open_mode: CollapsibleOpenMode,
    pub label_source: CollapsibleLabelSource,
    pub class_source: CollapsibleClassSource,
    pub motion_source: CollapsibleMotionSource,
    pub open_value_source: CollapsibleOpenValueSource,
    pub open_change_source: CollapsibleOpenChangeSource,
    pub is_open: bool,
    pub is_closed: bool,
    pub is_disabled: bool,
    pub is_controlled: bool,
    pub state_attr: &'static str,
    pub open_mode_attr: &'static str,
    pub label_source_attr: &'static str,
    pub class_source_attr: &'static str,
    pub motion_source_attr: &'static str,
    pub open_value_source_attr: &'static str,
    pub open_change_source_attr: &'static str,
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.into())
    })
}

pub fn normalize_id_base(value: String) -> String {
    let trimmed = value.trim();

    if trimmed.is_empty() {
        return DEFAULT_ID_BASE.into();
    }

    let mut normalized = String::new();
    let mut previous_was_dash = false;

    for character in trimmed.chars() {
        let mapped = if character.is_ascii_alphanumeric() {
            character.to_ascii_lowercase()
        } else if character == '_' {
            '_'
        } else {
            '-'
        };

        if mapped == '-' {
            if previous_was_dash {
                continue;
            }
            previous_was_dash = true;
        } else {
            previous_was_dash = false;
        }

        normalized.push(mapped);
    }

    let normalized = normalized.trim_matches('-').trim_matches('_').to_string();

    if normalized.is_empty() {
        DEFAULT_ID_BASE.into()
    } else {
        normalized
    }
}

pub fn resolve_title(value: String) -> String {
    normalize_optional_text(Some(value)).unwrap_or_else(|| DEFAULT_TITLE.into())
}

pub fn resolve_aria_label(title: &str, value: Option<String>) -> (String, bool) {
    if let Some(label) = normalize_optional_text(value) {
        return (label, true);
    }

    (title.into(), false)
}

pub fn resolve_state(input: CollapsibleStateInput) -> CollapsibleState {
    let status = input.status;
    let open_mode = input.open_mode;
    let label_source = input.label_source;
    let class_source = input.class_source;
    let motion_source = input.motion_source;
    let open_value_source = input.open_value_source;
    let open_change_source = input.open_change_source;

    CollapsibleState {
        status,
        open_mode,
        label_source,
        class_source,
        motion_source,
        open_value_source,
        open_change_source,
        is_open: status.is_open(),
        is_closed: status.is_closed(),
        is_disabled: status.is_disabled(),
        is_controlled: open_mode.is_controlled(),
        state_attr: status.as_attr(),
        open_mode_attr: open_mode.as_attr(),
        label_source_attr: label_source.as_attr(),
        class_source_attr: class_source.as_attr(),
        motion_source_attr: motion_source.as_attr(),
        open_value_source_attr: open_value_source.as_attr(),
        open_change_source_attr: open_change_source.as_attr(),
    }
}

#[cfg(test)]
#[path = "test/collapsible.rs"]
mod tests;
