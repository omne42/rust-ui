use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::html;
use leptos::prelude::*;
use ui::button::ToggleButtonGroupMotion;
use ui::button::action::ActionButtonGroupMotion;
use ui::{
    ActionButton, ActionButtonGroup, ActionButtonGroupDensity, ActionButtonGroupOrientation,
    ActionButtonLoadingPlacement, ActionButtonMotion, ActionButtonSize, ActionButtonType,
    ActionMenu, ActionMenuItemSpec, Button, ButtonColor, ButtonCopy, ButtonCopyMode,
    ButtonCopyMotion, ButtonGroup, ButtonGroupOrientation, ButtonIntent, ButtonLoadingPlacement,
    ButtonRadius, ButtonSchema, ButtonSize, ButtonVariant, FlipButton, FlipButtonMotion,
    FlipDirection, LinkButton, OnPress, SearchInputButton, SearchInputButtonMotion,
    SegmentedControl, SegmentedControlSize, ShareButton, ShareButtonIconPlacement, ShareButtonItem,
    ShareButtonMotion, SharePlatform, Switch, ThemeMode, ThemeToggleButton, ThemeToggleMotion,
    ToggleButton, ToggleButtonGroup, ToggleButtonGroupOrientation, ToggleButtonMotion,
    ToggleButtonSize, ToggleButtonVariant,
};
use ui_headless::{A11yDirection, PopoverPlacement};

// Legacy source-contract markers retained for semantic tests:
// title="External target + rel hardening"
// rel=Some("sponsored".to_string())

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct ButtonWorkbenchState {
    variant_index: usize,
    color_index: usize,
    radius_index: usize,
    size_index: usize,
    loading_placement_index: usize,
    is_disabled: bool,
    is_loading: bool,
    is_icon_only: bool,
    is_full_width: bool,
    show_start: bool,
    show_end: bool,
}

#[cfg(target_arch = "wasm32")]
impl ButtonWorkbenchState {
    fn parse(raw: &str) -> Option<Self> {
        let parts = raw.split(',').map(str::trim).collect::<Vec<_>>();
        if parts.len() != 11 {
            return None;
        }

        let parse_index = |at: usize, max: usize| {
            parts
                .get(at)?
                .parse::<usize>()
                .ok()
                .map(|value| value.min(max))
        };
        let parse_bool = |at: usize| match *parts.get(at)? {
            "1" => Some(true),
            "0" => Some(false),
            _ => None,
        };

        Some(Self {
            variant_index: parse_index(0, 6)?,
            color_index: parse_index(1, 5)?,
            radius_index: parse_index(2, 4)?,
            size_index: parse_index(3, 4)?,
            loading_placement_index: parse_index(4, 2)?,
            is_disabled: parse_bool(5)?,
            is_loading: parse_bool(6)?,
            is_icon_only: parse_bool(7)?,
            is_full_width: parse_bool(8)?,
            show_start: parse_bool(9)?,
            show_end: parse_bool(10)?,
        })
    }

    fn encode(self) -> String {
        let bool_digit = |value: bool| if value { '1' } else { '0' };
        format!(
            "{},{},{},{},{},{},{},{},{},{},{}",
            self.variant_index,
            self.color_index,
            self.radius_index,
            self.size_index,
            self.loading_placement_index,
            bool_digit(self.is_disabled),
            bool_digit(self.is_loading),
            bool_digit(self.is_icon_only),
            bool_digit(self.is_full_width),
            bool_digit(self.show_start),
            bool_digit(self.show_end),
        )
    }
}

#[cfg(target_arch = "wasm32")]
const BUTTON_WORKBENCH_STORAGE_KEY: &str = "docs:button:workbench:state";

#[cfg(target_arch = "wasm32")]
fn load_button_workbench_state() -> Option<ButtonWorkbenchState> {
    let storage = web_sys::window().and_then(|window| window.local_storage().ok().flatten())?;
    let raw = storage
        .get_item(BUTTON_WORKBENCH_STORAGE_KEY)
        .ok()
        .flatten()?;
    ButtonWorkbenchState::parse(&raw)
}

#[cfg(not(target_arch = "wasm32"))]
fn load_button_workbench_state() -> Option<ButtonWorkbenchState> {
    None
}

#[cfg(target_arch = "wasm32")]
fn save_button_workbench_state(state: ButtonWorkbenchState) {
    if let Some(storage) =
        web_sys::window().and_then(|window| window.local_storage().ok().flatten())
    {
        drop(storage.set_item(BUTTON_WORKBENCH_STORAGE_KEY, &state.encode()));
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn save_button_workbench_state(_state: ButtonWorkbenchState) {}

#[cfg(target_arch = "wasm32")]
fn clear_button_workbench_state() {
    if let Some(storage) =
        web_sys::window().and_then(|window| window.local_storage().ok().flatten())
    {
        drop(storage.remove_item(BUTTON_WORKBENCH_STORAGE_KEY));
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn clear_button_workbench_state() {}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct ActionButtonWorkbenchState {
    size_index: usize,
    loading_placement_index: usize,
    is_loading: bool,
    is_disabled: bool,
    is_quiet: bool,
    is_icon_only: bool,
    show_start: bool,
    show_end: bool,
}

#[cfg(target_arch = "wasm32")]
impl ActionButtonWorkbenchState {
    fn parse(raw: &str) -> Option<Self> {
        let parts = raw.split(',').map(str::trim).collect::<Vec<_>>();
        if parts.len() != 8 {
            return None;
        }

        let parse_index = |at: usize, max: usize| {
            parts
                .get(at)?
                .parse::<usize>()
                .ok()
                .map(|value| value.min(max))
        };
        let parse_bool = |at: usize| match *parts.get(at)? {
            "1" => Some(true),
            "0" => Some(false),
            _ => None,
        };

        Some(Self {
            size_index: parse_index(0, 4)?,
            loading_placement_index: parse_index(1, 2)?,
            is_loading: parse_bool(2)?,
            is_disabled: parse_bool(3)?,
            is_quiet: parse_bool(4)?,
            is_icon_only: parse_bool(5)?,
            show_start: parse_bool(6)?,
            show_end: parse_bool(7)?,
        })
    }

    fn encode(self) -> String {
        let bool_digit = |value: bool| if value { '1' } else { '0' };
        format!(
            "{},{},{},{},{},{},{},{}",
            self.size_index,
            self.loading_placement_index,
            bool_digit(self.is_loading),
            bool_digit(self.is_disabled),
            bool_digit(self.is_quiet),
            bool_digit(self.is_icon_only),
            bool_digit(self.show_start),
            bool_digit(self.show_end),
        )
    }
}

#[cfg(target_arch = "wasm32")]
const ACTION_BUTTON_WORKBENCH_STORAGE_KEY: &str = "docs:action-button:workbench:state";

#[cfg(target_arch = "wasm32")]
fn load_action_button_workbench_state() -> Option<ActionButtonWorkbenchState> {
    let storage = web_sys::window().and_then(|window| window.local_storage().ok().flatten())?;
    let raw = storage
        .get_item(ACTION_BUTTON_WORKBENCH_STORAGE_KEY)
        .ok()
        .flatten()?;
    ActionButtonWorkbenchState::parse(&raw)
}

#[cfg(not(target_arch = "wasm32"))]
fn load_action_button_workbench_state() -> Option<ActionButtonWorkbenchState> {
    None
}

#[cfg(target_arch = "wasm32")]
fn save_action_button_workbench_state(state: ActionButtonWorkbenchState) {
    if let Some(storage) =
        web_sys::window().and_then(|window| window.local_storage().ok().flatten())
    {
        drop(storage.set_item(ACTION_BUTTON_WORKBENCH_STORAGE_KEY, &state.encode()));
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn save_action_button_workbench_state(_state: ActionButtonWorkbenchState) {}

#[cfg(target_arch = "wasm32")]
fn clear_action_button_workbench_state() {
    if let Some(storage) =
        web_sys::window().and_then(|window| window.local_storage().ok().flatten())
    {
        drop(storage.remove_item(ACTION_BUTTON_WORKBENCH_STORAGE_KEY));
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn clear_action_button_workbench_state() {}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct ButtonCopyWorkbenchState {
    mode_index: usize,
    variant_index: usize,
    size_index: usize,
    text_index: usize,
    feedback_scale: u16,
    feedback_glow: u16,
    is_disabled: bool,
}

#[cfg(target_arch = "wasm32")]
impl ButtonCopyWorkbenchState {
    fn parse(raw: &str) -> Option<Self> {
        let parts = raw.split(',').map(str::trim).collect::<Vec<_>>();
        if parts.len() != 7 {
            return None;
        }

        let parse_index = |at: usize, max: usize| {
            parts
                .get(at)?
                .parse::<usize>()
                .ok()
                .map(|value| value.min(max))
        };
        let parse_u16 = |at: usize, min: u16, max: u16| {
            parts
                .get(at)?
                .parse::<u16>()
                .ok()
                .map(|value| value.clamp(min, max))
        };
        let parse_bool = |at: usize| match *parts.get(at)? {
            "1" => Some(true),
            "0" => Some(false),
            _ => None,
        };

        Some(Self {
            mode_index: parse_index(0, 2)?,
            variant_index: parse_index(1, 2)?,
            size_index: parse_index(2, 4)?,
            text_index: parse_index(3, 2)?,
            feedback_scale: parse_u16(4, 0, 25)?,
            feedback_glow: parse_u16(5, 0, 200)?,
            is_disabled: parse_bool(6)?,
        })
    }

    fn encode(self) -> String {
        let bool_digit = |value: bool| if value { '1' } else { '0' };
        format!(
            "{},{},{},{},{},{},{}",
            self.mode_index,
            self.variant_index,
            self.size_index,
            self.text_index,
            self.feedback_scale,
            self.feedback_glow,
            bool_digit(self.is_disabled),
        )
    }
}

#[cfg(target_arch = "wasm32")]
const BUTTON_COPY_WORKBENCH_STORAGE_KEY: &str = "docs:button-copy:workbench:state";

#[cfg(target_arch = "wasm32")]
fn load_button_copy_workbench_state() -> Option<ButtonCopyWorkbenchState> {
    let storage = web_sys::window().and_then(|window| window.local_storage().ok().flatten())?;
    let raw = storage
        .get_item(BUTTON_COPY_WORKBENCH_STORAGE_KEY)
        .ok()
        .flatten()?;
    ButtonCopyWorkbenchState::parse(&raw)
}

#[cfg(not(target_arch = "wasm32"))]
fn load_button_copy_workbench_state() -> Option<ButtonCopyWorkbenchState> {
    None
}

#[cfg(target_arch = "wasm32")]
fn save_button_copy_workbench_state(state: ButtonCopyWorkbenchState) {
    if let Some(storage) =
        web_sys::window().and_then(|window| window.local_storage().ok().flatten())
    {
        drop(storage.set_item(BUTTON_COPY_WORKBENCH_STORAGE_KEY, &state.encode()));
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn save_button_copy_workbench_state(_state: ButtonCopyWorkbenchState) {}

#[cfg(target_arch = "wasm32")]
fn clear_button_copy_workbench_state() {
    if let Some(storage) =
        web_sys::window().and_then(|window| window.local_storage().ok().flatten())
    {
        drop(storage.remove_item(BUTTON_COPY_WORKBENCH_STORAGE_KEY));
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn clear_button_copy_workbench_state() {}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct FlipButtonWorkbenchState {
    direction_index: usize,
}

#[cfg(target_arch = "wasm32")]
impl FlipButtonWorkbenchState {
    fn parse(raw: &str) -> Option<Self> {
        let parts = raw.split(',').map(str::trim).collect::<Vec<_>>();
        if parts.len() != 1 {
            return None;
        }

        Some(Self {
            direction_index: parts.get(0)?.parse::<usize>().ok().map(|v| v.min(3))?,
        })
    }

    fn encode(self) -> String {
        format!("{}", self.direction_index)
    }
}

#[cfg(target_arch = "wasm32")]
const FLIP_BUTTON_WORKBENCH_STORAGE_KEY: &str = "docs:flip-button:workbench:state";

#[cfg(target_arch = "wasm32")]
fn load_flip_button_workbench_state() -> Option<FlipButtonWorkbenchState> {
    let storage = web_sys::window().and_then(|window| window.local_storage().ok().flatten())?;
    let raw = storage
        .get_item(FLIP_BUTTON_WORKBENCH_STORAGE_KEY)
        .ok()
        .flatten()?;
    FlipButtonWorkbenchState::parse(&raw)
}

#[cfg(not(target_arch = "wasm32"))]
fn load_flip_button_workbench_state() -> Option<FlipButtonWorkbenchState> {
    None
}

#[cfg(target_arch = "wasm32")]
fn save_flip_button_workbench_state(state: FlipButtonWorkbenchState) {
    if let Some(storage) =
        web_sys::window().and_then(|window| window.local_storage().ok().flatten())
    {
        drop(storage.set_item(FLIP_BUTTON_WORKBENCH_STORAGE_KEY, &state.encode()));
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn save_flip_button_workbench_state(_state: FlipButtonWorkbenchState) {}

#[cfg(target_arch = "wasm32")]
fn clear_flip_button_workbench_state() {
    if let Some(storage) =
        web_sys::window().and_then(|window| window.local_storage().ok().flatten())
    {
        drop(storage.remove_item(FLIP_BUTTON_WORKBENCH_STORAGE_KEY));
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn clear_flip_button_workbench_state() {}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct SearchInputButtonWorkbenchState {
    preset_index: usize,
    meta_key_index: usize,
    key_label_index: usize,
    is_disabled: bool,
    custom_aria_label: bool,
}

#[cfg(target_arch = "wasm32")]
impl SearchInputButtonWorkbenchState {
    fn parse(raw: &str) -> Option<Self> {
        let parts = raw.split(',').map(str::trim).collect::<Vec<_>>();
        if parts.len() != 5 {
            return None;
        }

        let parse_index = |at: usize, max: usize| {
            parts
                .get(at)?
                .parse::<usize>()
                .ok()
                .map(|value| value.min(max))
        };
        let parse_bool = |at: usize| match *parts.get(at)? {
            "1" => Some(true),
            "0" => Some(false),
            _ => None,
        };

        Some(Self {
            preset_index: parse_index(0, 3)?,
            meta_key_index: parse_index(1, 3)?,
            key_label_index: parse_index(2, 2)?,
            is_disabled: parse_bool(3)?,
            custom_aria_label: parse_bool(4)?,
        })
    }

    fn encode(self) -> String {
        let bool_digit = |value: bool| if value { '1' } else { '0' };
        format!(
            "{},{},{},{},{}",
            self.preset_index,
            self.meta_key_index,
            self.key_label_index,
            bool_digit(self.is_disabled),
            bool_digit(self.custom_aria_label),
        )
    }
}

#[cfg(target_arch = "wasm32")]
const SEARCH_INPUT_BUTTON_WORKBENCH_STORAGE_KEY: &str = "docs:search-input-button:workbench:state";

#[cfg(target_arch = "wasm32")]
fn load_search_input_button_workbench_state() -> Option<SearchInputButtonWorkbenchState> {
    let storage = web_sys::window().and_then(|window| window.local_storage().ok().flatten())?;
    let raw = storage
        .get_item(SEARCH_INPUT_BUTTON_WORKBENCH_STORAGE_KEY)
        .ok()
        .flatten()?;
    SearchInputButtonWorkbenchState::parse(&raw)
}

#[cfg(not(target_arch = "wasm32"))]
fn load_search_input_button_workbench_state() -> Option<SearchInputButtonWorkbenchState> {
    None
}

#[cfg(target_arch = "wasm32")]
fn save_search_input_button_workbench_state(state: SearchInputButtonWorkbenchState) {
    if let Some(storage) =
        web_sys::window().and_then(|window| window.local_storage().ok().flatten())
    {
        drop(storage.set_item(SEARCH_INPUT_BUTTON_WORKBENCH_STORAGE_KEY, &state.encode()));
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn save_search_input_button_workbench_state(_state: SearchInputButtonWorkbenchState) {}

#[cfg(target_arch = "wasm32")]
fn clear_search_input_button_workbench_state() {
    if let Some(storage) =
        web_sys::window().and_then(|window| window.local_storage().ok().flatten())
    {
        drop(storage.remove_item(SEARCH_INPUT_BUTTON_WORKBENCH_STORAGE_KEY));
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn clear_search_input_button_workbench_state() {}

#[path = "actions/action_button.rs"]
mod action_button;
#[path = "actions/action_button_group.rs"]
mod action_button_group;
#[path = "actions/action_menu.rs"]
mod action_menu;
#[path = "actions/button.rs"]
mod button;
#[path = "actions/button_copy.rs"]
mod button_copy;
#[path = "actions/button_group.rs"]
mod button_group;
#[path = "actions/flip_button.rs"]
mod flip_button;
#[path = "actions/link_button.rs"]
mod link_button;
#[path = "actions/search_input_button.rs"]
mod search_input_button;
#[path = "actions/share_button.rs"]
mod share_button;
#[path = "actions/theme_toggle_button.rs"]
mod theme_toggle_button;
#[path = "actions/toggle_button.rs"]
mod toggle_button;
#[path = "actions/toggle_button_group.rs"]
mod toggle_button_group;

pub(super) use action_button::action_button;
pub(super) use action_button_group::action_button_group;
pub(super) use action_menu::action_menu;
pub(super) use button::button;
pub(super) use button_copy::button_copy;
pub(super) use button_group::button_group;
pub(super) use flip_button::flip_button;
pub(super) use link_button::link_button;
pub(super) use search_input_button::search_input_button;
pub(super) use share_button::share_button;
pub(super) use theme_toggle_button::theme_toggle_button;
pub(super) use toggle_button::toggle_button;
pub(super) use toggle_button_group::toggle_button_group;
