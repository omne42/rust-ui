pub use crate::button::normalize_optional_text;
use std::borrow::Cow;

pub const DEFAULT_OPEN_DELAY_MS: u64 = 140;
pub const DEFAULT_CLOSE_DELAY_MS: u64 = 180;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HoverCardSlot {
    Root,
    Trigger,
    Panel,
}

impl HoverCardSlot {
    pub fn as_attr(self) -> &'static str {
        match self {
            HoverCardSlot::Root => "hover-card",
            HoverCardSlot::Trigger => "hover-card-trigger",
            HoverCardSlot::Panel => "hover-card-panel",
        }
    }

    pub fn base_class(self) -> &'static str {
        match self {
            HoverCardSlot::Root => "ui-hover-card",
            HoverCardSlot::Trigger => "ui-hover-card__trigger",
            HoverCardSlot::Panel => "ui-hover-card__panel",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HoverCardPartStateInput {
    pub slot: HoverCardSlot,
    pub open: bool,
    pub disabled: bool,
    pub has_custom_class_name: bool,
    pub has_custom_motion: bool,
    pub has_custom_delays: bool,
    pub has_custom_id: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HoverCardPartState {
    pub slot: HoverCardSlot,
    pub slot_attr: &'static str,
    pub base_class: &'static str,
    pub state_attr: &'static str,
    pub is_open: bool,
    pub is_disabled: bool,
    pub has_custom_class_name: bool,
    pub has_custom_motion: bool,
    pub has_custom_delays: bool,
    pub has_custom_id: bool,
    pub class_source_attr: &'static str,
    pub motion_source_attr: &'static str,
    pub delay_source_attr: &'static str,
    pub id_source_attr: &'static str,
}

pub fn state_attr_for_open(is_open: bool) -> &'static str {
    if is_open { "open" } else { "closed" }
}

pub fn resolve_id(custom_id: Option<String>, fallback_id: Cow<'static, str>) -> (String, bool) {
    if let Some(custom_id) = normalize_optional_text(custom_id) {
        return (custom_id, true);
    }

    (fallback_id.into_owned(), false)
}

pub fn has_custom_delays(open_delay_ms: u64, close_delay_ms: u64) -> bool {
    open_delay_ms != DEFAULT_OPEN_DELAY_MS || close_delay_ms != DEFAULT_CLOSE_DELAY_MS
}

fn source_attr(is_custom: bool) -> &'static str {
    if is_custom { "custom" } else { "default" }
}

pub fn resolve_state(input: HoverCardPartStateInput) -> HoverCardPartState {
    HoverCardPartState {
        slot: input.slot,
        slot_attr: input.slot.as_attr(),
        base_class: input.slot.base_class(),
        state_attr: match input.slot {
            HoverCardSlot::Root => state_attr_for_open(input.open),
            HoverCardSlot::Trigger => "trigger",
            HoverCardSlot::Panel => "panel",
        },
        is_open: input.open,
        is_disabled: input.disabled,
        has_custom_class_name: input.has_custom_class_name,
        has_custom_motion: input.has_custom_motion,
        has_custom_delays: input.has_custom_delays,
        has_custom_id: input.has_custom_id,
        class_source_attr: source_attr(input.has_custom_class_name),
        motion_source_attr: source_attr(input.has_custom_motion),
        delay_source_attr: source_attr(input.has_custom_delays),
        id_source_attr: source_attr(input.has_custom_id),
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: HoverCardPartState) -> String {
    let mut classes = vec![state.base_class.into()];

    if state.slot == HoverCardSlot::Root {
        if state.has_custom_motion {
            classes.push("ui-hover-card--custom-motion".to_string());
        }

        if state.has_custom_delays {
            classes.push("ui-hover-card--custom-delay".to_string());
        }

        if state.has_custom_id {
            classes.push("ui-hover-card--custom-id".to_string());
        }

        if state.has_custom_class_name {
            classes.push("ui-hover-card--custom-class".to_string());
            if let Some(base_class_name) = base_class_name {
                classes.push(base_class_name);
            }
        }
    }

    classes.join(" ")
}

pub fn compose_panel_vars(top_px: f64, left_px: f64, anchor_width_px: f64) -> String {
    format!(
        "--ui-hover-card-top: {top_px}px; --ui-hover-card-left: {left_px}px; --ui-hover-card-anchor-width: {anchor_width_px}px;"
    )
}

#[cfg(test)]
#[path = "test/hover_card.rs"]
mod tests;
