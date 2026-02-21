use leptos::prelude::Callback;

pub use ui_state_primitives::tray::{
    DEFAULT_DISMISSABLE, DEFAULT_FIXED_HEIGHT, DEFAULT_ID_BASE, DEFAULT_KEYBOARD_DISMISS_DISABLED,
    DEFAULT_SHOW_CLOSE_BUTTON, DEFAULT_TITLE, TrayPartState, TraySlot, normalize_id_base,
    normalize_optional_text, normalize_required_text, resolve_state,
};

pub const DEFAULT_CLOSE_LABEL: &str = "Close tray";

pub fn normalize_on_exit_complete(callback: Option<Callback<()>>) -> Callback<()> {
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
