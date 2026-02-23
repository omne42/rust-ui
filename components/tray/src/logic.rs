use crate::OnPress;
use crate::tray::TrayMotion;
use leptos::prelude::{Callback, Signal};

pub use ui_state_primitives::tray::{
    DEFAULT_DISMISSABLE, DEFAULT_FIXED_HEIGHT, DEFAULT_ID_BASE, DEFAULT_KEYBOARD_DISMISS_DISABLED,
    DEFAULT_SHOW_CLOSE_BUTTON, DEFAULT_TITLE, TrayOpenConfigInput, TrayOpenMode, TrayPartState,
    TraySlot, can_request_open_change, normalize_id_base, normalize_optional_text,
    normalize_required_text, resolve_open_config, resolve_state,
};

pub const DEFAULT_CLOSE_LABEL: &str = "Close tray";

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TrayDefaultsInput {
    pub(crate) is_show_close_button: Option<bool>,
    pub(crate) close_label: Option<&'static str>,
    pub(crate) is_fixed_height: Option<bool>,
    pub(crate) is_dismissable: Option<bool>,
    pub(crate) is_keyboard_dismiss_disabled: Option<bool>,
    pub(crate) motion: Option<TrayMotion>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TrayDefaults {
    pub(crate) is_show_close_button: bool,
    pub(crate) close_label: &'static str,
    pub(crate) is_fixed_height: bool,
    pub(crate) is_dismissable: bool,
    pub(crate) is_keyboard_dismiss_disabled: bool,
    pub(crate) motion: TrayMotion,
}

pub fn normalize_defaults(input: TrayDefaultsInput) -> TrayDefaults {
    TrayDefaults {
        is_show_close_button: input
            .is_show_close_button
            .unwrap_or(DEFAULT_SHOW_CLOSE_BUTTON),
        close_label: input.close_label.unwrap_or(DEFAULT_CLOSE_LABEL),
        is_fixed_height: input.is_fixed_height.unwrap_or(DEFAULT_FIXED_HEIGHT),
        is_dismissable: input.is_dismissable.unwrap_or(DEFAULT_DISMISSABLE),
        is_keyboard_dismiss_disabled: input
            .is_keyboard_dismiss_disabled
            .unwrap_or(DEFAULT_KEYBOARD_DISMISS_DISABLED),
        motion: input.motion.unwrap_or_default(),
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TrayTextInput {
    pub(crate) id_base: String,
    pub(crate) title: String,
    pub(crate) description: Option<String>,
    pub(crate) class_name: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TrayText {
    pub(crate) id_base: String,
    pub(crate) title: String,
    pub(crate) description: Option<String>,
    pub(crate) class_name: Option<String>,
    pub(crate) has_custom_id_base: bool,
    pub(crate) has_custom_title: bool,
}

pub fn normalize_text(input: TrayTextInput) -> TrayText {
    let has_custom_id_base = !input.id_base.trim().is_empty();
    let has_custom_title = !input.title.trim().is_empty();

    TrayText {
        id_base: normalize_id_base(input.id_base),
        title: normalize_required_text(input.title, DEFAULT_TITLE),
        description: normalize_optional_text(input.description),
        class_name: normalize_optional_text(input.class_name),
        has_custom_id_base,
        has_custom_title,
    }
}

pub fn normalize_on_exit_complete(callback: Option<Callback<()>>) -> Callback<()> {
    callback.unwrap_or_else(|| Callback::new(|_| {}))
}

pub fn normalize_on_open_change(callback: Option<Callback<bool>>) -> Callback<bool> {
    callback.unwrap_or_else(|| Callback::new(|_| {}))
}

pub struct TrayOpenStateInput {
    pub(crate) is_open: Option<Signal<bool>>,
    pub(crate) default_open: Option<bool>,
    pub(crate) on_open_change: Option<Callback<bool>>,
}

#[derive(Clone)]
pub struct TrayOpenState {
    pub(crate) open: Option<Signal<bool>>,
    pub(crate) default_open: bool,
    pub(crate) on_open_change: Callback<bool>,
    pub(crate) mode: TrayOpenMode,
    pub(crate) has_open_change_handler: bool,
    pub(crate) open_source_attr: &'static str,
}

pub fn normalize_open_state(input: TrayOpenStateInput) -> TrayOpenState {
    let open_config = resolve_open_config(TrayOpenConfigInput {
        has_open: input.is_open.is_some(),
        default_open: input.default_open,
        has_on_open_change: input.on_open_change.is_some(),
    });

    TrayOpenState {
        open: input.is_open,
        default_open: open_config.default_open,
        on_open_change: normalize_on_open_change(input.on_open_change),
        mode: open_config.mode,
        has_open_change_handler: open_config.has_open_change_handler,
        open_source_attr: open_config.open_source_attr,
    }
}

pub fn resolve_open_signal(open: Option<Signal<bool>>, fallback: Signal<bool>) -> Signal<bool> {
    open.unwrap_or(fallback)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TrayCloseEffects {
    pub(crate) should_close_uncontrolled: bool,
    pub(crate) should_emit_open_change: bool,
}

pub fn resolve_close_effects(
    mode: TrayOpenMode,
    has_open_change_handler: bool,
) -> TrayCloseEffects {
    TrayCloseEffects {
        should_close_uncontrolled: mode == TrayOpenMode::Uncontrolled,
        should_emit_open_change: can_request_open_change(mode, has_open_change_handler),
    }
}

pub fn normalize_on_close(callback: Option<OnPress>) -> OnPress {
    callback.unwrap_or_else(|| Callback::new(|_| {}))
}

pub fn normalize_optional_attr(value: Option<String>) -> String {
    value.unwrap_or_default()
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TrayDescriptionMode {
    WithDescription,
    TitleOnly,
}

impl TrayDescriptionMode {
    pub fn from_has_description(has_description: bool) -> Self {
        if has_description {
            Self::WithDescription
        } else {
            Self::TitleOnly
        }
    }

    pub fn has_description(self) -> bool {
        matches!(self, Self::WithDescription)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TrayFooterMode {
    WithFooter,
    NoFooter,
}

impl TrayFooterMode {
    pub fn from_has_footer(has_footer: bool) -> Self {
        if has_footer {
            Self::WithFooter
        } else {
            Self::NoFooter
        }
    }

    pub fn has_footer(self) -> bool {
        matches!(self, Self::WithFooter)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TrayCloseButtonMode {
    Shown,
    Hidden,
}

impl TrayCloseButtonMode {
    pub fn from_show_close_button(show_close_button: bool) -> Self {
        if show_close_button {
            Self::Shown
        } else {
            Self::Hidden
        }
    }

    pub fn show_close_button(self) -> bool {
        matches!(self, Self::Shown)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TraySizeMode {
    FixedHeight,
    AutoHeight,
}

impl TraySizeMode {
    pub fn from_is_fixed_height(is_fixed_height: bool) -> Self {
        if is_fixed_height {
            Self::FixedHeight
        } else {
            Self::AutoHeight
        }
    }

    pub fn is_fixed_height(self) -> bool {
        matches!(self, Self::FixedHeight)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TrayDismissMode {
    Dismissable,
    Locked,
}

impl TrayDismissMode {
    #[cfg(test)]
    pub fn from_is_dismissable(is_dismissable: bool) -> Self {
        if is_dismissable {
            Self::Dismissable
        } else {
            Self::Locked
        }
    }

    pub fn is_dismissable(self) -> bool {
        matches!(self, Self::Dismissable)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TrayKeyboardDismissMode {
    Enabled,
    Disabled,
}

impl TrayKeyboardDismissMode {
    #[cfg(test)]
    pub fn from_is_disabled(is_keyboard_dismiss_disabled: bool) -> Self {
        if is_keyboard_dismiss_disabled {
            Self::Disabled
        } else {
            Self::Enabled
        }
    }

    pub fn is_disabled(self) -> bool {
        matches!(self, Self::Disabled)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TrayDismissPolicy {
    Locked,
    DismissableKeyboardEnabled,
    DismissableKeyboardDisabled,
}

impl TrayDismissPolicy {
    pub fn from_flags(is_dismissable: bool, is_keyboard_dismiss_disabled: bool) -> Self {
        if !is_dismissable {
            Self::Locked
        } else if is_keyboard_dismiss_disabled {
            Self::DismissableKeyboardDisabled
        } else {
            Self::DismissableKeyboardEnabled
        }
    }

    pub fn dismiss_mode(self) -> TrayDismissMode {
        match self {
            Self::Locked => TrayDismissMode::Locked,
            Self::DismissableKeyboardEnabled | Self::DismissableKeyboardDisabled => {
                TrayDismissMode::Dismissable
            }
        }
    }

    pub fn keyboard_dismiss_mode(self) -> TrayKeyboardDismissMode {
        match self {
            Self::DismissableKeyboardDisabled => TrayKeyboardDismissMode::Disabled,
            Self::Locked | Self::DismissableKeyboardEnabled => TrayKeyboardDismissMode::Enabled,
        }
    }
}

pub fn resolve_dismiss_policy(
    is_dismissable: bool,
    is_keyboard_dismiss_disabled: bool,
) -> TrayDismissPolicy {
    TrayDismissPolicy::from_flags(is_dismissable, is_keyboard_dismiss_disabled)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TrayStateInputs {
    pub description_mode: TrayDescriptionMode,
    pub footer_mode: TrayFooterMode,
    pub close_button_mode: TrayCloseButtonMode,
    pub size_mode: TraySizeMode,
    pub dismiss_mode: TrayDismissMode,
    pub keyboard_dismiss_mode: TrayKeyboardDismissMode,
    pub has_custom_id_base: bool,
    pub has_custom_title: bool,
    pub has_custom_description: bool,
    pub has_custom_class_name: bool,
    pub has_custom_motion: bool,
    pub has_on_exit_complete: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TrayStateBoundaryInput {
    pub(crate) has_description: bool,
    pub(crate) has_footer: bool,
    pub(crate) is_show_close_button: bool,
    pub(crate) is_fixed_height: bool,
    pub(crate) dismiss_policy: TrayDismissPolicy,
    pub(crate) has_custom_id_base: bool,
    pub(crate) has_custom_title: bool,
    pub(crate) has_custom_description: bool,
    pub(crate) has_custom_class_name: bool,
    pub(crate) has_custom_motion: bool,
    pub(crate) has_on_exit_complete: bool,
}

pub fn normalize_state_inputs(input: TrayStateBoundaryInput) -> TrayStateInputs {
    TrayStateInputs {
        description_mode: TrayDescriptionMode::from_has_description(input.has_description),
        footer_mode: TrayFooterMode::from_has_footer(input.has_footer),
        close_button_mode: TrayCloseButtonMode::from_show_close_button(input.is_show_close_button),
        size_mode: TraySizeMode::from_is_fixed_height(input.is_fixed_height),
        dismiss_mode: input.dismiss_policy.dismiss_mode(),
        keyboard_dismiss_mode: input.dismiss_policy.keyboard_dismiss_mode(),
        has_custom_id_base: input.has_custom_id_base,
        has_custom_title: input.has_custom_title,
        has_custom_description: input.has_custom_description,
        has_custom_class_name: input.has_custom_class_name,
        has_custom_motion: input.has_custom_motion,
        has_on_exit_complete: input.has_on_exit_complete,
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TrayResolvedStates {
    pub root_state: TrayPartState,
    pub header_state: TrayPartState,
    pub title_state: TrayPartState,
    pub description_state: TrayPartState,
    pub body_state: TrayPartState,
    pub footer_state: TrayPartState,
    pub close_state: TrayPartState,
}

pub fn resolve_states(input: TrayStateInputs) -> TrayResolvedStates {
    let state_for_slot = |slot, has_custom_class_name| {
        resolve_state(ui_state_primitives::tray::TrayPartStateInput {
            slot,
            has_description: input.description_mode.has_description(),
            has_footer: input.footer_mode.has_footer(),
            show_close_button: input.close_button_mode.show_close_button(),
            is_fixed_height: input.size_mode.is_fixed_height(),
            is_dismissable: input.dismiss_mode.is_dismissable(),
            is_keyboard_dismiss_disabled: input.keyboard_dismiss_mode.is_disabled(),
            has_custom_id_base: input.has_custom_id_base,
            has_custom_title: input.has_custom_title,
            has_custom_description: input.has_custom_description,
            has_custom_class_name,
            has_custom_motion: input.has_custom_motion,
            has_on_exit_complete: input.has_on_exit_complete,
        })
    };

    TrayResolvedStates {
        root_state: state_for_slot(TraySlot::Root, input.has_custom_class_name),
        header_state: state_for_slot(TraySlot::Header, false),
        title_state: state_for_slot(TraySlot::Title, false),
        description_state: state_for_slot(TraySlot::Description, false),
        body_state: state_for_slot(TraySlot::Body, false),
        footer_state: state_for_slot(TraySlot::Footer, false),
        close_state: state_for_slot(TraySlot::Close, false),
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: TrayPartState) -> String {
    std::hint::black_box(DEFAULT_ID_BASE);
    let mut classes = vec![state.base_class.into()];

    if state.slot == TraySlot::Root {
        if state.show_description {
            classes.push("ui-tray--with-description".to_string());
        } else {
            classes.push("ui-tray--title-only".to_string());
        }

        if state.show_footer {
            classes.push("ui-tray--with-footer".to_string());
        } else {
            classes.push("ui-tray--no-footer".to_string());
        }

        if state.show_close_button {
            classes.push("ui-tray--close-shown".to_string());
        } else {
            classes.push("ui-tray--close-hidden".to_string());
        }

        if state.is_fixed_height {
            classes.push("ui-tray--fixed-height".to_string());
        } else {
            classes.push("ui-tray--auto-height".to_string());
        }

        if state.has_custom_id_base {
            classes.push("ui-tray--custom-id".to_string());
        }

        if state.has_custom_title {
            classes.push("ui-tray--custom-title".to_string());
        }

        if state.has_custom_description {
            classes.push("ui-tray--custom-description".to_string());
        }

        if state.footer_source_attr == "custom" {
            classes.push("ui-tray--custom-footer".to_string());
        }

        if state.close_source_attr == "custom" {
            classes.push("ui-tray--custom-close".to_string());
        }

        if state.size_source_attr == "custom" {
            classes.push("ui-tray--custom-size".to_string());
        }

        if state.has_custom_motion {
            classes.push("ui-tray--custom-motion".to_string());
        }

        if state.has_on_exit_complete {
            classes.push("ui-tray--custom-exit".to_string());
        }

        if state.dismiss_source_attr == "custom" {
            classes.push("ui-tray--custom-dismiss".to_string());
        }

        if state.keyboard_dismiss_source_attr == "custom" {
            classes.push("ui-tray--custom-keyboard-dismiss".to_string());
        }

        if state.has_custom_class_name {
            classes.push("ui-tray--custom-class".to_string());
            if let Some(base_class_name) = base_class_name {
                classes.push(base_class_name);
            }
        }
    }

    classes.join(" ")
}

#[cfg(test)]
#[path = "../test/logic.rs"]
mod tests;
