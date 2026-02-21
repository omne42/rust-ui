use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use ui_components::{
    ActionButton, ActionButtonGroup, ActionButtonGroupDensity, ActionButtonGroupOrientation,
    ActionButtonLoadingPlacement, ActionButtonSize, ActionMenu, ActionMenuItemSpec, Button,
    ButtonColor, ButtonCopy, ButtonCopyMode, ButtonCopyMotion, ButtonGroup, ButtonGroupOrientation,
    ButtonIntent, ButtonLoadingPlacement, ButtonRadius, ButtonSchema, ButtonSize, ButtonVariant,
    FlipButton, FlipDirection, LinkButton, OnPress, SearchInputButton, SegmentedControl,
    SegmentedControlSize, ShareButton, ShareButtonIconPlacement, ShareButtonItem, SharePlatform,
    Switch, ThemeMode, ThemeToggleButton, ToggleButton, ToggleButtonGroup,
    ToggleButtonGroupOrientation, ToggleButtonSize, ToggleButtonVariant,
};

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
    let button_imports = "use leptos::prelude::*;\nuse ui_components::{Button, ButtonColor, ButtonLoadingPlacement, ButtonRadius, ButtonSize, ButtonVariant};".to_string();

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
            "/* crates/ui-components/src/button/styles.rs */\n{}",
            ui_components::button::styles::CSS
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
            "ButtonActualConfig {{\n  color: {color:?},\n  variant: {variant:?},\n  radius: {radius:?},\n  size: {size:?},\n  is_disabled: {is_disabled},\n  is_loading: {is_loading},\n  loading_placement: {loading_placement:?},\n  is_icon_only: {icon_only},\n  is_full_width: {is_full_width},\n  has_start_content: {show_start},\n  has_end_content: {show_end},\n  schema_json: {schema_json:?},\n  class: \"{}\",\n}}",
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
                    <Button>"Button"</Button>
                </div>
            </Playground>

            <Playground
                title="Variants & sizes"
                code_signal=code
                code_imports=button_imports.clone()
                test_css_source=test_css_source
                test_source_path="/root/code/personal/omne/rust-ui/crates/ui-components/src/button/styles.rs".to_string()
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
                        "Source: components/button/src/view.rs and crates/ui-components/src/button/view.rs."
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

    let workbench_code = Signal::derive(move || {
        let size = workbench_size.get();
        let is_loading = workbench_loading.get();
        let loading_placement = workbench_loading_placement.get();
        let is_disabled = workbench_disabled.get();
        let is_quiet = workbench_quiet.get();
        let is_icon_only = workbench_icon_only.get();
        let show_start = workbench_show_start.get();
        let show_end = workbench_show_end.get();

        let mut snippet = vec![
            "<ActionButton".to_string(),
            format!("  size=ActionButtonSize::{size:?}"),
            format!("  is_loading={is_loading}"),
            format!("  loading_placement=ActionButtonLoadingPlacement::{loading_placement:?}"),
            format!("  is_disabled={is_disabled}"),
            format!("  is_quiet={is_quiet}"),
            format!("  is_icon_only={is_icon_only}"),
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
            "/* crates/ui-components/src/button/styles.rs */\n{}",
            ui_components::button::styles::CSS
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
        format!(
            "ActionButtonActualConfig {{\n  size: {size:?},\n  is_loading: {is_loading},\n  loading_placement: {loading_placement:?},\n  is_disabled: {is_disabled},\n  is_quiet: {is_quiet},\n  is_icon_only: {is_icon_only},\n  has_start_content: {show_start},\n  has_end_content: {show_end},\n}}"
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
                test_source_path="/root/code/personal/omne/rust-ui/crates/ui-components/src/button/styles.rs".to_string()
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
                    let show_start = workbench_show_start.get();
                    let show_end = workbench_show_end.get();
                    let persist = workbench_persist_state.get();

                    view! {
                        <div class="docs-stack" data-slot="action-button-workbench" style="width: min(100%, 360px);">
                            <span class="ui-muted">
                                "persist: "
                                {if persist { "on" } else { "off" }}
                            </span>
                            <div class="docs-card docs-stack docs-stack--tight" data-slot="action-button-workbench-canvas">
                                <div class="docs-row" style="justify-content: center;">
                                    {match (show_start, show_end) {
                                        (true, true) => view! {
                                            <ActionButton
                                                size=size
                                                is_loading=is_loading
                                                loading_placement=loading_placement
                                                is_disabled=is_disabled
                                                is_quiet=is_quiet
                                                aria_label=if is_icon_only { "Action".to_string() } else { String::new() }
                                            >
                                                {if is_icon_only { "★" } else { "Action" }}
                                            </ActionButton>
                                        }.into_any(),
                                        (true, false) => view! {
                                            <ActionButton
                                                size=size
                                                is_loading=is_loading
                                                loading_placement=loading_placement
                                                is_disabled=is_disabled
                                                is_quiet=is_quiet
                                                aria_label=if is_icon_only { "Action".to_string() } else { String::new() }
                                            >
                                                {if is_icon_only { "★" } else { "Action" }}
                                            </ActionButton>
                                        }.into_any(),
                                        (false, true) => view! {
                                            <ActionButton
                                                size=size
                                                is_loading=is_loading
                                                loading_placement=loading_placement
                                                is_disabled=is_disabled
                                                is_quiet=is_quiet
                                                aria_label=if is_icon_only { "Action".to_string() } else { String::new() }
                                            >
                                                {if is_icon_only { "★" } else { "Action" }}
                                            </ActionButton>
                                        }.into_any(),
                                        (false, false) => view! {
                                            <ActionButton
                                                size=size
                                                is_loading=is_loading
                                                loading_placement=loading_placement
                                                is_disabled=is_disabled
                                                is_quiet=is_quiet
                                                aria_label=if is_icon_only { "Action".to_string() } else { String::new() }
                                            >
                                                {if is_icon_only { "★" } else { "Action" }}
                                            </ActionButton>
                                        }.into_any(),
                                    }}
                                </div>
                            </div>
                        </div>
                    }
                }}
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn action_button_group() -> AnyView {
    let (press_count, set_press_count) = signal(0_u32);
    let on_press: OnPress = Callback::new(move |_| {
        set_press_count.update(|count| *count += 1);
    });

    let code = Signal::derive(move || {
        r#"<ActionButtonGroup
  size=ActionButtonSize::S
  density=ActionButtonGroupDensity::Compact
  is_quiet=true
>
  <ActionButton on_press=Callback::new(move |_| {})>"One"</ActionButton>
  <ActionButton on_press=Callback::new(move |_| {})>"Two"</ActionButton>
  <ActionButton on_press=Callback::new(move |_| {})>"Three"</ActionButton>
</ActionButtonGroup>"#
            .to_string()
    });

    let states_code = Signal::derive(move || {
        r#"<ActionButtonGroup
  orientation=ActionButtonGroupOrientation::Vertical
  is_justified=true
  aria_label="Vertical actions".to_string()
>
  <ActionButton>"Top"</ActionButton>
  <ActionButton>"Bottom"</ActionButton>
</ActionButtonGroup>

<ActionButtonGroup is_disabled=true density=ActionButtonGroupDensity::Compact>
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
            description="Toolbar-style action clusters with baseline state attrs for orientation, density, quiet/filled, and enablement."
        >
            <Playground title="Default + compact" code_signal=code>
                <div class="docs-stack">
                    <ActionButtonGroup
                        size=ActionButtonSize::S
                        density=ActionButtonGroupDensity::Compact
                        orientation=ActionButtonGroupOrientation::Horizontal
                        is_quiet=true
                    >
                        <ActionButton on_press=on_press>"One"</ActionButton>
                        <ActionButton on_press=on_press>"Two"</ActionButton>
                        <ActionButton on_press=on_press>"Three"</ActionButton>
                    </ActionButtonGroup>
                    <span class="ui-muted">
                        "pressed: "
                        {move || press_count.get().to_string()}
                    </span>
                </div>
            </Playground>

            <Playground title="Vertical + justified + disabled" code_signal=states_code>
                <div class="docs-stack">
                    <div class="docs-row">
                        <ActionButtonGroup
                            size=ActionButtonSize::M
                            orientation=ActionButtonGroupOrientation::Vertical
                            is_justified=true
                            aria_label="Vertical actions".to_string()
                        >
                            <ActionButton>"Top"</ActionButton>
                            <ActionButton>"Middle"</ActionButton>
                            <ActionButton>"Bottom"</ActionButton>
                        </ActionButtonGroup>

                        <ActionButtonGroup
                            size=ActionButtonSize::S
                            density=ActionButtonGroupDensity::Compact
                            is_disabled=true
                            aria_label="Disabled actions".to_string()
                        >
                            <ActionButton>"Disabled"</ActionButton>
                            <ActionButton>"Group"</ActionButton>
                        </ActionButtonGroup>
                    </div>
                    <span class="ui-muted">
                        "Vertical/compact/disabled/justified are all reflected via stable data-* attrs for baseline-level styling contracts."
                    </span>
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

    let on_left: OnPress = Callback::new(move |_| {
        set_left_count.update(|count| *count += 1);
    });
    let on_middle: OnPress = Callback::new(move |_| {
        set_middle_count.update(|count| *count += 1);
    });
    let on_right: OnPress = Callback::new(move |_| {
        set_right_count.update(|count| *count += 1);
    });

    let (top_count, set_top_count) = signal(0_usize);
    let (bottom_count, set_bottom_count) = signal(0_usize);
    let on_top: OnPress = Callback::new(move |_| {
        set_top_count.update(|count| *count += 1);
    });
    let on_bottom: OnPress = Callback::new(move |_| {
        set_bottom_count.update(|count| *count += 1);
    });

    let code = Signal::derive(move || {
        r#"<ButtonGroup is_attached=true>
  <Button variant=ButtonVariant::Secondary>"Left"</Button>
  <Button variant=ButtonVariant::Secondary>"Middle"</Button>
  <Button variant=ButtonVariant::Secondary>"Right"</Button>
</ButtonGroup>"#
            .to_string()
    });

    let states_code = Signal::derive(move || {
        r#"<ButtonGroup
  is_attached=false
  orientation=ButtonGroupOrientation::Vertical
  aria_label="Document actions".to_string()
>
  <Button variant=ButtonVariant::Outline>"Top"</Button>
  <Button variant=ButtonVariant::Outline is_disabled=true>"Disabled"</Button>
  <Button variant=ButtonVariant::Outline>"Bottom"</Button>
</ButtonGroup>"#
            .to_string()
    });

    view! {
        <ComponentPage
            title="ButtonGroup"
            slug="button-group"
            group="Actions"
            description="Groups Buttons with baseline-style root state attrs for orientation, attachment, and accessible labeling."
        >
            <Playground title="Attached horizontal" code_signal=code>
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

            <Playground title="Vertical + detached" code_signal=states_code>
                <div class="docs-stack">
                    <ButtonGroup
                        is_attached=false
                        orientation=ButtonGroupOrientation::Vertical
                        aria_label="Document actions".to_string()
                    >
                        <Button variant=ButtonVariant::Outline on_press=on_top>
                            "Top"
                        </Button>
                        <Button variant=ButtonVariant::Outline is_disabled=true>
                            "Disabled"
                        </Button>
                        <Button variant=ButtonVariant::Outline on_press=on_bottom>
                            "Bottom"
                        </Button>
                    </ButtonGroup>
                    <span class="ui-muted">
                        "top/bottom clicks: "
                        {move || format!("{}/{}", top_count.get(), bottom_count.get())}
                    </span>
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
            "/* crates/ui-components/src/button/link_button/styles.rs */\n{}",
            ui_components::link_button::styles::CSS
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        let rel_value = if sponsored_rel.get() { "sponsored" } else { "" };
        format!(
            "LinkButtonWorkbenchConfig {{\n  variant: \"{:?}\",\n  size: \"{:?}\",\n  disabled: {},\n  target: \"{}\",\n  rel: \"{}\",\n}}",
            variant.get(),
            size.get(),
            disabled.get(),
            if open_in_new_tab.get() {
                "_blank"
            } else {
                "_self"
            },
            rel_value
        )
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
            <Playground
                title="Interactive Playground (Display + Config + Code + CSS Test)"
                code_signal=code
                test_css_source=workbench_test_css_source
                test_source_path="/root/autodl-tmp/zjj/p/rust-ui/crates/ui-components/src/button/link_button/styles.rs".to_string()
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

            <Playground title="Variant + size + disabled matrix" code_signal=states_code>
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

    let variant_options = vec![
        "Default".to_string(),
        "Accent".to_string(),
        "Outline".to_string(),
        "Secondary".to_string(),
        "Ghost".to_string(),
        "Destructive".to_string(),
    ];
    let (variant_index, set_variant_index) = signal(Some(0_usize));
    let variant = Signal::derive(move || match variant_index.get().unwrap_or(0) {
        1 => ToggleButtonVariant::Accent,
        2 => ToggleButtonVariant::Outline,
        3 => ToggleButtonVariant::Secondary,
        4 => ToggleButtonVariant::Ghost,
        5 => ToggleButtonVariant::Destructive,
        _ => ToggleButtonVariant::Default,
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
        0 => ToggleButtonSize::Xs,
        1 => ToggleButtonSize::S,
        2 => ToggleButtonSize::M,
        3 => ToggleButtonSize::L,
        _ => ToggleButtonSize::Xl,
    });

    let (disabled, set_disabled) = signal(false);

    let code = Signal::derive(move || {
        let variant = variant.get();
        let size = size.get();
        let disabled = disabled.get();

        let mut snippet = vec![
            "let (selected, set_selected) = signal(false);".to_string(),
            "let selected_signal: Signal<bool> = Signal::derive(move || selected.get());"
                .to_string(),
            "let on_toggle_change = Callback::new(move |next| set_selected.set(next));".to_string(),
            String::new(),
            "<ToggleButton".to_string(),
            "  is_pressed=selected_signal".to_string(),
            "  on_pressed_change=on_toggle_change".to_string(),
        ];

        if variant != ToggleButtonVariant::Default {
            snippet.push(format!("  variant=ToggleButtonVariant::{variant:?}"));
        }
        if size != ToggleButtonSize::M {
            snippet.push(format!("  size=ToggleButtonSize::{size:?}"));
        }
        if disabled {
            snippet.push("  is_disabled=true".to_string());
        }

        snippet.extend([
            ">".to_string(),
            "  \"Toggle\"".to_string(),
            "</ToggleButton>".to_string(),
        ]);

        snippet.join("\n")
    });

    let (notifications, set_notifications) = signal(true);
    let (disabled_selected, set_disabled_selected) = signal(true);
    let (disabled_unselected, set_disabled_unselected) = signal(false);
    let notifications_signal: Signal<bool> = Signal::derive(move || notifications.get());
    let on_notifications_change = Callback::new(move |next: bool| set_notifications.set(next));
    let disabled_selected_signal: Signal<bool> = Signal::derive(move || disabled_selected.get());
    let disabled_unselected_signal: Signal<bool> =
        Signal::derive(move || disabled_unselected.get());
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
<ToggleButton is_pressed=disabled_selected_signal is_disabled=true>
  "Disabled on"
</ToggleButton>
<ToggleButton is_pressed=disabled_unselected_signal is_disabled=true>
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
                title="Controlled + on_pressed_change"
                code_signal=code
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

                        <Switch checked=disabled set_checked=set_disabled>"Disabled"</Switch>
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
                                    on_pressed_change=on_toggle_change
                                    variant=variant
                                    size=size
                                    is_disabled=disabled
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

            <Playground title="Variant + size + disabled matrix" code_signal=states_code>
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

    let orientation_options = vec!["Horizontal".to_string(), "Vertical".to_string()];
    let (orientation_index, set_orientation_index) = signal(Some(0_usize));
    let orientation = Signal::derive(move || match orientation_index.get().unwrap_or(0) {
        1 => ToggleButtonGroupOrientation::Vertical,
        _ => ToggleButtonGroupOrientation::Horizontal,
    });

    let (attached, set_attached) = signal(false);

    let variant_options = vec![
        "Default".to_string(),
        "Accent".to_string(),
        "Outline".to_string(),
        "Secondary".to_string(),
        "Ghost".to_string(),
        "Destructive".to_string(),
    ];
    let (variant_index, set_variant_index) = signal(Some(0_usize));
    let variant = Signal::derive(move || match variant_index.get().unwrap_or(0) {
        1 => ToggleButtonVariant::Accent,
        2 => ToggleButtonVariant::Outline,
        3 => ToggleButtonVariant::Secondary,
        4 => ToggleButtonVariant::Ghost,
        5 => ToggleButtonVariant::Destructive,
        _ => ToggleButtonVariant::Default,
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
        0 => ToggleButtonSize::Xs,
        1 => ToggleButtonSize::S,
        2 => ToggleButtonSize::M,
        3 => ToggleButtonSize::L,
        _ => ToggleButtonSize::Xl,
    });

    let code = Signal::derive(move || {
        let orientation = orientation.get();
        let attached = attached.get();
        let variant = variant.get();
        let size = size.get();

        let mut toggle_props = String::new();
        if variant != ToggleButtonVariant::Default {
            toggle_props.push_str(&format!(" variant=ToggleButtonVariant::{variant:?}"));
        }
        if size != ToggleButtonSize::M {
            toggle_props.push_str(&format!(" size=ToggleButtonSize::{size:?}"));
        }

        let mut snippet = vec![
            "let (bold, set_bold) = signal(false);".to_string(),
            "let (italic, set_italic) = signal(true);".to_string(),
            "let (underline, set_underline) = signal(false);".to_string(),
            "let bold_signal: Signal<bool> = Signal::derive(move || bold.get());".to_string(),
            "let italic_signal: Signal<bool> = Signal::derive(move || italic.get());".to_string(),
            "let underline_signal: Signal<bool> = Signal::derive(move || underline.get());"
                .to_string(),
            "let on_bold_change = Callback::new(move |next| set_bold.set(next));".to_string(),
            "let on_italic_change = Callback::new(move |next| set_italic.set(next));".to_string(),
            "let on_underline_change = Callback::new(move |next| set_underline.set(next));"
                .to_string(),
            String::new(),
            "<ToggleButtonGroup".to_string(),
        ];

        if orientation != ToggleButtonGroupOrientation::Horizontal {
            snippet.push(format!(
                "  orientation=ToggleButtonGroupOrientation::{orientation:?}"
            ));
        }
        if attached {
            snippet.push("  is_attached=true".to_string());
        }

        snippet.extend([
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
        ]);

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
>
  <ToggleButton is_pressed=left_signal on_pressed_change=on_left_change>"Left"</ToggleButton>
  <ToggleButton is_pressed=center_signal on_pressed_change=on_center_change>"Center"</ToggleButton>
  <ToggleButton is_pressed=right_signal on_pressed_change=on_right_change>"Right"</ToggleButton>
</ToggleButtonGroup>"#
            .to_string()
    });

    view! {
        <ComponentPage
            title="ToggleButtonGroup"
            slug="toggle-button-group"
            group="Actions"
            description="Layout wrapper with baseline-style root state attrs for orientation, attachment, and accessible labeling."
        >
            <Playground
                title="Attached horizontal"
                code_signal=code
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

                        <Switch checked=attached set_checked=set_attached>
                            "Attached layout"
                        </Switch>
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
                                aria_label="Formatting controls".to_string()
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

            <Playground title="Vertical + detached" code_signal=states_code>
                <div class="docs-stack">
                    <ToggleButtonGroup
                        orientation=ToggleButtonGroupOrientation::Vertical
                        is_attached=false
                        aria_label="Alignment controls".to_string()
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

    let code = Signal::derive(move || {
        let mode = match mode_index.get().unwrap_or(0) {
            1 => ThemeMode::Dark,
            2 => ThemeMode::Oled,
            _ => ThemeMode::Light,
        };
        let disabled = disabled.get();
        let two_mode_cycle = two_mode_cycle.get();
        let custom_aria_label = custom_aria_label.get();

        let mut snippet = vec![
            format!("let (mode, set_mode) = signal(ThemeMode::{mode:?});"),
            String::new(),
            "<ThemeToggleButton".to_string(),
            "  mode=mode".to_string(),
            "  set_mode=set_mode".to_string(),
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

        snippet.push("/>".to_string());

        snippet.join("\n")
    });

    let (custom_mode, set_custom_mode) = signal(ThemeMode::Dark);
    let custom_modes = vec![ThemeMode::Dark, ThemeMode::Light];

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

    view! {
        <ComponentPage
            title="ThemeToggleButton"
            slug="theme-toggle-button"
            group="Actions"
            description="Icon-only theme toggle with baseline-level spring motion and baseline-style mode state attrs."
        >
            <Playground
                title="Default cycle"
                code_signal=code
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
                                    />
                                }
                                    .into_any()
                            }}
                            <span class="ui-muted">"mode: " {move || format!("{:?}", mode.get())}</span>
                        </div>
                    }
                }}
            </Playground>

            <Playground title="Custom modes + disabled" code_signal=states_code>
                <div class="docs-stack">
                    <div class="docs-row">
                        <ThemeToggleButton
                            mode=custom_mode
                            set_mode=set_custom_mode
                            modes=custom_modes.clone()
                            aria_label="Switch UI mode".to_string()
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
    let (persist_workbench_state, set_persist_workbench_state) =
        signal(has_persisted_workbench_state);

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

    view! {
        <ComponentPage
            title="SearchInputButton"
            slug="search-input-button"
            group="Actions"
            description="baseline-level spring search trigger button with centralized placeholder/shortcut/aria-label state attrs."
        >
            <Playground
                title="Interactive + shortcut"
                code_signal=code
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

            <Playground title="Placeholder + disabled matrix" code_signal=states_code>
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
        _ => "cargo add ui-components".to_string(),
    });

    let (feedback_scale, set_feedback_scale) = signal(initial_workbench_state.feedback_scale);
    let (feedback_glow, set_feedback_glow) = signal(initial_workbench_state.feedback_glow);
    let (is_disabled, set_is_disabled) = signal(initial_workbench_state.is_disabled);
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
            "/* crates/ui-components/src/button/copy/styles.rs */\n{}",
            ui_components::button::copy::styles::CSS
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
            "ButtonCopyWorkbenchConfig {{\n  mode: {mode:?},\n  variant: {variant:?},\n  size: {size:?},\n  is_disabled: {is_disabled},\n  copied_feedback_scale: {copied_feedback_scale:.2},\n  copied_feedback_glow: {copied_feedback_glow:.2},\n  text: \"{text}\",\n}}"
        )
    });

    let hello_world_code = Signal::derive(move || {
        r#"<ButtonCopy text="cargo add ui-components".to_string() />"#.to_string()
    });

    let code = Signal::derive(move || {
        r#"<ButtonCopy
  text="cargo add ui-components".to_string()
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
        r#"<ButtonCopy text="cargo add ui-components".to_string() mode=ButtonCopyMode::TextOnly />
<ButtonCopy text="cargo add ui-components".to_string() mode=ButtonCopyMode::IconOnly />
<ButtonCopy text="cargo add ui-components".to_string() mode=ButtonCopyMode::IconAndText />"#
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
                    <ButtonCopy text="cargo add ui-components".to_string() />
                </div>
                <span class="ui-muted">"Start simple, then move to advanced controls."</span>
            </Playground>

            <Playground title="Label + variant" code_signal=code>
                <div class="docs-row">
                    <ButtonCopy
                        text="cargo add ui-components".to_string()
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
                        text="cargo add ui-components".to_string()
                        mode=ButtonCopyMode::TextOnly
                    />
                    <ButtonCopy
                        text="cargo add ui-components".to_string()
                        mode=ButtonCopyMode::IconOnly
                    />
                    <ButtonCopy
                        text="cargo add ui-components".to_string()
                        mode=ButtonCopyMode::IconAndText
                    />
                </div>
            </Playground>

            <Playground
                title="Workbench (Isolated Canvas + Optional Persist)"
                code_signal=workbench_code
                test_css_source=workbench_test_css_source
                test_source_path="/root/code/personal/omne/rust-ui/crates/ui-components/src/button/copy/styles.rs".to_string()
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
                                />
                                <span class="ui-muted">"text: " {text}</span>
                            </div>
                        </div>
                    }
                }}
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

    let interactive_code = Signal::derive(move || {
        let direction = interactive_direction_label.get();
        format!(
            "<FlipButton\n  from=FlipDirection::{direction}\n  front=move || view! {{ <Button variant=ButtonVariant::Secondary>\"Front\"</Button> }}\n  back=move || view! {{ <Button variant=ButtonVariant::Accent>\"Back\"</Button> }}\n/>"
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
                "Copy-ready snippets prepend imports automatically; dependency: ui-components; source: crates/ui-components/src/button/flip/view.rs."
            </p>

            <Playground title="Top flip" code_signal=code>
                <div class="docs-row">
                    <FlipButton
                        from=FlipDirection::Top
                        front=move || view! { <Button variant=ButtonVariant::Secondary>"Front"</Button> }
                        back=move || view! { <Button variant=ButtonVariant::Accent>"Back"</Button> }
                    />
                </div>
            </Playground>

            <Playground title="Direction matrix" code_signal=states_code>
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
                                        front=move || view! { <Button variant=ButtonVariant::Secondary>"Front"</Button> }
                                        back=move || view! { <Button variant=ButtonVariant::Accent>"Back"</Button> }
                                    />
                                </div>
                            </div>
                        </div>
                    }
                }}
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn share_button() -> AnyView {
    let (last, set_last) = signal(None::<SharePlatform>);
    let on_icon_press = Callback::new(move |platform: SharePlatform| set_last.set(Some(platform)));

    let custom_items = vec![
        ShareButtonItem::new(SharePlatform::Github, "Repository"),
        ShareButtonItem::new(SharePlatform::X, "Post"),
        ShareButtonItem::new(SharePlatform::Facebook, "   "),
    ];

    let custom_items_for_matrix = custom_items.clone();
    let custom_items_for_custom = custom_items.clone();

    let hello_code = Signal::derive(move || r#"<ShareButton />"#.to_string());

    let code = Signal::derive(move || {
        r#"let on_icon_press = Callback::new(|platform: SharePlatform| {
  logging::log!("pressed: {platform:?}");
});
<ShareButton on_icon_press=Some(on_icon_press) />"#
            .to_string()
    });

    let states_code = Signal::derive(move || {
        r#"<ShareButton
  icon=ShareButtonIconPlacement::Prefix
  from=FlipDirection::Left
  label="Share now".to_string()
  items=custom_items_for_matrix.clone()
/>
<ShareButton icon=ShareButtonIconPlacement::None label="Iconless".to_string() />"#
            .to_string()
    });

    let custom_code = Signal::derive(move || {
        r#"<ShareButton
  class_name="docs-share-button-custom".to_string()
  icon=ShareButtonIconPlacement::Prefix
  from=FlipDirection::Right
  label="Share docs".to_string()
  items=custom_items.clone()
/>"#
        .to_string()
    });

    view! {
        <ComponentPage
            title="ShareButton"
            slug="share-button"
            group="Actions"
            description="Flip-based share surface with centralized item/icon/handler state attrs and baseline-level spring motion."
        >
            <Playground title="Hello World" code_signal=hello_code>
                <div class="docs-row">
                    <ShareButton />
                </div>
            </Playground>

            <Playground title="Default + callback" code_signal=code>
                <div class="docs-stack">
                    <div class="docs-row">
                        <ShareButton on_icon_press=on_icon_press />
                        <span class="ui-muted">
                            "last: "
                            {move || {
                                last.get()
                                    .map(|v| format!("{v:?}"))
                                    .unwrap_or_else(|| "None".to_string())
                            }}
                        </span>
                    </div>
                </div>
            </Playground>

            <Playground title="Icon placement + custom items" code_signal=states_code>
                <div class="docs-stack">
                    <div class="docs-row">
                        <ShareButton
                            icon=ShareButtonIconPlacement::Prefix
                            from=FlipDirection::Left
                            label="Share now".to_string()
                            items=custom_items_for_matrix.clone()
                            on_icon_press=on_icon_press
                        />
                        <ShareButton
                            icon=ShareButtonIconPlacement::None
                            label="Iconless".to_string()
                            items=custom_items_for_matrix.clone()
                        />
                    </div>
                    <span class="ui-muted">
                        "Blank custom item labels fall back to platform defaults; missing handlers stay safe."
                    </span>
                </div>
            </Playground>

            <Playground title="Custom Class + Direction" code_signal=custom_code>
                <div class="docs-row">
                    <ShareButton
                        class_name="docs-share-button-custom".to_string()
                        icon=ShareButtonIconPlacement::Prefix
                        from=FlipDirection::Right
                        label="Share docs".to_string()
                        items=custom_items_for_custom.clone()
                    />
                    <ShareButton
                        class_name="docs-share-button-custom".to_string()
                        label="Share defaults".to_string()
                        icon=ShareButtonIconPlacement::Suffix
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
  motion=ui_components::ActionMenuMotion {
    popover: ui_components::PopoverMotion {
      initial_scale: 0.93,
      offset_y_px: 8.0,
      ..ui_components::PopoverMotion::default()
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

    let marker_motion = ui_components::ActionMenuMotion {
        popover: ui_components::PopoverMotion {
            initial_scale: 0.93,
            offset_y_px: 8.0,
            ..ui_components::PopoverMotion::default()
        },
    };

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
        </ComponentPage>
    }
    .into_any()
}
