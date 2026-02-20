pub use crate::button::normalize_optional_text;

pub const DEFAULT_DELAY_MS: u64 = 1500;
pub const DEFAULT_CLOSE_DELAY_MS: u64 = 500;
pub const DEFAULT_SHOULD_CLOSE_ON_PRESS: bool = true;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TooltipSlot {
    Root,
    Panel,
}

impl TooltipSlot {
    pub fn as_attr(self) -> &'static str {
        match self {
            TooltipSlot::Root => "tooltip",
            TooltipSlot::Panel => "tooltip-panel",
        }
    }

    pub fn base_class(self) -> &'static str {
        match self {
            TooltipSlot::Root => "ui-tooltip",
            TooltipSlot::Panel => "ui-tooltip__panel",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TooltipTriggerMode {
    Hover,
    Focus,
}

impl TooltipTriggerMode {
    pub fn as_attr(self) -> &'static str {
        match self {
            TooltipTriggerMode::Hover => "hover",
            TooltipTriggerMode::Focus => "focus",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TooltipPartStateInput {
    pub slot: TooltipSlot,
    pub open: bool,
    pub disabled: bool,
    pub has_custom_class_name: bool,
    pub has_custom_motion: bool,
    pub has_custom_delays: bool,
    pub has_custom_trigger_mode: bool,
    pub has_custom_press_behavior: bool,
    pub has_custom_id: bool,
    pub trigger_attr: &'static str,
    pub press_behavior_attr: &'static str,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TooltipPartState {
    pub slot: TooltipSlot,
    pub slot_attr: &'static str,
    pub base_class: &'static str,
    pub state_attr: &'static str,
    pub is_open: bool,
    pub is_disabled: bool,
    pub has_custom_class_name: bool,
    pub has_custom_motion: bool,
    pub has_custom_delays: bool,
    pub has_custom_trigger_mode: bool,
    pub has_custom_press_behavior: bool,
    pub has_custom_id: bool,
    pub trigger_attr: &'static str,
    pub press_behavior_attr: &'static str,
    pub class_source_attr: &'static str,
    pub motion_source_attr: &'static str,
    pub delay_source_attr: &'static str,
    pub trigger_source_attr: &'static str,
    pub press_source_attr: &'static str,
    pub id_source_attr: &'static str,
}

pub fn state_attr_for_open(is_open: bool) -> &'static str {
    if is_open { "open" } else { "closed" }
}

pub fn trigger_attr(trigger: TooltipTriggerMode) -> &'static str {
    trigger.as_attr()
}

pub fn press_behavior_attr(should_close_on_press: bool) -> &'static str {
    if should_close_on_press {
        "close"
    } else {
        "persist"
    }
}

pub fn resolve_id(custom_id: Option<String>, fallback_id: String) -> (String, bool) {
    if let Some(custom_id) = normalize_optional_text(custom_id) {
        return (custom_id, true);
    }

    (fallback_id, false)
}

pub fn has_custom_delays(delay_ms: u64, close_delay_ms: u64) -> bool {
    delay_ms != DEFAULT_DELAY_MS || close_delay_ms != DEFAULT_CLOSE_DELAY_MS
}

fn source_attr(is_custom: bool) -> &'static str {
    if is_custom { "custom" } else { "default" }
}

pub fn resolve_state(input: TooltipPartStateInput) -> TooltipPartState {
    TooltipPartState {
        slot: input.slot,
        slot_attr: input.slot.as_attr(),
        base_class: input.slot.base_class(),
        state_attr: match input.slot {
            TooltipSlot::Root => state_attr_for_open(input.open),
            TooltipSlot::Panel => "panel",
        },
        is_open: input.open,
        is_disabled: input.disabled,
        has_custom_class_name: input.has_custom_class_name,
        has_custom_motion: input.has_custom_motion,
        has_custom_delays: input.has_custom_delays,
        has_custom_trigger_mode: input.has_custom_trigger_mode,
        has_custom_press_behavior: input.has_custom_press_behavior,
        has_custom_id: input.has_custom_id,
        trigger_attr: input.trigger_attr,
        press_behavior_attr: input.press_behavior_attr,
        class_source_attr: source_attr(input.has_custom_class_name),
        motion_source_attr: source_attr(input.has_custom_motion),
        delay_source_attr: source_attr(input.has_custom_delays),
        trigger_source_attr: source_attr(input.has_custom_trigger_mode),
        press_source_attr: source_attr(input.has_custom_press_behavior),
        id_source_attr: source_attr(input.has_custom_id),
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: TooltipPartState) -> String {
    let mut classes = vec![state.base_class.into()];

    if state.slot == TooltipSlot::Root {
        if state.has_custom_motion {
            classes.push("ui-tooltip--custom-motion".to_string());
        }

        if state.has_custom_delays {
            classes.push("ui-tooltip--custom-delay".to_string());
        }

        if state.has_custom_trigger_mode {
            classes.push("ui-tooltip--custom-trigger".to_string());
        }

        if state.has_custom_press_behavior {
            classes.push("ui-tooltip--custom-press".to_string());
        }

        if state.has_custom_id {
            classes.push("ui-tooltip--custom-id".to_string());
        }

        if state.has_custom_class_name {
            classes.push("ui-tooltip--custom-class".to_string());
            if let Some(base_class_name) = base_class_name {
                classes.push(base_class_name);
            }
        }
    }

    classes.join(" ")
}

pub fn compose_panel_vars(top_px: f64, left_px: f64) -> String {
    format!("--ui-tooltip-top: {top_px}px; --ui-tooltip-left: {left_px}px;")
}

#[cfg(test)]
#[path = "test/tooltip.rs"]
mod tests;
