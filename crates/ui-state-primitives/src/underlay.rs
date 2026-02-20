pub const DEFAULT_OPEN: bool = false;
pub const DEFAULT_TRANSPARENT: bool = false;
pub const DEFAULT_DISABLED: bool = false;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UnderlaySlot {
    Root,
}

impl UnderlaySlot {
    pub fn as_attr(self) -> &'static str {
        match self {
            UnderlaySlot::Root => "underlay",
        }
    }

    pub fn base_class(self) -> &'static str {
        match self {
            UnderlaySlot::Root => "ui-underlay",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UnderlayPartStateInput {
    pub slot: UnderlaySlot,
    pub open: bool,
    pub transparent: bool,
    pub disabled: bool,
    pub has_on_close: bool,
    pub has_custom_transparent: bool,
    pub has_custom_disabled: bool,
    pub has_custom_close_handler: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UnderlayPartState {
    pub slot: UnderlaySlot,
    pub slot_attr: &'static str,
    pub base_class: &'static str,
    pub state_attr: &'static str,
    pub tone_attr: &'static str,
    pub close_mode_attr: &'static str,
    pub open_attr: Option<&'static str>,
    pub transparent_attr: Option<&'static str>,
    pub disabled_attr: Option<&'static str>,
    pub interactive_attr: Option<&'static str>,
    pub is_open: bool,
    pub is_transparent: bool,
    pub is_disabled: bool,
    pub is_interactive: bool,
    pub has_custom_transparent: bool,
    pub has_custom_disabled: bool,
    pub has_custom_close_handler: bool,
    pub has_custom_class_name: bool,
    pub transparent_source_attr: &'static str,
    pub disabled_source_attr: &'static str,
    pub close_source_attr: &'static str,
    pub class_source_attr: &'static str,
}

pub fn state_attr(is_open: bool, is_disabled: bool) -> &'static str {
    if is_disabled {
        "disabled"
    } else if is_open {
        "open"
    } else {
        "closed"
    }
}

pub fn tone_attr(is_transparent: bool) -> &'static str {
    if is_transparent {
        "transparent"
    } else {
        "scrim"
    }
}

pub fn close_mode_attr(is_interactive: bool) -> &'static str {
    if is_interactive {
        "interactive"
    } else {
        "static"
    }
}

fn source_attr(is_custom: bool) -> &'static str {
    if is_custom { "custom" } else { "default" }
}

pub fn resolve_state(input: UnderlayPartStateInput) -> UnderlayPartState {
    let is_open = input.open && !input.disabled;
    let is_interactive = is_open && input.has_on_close;

    UnderlayPartState {
        slot: input.slot,
        slot_attr: input.slot.as_attr(),
        base_class: input.slot.base_class(),
        state_attr: state_attr(is_open, input.disabled),
        tone_attr: tone_attr(input.transparent),
        close_mode_attr: close_mode_attr(is_interactive),
        open_attr: is_open.then_some("true"),
        transparent_attr: input.transparent.then_some("true"),
        disabled_attr: input.disabled.then_some("true"),
        interactive_attr: is_interactive.then_some("true"),
        is_open,
        is_transparent: input.transparent,
        is_disabled: input.disabled,
        is_interactive,
        has_custom_transparent: input.has_custom_transparent,
        has_custom_disabled: input.has_custom_disabled,
        has_custom_close_handler: input.has_custom_close_handler,
        has_custom_class_name: input.has_custom_class_name,
        transparent_source_attr: source_attr(input.has_custom_transparent),
        disabled_source_attr: source_attr(input.has_custom_disabled),
        close_source_attr: source_attr(input.has_custom_close_handler),
        class_source_attr: source_attr(input.has_custom_class_name),
    }
}

#[cfg(test)]
#[path = "test/underlay.rs"]
mod tests;
