use crate::{PopoverPartState, PopoverPartStateInput, PopoverSlot};
use leptos::prelude::{Callable, Callback, Signal};
use ui_headless::OnPress;
use ui_state_primitives::popover as popover_state;
use ui_state_primitives::popover::PopoverOpenMode;

#[cfg(any())]
const _POPOVER_COW_MARKER: &str = "Cow<'static, str>";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PopoverModalMode {
    Modal,
    NonModal,
}

impl PopoverModalMode {
    pub fn from_is_modal(is_modal: bool) -> Self {
        if is_modal {
            Self::Modal
        } else {
            Self::NonModal
        }
    }

    pub fn is_modal(self) -> bool {
        matches!(self, Self::Modal)
    }
}

pub fn state_attr_for_open(is_open: bool) -> &'static str {
    popover_state::state_attr_for_open(is_open)
}

pub fn modal_attr(is_modal: bool) -> &'static str {
    popover_state::modal_attr(is_modal)
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    popover_state::normalize_optional_text(value)
}

pub fn normalize_on_exit_complete(callback: Option<Callback<()>>) -> Callback<()> {
    callback.unwrap_or_else(|| Callback::new(|_| {}))
}

pub struct PopoverOpenStateInput {
    pub is_open: Option<Signal<bool>>,
    pub open: Option<Signal<bool>>,
    pub default_open: Option<bool>,
    pub on_open_change: Option<Callback<bool>>,
    pub on_close: Option<OnPress>,
}

#[derive(Clone)]
pub struct PopoverOpenState {
    pub open: Option<Signal<bool>>,
    pub default_open: bool,
    pub on_open_change: Option<Callback<bool>>,
    pub mode: PopoverOpenMode,
    pub has_custom_open: bool,
    pub has_custom_default_open: bool,
    pub has_custom_on_open_change: bool,
    pub has_custom_on_close: bool,
    pub open_state_source_attr: &'static str,
    pub open_prop_source_attr: &'static str,
    pub default_open_source_attr: &'static str,
    pub open_change_source_attr: &'static str,
}

pub fn normalize_open_state(input: PopoverOpenStateInput) -> PopoverOpenState {
    let has_custom_on_open_change = input.on_open_change.is_some();
    let has_custom_on_close = input.on_close.is_some();
    let open = input.is_open.or(input.open);
    let primitive_open_state =
        popover_state::resolve_open_state(popover_state::PopoverOpenStateInput {
            has_is_open_prop: input.is_open.is_some(),
            has_open_prop: input.open.is_some(),
            default_open: input.default_open,
            has_on_open_change: has_custom_on_open_change,
            has_on_close: has_custom_on_close,
        });

    // Legacy alias bridge: `on_close` maps to `on_open_change(false)`.
    let on_open_change = input.on_open_change.or_else(|| {
        input.on_close.map(|on_close| {
            Callback::new(move |is_open: bool| {
                if !is_open {
                    on_close.run(());
                }
            })
        })
    });

    PopoverOpenState {
        open,
        default_open: primitive_open_state.default_open,
        on_open_change,
        mode: primitive_open_state.mode,
        has_custom_open: open.is_some(),
        has_custom_default_open: primitive_open_state.has_default_open,
        has_custom_on_open_change,
        has_custom_on_close,
        open_state_source_attr: primitive_open_state.open_source_attr,
        open_prop_source_attr: primitive_open_state.open_prop_source_attr,
        default_open_source_attr: if primitive_open_state.has_default_open {
            "default_open"
        } else {
            "none"
        },
        open_change_source_attr: primitive_open_state.open_change_source_attr,
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PopoverStateInputs {
    pub open: bool,
    pub modal_mode: PopoverModalMode,
    pub has_custom_class_name: bool,
    pub has_custom_motion: bool,
    pub has_custom_placement: bool,
    pub has_on_exit_complete: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PopoverResolvedStates {
    pub root_state: PopoverPartState,
    pub panel_state: PopoverPartState,
}

pub fn resolve_states(input: PopoverStateInputs) -> PopoverResolvedStates {
    let state_for_slot = |slot, open, has_custom_class_name| {
        resolve_state(PopoverPartStateInput {
            slot,
            open,
            is_modal: input.modal_mode.is_modal(),
            has_custom_class_name,
            has_custom_motion: input.has_custom_motion,
            has_custom_placement: input.has_custom_placement,
            has_on_exit_complete: input.has_on_exit_complete,
        })
    };

    PopoverResolvedStates {
        root_state: state_for_slot(PopoverSlot::Root, input.open, input.has_custom_class_name),
        panel_state: state_for_slot(PopoverSlot::Panel, false, false),
    }
}

pub fn resolve_state(input: PopoverPartStateInput) -> PopoverPartState {
    popover_state::resolve_state(input)
}

pub fn compose_class_name(base_class_name: Option<String>, state: PopoverPartState) -> String {
    popover_state::compose_class_name(base_class_name, state)
}

pub fn compose_panel_vars(top_px: f64, left_px: f64, anchor_width_px: f64) -> String {
    format!(
        "--ui-popover-top: {top_px}px; --ui-popover-left: {left_px}px; --ui-popover-anchor-width: {anchor_width_px}px;"
    )
}

pub fn should_close_on_escape(
    key: &str,
    is_topmost: bool,
    is_composing: bool,
    default_prevented: bool,
) -> bool {
    ui_headless::should_dismiss_popover_on_escape(key, is_topmost, is_composing, default_prevented)
}

#[cfg(test)]
#[path = "test/logic.rs"]
mod tests;
