use crate::{FlipCardPartState, FlipCardPartStateInput, FlipCardSlot};
use leptos::prelude::*;
use std::borrow::Cow;

pub use ui_state_primitives::flip_card::{
    DEFAULT_FLIPPED, DEFAULT_ID_PREFIX, FlipCardBehaviorFlags, FlipCardBehaviorFlagsInput,
    FlipCardFlipMode, normalize_behavior_flags, normalize_optional_text, resolve_id,
    resolve_part_state,
};

pub struct FlipCardFlippedAxisInput {
    pub is_flipped: Option<Signal<bool>>,
    pub default_is_flipped: Option<bool>,
    pub default_flipped: Option<bool>,
    pub on_is_flipped_change: Option<Callback<bool>>,
}

#[derive(Clone)]
pub struct FlipCardFlippedAxis {
    pub controlled_is_flipped: Option<Signal<bool>>,
    pub default_is_flipped: bool,
    pub on_is_flipped_change: Option<Callback<bool>>,
    pub flipped_is_controlled: bool,
    pub flipped_control_mode_attr: &'static str,
    pub flipped_prop_source_attr: &'static str,
    pub flipped_default_source_attr: &'static str,
    pub flipped_change_source_attr: &'static str,
}

pub fn normalize_flipped_axis(input: FlipCardFlippedAxisInput) -> FlipCardFlippedAxis {
    let controlled_is_flipped = input.is_flipped;
    let flipped_is_controlled = controlled_is_flipped.is_some();

    FlipCardFlippedAxis {
        controlled_is_flipped,
        default_is_flipped: input
            .default_is_flipped
            .or(input.default_flipped)
            .unwrap_or(DEFAULT_FLIPPED),
        on_is_flipped_change: input.on_is_flipped_change,
        flipped_is_controlled,
        flipped_control_mode_attr: if flipped_is_controlled {
            "controlled"
        } else {
            "uncontrolled"
        },
        flipped_prop_source_attr: if input.is_flipped.is_some() {
            "is_flipped"
        } else {
            "none"
        },
        flipped_default_source_attr: if input.default_is_flipped.is_some() {
            "default_is_flipped"
        } else if input.default_flipped.is_some() {
            "default_flipped"
        } else {
            "none"
        },
        flipped_change_source_attr: if input.on_is_flipped_change.is_some() {
            "on_is_flipped_change"
        } else {
            "none"
        },
    }
}

fn bool_data_attr(value: bool) -> Option<&'static str> {
    value.then_some("true")
}

pub fn state_attr(is_flipped: bool) -> &'static str {
    ui_state_primitives::flip_card::state_attr(is_flipped)
}

pub fn flip_mode_attr(flip_on_hover: bool) -> &'static str {
    ui_state_primitives::flip_card::flip_mode_attr(flip_on_hover)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FlipCardRootSemanticMarkers {
    pub flipped_control_mode_attr: &'static str,
    pub flipped_prop_source_attr: &'static str,
    pub flipped_default_source_attr: &'static str,
    pub flipped_change_source_attr: &'static str,
    pub flipped_attr: Option<&'static str>,
    pub default_attr: Option<&'static str>,
    pub flipped_controlled_attr: Option<&'static str>,
    pub flipped_uncontrolled_attr: Option<&'static str>,
    pub disabled_attr: Option<&'static str>,
    pub enabled_attr: Option<&'static str>,
    pub hovered_attr: Option<&'static str>,
    pub flip_mode_source_attr: &'static str,
    pub custom_class_attr: Option<&'static str>,
    pub custom_motion_attr: Option<&'static str>,
    pub custom_id_attr: Option<&'static str>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FlipCardFaceSemanticMarkers {
    pub visible_attr: Option<&'static str>,
    pub hidden_attr: Option<&'static str>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FlipCardDerivedRenderStateInput {
    pub is_disabled: bool,
    pub is_flipped: bool,
    pub is_hovered: bool,
    pub flip_mode: FlipCardFlipMode,
    pub flip_mode_source_attr: &'static str,
    pub has_custom_class_name: bool,
    pub has_custom_motion: bool,
    pub has_custom_id: bool,
    pub flipped_is_controlled: bool,
    pub flipped_control_mode_attr: &'static str,
    pub flipped_prop_source_attr: &'static str,
    pub flipped_default_source_attr: &'static str,
    pub flipped_change_source_attr: &'static str,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FlipCardDerivedRenderState {
    pub root: FlipCardPartState,
    pub front: FlipCardPartState,
    pub back: FlipCardPartState,
    pub root_markers: FlipCardRootSemanticMarkers,
    pub front_markers: FlipCardFaceSemanticMarkers,
    pub back_markers: FlipCardFaceSemanticMarkers,
}

fn face_semantic_markers(state: FlipCardPartState) -> FlipCardFaceSemanticMarkers {
    FlipCardFaceSemanticMarkers {
        visible_attr: bool_data_attr(state.visibility_attr == "visible"),
        hidden_attr: bool_data_attr(state.visibility_attr == "hidden"),
    }
}

pub fn derive_render_state(input: FlipCardDerivedRenderStateInput) -> FlipCardDerivedRenderState {
    let root = resolve_part_state(FlipCardPartStateInput {
        slot: FlipCardSlot::Root,
        disabled: input.is_disabled,
        is_flipped: input.is_flipped,
        flip_on_hover: input.flip_mode.is_hover(),
        has_custom_class_name: input.has_custom_class_name,
        has_custom_motion: input.has_custom_motion,
        has_custom_id: input.has_custom_id,
    });

    let front = resolve_part_state(FlipCardPartStateInput {
        slot: FlipCardSlot::Front,
        disabled: input.is_disabled,
        is_flipped: input.is_flipped,
        flip_on_hover: input.flip_mode.is_hover(),
        has_custom_class_name: false,
        has_custom_motion: input.has_custom_motion,
        has_custom_id: input.has_custom_id,
    });

    let back = resolve_part_state(FlipCardPartStateInput {
        slot: FlipCardSlot::Back,
        disabled: input.is_disabled,
        is_flipped: input.is_flipped,
        flip_on_hover: input.flip_mode.is_hover(),
        has_custom_class_name: false,
        has_custom_motion: input.has_custom_motion,
        has_custom_id: input.has_custom_id,
    });

    FlipCardDerivedRenderState {
        root,
        front,
        back,
        root_markers: FlipCardRootSemanticMarkers {
            flipped_control_mode_attr: input.flipped_control_mode_attr,
            flipped_prop_source_attr: input.flipped_prop_source_attr,
            flipped_default_source_attr: input.flipped_default_source_attr,
            flipped_change_source_attr: input.flipped_change_source_attr,
            flipped_attr: bool_data_attr(input.is_flipped),
            default_attr: bool_data_attr(!input.is_flipped),
            flipped_controlled_attr: bool_data_attr(input.flipped_is_controlled),
            flipped_uncontrolled_attr: bool_data_attr(!input.flipped_is_controlled),
            disabled_attr: bool_data_attr(root.is_disabled),
            enabled_attr: bool_data_attr(!root.is_disabled),
            hovered_attr: bool_data_attr(input.is_hovered),
            flip_mode_source_attr: input.flip_mode_source_attr,
            custom_class_attr: bool_data_attr(root.has_custom_class_name),
            custom_motion_attr: bool_data_attr(root.has_custom_motion),
            custom_id_attr: bool_data_attr(root.has_custom_id),
        },
        front_markers: face_semantic_markers(front),
        back_markers: face_semantic_markers(back),
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: FlipCardPartState) -> String {
    let mut classes: Vec<Cow<'static, str>> = vec![Cow::Borrowed(state.base_class)];

    match state.slot {
        FlipCardSlot::Root => {
            if state.is_disabled {
                classes.push(Cow::Borrowed("ui-flip-card--disabled"));
            } else {
                classes.push(Cow::Borrowed("ui-flip-card--enabled"));
            }

            if state.is_flipped {
                classes.push(Cow::Borrowed("ui-flip-card--flipped"));
            } else {
                classes.push(Cow::Borrowed("ui-flip-card--default"));
            }

            if state.flip_mode_attr == "hover" {
                classes.push(Cow::Borrowed("ui-flip-card--hover"));
            } else {
                classes.push(Cow::Borrowed("ui-flip-card--toggle"));
            }

            if state.has_custom_class_name {
                classes.push(Cow::Borrowed("ui-flip-card--custom-class"));
            }

            if state.has_custom_motion {
                classes.push(Cow::Borrowed("ui-flip-card--custom-motion"));
            }

            if state.has_custom_id {
                classes.push(Cow::Borrowed("ui-flip-card--custom-id"));
            }

            if let Some(base_class_name) = base_class_name {
                classes.push(Cow::Owned(base_class_name));
            }
        }
        FlipCardSlot::Front | FlipCardSlot::Back => {
            if state.visibility_attr == "visible" {
                classes.push(Cow::Borrowed("ui-flip-card__face--visible"));
            } else {
                classes.push(Cow::Borrowed("ui-flip-card__face--hidden"));
            }
        }
    }

    classes
        .iter()
        .map(|class| class.as_ref())
        .collect::<Vec<_>>()
        .join(" ")
}

pub const FLIP_CARD_AGENT_SCHEMA: &str = "ui.flip-card.agent-contract";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlipCardAgentSchemaVersion {
    V1,
}

impl FlipCardAgentSchemaVersion {
    pub const fn as_str(self) -> &'static str {
        match self {
            FlipCardAgentSchemaVersion::V1 => "v1",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlipCardAgentIntent {
    FlipInteraction,
}

impl FlipCardAgentIntent {
    pub const fn as_str(self) -> &'static str {
        match self {
            FlipCardAgentIntent::FlipInteraction => "flip.interaction",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlipCardAgentAction {
    SnapshotRender,
    Toggle,
    HoverEnter,
    HoverLeave,
    Focus,
    Blur,
}

impl FlipCardAgentAction {
    pub const fn as_str(self) -> &'static str {
        match self {
            FlipCardAgentAction::SnapshotRender => "snapshot-render",
            FlipCardAgentAction::Toggle => "toggle",
            FlipCardAgentAction::HoverEnter => "hover-enter",
            FlipCardAgentAction::HoverLeave => "hover-leave",
            FlipCardAgentAction::Focus => "focus",
            FlipCardAgentAction::Blur => "blur",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlipCardAgentState {
    Disabled,
    Flipped,
    Hovered,
    Default,
}

impl FlipCardAgentState {
    pub const fn as_str(self) -> &'static str {
        match self {
            FlipCardAgentState::Disabled => "disabled",
            FlipCardAgentState::Flipped => "flipped",
            FlipCardAgentState::Hovered => "hovered",
            FlipCardAgentState::Default => "default",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlipCardAgentSource {
    StatePrimitives,
}

impl FlipCardAgentSource {
    pub const fn as_str(self) -> &'static str {
        match self {
            FlipCardAgentSource::StatePrimitives => "state-primitives",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlipCardAgentConfigPolicy {
    Whitelist,
}

impl FlipCardAgentConfigPolicy {
    pub const fn as_str(self) -> &'static str {
        match self {
            FlipCardAgentConfigPolicy::Whitelist => "whitelist",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FlipCardAgentContractInput {
    pub render_state: FlipCardDerivedRenderState,
    pub action: FlipCardAgentAction,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FlipCardAgentContract {
    pub schema_name: &'static str,
    pub schema_version: FlipCardAgentSchemaVersion,
    pub intent: FlipCardAgentIntent,
    pub action: FlipCardAgentAction,
    pub state: FlipCardAgentState,
    pub source: FlipCardAgentSource,
    pub flipped_source: &'static str,
    pub mode_source: &'static str,
    pub motion_source: &'static str,
    pub class_source: &'static str,
    pub id_source: &'static str,
    pub config_policy: FlipCardAgentConfigPolicy,
}

fn resolve_agent_state(render_state: FlipCardDerivedRenderState) -> FlipCardAgentState {
    if render_state.root.is_disabled {
        return FlipCardAgentState::Disabled;
    }
    if render_state.root.is_flipped {
        return FlipCardAgentState::Flipped;
    }
    if render_state.root_markers.hovered_attr.is_some() {
        return FlipCardAgentState::Hovered;
    }
    FlipCardAgentState::Default
}

pub fn resolve_agent_contract(input: FlipCardAgentContractInput) -> FlipCardAgentContract {
    FlipCardAgentContract {
        schema_name: FLIP_CARD_AGENT_SCHEMA,
        schema_version: FlipCardAgentSchemaVersion::V1,
        intent: FlipCardAgentIntent::FlipInteraction,
        action: input.action,
        state: resolve_agent_state(input.render_state),
        source: FlipCardAgentSource::StatePrimitives,
        flipped_source: input.render_state.root_markers.flipped_control_mode_attr,
        mode_source: input.render_state.root_markers.flip_mode_source_attr,
        motion_source: input.render_state.root.motion_source_attr,
        class_source: input.render_state.root.class_source_attr,
        id_source: input.render_state.root.id_source_attr,
        config_policy: FlipCardAgentConfigPolicy::Whitelist,
    }
}

#[cfg(test)]
#[path = "../test/logic.rs"]
mod tests;
