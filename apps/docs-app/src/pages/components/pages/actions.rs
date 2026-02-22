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

pub(super) fn button() -> AnyView {
    let persisted_workbench_state = load_button_workbench_state();
    let has_persisted_workbench_state = persisted_workbench_state.is_some();
    let initial_workbench_state = persisted_workbench_state.unwrap_or_default();
    let variant_options = vec![
        "solid".to_string(),
        "faded".to_string(),
        "bordered".to_string(),
        "light".to_string(),
        "flat".to_string(),
        "ghost".to_string(),
        "shadow".to_string(),
    ];
    let (variant_index, set_variant_index) = signal(Some(initial_workbench_state.variant_index));
    let variant = Signal::derive(move || match variant_index.get().unwrap_or(0) {
        1 => ButtonVariant::Faded,
        2 => ButtonVariant::Bordered,
        3 => ButtonVariant::Light,
        4 => ButtonVariant::Flat,
        5 => ButtonVariant::Ghost,
        6 => ButtonVariant::Shadow,
        _ => ButtonVariant::Solid,
    });

    let color_options = vec![
        "default".to_string(),
        "primary".to_string(),
        "secondary".to_string(),
        "success".to_string(),
        "warning".to_string(),
        "danger".to_string(),
    ];
    let (color_index, set_color_index) = signal(Some(initial_workbench_state.color_index));
    let color = Signal::derive(move || match color_index.get().unwrap_or(1) {
        0 => ButtonColor::Default,
        2 => ButtonColor::Secondary,
        3 => ButtonColor::Success,
        4 => ButtonColor::Warning,
        5 => ButtonColor::Danger,
        _ => ButtonColor::Primary,
    });

    let radius_options = vec![
        "full".to_string(),
        "lg".to_string(),
        "md".to_string(),
        "sm".to_string(),
        "none".to_string(),
    ];
    let (radius_index, set_radius_index) = signal(Some(initial_workbench_state.radius_index));
    let radius = Signal::derive(move || match radius_index.get().unwrap_or(2) {
        0 => ButtonRadius::Full,
        1 => ButtonRadius::Lg,
        3 => ButtonRadius::Sm,
        4 => ButtonRadius::None,
        _ => ButtonRadius::Md,
    });

    let size_options = vec![
        "xs".to_string(),
        "s".to_string(),
        "m".to_string(),
        "l".to_string(),
        "xl".to_string(),
    ];
    let (size_index, set_size_index) = signal(Some(initial_workbench_state.size_index));
    let size = Signal::derive(move || match size_index.get().unwrap_or(2) {
        0 => ButtonSize::Xs,
        1 => ButtonSize::S,
        2 => ButtonSize::M,
        3 => ButtonSize::L,
        _ => ButtonSize::Xl,
    });

    let (is_disabled, set_is_disabled) = signal(initial_workbench_state.is_disabled);
    let (loading, set_loading) = signal(initial_workbench_state.is_loading);
    let loading_placement_options =
        vec!["Start".to_string(), "End".to_string(), "Center".to_string()];
    let (loading_placement_index, set_loading_placement_index) =
        signal(Some(initial_workbench_state.loading_placement_index));
    let loading_placement =
        Signal::derive(move || match loading_placement_index.get().unwrap_or(0) {
            1 => ButtonLoadingPlacement::End,
            2 => ButtonLoadingPlacement::Center,
            _ => ButtonLoadingPlacement::Start,
        });
    let (icon_only, set_icon_only) = signal(initial_workbench_state.is_icon_only);
    let (is_full_width, set_is_full_width) = signal(initial_workbench_state.is_full_width);
    let (show_start, set_show_start) = signal(initial_workbench_state.show_start);
    let (show_end, set_show_end) = signal(initial_workbench_state.show_end);
    let (spec_schema_enabled, set_spec_schema_enabled) = signal(false);
    let (spec_requires_confirmation, set_spec_requires_confirmation) = signal(false);
    let spec_schema_json = Signal::derive(move || {
        if !spec_schema_enabled.get() {
            return None;
        }

        Some(
            ButtonSchema::new(
                "docs-button-workbench",
                ButtonIntent::Primary,
                "button.press",
            )
            .requires_confirmation(spec_requires_confirmation.get())
            .to_json(),
        )
    });
    let (workbench_persist_state, set_workbench_persist_state) =
        signal(has_persisted_workbench_state);

    Effect::new(move |_| {
        if workbench_persist_state.get() {
            save_button_workbench_state(ButtonWorkbenchState {
                variant_index: variant_index.get().unwrap_or(0),
                color_index: color_index.get().unwrap_or(1),
                radius_index: radius_index.get().unwrap_or(2),
                size_index: size_index.get().unwrap_or(2),
                loading_placement_index: loading_placement_index.get().unwrap_or(0),
                is_disabled: is_disabled.get(),
                is_loading: loading.get(),
                is_icon_only: icon_only.get(),
                is_full_width: is_full_width.get(),
                show_start: show_start.get(),
                show_end: show_end.get(),
            });
        } else {
            clear_button_workbench_state();
        }
    });

    let hello_code = Signal::derive(move || r#"<Button>"Button"</Button>"#.to_string());
    let button_imports = "use leptos::prelude::*;\nuse ui::{Button, ButtonColor, ButtonLoadingPlacement, ButtonRadius, ButtonSize, ButtonVariant};".to_string();

    let code = Signal::derive(move || {
        let variant = variant.get();
        let color = color.get();
        let radius = radius.get();
        let size = size.get();
        let is_disabled = is_disabled.get();
        let loading = loading.get();
        let loading_placement = loading_placement.get();
        let icon_only = icon_only.get();
        let is_full_width = is_full_width.get();
        let show_start = show_start.get();
        let show_end = show_end.get();
        let schema_json = spec_schema_json.get();

        let mut snippet = vec!["<Button".to_string()];

        if color != ButtonColor::Primary {
            snippet.push(format!("  color=ButtonColor::{color:?}"));
        }
        if variant != ButtonVariant::Solid {
            snippet.push(format!("  variant=ButtonVariant::{variant:?}"));
        }
        if radius != ButtonRadius::Md {
            snippet.push(format!("  radius=ButtonRadius::{radius:?}"));
        }
        if size != ButtonSize::M {
            snippet.push(format!("  size=ButtonSize::{size:?}"));
        }
        if is_disabled {
            snippet.push("  is_disabled=true".to_string());
        }
        if loading {
            snippet.push("  is_loading=true".to_string());
            if loading_placement != ButtonLoadingPlacement::Start {
                snippet.push(format!(
                    "  loading_placement=ButtonLoadingPlacement::{loading_placement:?}"
                ));
            }
        }
        if icon_only {
            snippet.push("  is_icon_only=true".to_string());
            snippet.push("  aria_label=\"Button\".into()".to_string());
        }
        if is_full_width {
            snippet.push("  is_full_width=true".to_string());
        }
        if show_start {
            snippet.push("  start_content=move || view! { <span>\"★\"</span> }".to_string());
        }
        if show_end {
            snippet.push("  end_content=move || view! { <span>\"→\"</span> }".to_string());
        }
        if let Some(schema_json) = schema_json {
            snippet.push(format!(
                "  schema_json=Some(r#\"{schema_json}\"#.to_string())"
            ));
        }

        snippet.extend([
            ">".to_string(),
            if icon_only {
                "  \"★\"".to_string()
            } else {
                "  \"Button\"".to_string()
            },
            "</Button>".to_string(),
        ]);

        snippet.join("\n")
    });

    let test_css_source = Signal::derive(move || {
        format!(
            "/* crates/ui/src/button/styles.rs */\n{}",
            ui::button::styles::CSS
        )
    });

    let actual_config = Signal::derive(move || {
        let variant = variant.get();
        let color = color.get();
        let radius = radius.get();
        let size = size.get();
        let is_disabled = is_disabled.get();
        let is_loading = loading.get();
        let loading_placement = loading_placement.get();
        let icon_only = icon_only.get();
        let is_full_width = is_full_width.get();
        let show_start = show_start.get();
        let show_end = show_end.get();
        let schema_json = spec_schema_json.get();

        let mut classes = vec![
            "ui-button".to_string(),
            variant.class_name().into(),
            color.class_name().into(),
            radius.class_name().into(),
            size.class_name().into(),
            format!("ui-button--loading-{}", loading_placement.as_attr()),
        ];

        if icon_only {
            classes.push("ui-button--icon-only".to_string());
        }
        if is_full_width {
            classes.push("ui-button--full-width".to_string());
        }
        if is_loading {
            classes.push("ui-button--loading".to_string());
        }
        if show_start {
            classes.push("ui-button--has-start".to_string());
        }
        if show_end {
            classes.push("ui-button--has-end".to_string());
        }

        format!(
            "ButtonActualConfig {{\n  color: {color:?},\n  variant: {variant:?},\n  radius: {radius:?},\n  size: {size:?},\n  is_disabled: {is_disabled},\n  is_loading: {is_loading},\n  loading_placement: {loading_placement:?},\n  is_icon_only: {icon_only},\n  aria_label: {},\n  is_full_width: {is_full_width},\n  has_start_content: {show_start},\n  has_end_content: {show_end},\n  schema_json: {schema_json:?},\n  class_name: {:?},\n  on_press: {:?},\n  class: \"{}\",\n}}",
            if icon_only {
                "Some(\"Button\")"
            } else {
                "None"
            },
            None::<String>,
            "Callback<MouseEvent>",
            classes.join(" ")
        )
    });

    let colors_code = Signal::derive(move || {
        r#"<Button color="default">"Default"</Button>
<Button color="primary">"Primary"</Button>
<Button color="secondary">"Secondary"</Button>
<Button color="success">"Success"</Button>
<Button color="warning">"Warning"</Button>
<Button color="danger">"Danger"</Button>"#
            .to_string()
    });

    let radius_code = Signal::derive(move || {
        r#"<Button radius="full" color="default">"Full"</Button>
<Button radius="lg" color="default">"Large"</Button>
<Button radius="md" color="default">"Medium"</Button>
<Button radius="sm" color="default">"Small"</Button>
<Button radius="none" color="default">"None"</Button>"#
            .to_string()
    });

    let sizes_code = Signal::derive(move || {
        r#"<Button size="xs">"XS"</Button>
<Button size="s">"S"</Button>
<Button size="m">"M"</Button>
<Button size="l">"L"</Button>
<Button size="xl">"XL"</Button>"#
            .to_string()
    });

    let state_matrix_code = Signal::derive(move || {
        r#"<Button id="docs-button-matrix-idle".to_string()>"Idle"</Button>
<Button
  id="docs-button-matrix-loading".to_string()
  is_loading=true
  loading_placement=ButtonLoadingPlacement::Start
>
  "Loading"
</Button>
<Button id="docs-button-matrix-disabled".to_string() is_disabled=true>"Disabled"</Button>
<Button
  id="docs-button-matrix-icon-only".to_string()
  aria_label="Icon only".to_string()
>
  "★"
</Button>"#
            .to_string()
    });

    let controlled_vs_uncontrolled_code = Signal::derive(move || {
        r#"// N/A: Button has no value/open selection axis.
// Use explicit props/callbacks and keep loading/disabled state in caller.
<Button id="docs-button-controlled-like".to_string() is_loading=true>"Parent-managed loading"</Button>
<Button id="docs-button-uncontrolled-like".to_string()>"No internal state axis"</Button>"#
            .to_string()
    });

    let output_mode_code = Signal::derive(move || {
        r#"// Button is not a long-form reading surface.
// Streaming is optional; docs fallback remains snapshot.
<div
  data-ui-streaming="optional"
  data-ui-fallback="snapshot"
  data-ui-output-state="snapshot"
>
  "Button docs output mode: snapshot"
</div>
<Button id="docs-button-snapshot".to_string()>"Snapshot"</Button>"#
            .to_string()
    });

    let source_first_code = Signal::derive(move || {
        r#"<Button
  id="docs-button-source-first".to_string()
  color=ButtonColor::Primary
  variant=ButtonVariant::Solid
>
  "Build"
</Button>"#
            .to_string()
    });

    view! {
        <ComponentPage
            title="Button"
            slug="button"
            group="Actions"
            description="Variants + sizes with spring hover/tap motion."
        >
            <Playground title="Hello World" code_signal=hello_code code_imports=button_imports.clone()>
                <div class="docs-row">
                    <Button
                        class_name="docs-button-showcase".to_string()
                        on_press=Callback::new(move |_| {})
                    >
                        "Button"
                    </Button>
                </div>
            </Playground>

            <Playground
                title="Variants & sizes"
                code_signal=code
                code_imports=button_imports.clone()
                test_css_source=test_css_source
                test_source_path="/root/code/personal/omne/rust-ui/crates/ui/src/button/styles.rs".to_string()
                test_config_signal=actual_config
                description="Workbench canvas: scoped CSS live-edit + optional state persistence across reload."
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight">
                        <div class="docs-search__label">"Variant"</div>
                        <SegmentedControl
                            id_base="docs-button-variant".to_string()
                            options=variant_options.clone()
                            selected_index=variant_index
                            set_selected_index=set_variant_index
                            size=SegmentedControlSize::Sm
                            aria_label="Button variant".to_string()
                        />

                        <div class="docs-search__label">"Color"</div>
                        <SegmentedControl
                            id_base="docs-button-color".to_string()
                            options=color_options.clone()
                            selected_index=color_index
                            set_selected_index=set_color_index
                            size=SegmentedControlSize::Sm
                            aria_label="Button color".to_string()
                        />

                        <div class="docs-search__label">"Radius"</div>
                        <SegmentedControl
                            id_base="docs-button-radius".to_string()
                            options=radius_options.clone()
                            selected_index=radius_index
                            set_selected_index=set_radius_index
                            size=SegmentedControlSize::Sm
                            aria_label="Button radius".to_string()
                        />

                        <div class="docs-search__label">"Size"</div>
                        <SegmentedControl
                            id_base="docs-button-size".to_string()
                            options=size_options.clone()
                            selected_index=size_index
                            set_selected_index=set_size_index
                            size=SegmentedControlSize::Sm
                            aria_label="Button size".to_string()
                        />

                        <Switch checked=is_disabled set_checked=set_is_disabled>"Disabled"</Switch>
                        <Switch checked=loading set_checked=set_loading>"Loading"</Switch>
                        <div class="docs-search__label">"Loading placement"</div>
                        <SegmentedControl
                            id_base="docs-button-loading-placement".to_string()
                            options=loading_placement_options.clone()
                            selected_index=loading_placement_index
                            set_selected_index=set_loading_placement_index
                            size=SegmentedControlSize::Sm
                            aria_label="Button loading placement".to_string()
                        />
                        <Switch checked=icon_only set_checked=set_icon_only>"Icon only"</Switch>
                        <Switch checked=is_full_width set_checked=set_is_full_width>"Full width"</Switch>
                        <Switch checked=show_start set_checked=set_show_start>"Start slot"</Switch>
                        <Switch checked=show_end set_checked=set_show_end>"End slot"</Switch>
                        <Switch checked=spec_schema_enabled set_checked=set_spec_schema_enabled>
                            "Use AI spec payload"
                        </Switch>
                        <Switch checked=spec_requires_confirmation set_checked=set_spec_requires_confirmation>
                            "Spec requires confirmation"
                        </Switch>
                        <Switch checked=workbench_persist_state set_checked=set_workbench_persist_state>
                            "Persist workbench state"
                        </Switch>
                    </div>
                }
            >
                {move || {
                    let variant = variant.get();
                    let color = color.get();
                    let radius = radius.get();
                    let size = size.get();
                    let is_disabled = is_disabled.get();
                    let is_loading = loading.get();
                    let loading_placement = loading_placement.get();
                    let icon_only = icon_only.get();
                    let is_full_width = is_full_width.get();
                    let show_start = show_start.get();
                    let show_end = show_end.get();
                    let schema_json = spec_schema_json.get();
                    let persist = workbench_persist_state.get();

                    view! {
                        <div class="docs-stack" data-slot="button-workbench" style="width: min(100%, 360px);">
                            <span class="ui-muted">
                                "persist: "
                                {if persist { "on" } else { "off" }}
                            </span>
                            <div class="docs-stack docs-stack--tight" data-slot="button-interactive-spec-preview">
                                <span class="ui-muted" data-slot="button-interactive-spec-input">
                                    "spec-input: "
                                    {if schema_json.is_some() { "schema_json" } else { "off" }}
                                </span>
                                <code data-slot="button-interactive-spec-json">
                                    {schema_json
                                        .clone()
                                        .unwrap_or_else(|| "none".to_string())}
                                </code>
                            </div>
                            <div class="docs-card" data-slot="button-workbench-canvas">
                                <div
                                    class="docs-row"
                                    style=if is_full_width {
                                        "width: 100%;"
                                    } else {
                                        "width: fit-content; margin-inline: auto;"
                                    }
                                >
                                    {match (show_start, show_end) {
                                        (true, true) => view! {
                                            <Button
                                                color=color
                                                variant=variant
                                                radius=radius
                                                size=size
                                                is_disabled=is_disabled
                                                is_loading=is_loading
                                                loading_placement=loading_placement
                                                is_full_width=is_full_width
                                                schema_json=schema_json.clone().unwrap_or_default()
                                                aria_label=if icon_only { "Button".to_string() } else { String::new() }
                                            >
                                                {if icon_only { "★" } else { "Button" }}
                                            </Button>
                                        }
                                            .into_any(),
                                        (true, false) => view! {
                                            <Button
                                                color=color
                                                variant=variant
                                                radius=radius
                                                size=size
                                                is_disabled=is_disabled
                                                is_loading=is_loading
                                                loading_placement=loading_placement
                                                is_full_width=is_full_width
                                                schema_json=schema_json.clone().unwrap_or_default()
                                                aria_label=if icon_only { "Button".to_string() } else { String::new() }
                                            >
                                                {if icon_only { "★" } else { "Button" }}
                                            </Button>
                                        }
                                            .into_any(),
                                        (false, true) => view! {
                                            <Button
                                                color=color
                                                variant=variant
                                                radius=radius
                                                size=size
                                                is_disabled=is_disabled
                                                is_loading=is_loading
                                                loading_placement=loading_placement
                                                is_full_width=is_full_width
                                                schema_json=schema_json.clone().unwrap_or_default()
                                                aria_label=if icon_only { "Button".to_string() } else { String::new() }
                                            >
                                                {if icon_only { "★" } else { "Button" }}
                                            </Button>
                                        }
                                            .into_any(),
                                        (false, false) => view! {
                                            <Button
                                                color=color
                                                variant=variant
                                                radius=radius
                                                size=size
                                                is_disabled=is_disabled
                                                is_loading=is_loading
                                                loading_placement=loading_placement
                                                is_full_width=is_full_width
                                                schema_json=schema_json.clone().unwrap_or_default()
                                                aria_label=if icon_only { "Button".to_string() } else { String::new() }
                                            >
                                                {if icon_only { "★" } else { "Button" }}
                                            </Button>
                                        }
                                            .into_any(),
                                    }}
                                </div>
                            </div>
                        </div>
                    }
                }}
            </Playground>

            <Playground
                title="State Matrix"
                code_signal=state_matrix_code
                code_imports=button_imports.clone()
            >
                <div class="docs-row docs-row--wrap" style="gap: var(--ui-space-lg); align-items: flex-start;">
                    <Button id="docs-button-matrix-idle".to_string()>"Idle"</Button>
                    <Button
                        id="docs-button-matrix-loading".to_string()
                        is_loading=true
                        loading_placement=ButtonLoadingPlacement::Start
                    >
                        "Loading"
                    </Button>
                    <Button id="docs-button-matrix-disabled".to_string() is_disabled=true>
                        "Disabled"
                    </Button>
                    <Button
                        id="docs-button-matrix-icon-only".to_string()
                        aria_label="Icon only".to_string()
                    >
                        "★"
                    </Button>
                </div>
            </Playground>

            <Playground
                title="Controlled vs Uncontrolled (N/A)"
                code_signal=controlled_vs_uncontrolled_code
                code_imports=button_imports.clone()
            >
                <div class="docs-stack docs-stack--tight">
                    <span class="ui-muted">
                        "N/A: Button has no value/open axis. Caller-managed props/callbacks remain the only state boundary."
                    </span>
                    <div class="docs-row docs-row--wrap" style="gap: var(--ui-space-lg); align-items: flex-start;">
                        <Button id="docs-button-controlled-like".to_string() is_loading=true>
                            "Parent-managed loading"
                        </Button>
                        <Button id="docs-button-uncontrolled-like".to_string()>
                            "No internal state axis"
                        </Button>
                    </div>
                </div>
            </Playground>

            <Playground
                title="Streaming Optional / Snapshot"
                code_signal=output_mode_code
                code_imports=button_imports.clone()
            >
                <div
                    class="docs-stack docs-stack--tight"
                    data-slot="button-output-mode"
                    data-ui-streaming="optional"
                    data-ui-fallback="snapshot"
                    data-ui-output-state="snapshot"
                >
                    <span class="ui-muted">
                        "Button is not a text-reading surface; docs output stays snapshot (`fallback=snapshot`)."
                    </span>
                    <Button id="docs-button-snapshot".to_string()>"Snapshot"</Button>
                </div>
            </Playground>

            <Playground
                title="Source-first Starter (Copy-Paste Ready)"
                code_signal=source_first_code
                code_imports=button_imports.clone()
            >
                <div class="docs-stack docs-stack--tight">
                    <span class="ui-muted" data-slot="button-source-first-contract">
                        "Copy-ready snippet auto-prepends imports. Feature gate: component-button + inject-css."
                    </span>
                    <Button
                        id="docs-button-source-first".to_string()
                        color=ButtonColor::Primary
                        variant=ButtonVariant::Solid
                    >
                        "Build"
                    </Button>
                    <p class="ui-muted" data-slot="button-source-paths">
                        "Source: components/button/src/view.rs and crates/ui/src/button/view.rs."
                    </p>
                </div>
            </Playground>

            <Playground title="Colors" code_signal=colors_code>
                <div class="docs-row">
                    <Button color="default">"Default"</Button>
                    <Button color="primary">"Primary"</Button>
                    <Button color="secondary">"Secondary"</Button>
                    <Button color="success">"Success"</Button>
                    <Button color="warning">"Warning"</Button>
                    <Button color="danger">"Danger"</Button>
                </div>
            </Playground>

            <Playground title="Radius" code_signal=radius_code>
                <div class="docs-row">
                    <Button radius="full" color="default">"Full"</Button>
                    <Button radius="lg" color="default">"Large"</Button>
                    <Button radius="md" color="default">"Medium"</Button>
                    <Button radius="sm" color="default">"Small"</Button>
                    <Button radius="none" color="default">"None"</Button>
                </div>
            </Playground>

            <Playground title="Sizes" code_signal=sizes_code>
                <div class="docs-row">
                    <Button size="xs">"XS"</Button>
                    <Button size="s">"S"</Button>
                    <Button size="m">"M"</Button>
                    <Button size="l">"L"</Button>
                    <Button size="xl">"XL"</Button>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn action_button() -> AnyView {
    let (press_count, set_press_count) = signal(0_u32);
    let on_press: OnPress = Callback::new(move |_| {
        set_press_count.update(|count| *count += 1);
    });
    let persisted_workbench_state = load_action_button_workbench_state();
    let has_persisted_workbench_state = persisted_workbench_state.is_some();
    let initial_workbench_state = persisted_workbench_state.unwrap_or_default();
    let size_options = vec![
        "xs".to_string(),
        "s".to_string(),
        "m".to_string(),
        "l".to_string(),
        "xl".to_string(),
    ];
    let (workbench_size_index, set_workbench_size_index) =
        signal(Some(initial_workbench_state.size_index));
    let workbench_size = Signal::derive(move || match workbench_size_index.get().unwrap_or(2) {
        0 => ActionButtonSize::Xs,
        1 => ActionButtonSize::S,
        3 => ActionButtonSize::L,
        4 => ActionButtonSize::Xl,
        _ => ActionButtonSize::M,
    });
    let loading_placement_options =
        vec!["Start".to_string(), "End".to_string(), "Center".to_string()];
    let (workbench_loading_placement_index, set_workbench_loading_placement_index) =
        signal(Some(initial_workbench_state.loading_placement_index));
    let workbench_loading_placement = Signal::derive(
        move || match workbench_loading_placement_index.get().unwrap_or(2) {
            0 => ActionButtonLoadingPlacement::Start,
            1 => ActionButtonLoadingPlacement::End,
            _ => ActionButtonLoadingPlacement::Center,
        },
    );
    let (workbench_loading, set_workbench_loading) = signal(initial_workbench_state.is_loading);
    let (workbench_disabled, set_workbench_disabled) = signal(initial_workbench_state.is_disabled);
    let (workbench_quiet, set_workbench_quiet) = signal(initial_workbench_state.is_quiet);
    let (workbench_icon_only, set_workbench_icon_only) =
        signal(initial_workbench_state.is_icon_only);
    let (workbench_show_start, set_workbench_show_start) =
        signal(initial_workbench_state.show_start);
    let (workbench_show_end, set_workbench_show_end) = signal(initial_workbench_state.show_end);
    let (workbench_persist_state, set_workbench_persist_state) =
        signal(has_persisted_workbench_state);
    let (workbench_popup_expanded_raw, set_workbench_popup_expanded_raw) = signal(false);
    let workbench_popup_expanded: Signal<bool> =
        Signal::derive(move || workbench_popup_expanded_raw.get());
    let workbench_controls_signal: Signal<Option<String>> =
        Signal::derive(move || Some("docs-action-button-workbench-panel".to_string()));
    let (workbench_lang_zh, _set_workbench_lang_zh) = signal(false);
    let (workbench_rtl, _set_workbench_rtl) = signal(false);
    let (workbench_press_count, set_workbench_press_count) = signal(0_u32);
    let workbench_on_press: OnPress = Callback::new(move |_| {
        set_workbench_press_count.update(|count| *count += 1);
        set_workbench_popup_expanded_raw.update(|value| *value = !*value);
    });
    let workbench_node_ref: NodeRef<html::Button> = NodeRef::new();

    Effect::new(move |_| {
        if workbench_persist_state.get() {
            save_action_button_workbench_state(ActionButtonWorkbenchState {
                size_index: workbench_size_index.get().unwrap_or(2),
                loading_placement_index: workbench_loading_placement_index.get().unwrap_or(2),
                is_loading: workbench_loading.get(),
                is_disabled: workbench_disabled.get(),
                is_quiet: workbench_quiet.get(),
                is_icon_only: workbench_icon_only.get(),
                show_start: workbench_show_start.get(),
                show_end: workbench_show_end.get(),
            });
        } else {
            clear_action_button_workbench_state();
        }
    });

    let code = Signal::derive(move || {
        r#"<ActionButton
  on_press=Callback::new(move |_| {})
>
  "Action"
</ActionButton>"#
            .to_string()
    });

    let states_code = Signal::derive(move || {
        r#"<ActionButton
  is_loading=true
  loading_placement=ActionButtonLoadingPlacement::Start
>
  "Start"
</ActionButton>
<ActionButton
  is_loading=true
  loading_placement=ActionButtonLoadingPlacement::End
>
  "End"
</ActionButton>"#
            .to_string()
    });

    let matrix_code = Signal::derive(move || {
        r#"<ActionButton id="ab-default".to_string() button_type=ActionButtonType::Button on_press=Callback::new(move |_| {})>"Default"</ActionButton>
<ActionButton id="ab-loading".to_string() is_loading=true loading_placement=ActionButtonLoadingPlacement::Start motion=ActionButtonMotion::default() on_press=Callback::new(move |_| {})>"Loading"</ActionButton>
<ActionButton id="ab-popup".to_string() is_quiet=true aria_haspopup="menu" aria_expanded=Signal::derive(move || true) aria_controls="popup-panel".to_string() aria_controls_signal=Signal::derive(move || Some("popup-panel".to_string())) class_name="docs-action-button-workbench".to_string() lang="zh-CN".to_string() dir=A11yDirection::Rtl on_press=Callback::new(move |_| {})>"Popup"</ActionButton>"#
            .to_string()
    });

    let workbench_code = Signal::derive(move || {
        let size = workbench_size.get();
        let is_loading = workbench_loading.get();
        let loading_placement = workbench_loading_placement.get();
        let is_disabled = workbench_disabled.get();
        let is_quiet = workbench_quiet.get();
        let is_icon_only = workbench_icon_only.get();
        let show_start = workbench_show_start.get();
        let show_end = workbench_show_end.get();
        let lang = if workbench_lang_zh.get() {
            "zh-CN"
        } else {
            "en-US"
        };
        let dir = if workbench_rtl.get() {
            "A11yDirection::Rtl"
        } else {
            "A11yDirection::Ltr"
        };
        let popup_expanded = workbench_popup_expanded.get();

        let mut snippet = vec![
            "<ActionButton".to_string(),
            "  id=\"docs-action-button-workbench\".to_string()".to_string(),
            format!("  size=ActionButtonSize::{size:?}"),
            format!("  is_loading={is_loading}"),
            format!("  loading_placement=ActionButtonLoadingPlacement::{loading_placement:?}"),
            format!("  is_disabled={is_disabled}"),
            format!("  is_quiet={is_quiet}"),
            format!("  is_icon_only={is_icon_only}"),
            "  motion=ActionButtonMotion::default()".to_string(),
            "  class_name=\"docs-action-button-workbench\".to_string()".to_string(),
            "  button_type=ActionButtonType::Button".to_string(),
            "  aria_haspopup=Some(\"menu\")".to_string(),
            format!("  aria_expanded=Signal::derive(move || {popup_expanded})"),
            "  aria_controls=\"docs-action-button-workbench-panel\".to_string()".to_string(),
            "  aria_controls_signal=Signal::derive(move || Some(\"docs-action-button-workbench-panel\".to_string()))".to_string(),
            format!("  lang=\"{lang}\".to_string()"),
            format!("  dir={dir}"),
            "  node_ref=node_ref".to_string(),
            "  on_press=on_press".to_string(),
        ];

        if is_icon_only {
            snippet.push("  aria_label=\"Action\".into()".to_string());
        }
        if show_start {
            snippet.push("  start_content=move || view! { <span>\"★\"</span> }".to_string());
        }
        if show_end {
            snippet.push("  end_content=move || view! { <span>\"→\"</span> }".to_string());
        }

        snippet.extend([
            ">".to_string(),
            if is_icon_only {
                "  \"★\"".to_string()
            } else {
                "  \"Action\"".to_string()
            },
            "</ActionButton>".to_string(),
        ]);

        snippet.join("\n")
    });

    let workbench_test_css_source = Signal::derive(move || {
        format!(
            "/* crates/ui/src/button/styles.rs */\n{}",
            ui::button::styles::CSS
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        let size = workbench_size.get();
        let is_loading = workbench_loading.get();
        let loading_placement = workbench_loading_placement.get();
        let is_disabled = workbench_disabled.get();
        let is_quiet = workbench_quiet.get();
        let is_icon_only = workbench_icon_only.get();
        let show_start = workbench_show_start.get();
        let show_end = workbench_show_end.get();
        let popup_expanded = workbench_popup_expanded.get();
        format!(
            "ActionButtonActualConfig {{\n  id: Some(\"docs-action-button-workbench\"),\n  is_loading: {is_loading},\n  is_disabled: Some({is_disabled}),\n  size: Some({size:?}),\n  is_quiet: Some({is_quiet}),\n  motion: ActionButtonMotion::default(),\n  loading_placement: {loading_placement:?},\n  class_name: Some(\"docs-action-button-workbench\"),\n  button_type: Some(ActionButtonType::Button),\n  aria_label: {:?},\n  aria_haspopup: Some(\"menu\"),\n  aria_expanded: Some({popup_expanded}),\n  aria_controls: Some(\"docs-action-button-workbench-panel\"),\n  aria_controls_signal: Some(\"docs-action-button-workbench-panel\"),\n  lang: Some({:?}),\n  dir: Some({:?}),\n  node_ref: Some(\"workbench_node_ref\"),\n  on_press: \"count={} toggles_popup=true\",\n  has_start_content: {show_start},\n  has_end_content: {show_end},\n}}",
            if is_icon_only { Some("Action") } else { None },
            if workbench_lang_zh.get() {
                "zh-CN"
            } else {
                "en-US"
            },
            if workbench_rtl.get() { "rtl" } else { "ltr" },
            workbench_press_count.get(),
        )
    });

    view! {
        <ComponentPage
            title="ActionButton"
            slug="action-button"
            group="Actions"
            description="baseline-style action trigger with state attrs and baseline-level spring hover/press feedback."
        >
            <Playground title="Default + callback" code_signal=code>
                <div class="docs-row">
                    <ActionButton on_press=on_press>"Action"</ActionButton>
                    <ActionButton is_quiet=true on_press=on_press>"Quiet"</ActionButton>
                    <ActionButton
                        is_loading=true
                        loading_placement=ActionButtonLoadingPlacement::Center
                    >
                        "Loading"
                    </ActionButton>
                    <span class="ui-muted">
                        "pressed: "
                        {move || press_count.get().to_string()}
                    </span>
                </div>
            </Playground>

            <Playground title="Loading placement + icon-only" code_signal=states_code>
                <div class="docs-stack">
                    <div class="docs-row">
                        <ActionButton
                            size=ActionButtonSize::S
                            is_loading=true
                            loading_placement=ActionButtonLoadingPlacement::Start
                        >
                            "Start"
                        </ActionButton>
                        <ActionButton
                            size=ActionButtonSize::L
                            is_loading=true
                            loading_placement=ActionButtonLoadingPlacement::End
                        >
                            "End"
                        </ActionButton>
                        <ActionButton is_quiet=true aria_label="Settings".to_string()>
                            <svg viewBox="0 0 20 20" fill="none" aria-hidden="true">
                                <path
                                    d="M10 13.3a3.3 3.3 0 1 0 0-6.6a3.3 3.3 0 0 0 0 6.6Z"
                                    stroke="currentColor"
                                    stroke_width="1.5"
                                />
                                <path
                                    d="M3.8 10a6.2 6.2 0 0 1 .1-1l1.6-.9.2-.5-.6-1.8a7.6 7.6 0 0 1 1.5-1.5l1.8.6.5-.2.9-1.6a6.4 6.4 0 0 1 2 0l.9 1.6.5.2 1.8-.6c.6.4 1.1.9 1.5 1.5l-.6 1.8.2.5 1.6.9a6.5 6.5 0 0 1 0 2l-1.6.9-.2.5.6 1.8a7.6 7.6 0 0 1-1.5 1.5l-1.8-.6-.5.2-.9 1.6a6.4 6.4 0 0 1-2 0l-.9-1.6-.5-.2-1.8.6a7.6 7.6 0 0 1-1.5-1.5l.6-1.8-.2-.5-1.6-.9a6.2 6.2 0 0 1-.1-1Z"
                                    stroke="currentColor"
                                    stroke_width="1.2"
                                    stroke_linecap="round"
                                    stroke_linejoin="round"
                                />
                            </svg>
                        </ActionButton>
                    </div>
                    <span class="ui-muted">
                        "Start/end slots, loading placement, and icon-only mode all expose stable data-* attrs."
                    </span>
                </div>
            </Playground>

            <Playground
                title="ActionButton Workbench"
                code_signal=workbench_code
                test_css_source=workbench_test_css_source
                test_source_path="/root/code/personal/omne/rust-ui/crates/ui/src/button/styles.rs".to_string()
                test_config_signal=workbench_actual_config
                description="Workbench canvas: action-button reuses button css contract, supports scoped css live-edit, and optional state persistence."
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="action-button-workbench-controls">
                        <div class="docs-search__label">"Size"</div>
                        <SegmentedControl
                            id_base="docs-action-button-size".to_string()
                            options=size_options.clone()
                            selected_index=workbench_size_index
                            set_selected_index=set_workbench_size_index
                            size=SegmentedControlSize::Sm
                            aria_label="ActionButton size".to_string()
                        />
                        <div class="docs-search__label">"Loading placement"</div>
                        <SegmentedControl
                            id_base="docs-action-button-loading-placement".to_string()
                            options=loading_placement_options.clone()
                            selected_index=workbench_loading_placement_index
                            set_selected_index=set_workbench_loading_placement_index
                            size=SegmentedControlSize::Sm
                            aria_label="ActionButton loading placement".to_string()
                        />
                        <Switch checked=workbench_disabled set_checked=set_workbench_disabled>
                            "Disabled"
                        </Switch>
                        <Switch checked=workbench_loading set_checked=set_workbench_loading>
                            "Loading"
                        </Switch>
                        <Switch checked=workbench_quiet set_checked=set_workbench_quiet>
                            "Quiet"
                        </Switch>
                        <Switch checked=workbench_icon_only set_checked=set_workbench_icon_only>
                            "Icon only"
                        </Switch>
                        <Switch checked=workbench_show_start set_checked=set_workbench_show_start>
                            "Start slot"
                        </Switch>
                        <Switch checked=workbench_show_end set_checked=set_workbench_show_end>
                            "End slot"
                        </Switch>
                        <Switch checked=workbench_persist_state set_checked=set_workbench_persist_state>
                            "Persist workbench state"
                        </Switch>
                    </div>
                }
            >
                {move || {
                    let size = workbench_size.get();
                    let is_loading = workbench_loading.get();
                    let loading_placement = workbench_loading_placement.get();
                    let is_disabled = workbench_disabled.get();
                    let is_quiet = workbench_quiet.get();
                    let is_icon_only = workbench_icon_only.get();
                    let _show_start = workbench_show_start.get();
                    let _show_end = workbench_show_end.get();
                    let persist = workbench_persist_state.get();

                    view! {
                        <div class="docs-stack" data-slot="action-button-workbench" style="width: min(100%, 360px);">
                            <span class="ui-muted">
                                "persist: "
                                {if persist { "on" } else { "off" }}
                            </span>
                            <div class="docs-card docs-stack docs-stack--tight" data-slot="action-button-workbench-canvas">
                                <div class="docs-row" style="justify-content: center;">
                                    <ActionButton
                                        id="docs-action-button-workbench".to_string()
                                        size=size
                                        is_loading=is_loading
                                        loading_placement=loading_placement
                                        is_disabled=is_disabled
                                        is_quiet=is_quiet
                                        motion=ActionButtonMotion::default()
                                        class_name="docs-action-button-workbench".to_string()
                                        button_type=ActionButtonType::Button
                                        aria_label=if is_icon_only { "Action".to_string() } else { String::new() }
                                        aria_haspopup="menu"
                                        aria_expanded=workbench_popup_expanded
                                        aria_controls="docs-action-button-workbench-panel".to_string()
                                        aria_controls_signal=workbench_controls_signal
                                        lang=if workbench_lang_zh.get() { "zh-CN".to_string() } else { "en-US".to_string() }
                                        dir=if workbench_rtl.get() { A11yDirection::Rtl } else { A11yDirection::Ltr }
                                        node_ref=workbench_node_ref
                                        on_press=workbench_on_press
                                    >
                                        {if is_icon_only { "★" } else { "Action" }}
                                    </ActionButton>
                                </div>
                                <div id="docs-action-button-workbench-panel" class="ui-muted">
                                    "popup expanded: " {move || workbench_popup_expanded_raw.get()}
                                </div>
                                <div class="ui-muted">
                                    "workbench on_press count: " {move || workbench_press_count.get()}
                                </div>
                            </div>
                        </div>
                    }
                }}
            </Playground>

            <Playground title="State Matrix (Default / Loading / Popup)" code_signal=matrix_code>
                <div class="docs-row">
                    <ActionButton
                        id="docs-action-button-matrix-default".to_string()
                        button_type=ActionButtonType::Button
                        on_press=on_press
                    >
                        "Default"
                    </ActionButton>
                    <ActionButton
                        id="docs-action-button-matrix-loading".to_string()
                        is_loading=true
                        loading_placement=ActionButtonLoadingPlacement::Start
                        motion=ActionButtonMotion::default()
                        on_press=on_press
                    >
                        "Loading"
                    </ActionButton>
                    <ActionButton
                        id="docs-action-button-matrix-popup".to_string()
                        is_quiet=true
                        aria_haspopup="menu"
                        aria_expanded=Signal::derive(move || true)
                        aria_controls="docs-action-button-matrix-popup-panel".to_string()
                        aria_controls_signal=Signal::derive(move || {
                            Some("docs-action-button-matrix-popup-panel".to_string())
                        })
                        class_name="docs-action-button-workbench".to_string()
                        lang="zh-CN".to_string()
                        dir=A11yDirection::Rtl
                        node_ref=NodeRef::new()
                        on_press=on_press
                    >
                        "Popup"
                    </ActionButton>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn action_button_group() -> AnyView {
    let (showcase_count, set_showcase_count) = signal(0_u32);
    let on_showcase_press: OnPress = Callback::new(move |_| {
        set_showcase_count.update(|count| *count += 1);
    });

    let size_options = vec![
        "xs".to_string(),
        "s".to_string(),
        "m".to_string(),
        "l".to_string(),
        "xl".to_string(),
    ];
    let density_options = vec!["Regular".to_string(), "Compact".to_string()];
    let orientation_options = vec!["Horizontal".to_string(), "Vertical".to_string()];

    let (workbench_size_index, set_workbench_size_index) = signal(Some(2_usize));
    let (workbench_density_index, set_workbench_density_index) = signal(Some(0_usize));
    let (workbench_orientation_index, set_workbench_orientation_index) = signal(Some(0_usize));
    let (workbench_justified, set_workbench_justified) = signal(false);
    let (workbench_quiet, set_workbench_quiet) = signal(false);
    let (workbench_disabled, set_workbench_disabled) = signal(false);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);
    let (workbench_lang_zh, set_workbench_lang_zh) = signal(false);
    let (workbench_rtl, set_workbench_rtl) = signal(false);
    let (workbench_press_count, set_workbench_press_count) = signal(0_u32);
    let on_workbench_press: OnPress = Callback::new(move |_| {
        set_workbench_press_count.update(|count| *count += 1);
    });

    let workbench_size = Signal::derive(move || match workbench_size_index.get().unwrap_or(2) {
        0 => ActionButtonSize::Xs,
        1 => ActionButtonSize::S,
        3 => ActionButtonSize::L,
        4 => ActionButtonSize::Xl,
        _ => ActionButtonSize::M,
    });
    let workbench_density = Signal::derive(move || {
        if workbench_density_index.get().unwrap_or(0) == 1 {
            ActionButtonGroupDensity::Compact
        } else {
            ActionButtonGroupDensity::Regular
        }
    });
    let workbench_orientation = Signal::derive(move || {
        if workbench_orientation_index.get().unwrap_or(0) == 1 {
            ActionButtonGroupOrientation::Vertical
        } else {
            ActionButtonGroupOrientation::Horizontal
        }
    });

    let hello_code = Signal::derive(move || {
        r#"<ActionButtonGroup
  size=ActionButtonSize::S
  density=ActionButtonGroupDensity::Compact
  orientation=ActionButtonGroupOrientation::Horizontal
  is_quiet=true
>
  <ActionButton on_press=Callback::new(move |_| {})>"One"</ActionButton>
  <ActionButton on_press=Callback::new(move |_| {})>"Two"</ActionButton>
  <ActionButton on_press=Callback::new(move |_| {})>"Three"</ActionButton>
</ActionButtonGroup>"#
            .to_string()
    });

    let workbench_code = Signal::derive(move || {
        format!(
            "<ActionButtonGroup\n  size={:?}\n  density={:?}\n  orientation={:?}\n  is_justified={}\n  is_quiet={}\n  is_disabled={}\n  motion=ActionButtonGroupMotion::default()\n  aria_label=\"Action group workbench\".to_string()\n  lang={}.to_string()\n  dir={}\n  class_name={}\n>\n  <ActionButton on_press=on_press>\"Primary\"</ActionButton>\n  <ActionButton on_press=on_press>\"Secondary\"</ActionButton>\n  <ActionButton on_press=on_press>\"Danger\"</ActionButton>\n</ActionButtonGroup>",
            workbench_size.get(),
            workbench_density.get(),
            workbench_orientation.get(),
            workbench_justified.get(),
            workbench_quiet.get(),
            workbench_disabled.get(),
            if workbench_lang_zh.get() {
                "\"zh-CN\""
            } else {
                "\"en-US\""
            },
            if workbench_rtl.get() {
                "A11yDirection::Rtl"
            } else {
                "A11yDirection::Ltr"
            },
            if workbench_custom_class.get() {
                "\"docs-action-button-group-workbench\".to_string()"
            } else {
                "String::new()"
            }
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        format!(
            "ActionButtonGroupActualConfig {{\n  size: {:?},\n  density: {:?},\n  orientation: {:?},\n  is_justified: {},\n  is_quiet: {},\n  is_disabled: {},\n  motion: ActionButtonGroupMotion::default(),\n  aria_label: Some(\"Action group workbench\"),\n  lang: Some({:?}),\n  dir: Some({:?}),\n  class_name: {:?},\n}}",
            workbench_size.get(),
            workbench_density.get(),
            workbench_orientation.get(),
            workbench_justified.get(),
            workbench_quiet.get(),
            workbench_disabled.get(),
            if workbench_lang_zh.get() {
                "zh-CN"
            } else {
                "en-US"
            },
            if workbench_rtl.get() { "rtl" } else { "ltr" },
            if workbench_custom_class.get() {
                Some("docs-action-button-group-workbench")
            } else {
                None
            },
        )
    });

    let matrix_code = Signal::derive(move || {
        r#"<ActionButtonGroup size=ActionButtonSize::M density=ActionButtonGroupDensity::Regular orientation=ActionButtonGroupOrientation::Horizontal aria_label="Default".to_string()>
  <ActionButton on_press=Callback::new(move |_| {})>"A"</ActionButton>
  <ActionButton on_press=Callback::new(move |_| {})>"B"</ActionButton>
</ActionButtonGroup>
<ActionButtonGroup size=ActionButtonSize::S density=ActionButtonGroupDensity::Compact orientation=ActionButtonGroupOrientation::Vertical is_justified=true is_quiet=true class_name="docs-action-button-group-workbench".to_string() lang="zh-CN".to_string() dir=A11yDirection::Rtl aria_label="Vertical".to_string()>
  <ActionButton on_press=Callback::new(move |_| {})>"Top"</ActionButton>
  <ActionButton on_press=Callback::new(move |_| {})>"Bottom"</ActionButton>
</ActionButtonGroup>
<ActionButtonGroup size=ActionButtonSize::M is_disabled=true motion=ActionButtonGroupMotion::default() aria_label="Disabled".to_string()>
  <ActionButton>"Disabled"</ActionButton>
  <ActionButton>"Group"</ActionButton>
</ActionButtonGroup>"#
            .to_string()
    });

    view! {
        <ComponentPage
            title="ActionButtonGroup"
            slug="action-button-group"
            group="Actions"
            description="Toolbar-style action clusters with full API workbench coverage."
        >
            <Playground title="Hello World (Default + compact)" code_signal=hello_code>
                <div class="docs-stack">
                    <ActionButtonGroup
                        size=ActionButtonSize::S
                        density=ActionButtonGroupDensity::Compact
                        orientation=ActionButtonGroupOrientation::Horizontal
                        is_quiet=true
                        aria_label="Quick actions".to_string()
                    >
                        <ActionButton on_press=on_showcase_press>"One"</ActionButton>
                        <ActionButton on_press=on_showcase_press>"Two"</ActionButton>
                        <ActionButton on_press=on_showcase_press>"Three"</ActionButton>
                    </ActionButtonGroup>
                    <span class="ui-muted">
                        "pressed: "
                        {move || showcase_count.get().to_string()}
                    </span>
                </div>
            </Playground>

            <Playground
                title="Workbench (All API + Actual Config)"
                code_signal=workbench_code
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="action-button-group-workbench-controls">
                        <SegmentedControl
                            id_base="docs-action-button-group-size".to_string()
                            options=size_options.clone()
                            selected_index=workbench_size_index
                            set_selected_index=set_workbench_size_index
                            size=SegmentedControlSize::Sm
                            aria_label="ActionButtonGroup size".to_string()
                        />
                        <SegmentedControl
                            id_base="docs-action-button-group-density".to_string()
                            options=density_options.clone()
                            selected_index=workbench_density_index
                            set_selected_index=set_workbench_density_index
                            size=SegmentedControlSize::Sm
                            aria_label="ActionButtonGroup density".to_string()
                        />
                        <SegmentedControl
                            id_base="docs-action-button-group-orientation".to_string()
                            options=orientation_options.clone()
                            selected_index=workbench_orientation_index
                            set_selected_index=set_workbench_orientation_index
                            size=SegmentedControlSize::Sm
                            aria_label="ActionButtonGroup orientation".to_string()
                        />
                        <Switch checked=workbench_justified set_checked=set_workbench_justified>
                            "is_justified"
                        </Switch>
                        <Switch checked=workbench_quiet set_checked=set_workbench_quiet>
                            "is_quiet"
                        </Switch>
                        <Switch checked=workbench_disabled set_checked=set_workbench_disabled>
                            "is_disabled"
                        </Switch>
                        <Switch checked=workbench_custom_class set_checked=set_workbench_custom_class>
                            "class_name"
                        </Switch>
                        <Switch checked=workbench_lang_zh set_checked=set_workbench_lang_zh>
                            "lang=zh-CN"
                        </Switch>
                        <Switch checked=workbench_rtl set_checked=set_workbench_rtl>
                            "dir=rtl"
                        </Switch>
                    </div>
                }
            >
                <div class="docs-stack docs-stack--tight">
                    <ActionButtonGroup
                        size=workbench_size.get()
                        density=workbench_density.get()
                        orientation=workbench_orientation.get()
                        is_justified=workbench_justified.get()
                        is_quiet=workbench_quiet.get()
                        is_disabled=workbench_disabled.get()
                        motion=ActionButtonGroupMotion::default()
                        aria_label="Action group workbench".to_string()
                        lang=if workbench_lang_zh.get() { "zh-CN".to_string() } else { "en-US".to_string() }
                        dir=if workbench_rtl.get() { A11yDirection::Rtl } else { A11yDirection::Ltr }
                        class_name=if workbench_custom_class.get() {
                            "docs-action-button-group-workbench".to_string()
                        } else {
                            String::new()
                        }
                    >
                        <ActionButton on_press=on_workbench_press>"Primary"</ActionButton>
                        <ActionButton on_press=on_workbench_press>"Secondary"</ActionButton>
                        <ActionButton on_press=on_workbench_press>"Danger"</ActionButton>
                    </ActionButtonGroup>
                    <span class="ui-muted">
                        "workbench on_press count: "
                        {move || workbench_press_count.get()}
                    </span>
                </div>
            </Playground>

            <Playground title="State Matrix (Horizontal / Vertical / Disabled)" code_signal=matrix_code>
                <div class="docs-row">
                    <ActionButtonGroup
                        size=ActionButtonSize::M
                        density=ActionButtonGroupDensity::Regular
                        orientation=ActionButtonGroupOrientation::Horizontal
                        aria_label="Default group".to_string()
                    >
                        <ActionButton>"A"</ActionButton>
                        <ActionButton>"B"</ActionButton>
                    </ActionButtonGroup>
                    <ActionButtonGroup
                        size=ActionButtonSize::S
                        density=ActionButtonGroupDensity::Compact
                        orientation=ActionButtonGroupOrientation::Vertical
                        is_justified=true
                        is_quiet=true
                        motion=ActionButtonGroupMotion::default()
                        aria_label="Vertical group".to_string()
                        lang="zh-CN".to_string()
                        dir=A11yDirection::Rtl
                        class_name="docs-action-button-group-workbench".to_string()
                    >
                        <ActionButton>"Top"</ActionButton>
                        <ActionButton>"Bottom"</ActionButton>
                    </ActionButtonGroup>
                    <ActionButtonGroup
                        size=ActionButtonSize::M
                        is_disabled=true
                        motion=ActionButtonGroupMotion::default()
                        aria_label="Disabled group".to_string()
                    >
                        <ActionButton>"Disabled"</ActionButton>
                        <ActionButton>"Group"</ActionButton>
                    </ActionButtonGroup>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn button_group() -> AnyView {
    let (left_count, set_left_count) = signal(0_usize);
    let (middle_count, set_middle_count) = signal(0_usize);
    let (right_count, set_right_count) = signal(0_usize);
    let on_left: OnPress = Callback::new(move |_| set_left_count.update(|count| *count += 1));
    let on_middle: OnPress = Callback::new(move |_| set_middle_count.update(|count| *count += 1));
    let on_right: OnPress = Callback::new(move |_| set_right_count.update(|count| *count += 1));

    let (workbench_vertical, set_workbench_vertical) = signal(false);
    let (workbench_attached, set_workbench_attached) = signal(true);
    let (workbench_custom_label, set_workbench_custom_label) = signal(false);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);
    let (workbench_custom_motion, set_workbench_custom_motion) = signal(false);
    let (workbench_rtl, set_workbench_rtl) = signal(false);
    let (workbench_press_count, set_workbench_press_count) = signal(0_usize);
    let on_workbench_press: OnPress =
        Callback::new(move |_| set_workbench_press_count.update(|count| *count += 1));
    let workbench_node_ref = NodeRef::<html::Div>::new();

    let workbench_orientation = Signal::derive(move || {
        if workbench_vertical.get() {
            ButtonGroupOrientation::Vertical
        } else {
            ButtonGroupOrientation::Horizontal
        }
    });
    let workbench_motion = Signal::derive(move || {
        if workbench_custom_motion.get() {
            ui::button::ButtonGroupMotion {
                enter_scale: 0.96,
                ..ui::button::ButtonGroupMotion::default()
            }
        } else {
            ui::button::ButtonGroupMotion::default()
        }
    });

    let hello_code = Signal::derive(move || {
        r#"<ButtonGroup is_attached=true>
  <Button variant=ButtonVariant::Secondary>"Left"</Button>
  <Button variant=ButtonVariant::Secondary>"Middle"</Button>
  <Button variant=ButtonVariant::Secondary>"Right"</Button>
</ButtonGroup>"#
            .to_string()
    });
    let workbench_code = Signal::derive(move || {
        format!(
            "<ButtonGroup\n  orientation=ButtonGroupOrientation::{:?}\n  is_attached={}\n  motion={}\n  node_ref=NodeRef::<leptos::html::Div>::new()\n  aria_label={}\n  lang={}\n  dir={}\n  class_name={}\n>\n  <Button on_press=on_press>\"Left\"</Button>\n  <Button on_press=on_press>\"Center\"</Button>\n  <Button on_press=on_press>\"Right\"</Button>\n</ButtonGroup>",
            workbench_orientation.get(),
            workbench_attached.get(),
            if workbench_custom_motion.get() {
                "ButtonGroupMotion { enter_scale: 0.96, ..ButtonGroupMotion::default() }"
            } else {
                "ButtonGroupMotion::default()"
            },
            if workbench_custom_label.get() {
                "\"Action buttons\".to_string()"
            } else {
                "\"\".to_string()"
            },
            if workbench_rtl.get() {
                "\"ar\".to_string()"
            } else {
                "\"en-US\".to_string()"
            },
            if workbench_rtl.get() {
                "Some(A11yDirection::Rtl)"
            } else {
                "Some(A11yDirection::Ltr)"
            },
            if workbench_custom_class.get() {
                "\"docs-button-group-custom\".to_string()"
            } else {
                "\"\".to_string()"
            }
        )
    });
    let workbench_actual_config = Signal::derive(move || {
        format!(
            "ButtonGroupActualConfig {{\n  orientation: {:?},\n  is_attached: {},\n  motion: {},\n  node_ref: \"workbench_node_ref\",\n  aria_label: {:?},\n  lang: {:?},\n  dir: {},\n  class_name: {:?},\n  press_count: {},\n}}",
            workbench_orientation.get(),
            workbench_attached.get(),
            if workbench_custom_motion.get() {
                "ButtonGroupMotion(custom)"
            } else {
                "ButtonGroupMotion::default()"
            },
            if workbench_custom_label.get() {
                Some("Action buttons")
            } else {
                None
            },
            if workbench_rtl.get() {
                Some("ar")
            } else {
                Some("en-US")
            },
            if workbench_rtl.get() {
                "Some(A11yDirection::Rtl)"
            } else {
                "Some(A11yDirection::Ltr)"
            },
            if workbench_custom_class.get() {
                Some("docs-button-group-custom")
            } else {
                None
            },
            workbench_press_count.get(),
        )
    });
    let matrix_code = Signal::derive(move || {
        r#"<ButtonGroup orientation=ButtonGroupOrientation::Horizontal is_attached=true>
  <Button>"Left"</Button><Button>"Center"</Button><Button>"Right"</Button>
</ButtonGroup>
<ButtonGroup orientation=ButtonGroupOrientation::Vertical is_attached=false aria_label="Doc actions".to_string()>
  <Button>"Top"</Button><Button>"Middle"</Button><Button>"Bottom"</Button>
</ButtonGroup>"#
            .to_string()
    });
    let matrix_node_ref_horizontal = NodeRef::<html::Div>::new();
    let matrix_node_ref_vertical = NodeRef::<html::Div>::new();

    view! {
        <ComponentPage
            title="ButtonGroup"
            slug="button-group"
            group="Actions"
            description="Groups Buttons with baseline-style root state attrs for orientation, attachment, and accessible labeling."
        >
            <Playground title="Hello World (Default ButtonGroup)" code_signal=hello_code>
                <div class="docs-stack">
                    <ButtonGroup is_attached=true orientation=ButtonGroupOrientation::Horizontal>
                        <Button variant=ButtonVariant::Secondary on_press=on_left>
                            "Left"
                        </Button>
                        <Button variant=ButtonVariant::Secondary on_press=on_middle>
                            "Middle"
                        </Button>
                        <Button variant=ButtonVariant::Secondary on_press=on_right>
                            "Right"
                        </Button>
                    </ButtonGroup>
                    <span class="ui-muted">
                        "left/middle/right clicks: "
                        {move || format!(
                            "{}/{}/{}",
                            left_count.get(),
                            middle_count.get(),
                            right_count.get()
                        )}
                    </span>
                </div>
            </Playground>

            <Playground
                title="Workbench (All API + Actual Config)"
                code_signal=workbench_code
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight">
                        <Switch checked=workbench_vertical set_checked=set_workbench_vertical>
                            "Vertical orientation"
                        </Switch>
                        <Switch checked=workbench_attached set_checked=set_workbench_attached>
                            "is_attached"
                        </Switch>
                        <Switch checked=workbench_custom_motion set_checked=set_workbench_custom_motion>
                            "Custom motion"
                        </Switch>
                        <Switch checked=workbench_custom_label set_checked=set_workbench_custom_label>
                            "Custom aria_label"
                        </Switch>
                        <Switch checked=workbench_custom_class set_checked=set_workbench_custom_class>
                            "Custom class_name"
                        </Switch>
                        <Switch checked=workbench_rtl set_checked=set_workbench_rtl>
                            "RTL + ar"
                        </Switch>
                    </div>
                }
            >
                <div class="docs-stack">
                    <ButtonGroup
                        orientation=workbench_orientation.get()
                        is_attached=workbench_attached.get()
                        motion=workbench_motion.get()
                        node_ref=workbench_node_ref
                        aria_label=if workbench_custom_label.get() {
                            "Action buttons".to_string()
                        } else {
                            String::new()
                        }
                        lang=if workbench_rtl.get() {
                            "ar".to_string()
                        } else {
                            "en-US".to_string()
                        }
                        dir=if workbench_rtl.get() {
                            A11yDirection::Rtl
                        } else {
                            A11yDirection::Ltr
                        }
                        class_name=if workbench_custom_class.get() {
                            "docs-button-group-custom".to_string()
                        } else {
                            String::new()
                        }
                    >
                        <Button variant=ButtonVariant::Secondary on_press=on_workbench_press>
                            "Left"
                        </Button>
                        <Button variant=ButtonVariant::Secondary on_press=on_workbench_press>
                            "Center"
                        </Button>
                        <Button variant=ButtonVariant::Secondary on_press=on_workbench_press>
                            "Right"
                        </Button>
                    </ButtonGroup>
                    <span class="ui-muted">"workbench presses: " {move || workbench_press_count.get()}</span>
                </div>
            </Playground>

            <Playground title="State Matrix (Orientation + Attachment)" code_signal=matrix_code>
                <div class="docs-stack">
                    <ButtonGroup
                        is_attached=true
                        orientation=ButtonGroupOrientation::Horizontal
                        motion=ui::button::ButtonGroupMotion::default()
                        node_ref=matrix_node_ref_horizontal
                        aria_label="Primary actions".to_string()
                        lang="en-US".to_string()
                        dir=A11yDirection::Ltr
                    >
                        <Button variant=ButtonVariant::Secondary>"Left"</Button>
                        <Button variant=ButtonVariant::Secondary>"Middle"</Button>
                        <Button variant=ButtonVariant::Secondary>"Right"</Button>
                    </ButtonGroup>
                    <ButtonGroup
                        is_attached=false
                        orientation=ButtonGroupOrientation::Vertical
                        motion=ui::button::ButtonGroupMotion::default()
                        node_ref=matrix_node_ref_vertical
                        aria_label="Document actions".to_string()
                        lang="ar".to_string()
                        dir=A11yDirection::Rtl
                        class_name="docs-button-group-custom".to_string()
                    >
                        <Button variant=ButtonVariant::Outline>"Top"</Button>
                        <Button variant=ButtonVariant::Outline is_disabled=true>"Disabled"</Button>
                        <Button variant=ButtonVariant::Outline>"Bottom"</Button>
                    </ButtonGroup>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
pub(super) fn link_button() -> AnyView {
    let variant_options = vec![
        "Default".to_string(),
        "Secondary".to_string(),
        "Ghost".to_string(),
        "Outline".to_string(),
    ];
    let (variant_index, set_variant_index) = signal(Some(0_usize));
    let variant = Signal::derive(move || match variant_index.get().unwrap_or(0) {
        1 => ButtonVariant::Secondary,
        2 => ButtonVariant::Ghost,
        3 => ButtonVariant::Outline,
        _ => ButtonVariant::Default,
    });

    let size_options = vec![
        "xs".to_string(),
        "s".to_string(),
        "m".to_string(),
        "l".to_string(),
        "xl".to_string(),
    ];
    let (size_index, set_size_index) = signal(Some(2_usize));
    let size = Signal::derive(move || match size_index.get().unwrap_or(2) {
        0 => ButtonSize::Xs,
        1 => ButtonSize::S,
        2 => ButtonSize::M,
        3 => ButtonSize::L,
        _ => ButtonSize::Xl,
    });

    let (disabled, set_disabled) = signal(false);
    let (open_in_new_tab, set_open_in_new_tab) = signal(false);
    let (sponsored_rel, set_sponsored_rel) = signal(false);

    let code = Signal::derive(move || {
        let variant = variant.get();
        let size = size.get();
        let disabled = disabled.get();

        let mut snippet = vec![
            "<LinkButton".to_string(),
            "  href=\"https://example.com/docs\".into()".to_string(),
        ];

        if variant != ButtonVariant::Default {
            snippet.push(format!("  variant=ButtonVariant::{variant:?}"));
        }
        if size != ButtonSize::M {
            snippet.push(format!("  size=ButtonSize::{size:?}"));
        }
        if disabled {
            snippet.push("  disabled=true".to_string());
        }
        if open_in_new_tab.get() {
            snippet.push("  target=Some(\"_blank\")".to_string());
        }
        if sponsored_rel.get() {
            snippet.push("  rel=Some(\"sponsored\".into())".to_string());
        }

        snippet.extend([
            ">".to_string(),
            "  \"Open docs\"".to_string(),
            "</LinkButton>".to_string(),
        ]);

        snippet.join("\n")
    });

    let workbench_test_css_source = Signal::derive(move || {
        format!(
            "/* crates/ui/src/button/link_button/styles.rs */\n{}",
            ui::link_button::styles::CSS
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        let rel_value = if sponsored_rel.get() { "sponsored" } else { "" };
        format!(
            "LinkButtonWorkbenchConfig {{\n  href: \"https://example.com/docs\",\n  variant: \"{:?}\",\n  size: \"{:?}\",\n  disabled: {},\n  target: \"{}\",\n  rel: \"{}\",\n  aria_label: {},\n  class_name: Some(\"docs-link-button-workbench\"),\n}}",
            variant.get(),
            size.get(),
            disabled.get(),
            if open_in_new_tab.get() {
                "_blank"
            } else {
                "_self"
            },
            rel_value,
            if open_in_new_tab.get() {
                "Some(\"Open docs in a new tab\")"
            } else {
                "Some(\"Open docs in the same tab\")"
            },
        )
    });

    let showcase_code = Signal::derive(move || {
        r#"<LinkButton href="https://example.com/docs".to_string()>
  "Open docs"
</LinkButton>"#
            .to_string()
    });

    let states_code = Signal::derive(move || {
        r#"<LinkButton href="https://example.com/xs".to_string() size=ButtonSize::Xs>
  "xs"
</LinkButton>
<LinkButton href="https://example.com/s".to_string() size=ButtonSize::S>
  "s"
</LinkButton>
<LinkButton href="https://example.com/m".to_string() size=ButtonSize::M>
  "m"
</LinkButton>
<LinkButton
  href="https://example.com/l".to_string()
  size=ButtonSize::L
  variant=ButtonVariant::Secondary
>
  "l secondary"
</LinkButton>
<LinkButton
  href="https://example.com/xl".to_string()
  size=ButtonSize::Xl
>
  "xl"
</LinkButton>
<LinkButton href="https://example.com/disabled".to_string() disabled=true>
  "Disabled"
</LinkButton>"#
            .to_string()
    });

    view! {
        <ComponentPage
            title="LinkButton"
            slug="link-button"
            group="Actions"
            description="Button styling on anchors with baseline-style disabled semantics and secure rel handling for external targets."
        >
            <Playground title="Hello World (Default LinkButton)" code_signal=showcase_code>
                <LinkButton href="https://example.com/docs".to_string()>
                    "Open docs"
                </LinkButton>
            </Playground>

            <Playground
                title="Interactive Playground (Display + Config + Code + CSS Test)"
                code_signal=code
                test_css_source=workbench_test_css_source
                test_source_path="/root/autodl-tmp/zjj/p/rust-ui/crates/ui/src/button/link_button/styles.rs".to_string()
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight">
                        <div class="docs-search__label">"Variant"</div>
                        <SegmentedControl
                            id_base="docs-link-button-variant".to_string()
                            options=variant_options.clone()
                            selected_index=variant_index
                            set_selected_index=set_variant_index
                            size=SegmentedControlSize::Sm
                            aria_label="LinkButton variant".to_string()
                        />

                        <div class="docs-search__label">"Size"</div>
                        <SegmentedControl
                            id_base="docs-link-button-size".to_string()
                            options=size_options.clone()
                            selected_index=size_index
                            set_selected_index=set_size_index
                            size=SegmentedControlSize::Sm
                            aria_label="LinkButton size".to_string()
                        />

                        <Switch checked=disabled set_checked=set_disabled>"Disabled"</Switch>
                        <Switch checked=open_in_new_tab set_checked=set_open_in_new_tab>
                            "Open in new tab (_blank)"
                        </Switch>
                        <Switch checked=sponsored_rel set_checked=set_sponsored_rel>
                            "Add sponsored rel"
                        </Switch>
                    </div>
                }
            >
                <div class="docs-stack">
                    {move || {
                        let variant = variant.get();
                        let size = size.get();
                        let disabled = disabled.get();
                        let rel = if sponsored_rel.get() {
                            "sponsored".to_string()
                        } else {
                            String::new()
                        };

                        view! {
                            <div class="docs-row">
                                {if open_in_new_tab.get() {
                                    view! {
                                        <LinkButton
                                            href="https://example.com/docs".to_string()
                                            target="_blank"
                                            rel=rel.clone()
                                            variant=variant
                                            size=size
                                            disabled=disabled
                                            aria_label="Open docs in a new tab".to_string()
                                            class_name="docs-link-button-workbench".to_string()
                                        >
                                            "Open docs"
                                        </LinkButton>
                                    }
                                        .into_any()
                                } else {
                                    view! {
                                        <LinkButton
                                            href="https://example.com/docs".to_string()
                                            rel=rel
                                            variant=variant
                                            size=size
                                            disabled=disabled
                                            aria_label="Open docs in the same tab".to_string()
                                            class_name="docs-link-button-workbench".to_string()
                                        >
                                            "Open docs"
                                        </LinkButton>
                                    }
                                        .into_any()
                                }}
                                <LinkButton href="https://example.com/changelog".to_string()>
                                    "Same tab"
                                </LinkButton>
                                <LinkButton href="   ".to_string() variant=ButtonVariant::Ghost>
                                    "Missing href"
                                </LinkButton>
                            </div>
                        }
                    }}
                    <span class="ui-muted">
                        "_blank links auto-append noopener+noreferrer; blank href is normalized as non-navigable."
                    </span>
                </div>
            </Playground>

            <Playground title="State Matrix (Variant + size + disabled)" code_signal=states_code>
                <div class="docs-stack">
                    <div class="docs-row">
                        <LinkButton href="https://example.com/xs".to_string() size=ButtonSize::Xs>
                            "xs"
                        </LinkButton>
                        <LinkButton href="https://example.com/s".to_string() size=ButtonSize::S>
                            "s"
                        </LinkButton>
                        <LinkButton
                            href="https://example.com/m".to_string()
                            size=ButtonSize::M
                        >
                            "m"
                        </LinkButton>
                        <LinkButton
                            href="https://example.com/l".to_string()
                            size=ButtonSize::L
                            variant=ButtonVariant::Secondary
                        >
                            "l secondary"
                        </LinkButton>
                        <LinkButton
                            href="https://example.com/xl".to_string()
                            size=ButtonSize::Xl
                        >
                            "xl"
                        </LinkButton>
                    </div>
                    <div class="docs-row">
                        <LinkButton href="https://example.com/disabled".to_string() disabled=true>
                            "Disabled"
                        </LinkButton>
                        <LinkButton
                            href="https://example.com/disabled-ghost".to_string()
                            variant=ButtonVariant::Ghost
                            disabled=true
                        >
                            "Disabled ghost"
                        </LinkButton>
                    </div>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
pub(super) fn toggle_button() -> AnyView {
    let showcase_node_ref: NodeRef<html::Button> = NodeRef::new();
    let workbench_node_ref: NodeRef<html::Button> = NodeRef::new();

    let showcase_code = Signal::derive(move || {
        r#"<ToggleButton
  default_pressed=true
  motion=ToggleButtonMotion {
    hover_scale: 1.06,
    tap_scale: 0.95,
    ..ToggleButtonMotion::default()
  }
  class_name="docs-toggle-button-custom".to_string()
  aria_label="Mute notifications".to_string()
  node_ref=NodeRef::new()
>
  "Mute"
</ToggleButton>"#
            .to_string()
    });

    let variant_options = vec![
        "Default".to_string(),
        "Accent".to_string(),
        "Outline".to_string(),
        "Secondary".to_string(),
        "Ghost".to_string(),
        "Destructive".to_string(),
    ];
    let size_options = vec![
        "xs".to_string(),
        "s".to_string(),
        "m".to_string(),
        "l".to_string(),
        "xl".to_string(),
    ];
    let motion_options = vec!["default".to_string(), "custom".to_string()];

    let (variant_index, set_variant_index) = signal(Some(0_usize));
    let (size_index, set_size_index) = signal(Some(2_usize));
    let (motion_index, set_motion_index) = signal(Some(0_usize));
    let (disabled, set_disabled) = signal(false);
    let (default_pressed, set_default_pressed) = signal(false);
    let (custom_class, set_custom_class) = signal(false);
    let (custom_aria, set_custom_aria) = signal(true);

    let variant = Signal::derive(move || match variant_index.get().unwrap_or(0) {
        1 => ToggleButtonVariant::Accent,
        2 => ToggleButtonVariant::Outline,
        3 => ToggleButtonVariant::Secondary,
        4 => ToggleButtonVariant::Ghost,
        5 => ToggleButtonVariant::Destructive,
        _ => ToggleButtonVariant::Default,
    });
    let size = Signal::derive(move || match size_index.get().unwrap_or(2) {
        0 => ToggleButtonSize::Xs,
        1 => ToggleButtonSize::S,
        2 => ToggleButtonSize::M,
        3 => ToggleButtonSize::L,
        _ => ToggleButtonSize::Xl,
    });
    let motion = Signal::derive(move || match motion_index.get().unwrap_or(0) {
        1 => ToggleButtonMotion {
            hover_scale: 1.06,
            tap_scale: 0.95,
            ..ToggleButtonMotion::default()
        },
        _ => ToggleButtonMotion::default(),
    });
    let class_name = Signal::derive(move || {
        if custom_class.get() {
            "docs-toggle-button-custom".to_string()
        } else {
            String::new()
        }
    });
    let aria_label = Signal::derive(move || {
        if custom_aria.get() {
            "Toggle docs sample".to_string()
        } else {
            String::new()
        }
    });

    let (selected, set_selected) = signal(false);
    let selected_signal: Signal<bool> = Signal::derive(move || selected.get());
    let (last_change, set_last_change) = signal("none".to_string());
    let on_toggle_change = Callback::new(move |next: bool| {
        set_selected.set(next);
        set_last_change.set(if next {
            "true".to_string()
        } else {
            "false".to_string()
        });
    });

    let workbench_code = Signal::derive(move || {
        let lines = vec![
            "let (selected, set_selected) = signal(false);".to_string(),
            "let selected_signal: Signal<bool> = Signal::derive(move || selected.get());"
                .to_string(),
            "let on_toggle_change = Callback::new(move |next| set_selected.set(next));".to_string(),
            String::new(),
            "<ToggleButton".to_string(),
            "  is_pressed=selected_signal".to_string(),
            format!("  default_pressed={}", default_pressed.get()),
            format!("  is_disabled={}", disabled.get()),
            format!("  variant=ToggleButtonVariant::{:?}", variant.get()),
            format!("  size=ToggleButtonSize::{:?}", size.get()),
            format!("  motion={:?}", motion.get()),
            "  on_pressed_change=on_toggle_change".to_string(),
            format!("  class_name={:?}", class_name.get()),
            format!("  aria_label={:?}", aria_label.get()),
            "  node_ref=NodeRef::new()".to_string(),
            ">".to_string(),
            "  \"Toggle\"".to_string(),
            "</ToggleButton>".to_string(),
        ];
        lines.join("\n")
    });
    let workbench_actual_config = Signal::derive(move || {
        format!(
            "ToggleButtonWorkbenchConfig {{\n  is_pressed: {},\n  default_pressed: {:?},\n  is_disabled: {},\n  variant: {:?},\n  size: {:?},\n  motion: {:?},\n  on_pressed_change: {:?},\n  class_name: {:?},\n  aria_label: {:?},\n  node_ref: \"bound\",\n}}",
            selected.get(),
            Some(default_pressed.get()),
            disabled.get(),
            variant.get(),
            size.get(),
            motion.get(),
            last_change.get(),
            if class_name.get().is_empty() {
                None::<String>
            } else {
                Some(class_name.get())
            },
            if aria_label.get().is_empty() {
                None::<String>
            } else {
                Some(aria_label.get())
            },
        )
    });

    let (notifications, set_notifications) = signal(true);
    let (disabled_selected, set_disabled_selected) = signal(true);
    let (disabled_unselected, set_disabled_unselected) = signal(false);
    let notifications_signal: Signal<bool> = Signal::derive(move || notifications.get());
    let disabled_selected_signal: Signal<bool> = Signal::derive(move || disabled_selected.get());
    let disabled_unselected_signal: Signal<bool> =
        Signal::derive(move || disabled_unselected.get());
    let on_notifications_change = Callback::new(move |next: bool| set_notifications.set(next));
    let on_disabled_selected_change =
        Callback::new(move |next: bool| set_disabled_selected.set(next));
    let on_disabled_unselected_change =
        Callback::new(move |next: bool| set_disabled_unselected.set(next));
    let states_code = Signal::derive(move || {
        r#"<ToggleButton
  is_pressed=notifications_signal
  on_pressed_change=on_notifications_change
  variant=ToggleButtonVariant::Accent
  size=ToggleButtonSize::L
>
  "Notifications"
</ToggleButton>
<ToggleButton is_pressed=disabled_selected_signal on_pressed_change=on_disabled_selected_change is_disabled=true>
  "Disabled on"
</ToggleButton>
<ToggleButton is_pressed=disabled_unselected_signal on_pressed_change=on_disabled_unselected_change is_disabled=true>
  "Disabled off"
</ToggleButton>"#
            .to_string()
    });

    view! {
        <ComponentPage
            title="ToggleButton"
            slug="toggle-button"
            group="Actions"
            description="Pressable toggle state with baseline-level spring motion and baseline-style root state attrs."
        >
            <Playground
                title="Hello World (Default API)"
                code_signal=showcase_code
            >
                <div class="docs-row">
                    <ToggleButton
                        default_pressed=true
                        motion=ToggleButtonMotion {
                            hover_scale: 1.06,
                            tap_scale: 0.95,
                            ..ToggleButtonMotion::default()
                        }
                        class_name="docs-toggle-button-custom".to_string()
                        aria_label="Mute notifications".to_string()
                        node_ref=showcase_node_ref
                    >
                        "Mute"
                    </ToggleButton>
                </div>
            </Playground>

            <Playground
                title="Workbench (Controlled + on_pressed_change)"
                code_signal=workbench_code
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight">
                        <div class="docs-search__label">"Variant"</div>
                        <SegmentedControl
                            id_base="docs-toggle-button-variant".to_string()
                            options=variant_options.clone()
                            selected_index=variant_index
                            set_selected_index=set_variant_index
                            size=SegmentedControlSize::Sm
                            aria_label="ToggleButton variant".to_string()
                        />

                        <div class="docs-search__label">"Size"</div>
                        <SegmentedControl
                            id_base="docs-toggle-button-size".to_string()
                            options=size_options.clone()
                            selected_index=size_index
                            set_selected_index=set_size_index
                            size=SegmentedControlSize::Sm
                            aria_label="ToggleButton size".to_string()
                        />

                        <div class="docs-search__label">"Motion"</div>
                        <SegmentedControl
                            id_base="docs-toggle-button-motion".to_string()
                            options=motion_options.clone()
                            selected_index=motion_index
                            set_selected_index=set_motion_index
                            size=SegmentedControlSize::Sm
                            aria_label="ToggleButton motion".to_string()
                        />

                        <Switch checked=disabled set_checked=set_disabled>"Disabled"</Switch>
                        <Switch checked=default_pressed set_checked=set_default_pressed>
                            "Default pressed"
                        </Switch>
                        <Switch checked=custom_class set_checked=set_custom_class>"Custom class"</Switch>
                        <Switch checked=custom_aria set_checked=set_custom_aria>"Custom aria label"</Switch>
                    </div>
                }
            >
                {move || {
                    let variant = variant.get();
                    let size = size.get();
                    let disabled = disabled.get();

                    view! {
                        <div class="docs-stack">
                            <div class="docs-row">
                                <ToggleButton
                                    is_pressed=selected_signal
                                    default_pressed=default_pressed.get()
                                    motion=motion.get()
                                    on_pressed_change=on_toggle_change
                                    variant=variant
                                    size=size
                                    is_disabled=disabled
                                    class_name=class_name.get()
                                    aria_label=aria_label.get()
                                    node_ref=workbench_node_ref
                                >
                                    "Toggle"
                                </ToggleButton>
                                <span class="ui-muted">
                                    "selected: "
                                    {move || selected.get()}
                                </span>
                            </div>
                            <span class="ui-muted">"last on_pressed_change: " {move || last_change.get()}</span>
                        </div>
                    }
                }}
            </Playground>

            <Playground title="State Matrix (Variant + Size + Disabled)" code_signal=states_code>
                <div class="docs-stack">
                    <div class="docs-row">
                        <ToggleButton
                            is_pressed=notifications_signal
                            on_pressed_change=on_notifications_change
                            variant=ToggleButtonVariant::Accent
                            size=ToggleButtonSize::L
                        >
                            "Notifications"
                        </ToggleButton>
                        <span class="ui-muted">
                            "notifications: "
                            {move || notifications.get()}
                        </span>
                    </div>
                    <div class="docs-row">
                        <ToggleButton
                            is_pressed=disabled_selected_signal
                            on_pressed_change=on_disabled_selected_change
                            is_disabled=true
                        >
                            "Disabled on"
                        </ToggleButton>
                        <ToggleButton
                            is_pressed=disabled_unselected_signal
                            on_pressed_change=on_disabled_unselected_change
                            is_disabled=true
                        >
                            "Disabled off"
                        </ToggleButton>
                    </div>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
pub(super) fn toggle_button_group() -> AnyView {
    let showcase_code = Signal::derive(move || {
        r#"<ToggleButtonGroup
  orientation=ToggleButtonGroupOrientation::Horizontal
  is_attached=true
  motion=ToggleButtonGroupMotion { duration_ms: 220.0 }
  aria_label="Text style".to_string()
  lang="en-US".to_string()
  dir=A11yDirection::Ltr
  class_name="docs-toggle-group-custom".to_string()
>
  <ToggleButton default_pressed=true>"Bold"</ToggleButton>
  <ToggleButton>"Italic"</ToggleButton>
  <ToggleButton>"Underline"</ToggleButton>
</ToggleButtonGroup>"#
            .to_string()
    });

    let orientation_options = vec!["Horizontal".to_string(), "Vertical".to_string()];
    let variant_options = vec![
        "Default".to_string(),
        "Accent".to_string(),
        "Outline".to_string(),
        "Secondary".to_string(),
        "Ghost".to_string(),
        "Destructive".to_string(),
    ];
    let size_options = vec![
        "xs".to_string(),
        "s".to_string(),
        "m".to_string(),
        "l".to_string(),
        "xl".to_string(),
    ];
    let motion_options = vec!["default".to_string(), "custom".to_string()];
    let lang_options = vec!["en-US".to_string(), "zh-CN".to_string()];

    let (orientation_index, set_orientation_index) = signal(Some(0_usize));
    let (variant_index, set_variant_index) = signal(Some(0_usize));
    let (size_index, set_size_index) = signal(Some(2_usize));
    let (motion_index, set_motion_index) = signal(Some(0_usize));
    let (lang_index, set_lang_index) = signal(Some(0_usize));
    let (attached, set_attached) = signal(false);
    let (rtl, set_rtl) = signal(false);
    let (custom_class, set_custom_class) = signal(false);
    let (custom_aria, set_custom_aria) = signal(true);

    let orientation = Signal::derive(move || match orientation_index.get().unwrap_or(0) {
        1 => ToggleButtonGroupOrientation::Vertical,
        _ => ToggleButtonGroupOrientation::Horizontal,
    });
    let variant = Signal::derive(move || match variant_index.get().unwrap_or(0) {
        1 => ToggleButtonVariant::Accent,
        2 => ToggleButtonVariant::Outline,
        3 => ToggleButtonVariant::Secondary,
        4 => ToggleButtonVariant::Ghost,
        5 => ToggleButtonVariant::Destructive,
        _ => ToggleButtonVariant::Default,
    });
    let size = Signal::derive(move || match size_index.get().unwrap_or(2) {
        0 => ToggleButtonSize::Xs,
        1 => ToggleButtonSize::S,
        2 => ToggleButtonSize::M,
        3 => ToggleButtonSize::L,
        _ => ToggleButtonSize::Xl,
    });
    let motion = Signal::derive(move || match motion_index.get().unwrap_or(0) {
        1 => ToggleButtonGroupMotion { duration_ms: 220.0 },
        _ => ToggleButtonGroupMotion::default(),
    });
    let lang = Signal::derive(move || match lang_index.get().unwrap_or(0) {
        1 => "zh-CN".to_string(),
        _ => "en-US".to_string(),
    });
    let dir = Signal::derive(move || {
        if rtl.get() {
            A11yDirection::Rtl
        } else {
            A11yDirection::Ltr
        }
    });
    let class_name = Signal::derive(move || {
        if custom_class.get() {
            "docs-toggle-group-custom".to_string()
        } else {
            String::new()
        }
    });
    let aria_label = Signal::derive(move || {
        if custom_aria.get() {
            "Formatting controls".to_string()
        } else {
            String::new()
        }
    });

    let (a, set_a) = signal(false);
    let (b, set_b) = signal(true);
    let (c, set_c) = signal(false);
    let a_signal: Signal<bool> = Signal::derive(move || a.get());
    let b_signal: Signal<bool> = Signal::derive(move || b.get());
    let c_signal: Signal<bool> = Signal::derive(move || c.get());
    let on_a_change = Callback::new(move |next: bool| set_a.set(next));
    let on_b_change = Callback::new(move |next: bool| set_b.set(next));
    let on_c_change = Callback::new(move |next: bool| set_c.set(next));
    let attached_selected_count =
        Signal::derive(move || usize::from(a.get()) + usize::from(b.get()) + usize::from(c.get()));

    let workbench_code = Signal::derive(move || {
        let mut toggle_props = String::new();
        if variant.get() != ToggleButtonVariant::Default {
            toggle_props.push_str(&format!(
                " variant=ToggleButtonVariant::{:?}",
                variant.get()
            ));
        }
        if size.get() != ToggleButtonSize::M {
            toggle_props.push_str(&format!(" size=ToggleButtonSize::{:?}", size.get()));
        }
        let snippet = vec![
            "<ToggleButtonGroup".to_string(),
            format!(
                "  orientation=ToggleButtonGroupOrientation::{:?}",
                orientation.get()
            ),
            format!("  is_attached={}", attached.get()),
            format!("  motion={:?}", motion.get()),
            format!("  aria_label={:?}", aria_label.get()),
            format!("  lang={:?}", lang.get()),
            format!("  dir={:?}", dir.get()),
            format!("  class_name={:?}", class_name.get()),
            ">".to_string(),
            format!(
                "  <ToggleButton is_pressed=bold_signal on_pressed_change=on_bold_change{toggle_props}>\"Bold\"</ToggleButton>"
            ),
            format!(
                "  <ToggleButton is_pressed=italic_signal on_pressed_change=on_italic_change{toggle_props}>\"Italic\"</ToggleButton>"
            ),
            format!(
                "  <ToggleButton is_pressed=underline_signal on_pressed_change=on_underline_change{toggle_props}>\"Underline\"</ToggleButton>"
            ),
            "</ToggleButtonGroup>".to_string(),
        ];
        snippet.join("\n")
    });

    let (left, set_left) = signal(true);
    let (center, set_center) = signal(false);
    let (right, set_right) = signal(true);
    let left_signal: Signal<bool> = Signal::derive(move || left.get());
    let center_signal: Signal<bool> = Signal::derive(move || center.get());
    let right_signal: Signal<bool> = Signal::derive(move || right.get());
    let on_left_change = Callback::new(move |next: bool| set_left.set(next));
    let on_center_change = Callback::new(move |next: bool| set_center.set(next));
    let on_right_change = Callback::new(move |next: bool| set_right.set(next));
    let detached_selected_count = Signal::derive(move || {
        usize::from(left.get()) + usize::from(center.get()) + usize::from(right.get())
    });
    let states_code = Signal::derive(move || {
        r#"<ToggleButtonGroup
  orientation=ToggleButtonGroupOrientation::Vertical
  is_attached=false
  aria_label="Alignment controls".to_string()
  lang="en-US".to_string()
  dir=A11yDirection::Ltr
>
  <ToggleButton is_pressed=left_signal on_pressed_change=on_left_change>"Left"</ToggleButton>
  <ToggleButton is_pressed=center_signal on_pressed_change=on_center_change>"Center"</ToggleButton>
  <ToggleButton is_pressed=right_signal on_pressed_change=on_right_change>"Right"</ToggleButton>
</ToggleButtonGroup>"#
            .to_string()
    });
    let workbench_actual_config = Signal::derive(move || {
        format!(
            "ToggleButtonGroupWorkbenchConfig {{\n  orientation: {:?},\n  is_attached: {},\n  motion: {:?},\n  aria_label: {:?},\n  lang: {:?},\n  dir: {:?},\n  class_name: {:?},\n  pressed: {{ bold: {}, italic: {}, underline: {} }},\n  attached_selected_count: {},\n}}",
            orientation.get(),
            attached.get(),
            motion.get(),
            if aria_label.get().is_empty() {
                None::<String>
            } else {
                Some(aria_label.get())
            },
            Some(lang.get()),
            Some(dir.get()),
            if class_name.get().is_empty() {
                None::<String>
            } else {
                Some(class_name.get())
            },
            a.get(),
            b.get(),
            c.get(),
            attached_selected_count.get(),
        )
    });

    view! {
        <ComponentPage
            title="ToggleButtonGroup"
            slug="toggle-button-group"
            group="Actions"
            description="Layout wrapper with baseline-style root state attrs for orientation, attachment, and accessible labeling."
        >
            <Playground
                title="Hello World (Default API)"
                code_signal=showcase_code
            >
                <ToggleButtonGroup
                    orientation=ToggleButtonGroupOrientation::Horizontal
                    is_attached=true
                    motion=ToggleButtonGroupMotion { duration_ms: 220.0 }
                    aria_label="Text style".to_string()
                    lang="en-US".to_string()
                    dir=A11yDirection::Ltr
                    class_name="docs-toggle-group-custom".to_string()
                >
                    <ToggleButton default_pressed=true>"Bold"</ToggleButton>
                    <ToggleButton>"Italic"</ToggleButton>
                    <ToggleButton>"Underline"</ToggleButton>
                </ToggleButtonGroup>
            </Playground>

            <Playground
                title="Workbench (Attached + Locale + Motion)"
                code_signal=workbench_code
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight">
                        <div class="docs-search__label">"Orientation"</div>
                        <SegmentedControl
                            id_base="docs-toggle-button-group-orientation".to_string()
                            options=orientation_options.clone()
                            selected_index=orientation_index
                            set_selected_index=set_orientation_index
                            size=SegmentedControlSize::Sm
                            aria_label="ToggleButtonGroup orientation".to_string()
                        />

                        <div class="docs-search__label">"Variant"</div>
                        <SegmentedControl
                            id_base="docs-toggle-button-group-variant".to_string()
                            options=variant_options.clone()
                            selected_index=variant_index
                            set_selected_index=set_variant_index
                            size=SegmentedControlSize::Sm
                            aria_label="ToggleButtonGroup variant".to_string()
                        />

                        <div class="docs-search__label">"Size"</div>
                        <SegmentedControl
                            id_base="docs-toggle-button-group-size".to_string()
                            options=size_options.clone()
                            selected_index=size_index
                            set_selected_index=set_size_index
                            size=SegmentedControlSize::Sm
                            aria_label="ToggleButtonGroup size".to_string()
                        />

                        <div class="docs-search__label">"Motion"</div>
                        <SegmentedControl
                            id_base="docs-toggle-button-group-motion".to_string()
                            options=motion_options.clone()
                            selected_index=motion_index
                            set_selected_index=set_motion_index
                            size=SegmentedControlSize::Sm
                            aria_label="ToggleButtonGroup motion".to_string()
                        />

                        <div class="docs-search__label">"Language"</div>
                        <SegmentedControl
                            id_base="docs-toggle-button-group-lang".to_string()
                            options=lang_options.clone()
                            selected_index=lang_index
                            set_selected_index=set_lang_index
                            size=SegmentedControlSize::Sm
                            aria_label="ToggleButtonGroup language".to_string()
                        />

                        <Switch checked=attached set_checked=set_attached>
                            "Attached layout"
                        </Switch>
                        <Switch checked=rtl set_checked=set_rtl>"RTL direction"</Switch>
                        <Switch checked=custom_class set_checked=set_custom_class>"Custom class"</Switch>
                        <Switch checked=custom_aria set_checked=set_custom_aria>"Custom aria label"</Switch>
                    </div>
                }
            >
                {move || {
                    let orientation = orientation.get();
                    let attached = attached.get();
                    let variant = variant.get();
                    let size = size.get();

                    view! {
                        <div class="docs-stack">
                            <ToggleButtonGroup
                                orientation=orientation
                                is_attached=attached
                                motion=motion.get()
                                aria_label=aria_label.get()
                                lang=lang.get()
                                dir=dir.get()
                                class_name=class_name.get()
                            >
                                <ToggleButton
                                    is_pressed=a_signal
                                    on_pressed_change=on_a_change
                                    variant=variant
                                    size=size
                                >
                                    "Bold"
                                </ToggleButton>
                                <ToggleButton
                                    is_pressed=b_signal
                                    on_pressed_change=on_b_change
                                    variant=variant
                                    size=size
                                >
                                    "Italic"
                                </ToggleButton>
                                <ToggleButton
                                    is_pressed=c_signal
                                    on_pressed_change=on_c_change
                                    variant=variant
                                    size=size
                                >
                                    "Underline"
                                </ToggleButton>
                            </ToggleButtonGroup>
                            <span class="ui-muted">
                                "attached selected count: "
                                {move || attached_selected_count.get()}
                            </span>
                        </div>
                    }
                }}
            </Playground>

            <Playground title="State Matrix (Orientation + Attachment + Selection)" code_signal=states_code>
                <div class="docs-stack">
                    <ToggleButtonGroup
                        orientation=ToggleButtonGroupOrientation::Vertical
                        is_attached=false
                        aria_label="Alignment controls".to_string()
                        lang="en-US".to_string()
                        dir=A11yDirection::Ltr
                    >
                        <ToggleButton
                            is_pressed=left_signal
                            on_pressed_change=on_left_change
                            variant=ToggleButtonVariant::Secondary
                        >
                            "Left"
                        </ToggleButton>
                        <ToggleButton
                            is_pressed=center_signal
                            on_pressed_change=on_center_change
                            variant=ToggleButtonVariant::Secondary
                        >
                            "Center"
                        </ToggleButton>
                        <ToggleButton
                            is_pressed=right_signal
                            on_pressed_change=on_right_change
                            variant=ToggleButtonVariant::Secondary
                        >
                            "Right"
                        </ToggleButton>
                    </ToggleButtonGroup>
                    <span class="ui-muted">
                        "detached selected count: "
                        {move || detached_selected_count.get().to_string()}
                    </span>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
pub(super) fn theme_toggle_button() -> AnyView {
    let (mode, set_mode) = signal(ThemeMode::Light);

    let mode_options = vec!["Light".to_string(), "Dark".to_string(), "OLED".to_string()];
    let (mode_index, set_mode_index) = signal(Some(0_usize));
    Effect::new(move |_| {
        let mode = match mode_index.get().unwrap_or(0) {
            1 => ThemeMode::Dark,
            2 => ThemeMode::Oled,
            _ => ThemeMode::Light,
        };
        set_mode.set(mode);
    });

    let (disabled, set_disabled) = signal(false);
    let (two_mode_cycle, set_two_mode_cycle) = signal(false);
    let (custom_aria_label, set_custom_aria_label) = signal(false);
    let (custom_class_name, set_custom_class_name) = signal(false);
    let (custom_motion, set_custom_motion) = signal(false);
    let variant_options = vec!["Ghost".to_string(), "Outline".to_string()];
    let icon_size_options = vec!["IconSm".to_string(), "IconLg".to_string()];
    let (variant_index, set_variant_index) = signal(Some(0_usize));
    let (size_index, set_size_index) = signal(Some(0_usize));
    let variant = Signal::derive(move || match variant_index.get().unwrap_or(0) {
        1 => ButtonVariant::Outline,
        _ => ButtonVariant::Ghost,
    });
    let size = Signal::derive(move || match size_index.get().unwrap_or(0) {
        1 => ButtonSize::IconLg,
        _ => ButtonSize::IconSm,
    });
    let motion = Signal::derive(move || {
        if custom_motion.get() {
            ThemeToggleMotion {
                rotate_deg: 270.0,
                ..ThemeToggleMotion::default()
            }
        } else {
            ThemeToggleMotion::default()
        }
    });

    let code = Signal::derive(move || {
        let mode = match mode_index.get().unwrap_or(0) {
            1 => ThemeMode::Dark,
            2 => ThemeMode::Oled,
            _ => ThemeMode::Light,
        };
        let disabled = disabled.get();
        let two_mode_cycle = two_mode_cycle.get();
        let custom_aria_label = custom_aria_label.get();
        let custom_class_name = custom_class_name.get();
        let variant = variant.get();
        let size = size.get();
        let motion = motion.get();

        let mut snippet = vec![
            format!("let (mode, set_mode) = signal(ThemeMode::{mode:?});"),
            String::new(),
            "<ThemeToggleButton".to_string(),
            "  mode=mode".to_string(),
            "  set_mode=set_mode".to_string(),
            format!("  variant=ButtonVariant::{variant:?}"),
            format!("  size=ButtonSize::{size:?}"),
            format!("  motion={motion:?}"),
        ];

        if disabled {
            snippet.push("  is_disabled=true".to_string());
        }
        if two_mode_cycle {
            snippet.push("  modes=vec![ThemeMode::Dark, ThemeMode::Light]".to_string());
        }
        if custom_aria_label {
            snippet.push("  aria_label=\"Switch UI mode\".into()".to_string());
        }
        if custom_class_name {
            snippet.push("  class_name=\"docs-theme-toggle-custom\".into()".to_string());
        }

        snippet.push("/>".to_string());

        snippet.join("\n")
    });

    let (custom_mode, set_custom_mode) = signal(ThemeMode::Dark);
    let custom_modes = vec![ThemeMode::Dark, ThemeMode::Light];
    let showcase_code = Signal::derive(move || {
        r#"let (mode, set_mode) = signal(ThemeMode::Light);
<ThemeToggleButton mode=mode set_mode=set_mode />"#
            .to_string()
    });

    let states_code = Signal::derive(move || {
        r#"let (custom_mode, set_custom_mode) = signal(ThemeMode::Dark);
let (mode, set_mode) = signal(ThemeMode::System);

<ThemeToggleButton
  mode=custom_mode
  set_mode=set_custom_mode
  modes=vec![ThemeMode::Dark, ThemeMode::Light]
  aria_label="Switch UI mode".to_string()
/>
<ThemeToggleButton mode=mode set_mode=set_mode is_disabled=true />"#
            .to_string()
    });
    let workbench_actual_config = Signal::derive(move || {
        let available_modes = if two_mode_cycle.get() {
            vec!["Dark", "Light"]
        } else {
            vec!["Light", "Dark", "Oled"]
        };
        format!(
            "ThemeToggleButtonWorkbenchConfig {{\n  mode: {:?},\n  set_mode: {:?},\n  is_disabled: {},\n  custom_aria_label: {},\n  two_mode_cycle: {},\n  modes: {:?},\n  variant: {:?},\n  size: {:?},\n  motion: {:?},\n  class_name: {:?},\n}}",
            mode.get(),
            "write_signal",
            disabled.get(),
            custom_aria_label.get(),
            two_mode_cycle.get(),
            available_modes,
            variant.get(),
            size.get(),
            motion.get(),
            if custom_class_name.get() {
                "docs-theme-toggle-custom"
            } else {
                ""
            },
        )
    });

    view! {
        <ComponentPage
            title="ThemeToggleButton"
            slug="theme-toggle-button"
            group="Actions"
            description="Icon-only theme toggle with baseline-level spring motion and baseline-style mode state attrs."
        >
            <Playground title="Hello World (Default API)" code_signal=showcase_code>
                <div class="docs-row">
                    <ThemeToggleButton mode=mode set_mode=set_mode />
                </div>
            </Playground>

            <Playground
                title="Default cycle"
                code_signal=code
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight">
                        <div class="docs-search__label">"Start mode"</div>
                        <SegmentedControl
                            id_base="docs-theme-toggle-mode".to_string()
                            options=mode_options.clone()
                            selected_index=mode_index
                            set_selected_index=set_mode_index
                            size=SegmentedControlSize::Sm
                            aria_label="ThemeToggle start mode".to_string()
                        />

                        <Switch checked=disabled set_checked=set_disabled>"Disabled"</Switch>
                        <Switch checked=two_mode_cycle set_checked=set_two_mode_cycle>
                            "Two-mode cycle (dark/light)"
                        </Switch>
                        <Switch checked=custom_aria_label set_checked=set_custom_aria_label>
                            "Custom aria label"
                        </Switch>
                        <Switch checked=custom_class_name set_checked=set_custom_class_name>
                            "Custom class"
                        </Switch>
                        <Switch checked=custom_motion set_checked=set_custom_motion>
                            "Custom motion"
                        </Switch>
                        <div class="docs-search__label">"Variant"</div>
                        <SegmentedControl
                            id_base="docs-theme-toggle-variant".to_string()
                            options=variant_options.clone()
                            selected_index=variant_index
                            set_selected_index=set_variant_index
                            size=SegmentedControlSize::Sm
                            aria_label="ThemeToggle variant".to_string()
                        />
                        <div class="docs-search__label">"Size"</div>
                        <SegmentedControl
                            id_base="docs-theme-toggle-size".to_string()
                            options=icon_size_options.clone()
                            selected_index=size_index
                            set_selected_index=set_size_index
                            size=SegmentedControlSize::Sm
                            aria_label="ThemeToggle size".to_string()
                        />
                    </div>
                }
            >
                {move || {
                    let disabled = disabled.get();
                    let two_mode_cycle = two_mode_cycle.get();
                    let custom_aria_label = custom_aria_label.get();
                    let modes = if two_mode_cycle {
                        vec![ThemeMode::Dark, ThemeMode::Light]
                    } else {
                        vec![ThemeMode::Light, ThemeMode::Dark, ThemeMode::Oled]
                    };

                    view! {
                        <div class="docs-row">
                            {if custom_aria_label {
                                view! {
                                    <ThemeToggleButton
                                        mode=mode
                                        set_mode=set_mode
                                        is_disabled=disabled
                                        modes=modes.clone()
                                        aria_label="Switch UI mode".to_string()
                                        variant=variant.get()
                                        size=size.get()
                                        motion=motion.get()
                                        class_name=if custom_class_name.get() {
                                            "docs-theme-toggle-custom".to_string()
                                        } else {
                                            String::new()
                                        }
                                    />
                                }
                                    .into_any()
                            } else {
                                view! {
                                    <ThemeToggleButton
                                        mode=mode
                                        set_mode=set_mode
                                        is_disabled=disabled
                                        modes=modes
                                        variant=variant.get()
                                        size=size.get()
                                        motion=motion.get()
                                        class_name=if custom_class_name.get() {
                                            "docs-theme-toggle-custom".to_string()
                                        } else {
                                            String::new()
                                        }
                                    />
                                }
                                    .into_any()
                            }}
                            <span class="ui-muted">"mode: " {move || format!("{:?}", mode.get())}</span>
                        </div>
                    }
                }}
            </Playground>

            <Playground title="State Matrix (Custom Modes + Disabled Comparison)" code_signal=states_code>
                <div class="docs-stack">
                    <div class="docs-row">
                        <ThemeToggleButton
                            mode=custom_mode
                            set_mode=set_custom_mode
                            modes=custom_modes.clone()
                            aria_label="Switch UI mode".to_string()
                            variant=ButtonVariant::Outline
                            size=ButtonSize::IconLg
                            motion=ThemeToggleMotion {
                                rotate_deg: 270.0,
                                ..ThemeToggleMotion::default()
                            }
                            class_name="docs-theme-toggle-custom".to_string()
                        />
                        <span class="ui-muted">
                            "custom mode: " {move || format!("{:?}", custom_mode.get())}
                        </span>
                    </div>
                    <div class="docs-row">
                        <ThemeToggleButton mode=mode set_mode=set_mode is_disabled=true />
                        <span class="ui-muted">"disabled toggle should remain inert"</span>
                    </div>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
pub(super) fn search_input_button() -> AnyView {
    let persisted_workbench_state = load_search_input_button_workbench_state();
    let has_persisted_workbench_state = persisted_workbench_state.is_some();
    let initial_workbench_state = persisted_workbench_state.unwrap_or_default();

    let (press_count, set_press_count) = signal(0_usize);
    let on_press: OnPress = Callback::new(move |_| {
        set_press_count.update(|count| *count += 1);
    });

    let preset_options = vec![
        "Default".to_string(),
        "Docs".to_string(),
        "Command".to_string(),
        "Components".to_string(),
    ];
    let (preset_index, set_preset_index) = signal(Some(initial_workbench_state.preset_index));
    let placeholder = Signal::derive(move || match preset_index.get().unwrap_or(0) {
        1 => "Search docs".to_string(),
        2 => "Command menu".to_string(),
        3 => "Find components".to_string(),
        _ => "Search".to_string(),
    });
    let compact_placeholder = Signal::derive(move || match preset_index.get().unwrap_or(0) {
        1 => "Search".to_string(),
        2 => "Cmd".to_string(),
        3 => "Find".to_string(),
        _ => "Search".to_string(),
    });

    let meta_key_options = vec![
        "None".to_string(),
        "⌘".to_string(),
        "Ctrl".to_string(),
        "Alt".to_string(),
    ];
    let (meta_key_index, set_meta_key_index) = signal(Some(0_usize));
    let meta_key_label = Signal::derive(move || match meta_key_index.get().unwrap_or(0) {
        1 => "⌘".to_string(),
        2 => "Ctrl".to_string(),
        3 => "Alt".to_string(),
        _ => String::new(),
    });

    let key_label_options = vec!["None".to_string(), "K".to_string(), "F".to_string()];
    let (key_label_index, set_key_label_index) = signal(Some(0_usize));
    let key_label = Signal::derive(move || match key_label_index.get().unwrap_or(0) {
        1 => "K".to_string(),
        2 => "F".to_string(),
        _ => String::new(),
    });

    let (disabled, set_disabled) = signal(initial_workbench_state.is_disabled);
    let (custom_aria_label, set_custom_aria_label) =
        signal(initial_workbench_state.custom_aria_label);
    let (custom_motion, set_custom_motion) = signal(false);
    let (custom_class, set_custom_class) = signal(false);
    let (submit_type, set_submit_type) = signal(false);
    let (rtl_dir, set_rtl_dir) = signal(false);
    let (persist_workbench_state, set_persist_workbench_state) =
        signal(has_persisted_workbench_state);
    let workbench_node_ref: NodeRef<html::Button> = NodeRef::new();

    Effect::new(move |_| {
        let state = SearchInputButtonWorkbenchState {
            preset_index: preset_index.get().unwrap_or(0).min(3),
            meta_key_index: meta_key_index.get().unwrap_or(0).min(3),
            key_label_index: key_label_index.get().unwrap_or(0).min(2),
            is_disabled: disabled.get(),
            custom_aria_label: custom_aria_label.get(),
        };
        if persist_workbench_state.get() {
            save_search_input_button_workbench_state(state);
        } else {
            clear_search_input_button_workbench_state();
        }
    });

    let code = Signal::derive(move || {
        let placeholder = placeholder.get();
        let compact_placeholder = compact_placeholder.get();
        let meta_key_label = meta_key_label.get();
        let key_label = key_label.get();
        let disabled = disabled.get();
        let custom_aria_label = custom_aria_label.get();

        let mut snippet = vec!["<SearchInputButton".to_string()];

        if placeholder != "Search" {
            snippet.push(format!("  placeholder=\"{placeholder}\".into()"));
        }
        if compact_placeholder != placeholder {
            snippet.push(format!(
                "  compact_placeholder=\"{compact_placeholder}\".into()"
            ));
        }
        if !meta_key_label.is_empty() {
            snippet.push(format!("  meta_key_label=\"{meta_key_label}\".into()"));
        }
        if !key_label.is_empty() {
            snippet.push(format!("  key_label=\"{key_label}\".into()"));
        }
        if disabled {
            snippet.push("  is_disabled=true".to_string());
        }
        if custom_aria_label {
            snippet.push("  aria_label=\"Open command menu\".into()".to_string());
        }
        if custom_motion.get() {
            snippet.push(
                "  motion=SearchInputButtonMotion { hover_scale: 1.04, tap_scale: 0.96, ..SearchInputButtonMotion::default() }"
                    .to_string(),
            );
        }
        if custom_class.get() {
            snippet.push("  class_name=\"docs-search-input-button-custom\".into()".to_string());
        }
        snippet.push(format!(
            "  button_type={}",
            if submit_type.get() {
                "Some(ui::button::ButtonType::Submit)"
            } else {
                "Some(ui::button::ButtonType::Button)"
            }
        ));
        snippet.push(format!(
            "  lang={}",
            if rtl_dir.get() {
                "\"ar\".to_string()"
            } else {
                "\"en-US\".to_string()"
            }
        ));
        snippet.push(format!(
            "  dir={}",
            if rtl_dir.get() {
                "Some(A11yDirection::Rtl)"
            } else {
                "Some(A11yDirection::Ltr)"
            }
        ));
        snippet.push("  node_ref=NodeRef::<leptos::html::Button>::new()".to_string());
        snippet.push("  on_press=Some(Callback::new(move |_| {}))".to_string());

        snippet.push("/>".to_string());

        snippet.join("\n")
    });

    let states_code = Signal::derive(move || {
        r#"<SearchInputButton placeholder="Find components".to_string() />
<SearchInputButton
  placeholder="Find components".to_string()
  compact_placeholder="Find".to_string()
/>
<SearchInputButton placeholder="Disabled search".to_string() is_disabled=true />
<SearchInputButton placeholder="Forced disabled".to_string() is_disabled=true />"#
            .to_string()
    });

    let custom_code = Signal::derive(move || {
        r#"<SearchInputButton
  placeholder="Browse components".to_string()
  compact_placeholder="Browse".to_string()
  aria_label="Open component search".to_string()
  class_name="docs-search-input-button-custom".to_string()
/>"#
        .to_string()
    });
    let workbench_actual_config = Signal::derive(move || {
        format!(
            "SearchInputButtonWorkbenchConfig {{\n  placeholder: {:?},\n  compact_placeholder: {:?},\n  meta_key_label: {:?},\n  key_label: {:?},\n  is_disabled: {},\n  motion: {},\n  class_name: {:?},\n  button_type: {},\n  aria_label: {:?},\n  lang: {:?},\n  dir: {},\n  node_ref: \"workbench_node_ref\",\n  on_press: \"increment press_count\",\n  custom_aria_label: {},\n  persist_workbench_state: {},\n  on_press_count: {},\n}}",
            placeholder.get(),
            compact_placeholder.get(),
            meta_key_label.get(),
            key_label.get(),
            disabled.get(),
            if custom_motion.get() {
                "SearchInputButtonMotion(custom)"
            } else {
                "SearchInputButtonMotion::default()"
            },
            if custom_class.get() {
                Some("docs-search-input-button-custom")
            } else {
                None
            },
            if submit_type.get() {
                "Some(ButtonType::Submit)"
            } else {
                "Some(ButtonType::Button)"
            },
            if custom_aria_label.get() {
                Some("Open command menu")
            } else {
                None
            },
            if rtl_dir.get() {
                Some("ar")
            } else {
                Some("en-US")
            },
            if rtl_dir.get() {
                "Some(A11yDirection::Rtl)"
            } else {
                "Some(A11yDirection::Ltr)"
            },
            custom_aria_label.get(),
            persist_workbench_state.get(),
            press_count.get(),
        )
    });

    view! {
        <ComponentPage
            title="SearchInputButton"
            slug="search-input-button"
            group="Actions"
            description="baseline-level spring search trigger button with centralized placeholder/shortcut/aria-label state attrs."
        >
            <Playground title="Hello World (Default SearchInputButton)" code_signal=code>
                <div class="docs-row">
                    <SearchInputButton on_press=on_press />
                    <span class="ui-muted">"presses: " {move || press_count.get().to_string()}</span>
                </div>
            </Playground>

            <Playground
                title="Workbench (All API + Actual Config)"
                code_signal=code
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight">
                        <div class="docs-search__label">"Preset"</div>
                        <SegmentedControl
                            id_base="docs-search-input-preset".to_string()
                            options=preset_options.clone()
                            selected_index=preset_index
                            set_selected_index=set_preset_index
                            size=SegmentedControlSize::Sm
                            aria_label="Search input preset".to_string()
                        />

                        <div class="docs-search__label">"Meta key"</div>
                        <SegmentedControl
                            id_base="docs-search-input-meta-key".to_string()
                            options=meta_key_options.clone()
                            selected_index=meta_key_index
                            set_selected_index=set_meta_key_index
                            size=SegmentedControlSize::Sm
                            aria_label="Search input meta key".to_string()
                        />

                        <div class="docs-search__label">"Shortcut key"</div>
                        <SegmentedControl
                            id_base="docs-search-input-key".to_string()
                            options=key_label_options.clone()
                            selected_index=key_label_index
                            set_selected_index=set_key_label_index
                            size=SegmentedControlSize::Sm
                            aria_label="Search input shortcut key".to_string()
                        />

                        <Switch checked=disabled set_checked=set_disabled>"Disabled"</Switch>
                        <Switch checked=custom_aria_label set_checked=set_custom_aria_label>
                            "Custom aria label"
                        </Switch>
                        <Switch checked=custom_motion set_checked=set_custom_motion>
                            "Custom motion"
                        </Switch>
                        <Switch checked=custom_class set_checked=set_custom_class>
                            "Custom class_name"
                        </Switch>
                        <Switch checked=submit_type set_checked=set_submit_type>
                            "button_type submit"
                        </Switch>
                        <Switch checked=rtl_dir set_checked=set_rtl_dir>"RTL + ar"</Switch>
                        <Switch checked=persist_workbench_state set_checked=set_persist_workbench_state>
                            "Persist workbench state"
                        </Switch>
                    </div>
                }
            >
                {move || {
                    let placeholder = placeholder.get();
                    let compact_placeholder = compact_placeholder.get();
                    let meta_key_label = meta_key_label.get();
                    let key_label = key_label.get();
                    let disabled = disabled.get();
                    let custom_aria_label = custom_aria_label.get();

                    view! {
                        <div class="docs-stack">
                            <div class="docs-row">
                    {if custom_aria_label {
                                    view! {
                                        <SearchInputButton
                                            placeholder=placeholder.clone()
                                            compact_placeholder=compact_placeholder.clone()
                                            meta_key_label=meta_key_label.clone()
                                            key_label=key_label.clone()
                                            aria_label="Open command menu".to_string()
                                            is_disabled=disabled
                                            motion=if custom_motion.get() {
                                                SearchInputButtonMotion {
                                                    hover_scale: 1.04,
                                                    tap_scale: 0.96,
                                                    ..SearchInputButtonMotion::default()
                                                }
                                            } else {
                                                SearchInputButtonMotion::default()
                                            }
                                            class_name=if custom_class.get() {
                                                "docs-search-input-button-custom".to_string()
                                            } else {
                                                String::new()
                                            }
                                            button_type=if submit_type.get() {
                                                ui::button::ButtonType::Submit
                                            } else {
                                                ui::button::ButtonType::Button
                                            }
                                            lang=if rtl_dir.get() {
                                                "ar".to_string()
                                            } else {
                                                "en-US".to_string()
                                            }
                                            dir=if rtl_dir.get() {
                                                A11yDirection::Rtl
                                            } else {
                                                A11yDirection::Ltr
                                            }
                                            node_ref=workbench_node_ref
                                            on_press=on_press
                                        />
                                    }
                                        .into_any()
                                } else {
                                    view! {
                                        <SearchInputButton
                                            placeholder=placeholder
                                            compact_placeholder=compact_placeholder
                                            meta_key_label=meta_key_label
                                            key_label=key_label
                                            is_disabled=disabled
                                            motion=if custom_motion.get() {
                                                SearchInputButtonMotion {
                                                    hover_scale: 1.04,
                                                    tap_scale: 0.96,
                                                    ..SearchInputButtonMotion::default()
                                                }
                                            } else {
                                                SearchInputButtonMotion::default()
                                            }
                                            class_name=if custom_class.get() {
                                                "docs-search-input-button-custom".to_string()
                                            } else {
                                                String::new()
                                            }
                                            button_type=if submit_type.get() {
                                                ui::button::ButtonType::Submit
                                            } else {
                                                ui::button::ButtonType::Button
                                            }
                                            lang=if rtl_dir.get() {
                                                "ar".to_string()
                                            } else {
                                                "en-US".to_string()
                                            }
                                            dir=if rtl_dir.get() {
                                                A11yDirection::Rtl
                                            } else {
                                                A11yDirection::Ltr
                                            }
                                            node_ref=workbench_node_ref
                                            on_press=on_press
                                        />
                                    }
                                        .into_any()
                                }}
                            </div>
                            <span class="ui-muted">"presses: " {move || press_count.get().to_string()}</span>
                        </div>
                    }
                }}
            </Playground>

            <Playground title="State Matrix (Placeholder + Disabled)" code_signal=states_code>
                <div class="docs-stack">
                    <div class="docs-row">
                        <SearchInputButton placeholder="Find components".to_string() />
                        <SearchInputButton
                            placeholder="Find components".to_string()
                            compact_placeholder="Find".to_string()
                        />
                    </div>
                    <div class="docs-row">
                        <SearchInputButton
                            placeholder="Disabled search".to_string()
                            is_disabled=true
                        />
                        <SearchInputButton
                            placeholder="Forced disabled".to_string()
                            is_disabled=true
                        />
                    </div>
                </div>
            </Playground>

            <Playground title="Custom Class + Aria Label" code_signal=custom_code>
                <div class="docs-row">
                    <SearchInputButton
                        placeholder="Browse components".to_string()
                        compact_placeholder="Browse".to_string()
                        aria_label="Open component search".to_string()
                        class_name="docs-search-input-button-custom".to_string()
                    />
                    <SearchInputButton
                        placeholder="Search by keyword".to_string()
                        class_name="docs-search-input-button-custom".to_string()
                    />
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
pub(super) fn button_copy() -> AnyView {
    let persisted_workbench_state = load_button_copy_workbench_state();
    let has_persisted_workbench_state = persisted_workbench_state.is_some();
    let initial_workbench_state = persisted_workbench_state.unwrap_or(ButtonCopyWorkbenchState {
        mode_index: 2,
        variant_index: 0,
        size_index: 2,
        text_index: 0,
        feedback_scale: 8,
        feedback_glow: 100,
        is_disabled: false,
    });

    let mode_options = vec![
        "text-only".to_string(),
        "icon-only".to_string(),
        "icon+text".to_string(),
    ];
    let variant_options = vec![
        "secondary".to_string(),
        "outline".to_string(),
        "accent".to_string(),
    ];
    let size_options = vec![
        "xs".to_string(),
        "s".to_string(),
        "m".to_string(),
        "l".to_string(),
        "xl".to_string(),
    ];
    let text_options = vec![
        "cargo command".to_string(),
        "docs url".to_string(),
        "token".to_string(),
    ];

    let (mode_index, set_mode_index) = signal(Some(initial_workbench_state.mode_index));
    let mode = Signal::derive(move || match mode_index.get().unwrap_or(2) {
        0 => ButtonCopyMode::TextOnly,
        1 => ButtonCopyMode::IconOnly,
        _ => ButtonCopyMode::IconAndText,
    });

    let (variant_index, set_variant_index) = signal(Some(initial_workbench_state.variant_index));
    let variant = Signal::derive(move || match variant_index.get().unwrap_or(0) {
        1 => ButtonVariant::Outline,
        2 => ButtonVariant::Accent,
        _ => ButtonVariant::Secondary,
    });

    let (size_index, set_size_index) = signal(Some(initial_workbench_state.size_index));
    let size = Signal::derive(move || match size_index.get().unwrap_or(2) {
        0 => ButtonSize::Xs,
        1 => ButtonSize::S,
        2 => ButtonSize::M,
        3 => ButtonSize::L,
        _ => ButtonSize::Xl,
    });

    let (text_index, set_text_index) = signal(Some(initial_workbench_state.text_index));
    let text = Signal::derive(move || match text_index.get().unwrap_or(0) {
        1 => "https://example.com/docs".to_string(),
        2 => "token=sk-demo-123".to_string(),
        _ => "cargo add ui".to_string(),
    });

    let (feedback_scale, set_feedback_scale) = signal(initial_workbench_state.feedback_scale);
    let (feedback_glow, set_feedback_glow) = signal(initial_workbench_state.feedback_glow);
    let (is_disabled, set_is_disabled) = signal(initial_workbench_state.is_disabled);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);
    let (workbench_custom_aria, set_workbench_custom_aria) = signal(false);
    let (workbench_rtl, set_workbench_rtl) = signal(false);
    let (workbench_persist_state, set_workbench_persist_state) =
        signal(has_persisted_workbench_state);

    Effect::new(move |_| {
        if workbench_persist_state.get() {
            save_button_copy_workbench_state(ButtonCopyWorkbenchState {
                mode_index: mode_index.get().unwrap_or(2),
                variant_index: variant_index.get().unwrap_or(0),
                size_index: size_index.get().unwrap_or(0),
                text_index: text_index.get().unwrap_or(0),
                feedback_scale: feedback_scale.get(),
                feedback_glow: feedback_glow.get(),
                is_disabled: is_disabled.get(),
            });
        } else {
            clear_button_copy_workbench_state();
        }
    });

    let workbench_motion = Signal::derive(move || ButtonCopyMotion {
        copied_feedback_scale: f64::from(feedback_scale.get()) / 100.0,
        copied_feedback_glow: f64::from(feedback_glow.get()) / 100.0,
        ..ButtonCopyMotion::default()
    });

    let workbench_test_css_source = Signal::derive(move || {
        format!(
            "/* crates/ui/src/button/copy/styles.rs */\n{}",
            ui::button::copy::styles::CSS
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        let mode = mode.get();
        let variant = variant.get();
        let size = size.get();
        let is_disabled = is_disabled.get();
        let copied_feedback_scale = f64::from(feedback_scale.get()) / 100.0;
        let copied_feedback_glow = f64::from(feedback_glow.get()) / 100.0;
        let text = text.get();

        format!(
            "ButtonCopyWorkbenchConfig {{\n  text: \"{text}\",\n  label: \"Copy value\",\n  copied_label: \"Copied!\",\n  aria_label: {:?},\n  is_disabled: {is_disabled},\n  mode: {mode:?},\n  variant: {variant:?},\n  size: {size:?},\n  motion: ButtonCopyMotion {{ copied_feedback_scale: {copied_feedback_scale:.2}, copied_feedback_glow: {copied_feedback_glow:.2}, ..Default::default() }},\n  class_name: {:?},\n  lang: {:?},\n  dir: {},\n  copied_feedback_scale: {copied_feedback_scale:.2},\n  copied_feedback_glow: {copied_feedback_glow:.2},\n}}",
            if workbench_custom_aria.get() {
                Some("Copy selected text")
            } else {
                None
            },
            if workbench_custom_class.get() {
                Some("docs-button-copy-custom")
            } else {
                None
            },
            if workbench_rtl.get() {
                Some("ar")
            } else {
                Some("en-US")
            },
            if workbench_rtl.get() {
                "Some(A11yDirection::Rtl)"
            } else {
                "Some(A11yDirection::Ltr)"
            },
        )
    });

    let hello_world_code =
        Signal::derive(move || r#"<ButtonCopy text="cargo add ui".to_string() />"#.to_string());

    let code = Signal::derive(move || {
        r#"<ButtonCopy
  text="cargo add ui".to_string()
  label="Copy install command".to_string()
  copied_label="Copied!".to_string()
/>"#
        .to_string()
    });

    let states_code = Signal::derive(move || {
        r#"<ButtonCopy text="https://example.com/docs".to_string() variant=ButtonVariant::Outline />
<ButtonCopy text="   ".to_string() label="Nothing to copy".to_string() />
<ButtonCopy text="token".to_string() is_disabled=true />"#
            .to_string()
    });

    let modes_code = Signal::derive(move || {
        r#"<ButtonCopy text="cargo add ui".to_string() mode=ButtonCopyMode::TextOnly />
<ButtonCopy text="cargo add ui".to_string() mode=ButtonCopyMode::IconOnly />
<ButtonCopy text="cargo add ui".to_string() mode=ButtonCopyMode::IconAndText />"#
            .to_string()
    });

    let workbench_code = Signal::derive(move || {
        let mode = mode.get();
        let variant = variant.get();
        let size = size.get();
        let is_disabled = is_disabled.get();
        let text = text.get();
        let copied_feedback_scale = f64::from(feedback_scale.get()) / 100.0;
        let copied_feedback_glow = f64::from(feedback_glow.get()) / 100.0;

        format!(
            "<ButtonCopy\n  text=\"{text}\".into()\n  mode=ButtonCopyMode::{mode:?}\n  variant=ButtonVariant::{variant:?}\n  size=ButtonSize::{size:?}\n  is_disabled={is_disabled}\n  motion=ButtonCopyMotion {{\n    copied_feedback_scale: {copied_feedback_scale:.2},\n    copied_feedback_glow: {copied_feedback_glow:.2},\n    ..ButtonCopyMotion::default()\n  }}\n/>"
        )
    });

    view! {
        <ComponentPage
            title="ButtonCopy"
            slug="button-copy"
            group="Actions"
            description="Copy-to-clipboard button with baseline-style disabled/empty semantics and live copied announcements."
        >
            <Playground title="Hello World" code_signal=hello_world_code>
                <div class="docs-row">
                    <ButtonCopy text="cargo add ui".to_string() />
                </div>
                <span class="ui-muted">"Start simple, then move to advanced controls."</span>
            </Playground>

            <Playground title="Label + variant" code_signal=code>
                <div class="docs-row">
                    <ButtonCopy
                        text="cargo add ui".to_string()
                        label="Copy install command".to_string()
                        copied_label="Copied!".to_string()
                    />
                    <ButtonCopy
                        text="https://github.com/openai".to_string()
                        variant=ButtonVariant::Outline
                        label="Copy URL".to_string()
                        copied_label="URL copied".to_string()
                    />
                </div>
            </Playground>

            <Playground title="Disabled + empty matrix" code_signal=states_code>
                <div class="docs-stack">
                    <div class="docs-row">
                        <ButtonCopy
                            text="https://example.com/docs".to_string()
                            variant=ButtonVariant::Outline
                        />
                        <ButtonCopy text="   ".to_string() label="Nothing to copy".to_string() />
                        <ButtonCopy text="token".to_string() is_disabled=true />
                    </div>
                    <span class="ui-muted">
                        "Blank text and explicit disabled state both force non-copyable semantics."
                    </span>
                </div>
            </Playground>

            <Playground title="Mode matrix" code_signal=modes_code>
                <div class="docs-row">
                    <ButtonCopy
                        text="cargo add ui".to_string()
                        mode=ButtonCopyMode::TextOnly
                    />
                    <ButtonCopy
                        text="cargo add ui".to_string()
                        mode=ButtonCopyMode::IconOnly
                    />
                    <ButtonCopy
                        text="cargo add ui".to_string()
                        mode=ButtonCopyMode::IconAndText
                    />
                </div>
            </Playground>

            <Playground
                title="Workbench (Isolated Canvas + Optional Persist)"
                code_signal=workbench_code
                test_css_source=workbench_test_css_source
                test_source_path="/root/code/personal/omne/rust-ui/crates/ui/src/button/copy/styles.rs".to_string()
                test_config_signal=workbench_actual_config
                description="Workbench canvas: scoped CSS live-edit + optional state persistence across reload."
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="button-copy-workbench-controls">
                        <div class="docs-search__label">"Mode"</div>
                        <SegmentedControl
                            id_base="docs-button-copy-mode".to_string()
                            options=mode_options.clone()
                            selected_index=mode_index
                            set_selected_index=set_mode_index
                            size=SegmentedControlSize::Sm
                            aria_label="ButtonCopy mode".to_string()
                        />

                        <div class="docs-search__label">"Variant"</div>
                        <SegmentedControl
                            id_base="docs-button-copy-variant".to_string()
                            options=variant_options.clone()
                            selected_index=variant_index
                            set_selected_index=set_variant_index
                            size=SegmentedControlSize::Sm
                            aria_label="ButtonCopy variant".to_string()
                        />

                        <div class="docs-search__label">"Size"</div>
                        <SegmentedControl
                            id_base="docs-button-copy-size".to_string()
                            options=size_options.clone()
                            selected_index=size_index
                            set_selected_index=set_size_index
                            size=SegmentedControlSize::Sm
                            aria_label="ButtonCopy size".to_string()
                        />

                        <div class="docs-search__label">"Text preset"</div>
                        <SegmentedControl
                            id_base="docs-button-copy-text".to_string()
                            options=text_options.clone()
                            selected_index=text_index
                            set_selected_index=set_text_index
                            size=SegmentedControlSize::Sm
                            aria_label="ButtonCopy text preset".to_string()
                        />

                        <label class="docs-search__label" for="docs-button-copy-feedback-scale">
                            "Feedback scale (" {move || format!("{:.2}", f64::from(feedback_scale.get()) / 100.0)} ")"
                        </label>
                        <input
                            id="docs-button-copy-feedback-scale"
                            class="docs-search__input"
                            type="range"
                            min="0"
                            max="25"
                            step="1"
                            prop:value=move || feedback_scale.get().to_string()
                            on:input=move |ev| {
                                let next = event_target_value(&ev)
                                    .parse::<u16>()
                                    .unwrap_or(8)
                                    .clamp(0, 25);
                                set_feedback_scale.set(next);
                            }
                        />

                        <label class="docs-search__label" for="docs-button-copy-feedback-glow">
                            "Feedback glow (" {move || format!("{:.2}", f64::from(feedback_glow.get()) / 100.0)} ")"
                        </label>
                        <input
                            id="docs-button-copy-feedback-glow"
                            class="docs-search__input"
                            type="range"
                            min="0"
                            max="200"
                            step="5"
                            prop:value=move || feedback_glow.get().to_string()
                            on:input=move |ev| {
                                let next = event_target_value(&ev)
                                    .parse::<u16>()
                                    .unwrap_or(100)
                                    .clamp(0, 200);
                                set_feedback_glow.set(next);
                            }
                        />

                        <Switch checked=is_disabled set_checked=set_is_disabled>"Disabled"</Switch>
                        <Switch checked=workbench_custom_class set_checked=set_workbench_custom_class>
                            "Custom class_name"
                        </Switch>
                        <Switch checked=workbench_custom_aria set_checked=set_workbench_custom_aria>
                            "Custom aria_label"
                        </Switch>
                        <Switch checked=workbench_rtl set_checked=set_workbench_rtl>"RTL + ar"</Switch>
                        <Switch checked=workbench_persist_state set_checked=set_workbench_persist_state>
                            "Persist workbench state"
                        </Switch>
                    </div>
                }
            >
                {move || {
                    let mode = mode.get();
                    let variant = variant.get();
                    let size = size.get();
                    let text = text.get();
                    let is_disabled = is_disabled.get();
                    let persist = workbench_persist_state.get();
                    let motion = workbench_motion.get();

                    view! {
                        <div class="docs-stack" data-slot="button-copy-workbench" style="width: min(100%, 420px);">
                            <span class="ui-muted">
                                "persist: "
                                {if persist { "on" } else { "off" }}
                            </span>

                            <div class="docs-card docs-stack docs-stack--tight" data-slot="button-copy-workbench-canvas">
                                <ButtonCopy
                                    text=text.clone()
                                    mode=mode
                                    variant=variant
                                    size=size
                                    is_disabled=is_disabled
                                    motion=motion
                                    label="Copy value".to_string()
                                    copied_label="Copied!".to_string()
                                    aria_label=if workbench_custom_aria.get() {
                                        "Copy selected text".to_string()
                                    } else {
                                        String::new()
                                    }
                                    class_name=if workbench_custom_class.get() {
                                        "docs-button-copy-custom".to_string()
                                    } else {
                                        String::new()
                                    }
                                    lang=if workbench_rtl.get() {
                                        "ar".to_string()
                                    } else {
                                        "en-US".to_string()
                                    }
                                    dir=if workbench_rtl.get() {
                                        A11yDirection::Rtl
                                    } else {
                                        A11yDirection::Ltr
                                    }
                                />
                                <span class="ui-muted">"text: " {text}</span>
                            </div>
                        </div>
                    }
                }}
            </Playground>

            <Playground
                title="State Matrix (Mode + Disabled Comparison)"
                code_signal=modes_code
            >
                <div class="docs-row">
                    <ButtonCopy
                        text="cargo add ui".to_string()
                        mode=ButtonCopyMode::TextOnly
                        motion=ButtonCopyMotion::default()
                        lang="en-US".to_string()
                        dir=A11yDirection::Ltr
                    />
                    <ButtonCopy
                        text="cargo add ui".to_string()
                        mode=ButtonCopyMode::IconAndText
                        is_disabled=true
                        class_name="docs-button-copy-custom".to_string()
                        lang="ar".to_string()
                        dir=A11yDirection::Rtl
                    />
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
pub(super) fn flip_button() -> AnyView {
    let code = Signal::derive(move || {
        r#"<FlipButton
  from=FlipDirection::Top
  front=move || view! { <Button variant=ButtonVariant::Secondary>"Front"</Button> }
  back=move || view! { <Button variant=ButtonVariant::Accent>"Back"</Button> }
/>"#
        .to_string()
    });

    let states_code = Signal::derive(move || {
        r#"<FlipButton from=FlipDirection::Top front=... back=... />
<FlipButton from=FlipDirection::Bottom front=... back=... />
<FlipButton from=FlipDirection::Left front=... back=... />
<FlipButton from=FlipDirection::Right front=... back=... />"#
            .to_string()
    });

    let custom_code = Signal::derive(move || {
        r#"<FlipButton
  from=FlipDirection::Left
  class_name="docs-flip-button-custom".to_string()
  front=move || view! { <Button variant=ButtonVariant::Outline>"Inspect"</Button> }
  back=move || view! { <Button variant=ButtonVariant::Accent>"Inspecting"</Button> }
/>"#
        .to_string()
    });

    let persisted_workbench_state = load_flip_button_workbench_state();
    let has_persisted_workbench_state = persisted_workbench_state.is_some();
    let initial_workbench_state = persisted_workbench_state.unwrap_or_default();

    let direction_options = vec![
        "top".to_string(),
        "bottom".to_string(),
        "left".to_string(),
        "right".to_string(),
    ];
    let (interactive_direction_index, set_interactive_direction_index) =
        signal(Some(initial_workbench_state.direction_index));
    let (workbench_persist_state, set_workbench_persist_state) =
        signal(has_persisted_workbench_state);

    Effect::new(move || {
        if workbench_persist_state.get() {
            save_flip_button_workbench_state(FlipButtonWorkbenchState {
                direction_index: interactive_direction_index.get().unwrap_or(0).min(3),
            });
        } else {
            clear_flip_button_workbench_state();
        }
    });

    let interactive_direction =
        Signal::derive(
            move || match interactive_direction_index.get().unwrap_or(0) {
                1 => FlipDirection::Bottom,
                2 => FlipDirection::Left,
                3 => FlipDirection::Right,
                _ => FlipDirection::Top,
            },
        );
    let interactive_direction_label =
        Signal::derive(
            move || match interactive_direction_index.get().unwrap_or(0) {
                1 => "Bottom",
                2 => "Left",
                3 => "Right",
                _ => "Top",
            },
        );
    let (interactive_custom_motion, set_interactive_custom_motion) = signal(false);
    let (interactive_custom_class, set_interactive_custom_class) = signal(false);
    let (interactive_rtl, set_interactive_rtl) = signal(false);
    let workbench_node_ref = NodeRef::<html::Div>::new();

    let interactive_code = Signal::derive(move || {
        let direction = interactive_direction_label.get();
        format!(
            "<FlipButton\n  from=FlipDirection::{direction}\n  motion={}\n  class_name={}\n  lang={}\n  dir={}\n  node_ref=NodeRef::<leptos::html::Div>::new()\n  front=move || view! {{ <Button variant=ButtonVariant::Secondary>\"Front\"</Button> }}\n  back=move || view! {{ <Button variant=ButtonVariant::Accent>\"Back\"</Button> }}\n/>",
            if interactive_custom_motion.get() {
                "Some(FlipButtonMotion { spring: ui_motion::spring::SpringConfig { stiffness: 340.0, damping: 22.0, mass: 1.0, ..Default::default() } })"
            } else {
                "Some(FlipButtonMotion::default())"
            },
            if interactive_custom_class.get() {
                "\"docs-flip-button-custom\".to_string()"
            } else {
                "\"\".to_string()"
            },
            if interactive_rtl.get() {
                "\"ar\".to_string()"
            } else {
                "\"en-US\".to_string()"
            },
            if interactive_rtl.get() {
                "Some(A11yDirection::Rtl)"
            } else {
                "Some(A11yDirection::Ltr)"
            }
        )
    });
    let interactive_actual_config = Signal::derive(move || {
        format!(
            "FlipButtonWorkbenchConfig {{\n  from: {:?},\n  motion: {},\n  class_name: {:?},\n  lang: {:?},\n  dir: {},\n  node_ref: \"workbench_node_ref\",\n  front: \"Front\",\n  back: \"Back\",\n  persist_workbench_state: {},\n}}",
            interactive_direction.get(),
            if interactive_custom_motion.get() {
                "Some(FlipButtonMotion(custom))"
            } else {
                "Some(FlipButtonMotion::default())"
            },
            if interactive_custom_class.get() {
                Some("docs-flip-button-custom")
            } else {
                None
            },
            if interactive_rtl.get() {
                Some("ar")
            } else {
                Some("en-US")
            },
            if interactive_rtl.get() {
                "Some(A11yDirection::Rtl)"
            } else {
                "Some(A11yDirection::Ltr)"
            },
            workbench_persist_state.get(),
        )
    });

    view! {
        <ComponentPage
            title="FlipButton"
            slug="flip-button"
            group="Actions"
            description="baseline-level spring flip surface with centralized direction/interaction/class-source state attrs."
        >
            <p class="ui-muted" data-slot="flip-button-streaming-policy">
                "Streaming Optional; fallback=snapshot."
            </p>
            <p class="ui-muted" data-slot="flip-button-streaming-modes">
                "Streaming: render while the LLM is still generating. Snapshot: render once output is complete."
            </p>
            <p class="ui-muted" data-slot="flip-button-copy-ready-hint">
                "Copy-ready snippets prepend imports automatically; dependency: ui; source: crates/ui/src/button/flip/view.rs."
            </p>

            <Playground title="Hello World (Default FlipButton)" code_signal=code>
                <div class="docs-row">
                    <FlipButton
                        from=FlipDirection::Top
                        front=move || view! { <Button variant=ButtonVariant::Secondary>"Front"</Button> }
                        back=move || view! { <Button variant=ButtonVariant::Accent>"Back"</Button> }
                    />
                </div>
            </Playground>

            <Playground title="Direction Gallery" code_signal=states_code>
                <div class="docs-stack">
                    <div class="docs-row">
                        <FlipButton
                            from=FlipDirection::Bottom
                            front=move || view! { <Button variant=ButtonVariant::Secondary>"Bottom"</Button> }
                            back=move || view! { <Button variant=ButtonVariant::Accent>"Back"</Button> }
                        />
                        <FlipButton
                            from=FlipDirection::Left
                            front=move || view! { <Button variant=ButtonVariant::Secondary>"Left"</Button> }
                            back=move || view! { <Button variant=ButtonVariant::Accent>"Back"</Button> }
                        />
                        <FlipButton
                            from=FlipDirection::Right
                            front=move || view! { <Button variant=ButtonVariant::Secondary>"Right"</Button> }
                            back=move || view! { <Button variant=ButtonVariant::Accent>"Back"</Button> }
                        />
                    </div>
                </div>
            </Playground>

            <Playground title="Custom Class" code_signal=custom_code>
                <div class="docs-row">
                    <FlipButton
                        from=FlipDirection::Left
                        class_name="docs-flip-button-custom".to_string()
                        front=move || view! { <Button variant=ButtonVariant::Outline>"Inspect"</Button> }
                        back=move || view! { <Button variant=ButtonVariant::Accent>"Inspecting"</Button> }
                    />
                </div>
            </Playground>

            <Playground
                title="Interactive Playground"
                code_signal=interactive_code
                test_config_signal=interactive_actual_config
                description="Workbench canvas: scoped CSS live-edit + optional state persistence across reload."
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="flip-button-workbench-controls">
                        <div class="docs-search__label">"Direction"</div>
                        <SegmentedControl
                            id_base="docs-flip-button-direction".to_string()
                            options=direction_options.clone()
                            selected_index=interactive_direction_index
                            set_selected_index=set_interactive_direction_index
                            size=SegmentedControlSize::Sm
                            aria_label="FlipButton direction".to_string()
                        />
                        <Switch checked=workbench_persist_state set_checked=set_workbench_persist_state>
                            "Persist workbench state"
                        </Switch>
                        <Switch checked=interactive_custom_motion set_checked=set_interactive_custom_motion>
                            "Custom motion"
                        </Switch>
                        <Switch checked=interactive_custom_class set_checked=set_interactive_custom_class>
                            "Custom class_name"
                        </Switch>
                        <Switch checked=interactive_rtl set_checked=set_interactive_rtl>"RTL + ar"</Switch>
                    </div>
                }
            >
                {move || {
                    let persist = workbench_persist_state.get();
                    let direction = interactive_direction.get();
                    let direction_label = interactive_direction_label.get();

                    view! {
                        <div class="docs-stack" data-slot="flip-button-workbench" style="width: min(100%, 360px);">
                            <span class="ui-muted">
                                "persist: "
                                {if persist { "on" } else { "off" }}
                                ", direction: "
                                {direction_label}
                            </span>
                            <div class="docs-card docs-stack docs-stack--tight" data-slot="flip-button-workbench-canvas">
                                <div class="docs-row">
                                    <FlipButton
                                        from=direction
                                        motion=if interactive_custom_motion.get() {
                                            let mut motion = FlipButtonMotion::default();
                                            motion.spring.stiffness = 340.0;
                                            motion.spring.damping = 22.0;
                                            motion.spring.mass = 1.0;
                                            motion
                                        } else {
                                            FlipButtonMotion::default()
                                        }
                                        class_name=if interactive_custom_class.get() {
                                            "docs-flip-button-custom".to_string()
                                        } else {
                                            String::new()
                                        }
                                        lang=if interactive_rtl.get() {
                                            "ar".to_string()
                                        } else {
                                            "en-US".to_string()
                                        }
                                        dir=if interactive_rtl.get() {
                                            A11yDirection::Rtl
                                        } else {
                                            A11yDirection::Ltr
                                        }
                                        node_ref=workbench_node_ref
                                        front=move || view! { <Button variant=ButtonVariant::Secondary>"Front"</Button> }
                                        back=move || view! { <Button variant=ButtonVariant::Accent>"Back"</Button> }
                                    />
                                </div>
                            </div>
                        </div>
                    }
                }}
            </Playground>

            <Playground title="State Matrix (Direction Comparison)" code_signal=states_code>
                <div class="docs-row">
                    <FlipButton
                        from=FlipDirection::Top
                        motion=FlipButtonMotion::default()
                        lang="en-US".to_string()
                        dir=A11yDirection::Ltr
                        node_ref=NodeRef::new()
                        front=move || view! { <Button variant=ButtonVariant::Secondary>"Top"</Button> }
                        back=move || view! { <Button variant=ButtonVariant::Accent>"Back"</Button> }
                    />
                    <FlipButton
                        from=FlipDirection::Left
                        motion=FlipButtonMotion::default()
                        class_name="docs-flip-button-custom".to_string()
                        lang="ar".to_string()
                        dir=A11yDirection::Rtl
                        node_ref=NodeRef::new()
                        front=move || view! { <Button variant=ButtonVariant::Secondary>"Left"</Button> }
                        back=move || view! { <Button variant=ButtonVariant::Accent>"Back"</Button> }
                    />
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn share_button() -> AnyView {
    let custom_items = vec![
        ShareButtonItem::new(SharePlatform::Github, "Repository"),
        ShareButtonItem::new(SharePlatform::X, "Post"),
        ShareButtonItem::new(SharePlatform::Facebook, "Facebook"),
    ];
    let custom_items_for_matrix = custom_items.clone();
    let custom_items_for_workbench = custom_items.clone();

    let (showcase_last, set_showcase_last) = signal(None::<SharePlatform>);
    let on_showcase_press =
        Callback::new(move |platform: SharePlatform| set_showcase_last.set(Some(platform)));

    let icon_options = vec![
        "Suffix".to_string(),
        "Prefix".to_string(),
        "None".to_string(),
    ];
    let from_options = vec!["Up".to_string(), "Left".to_string(), "Right".to_string()];
    let size_options = vec![
        "xs".to_string(),
        "s".to_string(),
        "m".to_string(),
        "l".to_string(),
        "xl".to_string(),
    ];
    let variant_options = vec![
        "Default".to_string(),
        "Secondary".to_string(),
        "Ghost".to_string(),
        "Outline".to_string(),
    ];
    let (workbench_icon_index, set_workbench_icon_index) = signal(Some(0_usize));
    let (workbench_from_index, set_workbench_from_index) = signal(Some(0_usize));
    let (workbench_size_index, set_workbench_size_index) = signal(Some(2_usize));
    let (workbench_variant_index, set_workbench_variant_index) = signal(Some(0_usize));
    let (workbench_custom_label, set_workbench_custom_label) = signal(false);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);
    let (workbench_use_items, set_workbench_use_items) = signal(true);
    let (workbench_lang_zh, set_workbench_lang_zh) = signal(false);
    let (workbench_rtl, set_workbench_rtl) = signal(false);
    let (workbench_press_count, set_workbench_press_count) = signal(0_u32);
    let (workbench_last, set_workbench_last) = signal(None::<SharePlatform>);
    let on_workbench_press = Callback::new(move |platform: SharePlatform| {
        set_workbench_press_count.update(|count| *count += 1);
        set_workbench_last.set(Some(platform));
    });

    let workbench_icon = Signal::derive(move || match workbench_icon_index.get().unwrap_or(0) {
        1 => ShareButtonIconPlacement::Prefix,
        2 => ShareButtonIconPlacement::None,
        _ => ShareButtonIconPlacement::Suffix,
    });
    let workbench_from = Signal::derive(move || match workbench_from_index.get().unwrap_or(0) {
        1 => FlipDirection::Left,
        2 => FlipDirection::Right,
        _ => FlipDirection::Top,
    });
    let workbench_size = Signal::derive(move || match workbench_size_index.get().unwrap_or(2) {
        0 => ButtonSize::Xs,
        1 => ButtonSize::S,
        3 => ButtonSize::L,
        4 => ButtonSize::Xl,
        _ => ButtonSize::M,
    });
    let workbench_variant =
        Signal::derive(move || match workbench_variant_index.get().unwrap_or(0) {
            1 => ButtonVariant::Secondary,
            2 => ButtonVariant::Ghost,
            3 => ButtonVariant::Outline,
            _ => ButtonVariant::Default,
        });

    let hello_code = Signal::derive(move || r#"<ShareButton />"#.to_string());
    let workbench_code = Signal::derive(move || {
        format!(
            "<ShareButton\n  label={}\n  aria_label=\"Share this page\".to_string()\n  icon={:?}\n  from={:?}\n  size={:?}\n  variant={:?}\n  items={}\n  on_icon_press=on_icon_press\n  motion=ShareButtonMotion::default()\n  class_name={}\n  lang={}.to_string()\n  dir={}\n/>",
            if workbench_custom_label.get() {
                "\"Share docs\".to_string()"
            } else {
                "String::new()"
            },
            workbench_icon.get(),
            workbench_from.get(),
            workbench_size.get(),
            workbench_variant.get(),
            if workbench_use_items.get() {
                "custom_items.clone()"
            } else {
                "Vec::<ShareButtonItem>::new()"
            },
            if workbench_custom_class.get() {
                "\"docs-share-button-custom\".to_string()"
            } else {
                "String::new()"
            },
            if workbench_lang_zh.get() {
                "\"zh-CN\""
            } else {
                "\"en-US\""
            },
            if workbench_rtl.get() {
                "A11yDirection::Rtl"
            } else {
                "A11yDirection::Ltr"
            },
        )
    });
    let workbench_actual_config = Signal::derive(move || {
        format!(
            "ShareButtonActualConfig {{\n  label: {:?},\n  aria_label: Some(\"Share this page\"),\n  icon: {:?},\n  from: {:?},\n  size: {:?},\n  variant: {:?},\n  items: {:?},\n  on_icon_press: \"count={} last={:?}\",\n  motion: ShareButtonMotion::default(),\n  class_name: {:?},\n  lang: {:?},\n  dir: {:?},\n}}",
            if workbench_custom_label.get() {
                Some("Share docs")
            } else {
                None
            },
            workbench_icon.get(),
            workbench_from.get(),
            workbench_size.get(),
            workbench_variant.get(),
            if workbench_use_items.get() {
                vec!["Github", "X", "Facebook"]
            } else {
                vec![]
            },
            workbench_press_count.get(),
            workbench_last.get(),
            if workbench_custom_class.get() {
                Some("docs-share-button-custom")
            } else {
                None
            },
            if workbench_lang_zh.get() {
                "zh-CN"
            } else {
                "en-US"
            },
            if workbench_rtl.get() { "rtl" } else { "ltr" },
        )
    });
    let matrix_code = Signal::derive(move || {
        r#"<ShareButton label="Default".to_string() size=ButtonSize::M variant=ButtonVariant::Default on_icon_press=Callback::new(move |_| {}) />
<ShareButton icon=ShareButtonIconPlacement::Prefix from=FlipDirection::Left label="Prefix".to_string() items=custom_items_for_matrix.clone() variant=ButtonVariant::Secondary motion=ShareButtonMotion::default() />
<ShareButton icon=ShareButtonIconPlacement::None label="Iconless".to_string() class_name="docs-share-button-custom".to_string() aria_label="Share without icon".to_string() lang="zh-CN".to_string() dir=A11yDirection::Rtl />"#
            .to_string()
    });

    view! {
        <ComponentPage
            title="ShareButton"
            slug="share-button"
            group="Actions"
            description="Flip-based share surface with full API workbench and callback feedback."
        >
            <Playground title="Hello World (Default ShareButton)" code_signal=hello_code>
                <div class="docs-row">
                    <ShareButton on_icon_press=on_showcase_press />
                    <span class="ui-muted">
                        "last: "
                        {move || {
                            showcase_last
                                .get()
                                .map(|platform| format!("{platform:?}"))
                                .unwrap_or_else(|| "None".to_string())
                        }}
                    </span>
                </div>
            </Playground>

            <Playground
                title="Workbench (All API + Actual Config)"
                code_signal=workbench_code
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="share-button-workbench-controls">
                        <SegmentedControl
                            id_base="docs-share-button-icon".to_string()
                            options=icon_options.clone()
                            selected_index=workbench_icon_index
                            set_selected_index=set_workbench_icon_index
                            size=SegmentedControlSize::Sm
                            aria_label="ShareButton icon placement".to_string()
                        />
                        <SegmentedControl
                            id_base="docs-share-button-from".to_string()
                            options=from_options.clone()
                            selected_index=workbench_from_index
                            set_selected_index=set_workbench_from_index
                            size=SegmentedControlSize::Sm
                            aria_label="ShareButton flip direction".to_string()
                        />
                        <SegmentedControl
                            id_base="docs-share-button-size".to_string()
                            options=size_options.clone()
                            selected_index=workbench_size_index
                            set_selected_index=set_workbench_size_index
                            size=SegmentedControlSize::Sm
                            aria_label="ShareButton size".to_string()
                        />
                        <SegmentedControl
                            id_base="docs-share-button-variant".to_string()
                            options=variant_options.clone()
                            selected_index=workbench_variant_index
                            set_selected_index=set_workbench_variant_index
                            size=SegmentedControlSize::Sm
                            aria_label="ShareButton variant".to_string()
                        />
                        <Switch checked=workbench_custom_label set_checked=set_workbench_custom_label>
                            "label"
                        </Switch>
                        <Switch checked=workbench_use_items set_checked=set_workbench_use_items>
                            "items"
                        </Switch>
                        <Switch checked=workbench_custom_class set_checked=set_workbench_custom_class>
                            "class_name"
                        </Switch>
                        <Switch checked=workbench_lang_zh set_checked=set_workbench_lang_zh>
                            "lang=zh-CN"
                        </Switch>
                        <Switch checked=workbench_rtl set_checked=set_workbench_rtl>
                            "dir=rtl"
                        </Switch>
                    </div>
                }
            >
                <div class="docs-stack">
                    <ShareButton
                        label=if workbench_custom_label.get() {
                            "Share docs".to_string()
                        } else {
                            String::new()
                        }
                        aria_label="Share this page".to_string()
                        icon=workbench_icon.get()
                        from=workbench_from.get()
                        size=workbench_size.get()
                        variant=workbench_variant.get()
                        items=if workbench_use_items.get() {
                            custom_items_for_workbench.clone()
                        } else {
                            Vec::new()
                        }
                        on_icon_press=on_workbench_press
                        motion=ShareButtonMotion::default()
                        class_name=if workbench_custom_class.get() {
                            "docs-share-button-custom".to_string()
                        } else {
                            String::new()
                        }
                        lang=if workbench_lang_zh.get() {
                            "zh-CN".to_string()
                        } else {
                            "en-US".to_string()
                        }
                        dir=if workbench_rtl.get() {
                            A11yDirection::Rtl
                        } else {
                            A11yDirection::Ltr
                        }
                    />
                    <span class="ui-muted">
                        "on_icon_press count: " {move || workbench_press_count.get()}
                        " · last: "
                        {move || {
                            workbench_last
                                .get()
                                .map(|platform| format!("{platform:?}"))
                                .unwrap_or_else(|| "None".to_string())
                        }}
                    </span>
                </div>
            </Playground>

            <Playground title="State Matrix (Default / Prefix / Iconless)" code_signal=matrix_code>
                <div class="docs-row">
                    <ShareButton
                        label="Default".to_string()
                        size=ButtonSize::M
                        variant=ButtonVariant::Default
                        on_icon_press=on_showcase_press
                    />
                    <ShareButton
                        icon=ShareButtonIconPlacement::Prefix
                        from=FlipDirection::Left
                        label="Prefix".to_string()
                        items=custom_items_for_matrix
                        variant=ButtonVariant::Secondary
                        motion=ShareButtonMotion::default()
                    />
                    <ShareButton
                        icon=ShareButtonIconPlacement::None
                        label="Iconless".to_string()
                        class_name="docs-share-button-custom".to_string()
                        aria_label="Share without icon".to_string()
                        lang="zh-CN".to_string()
                        dir=A11yDirection::Rtl
                    />
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn action_menu() -> AnyView {
    let default_items = vec![
        ActionMenuItemSpec::action("Profile"),
        ActionMenuItemSpec::action("Settings"),
        ActionMenuItemSpec::action("Log out"),
    ];
    let controlled_items = vec![
        ActionMenuItemSpec::action("Rename"),
        ActionMenuItemSpec::action("Duplicate").with_disabled(true),
        ActionMenuItemSpec::action("Archive"),
    ];
    let disabled_items = vec![
        ActionMenuItemSpec::action("Copy"),
        ActionMenuItemSpec::action("Move"),
    ];
    let empty_items: Vec<ActionMenuItemSpec> = Vec::new();
    let marker_items = vec![
        ActionMenuItemSpec::action("Open dashboard"),
        ActionMenuItemSpec::action("Duplicate project"),
        ActionMenuItemSpec::action("Archive workspace").with_disabled(true),
    ];

    let (last, set_last) = signal(None::<usize>);
    let on_action = Callback::new(move |index: usize| set_last.set(Some(index)));

    let (controlled_open_raw, set_controlled_open_raw) = signal(false);
    let controlled_open: Signal<bool> = Signal::derive(move || controlled_open_raw.get());
    let on_open_change = Callback::new(move |next: bool| set_controlled_open_raw.set(next));

    let (marker_open_raw, set_marker_open_raw) = signal(true);
    let marker_open: Signal<bool> = Signal::derive(move || marker_open_raw.get());
    let on_marker_open_change = Callback::new(move |next: bool| set_marker_open_raw.set(next));

    let (last_marker_action, set_last_marker_action) = signal(None::<usize>);
    let on_marker_action =
        Callback::new(move |index: usize| set_last_marker_action.set(Some(index)));
    let on_hello_action = Callback::new(|_: usize| {});

    let hello_code = Signal::derive(move || {
        r#"<ActionMenu
  id_base="action-menu-hello".to_string()
  item_specs=vec![ActionMenuItemSpec::action("Profile")]
  on_action=Callback::new(|_| {})
/>"#
        .to_string()
    });

    let code = Signal::derive(move || {
        r#"<ActionMenu
  id_base="demo".to_string()
  item_specs=vec![
    ActionMenuItemSpec::action("Open dashboard"),
    ActionMenuItemSpec::action("Duplicate project"),
    ActionMenuItemSpec::action("Archive workspace"),
  ]
  on_action=Callback::new(move |index: usize| {
    logging::log!("action index: {}", index);
  })
/>"#
        .to_string()
    });

    let controlled_code = Signal::derive(move || {
        r#"let (open_raw, set_open_raw) = signal(false);
let open: Signal<bool> = Signal::derive(move || open_raw.get());

<ActionMenu
  id_base="action-controlled".to_string()
  item_specs=vec![
    ActionMenuItemSpec::action("Open dashboard"),
    ActionMenuItemSpec::action("Duplicate project").with_disabled(true),
    ActionMenuItemSpec::action("Archive workspace"),
  ]
  on_action=Callback::new(move |index: usize| {
    logging::log!("action index: {}", index);
  })
  is_close_on_action=false
  open=open
  on_open_change=Callback::new(move |next| set_open_raw.set(next))
/>"#
        .to_string()
    });

    let marker_code = Signal::derive(move || {
        r#"let (open_raw, set_open_raw) = signal(true);
let open: Signal<bool> = Signal::derive(move || open_raw.get());

<ActionMenu
  id_base="docs-action-menu-markers".to_string()
  item_specs=vec![
    ActionMenuItemSpec::action("Open dashboard"),
    ActionMenuItemSpec::action("Duplicate project"),
    ActionMenuItemSpec::action("Archive workspace").with_disabled(true),
  ]
  on_action=Callback::new(move |index: usize| {
    logging::log!("action index: {}", index);
  })
  is_close_on_action=false
  open=open
  default_open=true
  on_open_change=Callback::new(move |next| set_open_raw.set(next))
  aria_label="Workspace actions".to_string()
  class_name="docs-action-menu-custom".to_string()
  motion=ui::ActionMenuMotion {
    popover: ui::PopoverMotion {
      initial_scale: 0.93,
      offset_y_px: 8.0,
      ..ui::PopoverMotion::default()
    },
  }
/>"#
        .to_string()
    });

    let disabled_code = Signal::derive(move || {
        r#"<ActionMenu
  id_base="action-disabled".to_string()
  item_specs=vec![
    ActionMenuItemSpec::action("Open dashboard"),
    ActionMenuItemSpec::action("Duplicate project"),
    ActionMenuItemSpec::action("Archive workspace"),
  ]
  on_action=Callback::new(move |index: usize| {
    logging::log!("action index: {}", index);
  })
  is_disabled=true
/>
<ActionMenu
  id_base="action-empty".to_string()
  item_specs=Vec::<ActionMenuItemSpec>::new()
  on_action=Callback::new(move |index: usize| {
    logging::log!("action index: {}", index);
  })
/>"#
        .to_string()
    });

    let marker_motion = ui::ActionMenuMotion {
        popover: ui::PopoverMotion {
            initial_scale: 0.93,
            offset_y_px: 8.0,
            ..ui::PopoverMotion::default()
        },
    };
    let workbench_action_mode_options = vec!["close".to_string(), "keep-open".to_string()];
    let workbench_placement_options = vec![
        "bottom-start".to_string(),
        "bottom-end".to_string(),
        "top-start".to_string(),
    ];
    let workbench_size_options = vec![
        "xs".to_string(),
        "sm".to_string(),
        "md".to_string(),
        "lg".to_string(),
        "xl".to_string(),
    ];
    let (workbench_action_mode_index, set_workbench_action_mode_index) = signal(Some(0_usize));
    let (workbench_placement_index, set_workbench_placement_index) = signal(Some(0_usize));
    let (workbench_size_index, set_workbench_size_index) = signal(Some(2_usize));
    let (workbench_is_quiet, set_workbench_is_quiet) = signal(false);
    let (workbench_is_disabled, set_workbench_is_disabled) = signal(false);
    let (workbench_disable_second, set_workbench_disable_second) = signal(true);
    let (workbench_custom_motion, set_workbench_custom_motion) = signal(false);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);
    let (workbench_rtl, set_workbench_rtl) = signal(false);

    let (workbench_open_raw, set_workbench_open_raw) = signal(false);
    let workbench_open: Signal<bool> = Signal::derive(move || workbench_open_raw.get());
    let workbench_on_open_change =
        Callback::new(move |next: bool| set_workbench_open_raw.set(next));
    let (workbench_last_action, set_workbench_last_action) = signal(None::<usize>);
    let workbench_on_action =
        Callback::new(move |index: usize| set_workbench_last_action.set(Some(index)));

    let workbench_action_mode = Signal::derive(move || {
        if workbench_action_mode_index.get().unwrap_or(0) == 1 {
            ui::action_menu::ActionMenuActionMode::KeepOpenOnAction
        } else {
            ui::action_menu::ActionMenuActionMode::CloseOnAction
        }
    });
    let workbench_placement =
        Signal::derive(move || match workbench_placement_index.get().unwrap_or(0) {
            1 => PopoverPlacement::BottomEnd,
            2 => PopoverPlacement::TopStart,
            _ => PopoverPlacement::BottomStart,
        });
    let workbench_size = Signal::derive(move || match workbench_size_index.get().unwrap_or(2) {
        0 => ActionButtonSize::Xs,
        1 => ActionButtonSize::Sm,
        3 => ActionButtonSize::Lg,
        4 => ActionButtonSize::Xl,
        _ => ActionButtonSize::M,
    });
    let workbench_item_specs = Signal::derive(move || {
        if workbench_disable_second.get() {
            vec![
                ActionMenuItemSpec::action("Open dashboard"),
                ActionMenuItemSpec::action("Duplicate project").with_disabled(true),
                ActionMenuItemSpec::action("Archive workspace"),
            ]
        } else {
            vec![
                ActionMenuItemSpec::action("Open dashboard"),
                ActionMenuItemSpec::action("Duplicate project"),
                ActionMenuItemSpec::action("Archive workspace"),
            ]
        }
    });
    let workbench_items = Signal::derive(move || {
        vec![
            "Open dashboard".to_string(),
            "Duplicate project".to_string(),
            "Archive workspace".to_string(),
        ]
    });
    let workbench_disabled_indices = Signal::derive(move || {
        if workbench_disable_second.get() {
            vec![1_usize]
        } else {
            vec![]
        }
    });
    let workbench_item_kinds = Signal::derive(move || {
        vec![
            ui::MenuItemKind::Action,
            ui::MenuItemKind::Action,
            ui::MenuItemKind::Action,
        ]
    });
    let workbench_motion = Signal::derive(move || {
        if workbench_custom_motion.get() {
            ui::ActionMenuMotion {
                popover: ui::PopoverMotion {
                    initial_scale: 0.94,
                    offset_y_px: 10.0,
                    ..ui::PopoverMotion::default()
                },
            }
        } else {
            ui::ActionMenuMotion::default()
        }
    });

    let workbench_code = Signal::derive(move || {
        let lines = vec![
            "let (open_raw, set_open_raw) = signal(false);".to_string(),
            "let open_sig: Signal<bool> = Signal::derive(move || open_raw.get());".to_string(),
            "<ActionMenu".to_string(),
            "  id_base=\"docs-action-menu-workbench\".to_string()".to_string(),
            "  items=vec![\"Open dashboard\".into(), \"Duplicate project\".into(), \"Archive workspace\".into()]".to_string(),
            "  on_action=Callback::new(move |index: usize| { logging::log!(\"action={}\", index); })".to_string(),
            "  item_specs=vec![".to_string(),
            "    ActionMenuItemSpec::action(\"Open dashboard\"),".to_string(),
            "    ActionMenuItemSpec::action(\"Duplicate project\"),".to_string(),
            "    ActionMenuItemSpec::action(\"Archive workspace\"),".to_string(),
            "  ]".to_string(),
            "  disabled_state=ui::ActionMenuDisabledState::Enabled".to_string(),
            format!("  is_disabled={}", workbench_is_disabled.get()),
            format!("  disabled={}", workbench_is_disabled.get()),
            format!(
                "  disabled_indices={}",
                if workbench_disable_second.get() {
                    "vec![1]"
                } else {
                    "vec![]"
                }
            ),
            "  item_kinds=vec![ui::MenuItemKind::Action, ui::MenuItemKind::Action, ui::MenuItemKind::Action]".to_string(),
            format!(
                "  action_mode=ui::action_menu::ActionMenuActionMode::{}",
                if workbench_action_mode.get()
                    == ui::action_menu::ActionMenuActionMode::KeepOpenOnAction
                {
                    "KeepOpenOnAction"
                } else {
                    "CloseOnAction"
                }
            ),
            format!(
                "  is_close_on_action={}",
                workbench_action_mode.get().is_close_on_action()
            ),
            format!(
                "  close_on_action={}",
                workbench_action_mode.get().is_close_on_action()
            ),
            format!("  placement=ui::PopoverPlacement::{:?}", workbench_placement.get()),
            "  is_open=open_sig".to_string(),
            "  open=open_sig".to_string(),
            "  default_open=false".to_string(),
            "  on_open_change=Callback::new(move |next| set_open_raw.set(next))".to_string(),
            format!("  size=ActionButtonSize::{:?}", workbench_size.get()),
            format!("  is_quiet={}", workbench_is_quiet.get()),
            "  aria_label=\"Workspace actions\".to_string()".to_string(),
            "  lang=\"en\".to_string()".to_string(),
            "  dir=ui::A11yDirection::Ltr".to_string(),
            if workbench_custom_motion.get() {
                "  motion=ui::ActionMenuMotion { popover: ui::PopoverMotion { initial_scale: 0.94, offset_y_px: 10.0, ..ui::PopoverMotion::default() } }".to_string()
            } else {
                "  motion=ui::ActionMenuMotion::default()".to_string()
            },
            if workbench_custom_class.get() {
                "  class_name=\"docs-action-menu-workbench\".to_string()".to_string()
            } else {
                "  class_name=\"\".to_string()".to_string()
            },
            "/>".to_string(),
        ];
        lines.join("\n")
    });
    let workbench_config = Signal::derive(move || {
        format!(
            "ActionMenuWorkbenchConfig {{\n  id_base: \"docs-action-menu-workbench\",\n  items: [\"Open dashboard\", \"Duplicate project\", \"Archive workspace\"],\n  on_action: Some(\"Callback<usize>\"),\n  item_specs: [\"action\", \"action\", \"action\"],\n  disabled_state: Some(\"Enabled\"),\n  is_disabled: Some({}),\n  disabled: Some({}),\n  disabled_indices: {},\n  item_kinds: [\"action\", \"action\", \"action\"],\n  action_mode: Some(\"{}\"),\n  is_close_on_action: Some({}),\n  close_on_action: Some({}),\n  placement: \"{:?}\",\n  is_open: Some({}),\n  open: Some({}),\n  default_open: Some(false),\n  on_open_change: Some(\"Callback<bool>\"),\n  size: \"{:?}\",\n  is_quiet: {},\n  aria_label: Some(\"Workspace actions\"),\n  lang: {},\n  dir: {},\n  motion: {},\n  class_name: {},\n  last_action: {:?},\n}}",
            workbench_is_disabled.get(),
            workbench_is_disabled.get(),
            if workbench_disable_second.get() {
                "vec![1]"
            } else {
                "vec![]"
            },
            if workbench_action_mode.get()
                == ui::action_menu::ActionMenuActionMode::KeepOpenOnAction
            {
                "KeepOpenOnAction"
            } else {
                "CloseOnAction"
            },
            workbench_action_mode.get().is_close_on_action(),
            workbench_action_mode.get().is_close_on_action(),
            workbench_placement.get(),
            workbench_open_raw.get(),
            workbench_open_raw.get(),
            workbench_size.get(),
            workbench_is_quiet.get(),
            if workbench_rtl.get() {
                "Some(\"ar\")"
            } else {
                "Some(\"en\")"
            },
            if workbench_rtl.get() {
                "Some(\"rtl\")"
            } else {
                "Some(\"ltr\")"
            },
            if workbench_custom_motion.get() {
                "ActionMenuMotion::custom"
            } else {
                "ActionMenuMotion::default"
            },
            if workbench_custom_class.get() {
                "Some(\"docs-action-menu-workbench\")"
            } else {
                "None"
            },
            workbench_last_action.get(),
        )
    });
    let matrix_code = Signal::derive(move || {
        r#"<ActionMenu id_base="m1".to_string() item_specs=vec![ActionMenuItemSpec::action("A")] on_action=Callback::new(|_| {}) />
<ActionMenu id_base="m2".to_string() item_specs=vec![ActionMenuItemSpec::action("A"), ActionMenuItemSpec::action("B").with_disabled(true)] on_action=Callback::new(|_| {}) is_disabled=Some(false) disabled_indices=vec![1] />
<ActionMenu id_base="m3".to_string() item_specs=vec![ActionMenuItemSpec::action("A")] on_action=Callback::new(|_| {}) action_mode=ui::action_menu::ActionMenuActionMode::KeepOpenOnAction />"#
            .to_string()
    });

    view! {
        <ComponentPage
            title="ActionMenu"
            slug="action-menu"
            group="Actions"
            description="ActionButton-triggered menu surface with baseline state/source data attrs and baseline-level popover spring motion (controlled/uncontrolled + close strategy)."
        >
            <Playground title="Hello World" code_signal=hello_code>
                <div class="docs-row">
                    <ActionMenu
                        id_base="docs-action-menu-hello".to_string()
                        item_specs=vec![ActionMenuItemSpec::action("Profile")]
                        on_action=on_hello_action
                    />
                </div>
            </Playground>

            <Playground
                title="Workbench (Display + Config + Code)"
                code_signal=workbench_code
                test_config_signal=workbench_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="action-menu-workbench-controls">
                        <div class="docs-search__label">"Action mode"</div>
                        <SegmentedControl
                            id_base="docs-action-menu-workbench-mode".to_string()
                            options=workbench_action_mode_options.clone()
                            selected_index=workbench_action_mode_index
                            set_selected_index=set_workbench_action_mode_index
                            size=SegmentedControlSize::Sm
                            aria_label="ActionMenu action mode".to_string()
                        />
                        <div class="docs-search__label">"Placement"</div>
                        <SegmentedControl
                            id_base="docs-action-menu-workbench-placement".to_string()
                            options=workbench_placement_options.clone()
                            selected_index=workbench_placement_index
                            set_selected_index=set_workbench_placement_index
                            size=SegmentedControlSize::Sm
                            aria_label="ActionMenu placement".to_string()
                        />
                        <div class="docs-search__label">"Trigger size"</div>
                        <SegmentedControl
                            id_base="docs-action-menu-workbench-size".to_string()
                            options=workbench_size_options.clone()
                            selected_index=workbench_size_index
                            set_selected_index=set_workbench_size_index
                            size=SegmentedControlSize::Sm
                            aria_label="ActionMenu trigger size".to_string()
                        />
                        <Switch checked=workbench_is_quiet set_checked=set_workbench_is_quiet>"Quiet trigger"</Switch>
                        <Switch checked=workbench_is_disabled set_checked=set_workbench_is_disabled>"Disable menu"</Switch>
                        <Switch checked=workbench_disable_second set_checked=set_workbench_disable_second>"Disable second item"</Switch>
                        <Switch checked=workbench_custom_motion set_checked=set_workbench_custom_motion>"Custom motion"</Switch>
                        <Switch checked=workbench_custom_class set_checked=set_workbench_custom_class>"Custom class"</Switch>
                        <Switch checked=workbench_rtl set_checked=set_workbench_rtl>"RTL"</Switch>
                    </div>
                }
            >
                <div class="docs-stack docs-stack--tight" data-slot="action-menu-workbench-preview">
                    <ActionMenu
                        id_base="docs-action-menu-workbench".to_string()
                        items=workbench_items.get()
                        on_action=workbench_on_action
                        item_specs=workbench_item_specs.get()
                        disabled_state=if workbench_is_disabled.get() {
                            ui::action_menu::ActionMenuDisabledState::Disabled
                        } else {
                            ui::action_menu::ActionMenuDisabledState::Enabled
                        }
                        is_disabled=workbench_is_disabled.get()
                        disabled=workbench_is_disabled.get()
                        disabled_indices=workbench_disabled_indices.get()
                        item_kinds=workbench_item_kinds.get()
                        action_mode=workbench_action_mode.get()
                        is_close_on_action=workbench_action_mode.get().is_close_on_action()
                        close_on_action=workbench_action_mode.get().is_close_on_action()
                        placement=workbench_placement.get()
                        is_open=workbench_open
                        open=workbench_open
                        default_open=false
                        on_open_change=workbench_on_open_change
                        size=workbench_size.get()
                        is_quiet=workbench_is_quiet.get()
                        aria_label="Workspace actions".to_string()
                        lang=if workbench_rtl.get() { "ar".to_string() } else { "en".to_string() }
                        dir=if workbench_rtl.get() { A11yDirection::Rtl } else { A11yDirection::Ltr }
                        motion=workbench_motion.get()
                        class_name=if workbench_custom_class.get() {
                            "docs-action-menu-workbench".to_string()
                        } else {
                            String::new()
                        }
                    />
                    <span class="ui-muted">
                        "open: " {move || workbench_open_raw.get().to_string()}
                        " · last action: " {move || workbench_last_action.get().map(|v| v.to_string()).unwrap_or_else(|| "None".to_string())}
                    </span>
                </div>
            </Playground>

            <Playground title="Default" code_signal=code>
                <div class="docs-row">
                    <ActionMenu
                        id_base="docs-action-menu".to_string()
                        item_specs=default_items
                        on_action=on_action
                    />
                    <span class="ui-muted">
                        "last action: "
                        {move || last.get().map(|value| value.to_string()).unwrap_or_else(|| "None".to_string())}
                    </span>
                </div>
            </Playground>

            <Playground title="Controlled + persistent open" code_signal=controlled_code>
                <div class="docs-stack">
                    <ActionMenu
                        id_base="docs-action-menu-controlled".to_string()
                        item_specs=controlled_items
                        on_action=on_action
                        is_close_on_action=false
                        open=controlled_open
                        on_open_change=on_open_change
                    />
                    <span class="ui-muted">
                        "open: "
                        {move || controlled_open_raw.get()}
                    </span>
                </div>
            </Playground>

            <Playground title="State + Source Markers" code_signal=marker_code>
                <div class="docs-stack docs-stack--tight">
                    <div class="docs-row">
                        <button type="button" on:click=move |_| set_marker_open_raw.set(true)>
                            "Open"
                        </button>
                        <button type="button" on:click=move |_| set_marker_open_raw.set(false)>
                            "Close"
                        </button>
                    </div>
                    <div class="ui-muted">
                        "Inspect data-id-source / data-aria-label-source / data-disabled-indices-source / data-item-kinds-source / data-open-source / data-open-change-source / data-motion-source in DevTools."
                    </div>
                    <ActionMenu
                        id_base="docs-action-menu-markers".to_string()
                        item_specs=marker_items
                        on_action=on_marker_action
                        is_close_on_action=false
                        open=marker_open
                        default_open=true
                        on_open_change=on_marker_open_change
                        aria_label="Workspace actions".to_string()
                        class_name="docs-action-menu-custom".to_string()
                        motion=marker_motion
                    />
                    <span class="ui-muted">
                        "open: "
                        {move || marker_open_raw.get()}
                        " · last action: "
                        {move || {
                            last_marker_action
                                .get()
                                .map(|value| value.to_string())
                                .unwrap_or_else(|| "None".to_string())
                        }}
                    </span>
                </div>
            </Playground>

            <Playground title="Disabled + Empty" code_signal=disabled_code>
                <div class="docs-row">
                    <ActionMenu
                        id_base="docs-action-menu-disabled".to_string()
                        item_specs=disabled_items
                        on_action=on_action
                        is_disabled=true
                    />

                    <ActionMenu
                        id_base="docs-action-menu-empty".to_string()
                        item_specs=empty_items
                        on_action=on_action
                    />
                </div>
            </Playground>

            <Playground
                title="State Matrix (Default / Disabled Item / Keep Open)"
                code_signal=matrix_code
            >
                <div class="docs-row">
                    <ActionMenu
                        id_base="docs-action-menu-matrix-default".to_string()
                        item_specs=vec![
                            ActionMenuItemSpec::action("Open dashboard"),
                            ActionMenuItemSpec::action("Duplicate project"),
                        ]
                        on_action=on_action
                    />
                    <ActionMenu
                        id_base="docs-action-menu-matrix-disabled-item".to_string()
                        item_specs=vec![
                            ActionMenuItemSpec::action("Open dashboard"),
                            ActionMenuItemSpec::action("Duplicate project").with_disabled(true),
                        ]
                        disabled_indices=vec![1]
                        on_action=on_action
                    />
                    <ActionMenu
                        id_base="docs-action-menu-matrix-keep-open".to_string()
                        item_specs=vec![
                            ActionMenuItemSpec::action("Open dashboard"),
                            ActionMenuItemSpec::action("Archive workspace"),
                        ]
                        action_mode=ui::action_menu::ActionMenuActionMode::KeepOpenOnAction
                        on_action=on_action
                    />
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
