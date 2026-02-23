use std::borrow::Cow;

pub const DEFAULT_OPEN: bool = false;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PopoverOpenMode {
    Controlled,
    Uncontrolled,
}

impl PopoverOpenMode {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::Controlled => "controlled",
            Self::Uncontrolled => "uncontrolled",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PopoverOpenStateInput {
    pub has_is_open_prop: bool,
    pub has_open_prop: bool,
    pub default_open: Option<bool>,
    pub has_on_open_change: bool,
    pub has_on_close: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PopoverOpenState {
    pub default_open: bool,
    pub mode: PopoverOpenMode,
    pub has_default_open: bool,
    pub has_open_change_handler: bool,
    pub has_on_close_handler: bool,
    pub open_prop_source_attr: &'static str,
    pub open_mode_attr: &'static str,
    pub open_source_attr: &'static str,
    pub open_change_source_attr: &'static str,
}

pub fn resolve_open_state(input: PopoverOpenStateInput) -> PopoverOpenState {
    let mode = if input.has_is_open_prop || input.has_open_prop {
        PopoverOpenMode::Controlled
    } else {
        PopoverOpenMode::Uncontrolled
    };
    let has_default_open = input.default_open.is_some();

    PopoverOpenState {
        default_open: input.default_open.unwrap_or(DEFAULT_OPEN),
        mode,
        has_default_open,
        has_open_change_handler: input.has_on_open_change,
        has_on_close_handler: input.has_on_close,
        open_prop_source_attr: if input.has_is_open_prop {
            "is_open"
        } else if input.has_open_prop {
            "open"
        } else {
            "none"
        },
        open_mode_attr: mode.as_attr(),
        open_source_attr: if matches!(mode, PopoverOpenMode::Controlled) {
            "external"
        } else if has_default_open {
            "default"
        } else {
            "implicit-default"
        },
        open_change_source_attr: if input.has_on_open_change {
            "on_open_change"
        } else if input.has_on_close {
            "on_close"
        } else {
            "none"
        },
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PopoverSlot {
    Root,
    Panel,
}

impl PopoverSlot {
    pub fn as_attr(self) -> &'static str {
        match self {
            Self::Root => "popover",
            Self::Panel => "popover-panel",
        }
    }

    pub fn base_class(self) -> &'static str {
        match self {
            Self::Root => "ui-popover",
            Self::Panel => "ui-popover__panel",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PopoverPartStateInput {
    pub slot: PopoverSlot,
    pub open: bool,
    pub is_modal: bool,
    pub has_custom_class_name: bool,
    pub has_custom_motion: bool,
    pub has_custom_placement: bool,
    pub has_on_exit_complete: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PopoverPartState {
    pub slot: PopoverSlot,
    pub slot_attr: &'static str,
    pub base_class: &'static str,
    pub state_attr: &'static str,
    pub is_open: bool,
    pub is_modal: bool,
    pub has_custom_class_name: bool,
    pub has_custom_motion: bool,
    pub has_custom_placement: bool,
    pub has_on_exit_complete: bool,
    pub modal_attr: &'static str,
    pub class_source_attr: &'static str,
    pub motion_source_attr: &'static str,
    pub placement_source_attr: &'static str,
    pub modal_source_attr: &'static str,
    pub exit_source_attr: &'static str,
}

pub fn state_attr_for_open(is_open: bool) -> &'static str {
    if is_open { "open" } else { "closed" }
}

pub fn modal_attr(is_modal: bool) -> &'static str {
    if is_modal { "modal" } else { "non-modal" }
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.into())
    })
}

fn source_attr(is_custom: bool) -> &'static str {
    if is_custom { "custom" } else { "default" }
}

pub fn resolve_state(input: PopoverPartStateInput) -> PopoverPartState {
    PopoverPartState {
        slot: input.slot,
        slot_attr: input.slot.as_attr(),
        base_class: input.slot.base_class(),
        state_attr: match input.slot {
            PopoverSlot::Root => state_attr_for_open(input.open),
            PopoverSlot::Panel => "panel",
        },
        is_open: input.open,
        is_modal: input.is_modal,
        has_custom_class_name: input.has_custom_class_name,
        has_custom_motion: input.has_custom_motion,
        has_custom_placement: input.has_custom_placement,
        has_on_exit_complete: input.has_on_exit_complete,
        modal_attr: modal_attr(input.is_modal),
        class_source_attr: source_attr(input.has_custom_class_name),
        motion_source_attr: source_attr(input.has_custom_motion),
        placement_source_attr: source_attr(input.has_custom_placement),
        modal_source_attr: source_attr(!input.is_modal),
        exit_source_attr: source_attr(input.has_on_exit_complete),
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: PopoverPartState) -> String {
    let mut classes: Vec<Cow<'static, str>> = vec![Cow::Borrowed(state.base_class)];

    if state.slot == PopoverSlot::Root {
        if state.has_custom_motion {
            classes.push(Cow::Borrowed("ui-popover--custom-motion"));
        }

        if state.has_custom_placement {
            classes.push(Cow::Borrowed("ui-popover--custom-placement"));
        }

        if !state.is_modal {
            classes.push(Cow::Borrowed("ui-popover--non-modal"));
            classes.push(Cow::Borrowed("ui-popover--custom-modal"));
        }

        if state.has_on_exit_complete {
            classes.push(Cow::Borrowed("ui-popover--custom-exit"));
        }

        if state.has_custom_class_name {
            classes.push(Cow::Borrowed("ui-popover--custom-class"));
            if let Some(base_class_name) = base_class_name {
                classes.push(Cow::Owned(base_class_name));
            }
        }
    }

    classes
        .iter()
        .map(Cow::as_ref)
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
#[path = "test/popover.rs"]
mod tests;
