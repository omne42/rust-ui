use std::borrow::Cow;

use crate::OnPress;
use leptos::prelude::*;
use ui_headless::PopoverPlacement;

use super::{CoachmarkAssetVariant, CoachmarkVariant};

pub use ui_state_primitives::coachmark::{
    CoachmarkAssetSource, CoachmarkState, CoachmarkStateInput, DEFAULT_ASSET_LABEL, DEFAULT_TITLE,
    compose_class_name, compose_heading, compose_step_label, normalize_modifier_keys,
    normalize_optional_text, resolve_asset_source, resolve_cta_mode, resolve_state,
};

pub const COACHMARK_AGENT_SCHEMA: &str = "ui.coachmark.agent-contract.v1";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CoachmarkAgentSchemaVersion {
    V1,
}

impl CoachmarkAgentSchemaVersion {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::V1 => "1",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CoachmarkAgentIntent {
    GuidedTour,
}

impl CoachmarkAgentIntent {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::GuidedTour => "guided-tour",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CoachmarkAgentAction {
    ReadGuidance,
    NavigateStep,
}

impl CoachmarkAgentAction {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ReadGuidance => "read-guidance",
            Self::NavigateStep => "navigate-step",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CoachmarkAgentState {
    Open,
    Closed,
    Disabled,
}

impl CoachmarkAgentState {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Open => "open",
            Self::Closed => "closed",
            Self::Disabled => "disabled",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CoachmarkAgentSource {
    Internal,
    External,
}

impl CoachmarkAgentSource {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Internal => "internal",
            Self::External => "external",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CoachmarkAgentOutputStatus {
    Draft,
    Verified,
}

impl CoachmarkAgentOutputStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Draft => "draft",
            Self::Verified => "verified",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CoachmarkAgentContract {
    pub schema_name: &'static str,
    pub schema_version: CoachmarkAgentSchemaVersion,
    pub intent: CoachmarkAgentIntent,
    pub action: CoachmarkAgentAction,
    pub state: CoachmarkAgentState,
    pub source: CoachmarkAgentSource,
    pub state_source: &'static str,
    pub action_source: &'static str,
    pub render_path: &'static str,
    pub output_status: CoachmarkAgentOutputStatus,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CoachmarkAgentContractInput {
    pub has_footer: bool,
    pub open_mode_attr: &'static str,
    pub state_attr: &'static str,
    pub is_disabled: bool,
}

pub struct CoachmarkViewModelInput {
    pub variant: CoachmarkVariant,
    pub placement: PopoverPlacement,
    pub is_disabled: Option<bool>,
    pub disabled: Option<bool>,
    pub is_controlled: bool,
    pub aria_label: Option<String>,
    pub class_name: Option<String>,
    pub title: Option<String>,
    pub current_step: Option<usize>,
    pub total_steps: Option<usize>,
    pub primary_cta: Option<String>,
    pub secondary_cta: Option<String>,
    pub shortcut_key: Option<String>,
    pub modifier_keys: Vec<String>,
    pub has_actions_slot: bool,
    pub asset_variant: Option<CoachmarkAssetVariant>,
    pub asset_label: Option<String>,
    pub asset_src: Option<String>,
    pub asset_alt: Option<String>,
    pub lang: Option<String>,
}

pub struct CoachmarkViewModel {
    pub is_disabled: bool,
    pub class_name: String,
    pub trigger_label: String,
    pub heading: String,
    pub step_label: Option<String>,
    pub primary_cta: Option<String>,
    pub secondary_cta: Option<String>,
    pub asset_variant: Option<CoachmarkAssetVariant>,
    pub asset_src: Option<String>,
    pub asset_label: String,
    pub asset_alt: String,
    pub lang: Option<String>,
    pub has_footer: bool,
    pub state: CoachmarkState,
    pub agent_contract: CoachmarkAgentContract,
}

fn resolve_agent_state(state_attr: &'static str, is_disabled: bool) -> CoachmarkAgentState {
    if is_disabled {
        return CoachmarkAgentState::Disabled;
    }
    match state_attr {
        "open" => CoachmarkAgentState::Open,
        "disabled" => CoachmarkAgentState::Disabled,
        _ => CoachmarkAgentState::Closed,
    }
}

pub fn resolve_agent_contract(input: CoachmarkAgentContractInput) -> CoachmarkAgentContract {
    let source = if input.open_mode_attr == "controlled" {
        CoachmarkAgentSource::External
    } else {
        CoachmarkAgentSource::Internal
    };
    let action = if input.has_footer {
        CoachmarkAgentAction::NavigateStep
    } else {
        CoachmarkAgentAction::ReadGuidance
    };
    let state = resolve_agent_state(input.state_attr, input.is_disabled);
    let output_status = if input.is_disabled {
        CoachmarkAgentOutputStatus::Draft
    } else {
        CoachmarkAgentOutputStatus::Verified
    };

    CoachmarkAgentContract {
        schema_name: COACHMARK_AGENT_SCHEMA,
        schema_version: CoachmarkAgentSchemaVersion::V1,
        intent: CoachmarkAgentIntent::GuidedTour,
        action,
        state,
        source,
        state_source: "ui-state-primitives::coachmark::resolve_state",
        action_source: if input.has_footer {
            "footer-actions"
        } else {
            "content-body"
        },
        render_path: "render_content_fragment + render_footer_fragment",
        output_status,
    }
}

pub fn resolve_view_model(input: CoachmarkViewModelInput) -> CoachmarkViewModel {
    let is_disabled = resolve_is_disabled(input.is_disabled, input.disabled);
    let normalized_aria_label = normalize_optional_text(input.aria_label);
    let normalized_class_name = normalize_optional_text(input.class_name);
    let normalized_primary_cta = normalize_optional_text(input.primary_cta);
    let normalized_secondary_cta = normalize_optional_text(input.secondary_cta);
    let normalized_asset_src = normalize_optional_text(input.asset_src);
    let normalized_asset_label = resolve_asset_label(input.asset_label);
    let normalized_asset_alt = resolve_asset_alt(input.asset_alt, &normalized_asset_label);
    let normalized_shortcut_key = normalize_optional_text(input.shortcut_key);
    let normalized_modifier_keys = normalize_modifier_keys(input.modifier_keys);
    let normalized_lang = normalize_optional_text(input.lang);

    let has_shortcut = normalized_shortcut_key.is_some();
    let heading = compose_heading(
        input.title,
        normalized_modifier_keys,
        normalized_shortcut_key,
    );
    let step_label = compose_step_label(input.current_step, input.total_steps);
    let cta_mode = resolve_cta_mode(
        normalized_primary_cta.as_deref(),
        normalized_secondary_cta.as_deref(),
    );
    let asset_source = resolve_asset_source(input.asset_variant, normalized_asset_src.as_deref());
    let (asset_variant, asset_src) = match asset_source {
        CoachmarkAssetSource::None => (None, None),
        CoachmarkAssetSource::Variant => (input.asset_variant, None),
        CoachmarkAssetSource::Image => (None, normalized_asset_src),
    };
    let has_footer = step_label.is_some()
        || normalized_primary_cta.is_some()
        || normalized_secondary_cta.is_some()
        || input.has_actions_slot;

    let state = resolve_state(CoachmarkStateInput {
        variant_attr: input.variant.as_attr(),
        placement_attr: input.placement.as_str(),
        disabled: is_disabled,
        is_controlled: input.is_controlled,
        has_footer,
        has_custom_aria_label: normalized_aria_label.is_some(),
        has_custom_class_name: normalized_class_name.is_some(),
        has_shortcut,
        cta_mode,
        has_actions_slot: input.has_actions_slot,
        has_step_label: step_label.is_some(),
        asset_source,
    });
    let agent_contract = resolve_agent_contract(CoachmarkAgentContractInput {
        has_footer,
        open_mode_attr: state.open_mode_attr,
        state_attr: state.state_attr,
        is_disabled,
    });

    CoachmarkViewModel {
        is_disabled,
        class_name: compose_class_name(normalized_class_name, state),
        trigger_label: resolve_trigger_label(normalized_aria_label, input.variant),
        heading,
        step_label,
        primary_cta: normalized_primary_cta,
        secondary_cta: normalized_secondary_cta,
        asset_variant,
        asset_src,
        asset_label: normalized_asset_label,
        asset_alt: normalized_asset_alt,
        lang: normalized_lang,
        has_footer,
        state,
        agent_contract,
    }
}

pub fn resolve_is_disabled(is_disabled: Option<bool>, disabled: Option<bool>) -> bool {
    is_disabled.or(disabled).unwrap_or(false)
}

pub fn resolve_trigger_label(
    normalized_aria_label: Option<String>,
    variant: CoachmarkVariant,
) -> String {
    normalized_aria_label.unwrap_or_else(|| variant.default_label().to_string())
}

pub fn resolve_asset_label(asset_label: Option<String>) -> String {
    normalize_optional_text(asset_label).unwrap_or_else(|| DEFAULT_ASSET_LABEL.into())
}

pub fn resolve_asset_alt(asset_alt: Option<String>, asset_label: &str) -> String {
    let normalized_alt: Cow<'_, str> = normalize_optional_text(asset_alt)
        .map(Cow::Owned)
        .unwrap_or_else(|| Cow::Borrowed(asset_label));
    normalized_alt.into_owned()
}

pub fn resolve_default_open(default_open: Option<bool>) -> bool {
    default_open.unwrap_or(false)
}

pub fn resolve_on_open_change(on_open_change: Option<Callback<bool>>) -> Callback<bool> {
    on_open_change.unwrap_or_else(|| Callback::new(|_: bool| {}))
}

pub fn resolve_on_press(on_press: Option<OnPress>) -> OnPress {
    on_press.unwrap_or_else(|| Callback::new(|()| {}))
}

const _: Option<CoachmarkState> = None;
const _: &str = DEFAULT_TITLE;
