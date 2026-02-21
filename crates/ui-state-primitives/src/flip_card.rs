pub const DEFAULT_DISABLED: bool = false;
pub const DEFAULT_FLIPPED: bool = false;
pub const DEFAULT_HOVER_FLIP: bool = false;
pub const DEFAULT_ID_PREFIX: &str = "ui-flip-card";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlipCardFlipMode {
    Toggle,
    Hover,
}

impl FlipCardFlipMode {
    pub const fn from_hover_flag(value: bool) -> Self {
        if value { Self::Hover } else { Self::Toggle }
    }

    pub const fn is_hover(self) -> bool {
        matches!(self, Self::Hover)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FlipCardBehaviorFlagsInput {
    pub is_disabled: Option<bool>,
    pub disabled: Option<bool>,
    pub flip_mode: Option<FlipCardFlipMode>,
    pub is_flip_on_hover: Option<bool>,
    pub flip_on_hover: Option<bool>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FlipCardBehaviorFlags {
    pub is_disabled: bool,
    pub flip_mode: FlipCardFlipMode,
    pub disabled_source_attr: &'static str,
    pub flip_mode_source_attr: &'static str,
}

pub fn normalize_behavior_flags(input: FlipCardBehaviorFlagsInput) -> FlipCardBehaviorFlags {
    let flip_mode = if let Some(mode) = input.flip_mode {
        mode
    } else if let Some(value) = input.is_flip_on_hover {
        FlipCardFlipMode::from_hover_flag(value)
    } else if let Some(value) = input.flip_on_hover {
        FlipCardFlipMode::from_hover_flag(value)
    } else {
        FlipCardFlipMode::from_hover_flag(DEFAULT_HOVER_FLIP)
    };

    FlipCardBehaviorFlags {
        is_disabled: input
            .is_disabled
            .or(input.disabled)
            .unwrap_or(DEFAULT_DISABLED),
        flip_mode,
        disabled_source_attr: if input.is_disabled.is_some() {
            "is_disabled"
        } else if input.disabled.is_some() {
            "disabled"
        } else {
            "none"
        },
        flip_mode_source_attr: if input.flip_mode.is_some() {
            "flip_mode"
        } else if input.is_flip_on_hover.is_some() {
            "is_flip_on_hover"
        } else if input.flip_on_hover.is_some() {
            "flip_on_hover"
        } else {
            "none"
        },
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlipCardSlot {
    Root,
    Front,
    Back,
}

impl FlipCardSlot {
    pub fn as_attr(self) -> &'static str {
        match self {
            FlipCardSlot::Root => "flip-card",
            FlipCardSlot::Front => "flip-card-front",
            FlipCardSlot::Back => "flip-card-back",
        }
    }

    pub fn base_class(self) -> &'static str {
        match self {
            FlipCardSlot::Root => "ui-flip-card",
            FlipCardSlot::Front => "ui-flip-card__face ui-flip-card__front",
            FlipCardSlot::Back => "ui-flip-card__face ui-flip-card__back",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FlipCardPartStateInput {
    pub slot: FlipCardSlot,
    pub disabled: bool,
    pub is_flipped: bool,
    pub flip_on_hover: bool,
    pub has_custom_class_name: bool,
    pub has_custom_motion: bool,
    pub has_custom_id: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FlipCardPartState {
    pub slot: FlipCardSlot,
    pub slot_attr: &'static str,
    pub base_class: &'static str,
    pub state_attr: &'static str,
    pub visibility_attr: &'static str,
    pub is_disabled: bool,
    pub is_flipped: bool,
    pub flip_mode_attr: &'static str,
    pub has_custom_class_name: bool,
    pub has_custom_motion: bool,
    pub has_custom_id: bool,
    pub class_source_attr: &'static str,
    pub motion_source_attr: &'static str,
    pub id_source_attr: &'static str,
    pub flip_mode_source_attr: &'static str,
}

pub fn state_attr(is_flipped: bool) -> &'static str {
    if is_flipped { "flipped" } else { "default" }
}

pub fn flip_mode_attr(flip_on_hover: bool) -> &'static str {
    if flip_on_hover { "hover" } else { "toggle" }
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.into())
    })
}

pub fn resolve_id(custom_id: Option<String>, fallback_id: String) -> (String, bool) {
    if let Some(custom_id) = normalize_optional_text(custom_id) {
        return (custom_id, true);
    }

    (fallback_id, false)
}

fn source_attr(is_custom: bool) -> &'static str {
    if is_custom { "custom" } else { "default" }
}

pub fn resolve_part_state(input: FlipCardPartStateInput) -> FlipCardPartState {
    FlipCardPartState {
        slot: input.slot,
        slot_attr: input.slot.as_attr(),
        base_class: input.slot.base_class(),
        state_attr: match input.slot {
            FlipCardSlot::Root => state_attr(input.is_flipped),
            FlipCardSlot::Front => "front",
            FlipCardSlot::Back => "back",
        },
        visibility_attr: match input.slot {
            FlipCardSlot::Root => state_attr(input.is_flipped),
            FlipCardSlot::Front => {
                if input.is_flipped {
                    "hidden"
                } else {
                    "visible"
                }
            }
            FlipCardSlot::Back => {
                if input.is_flipped {
                    "visible"
                } else {
                    "hidden"
                }
            }
        },
        is_disabled: input.disabled,
        is_flipped: input.is_flipped,
        flip_mode_attr: flip_mode_attr(input.flip_on_hover),
        has_custom_class_name: input.has_custom_class_name,
        has_custom_motion: input.has_custom_motion,
        has_custom_id: input.has_custom_id,
        class_source_attr: source_attr(input.has_custom_class_name),
        motion_source_attr: source_attr(input.has_custom_motion),
        id_source_attr: source_attr(input.has_custom_id),
        flip_mode_source_attr: source_attr(input.flip_on_hover),
    }
}

#[cfg(test)]
#[path = "test/flip_card.rs"]
mod tests;
