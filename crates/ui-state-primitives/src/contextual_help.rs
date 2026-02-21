#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ContextualHelpOpenConfigInput {
    pub has_custom_open: bool,
    pub default_open: Option<bool>,
    pub has_custom_on_open_change: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ContextualHelpOpenConfig {
    pub default_open: Option<bool>,
    pub has_custom_open: bool,
    pub has_custom_default_open: bool,
    pub has_custom_on_open_change: bool,
    pub is_controlled: bool,
}

pub fn resolve_open_config(input: ContextualHelpOpenConfigInput) -> ContextualHelpOpenConfig {
    ContextualHelpOpenConfig {
        // Controlled `open` stays the single source of truth.
        // `default_open` only seeds uncontrolled mode.
        default_open: if input.has_custom_open {
            None
        } else {
            input.default_open
        },
        has_custom_open: input.has_custom_open,
        has_custom_default_open: input.default_open.is_some(),
        has_custom_on_open_change: input.has_custom_on_open_change,
        is_controlled: input.has_custom_open,
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ContextualHelpOpenInteractionSource {
    #[default]
    Initial,
    TriggerPress,
    DismissPress,
    ExternalSync,
}

impl ContextualHelpOpenInteractionSource {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::Initial => "initial",
            Self::TriggerPress => "trigger-press",
            Self::DismissPress => "dismiss-press",
            Self::ExternalSync => "external-sync",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ContextualHelpOpenInteractionIntent {
    TriggerPress,
    DismissPress,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ContextualHelpOpenInteractionIntentOutput {
    pub next_source: ContextualHelpOpenInteractionSource,
    pub has_pending_local_open_change: bool,
}

pub fn resolve_open_interaction_intent(
    intent: ContextualHelpOpenInteractionIntent,
) -> ContextualHelpOpenInteractionIntentOutput {
    let next_source = match intent {
        ContextualHelpOpenInteractionIntent::TriggerPress => {
            ContextualHelpOpenInteractionSource::TriggerPress
        }
        ContextualHelpOpenInteractionIntent::DismissPress => {
            ContextualHelpOpenInteractionSource::DismissPress
        }
    };

    ContextualHelpOpenInteractionIntentOutput {
        next_source,
        has_pending_local_open_change: true,
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ContextualHelpOpenInteractionSyncInput {
    pub previous_open: bool,
    pub current_open: bool,
    pub current_source: ContextualHelpOpenInteractionSource,
    pub has_pending_local_open_change: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ContextualHelpOpenInteractionSyncOutput {
    pub next_previous_open: bool,
    pub next_source: ContextualHelpOpenInteractionSource,
    pub has_pending_local_open_change: bool,
}

pub fn resolve_open_interaction_sync(
    input: ContextualHelpOpenInteractionSyncInput,
) -> ContextualHelpOpenInteractionSyncOutput {
    if input.current_open != input.previous_open {
        if input.has_pending_local_open_change {
            return ContextualHelpOpenInteractionSyncOutput {
                next_previous_open: input.current_open,
                next_source: input.current_source,
                has_pending_local_open_change: false,
            };
        }

        return ContextualHelpOpenInteractionSyncOutput {
            next_previous_open: input.current_open,
            next_source: ContextualHelpOpenInteractionSource::ExternalSync,
            has_pending_local_open_change: false,
        };
    }

    ContextualHelpOpenInteractionSyncOutput {
        next_previous_open: input.previous_open,
        next_source: input.current_source,
        has_pending_local_open_change: input.has_pending_local_open_change,
    }
}

#[cfg(test)]
#[path = "test/contextual_help.rs"]
mod tests;
