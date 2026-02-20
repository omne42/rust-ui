pub enum ActionBarPosition {
    Top,
    Bottom,
}

pub enum ActionBarPhase {
    Visible,
    Hidden,
}

pub enum ActionBarSelectionKind {
    Empty,
    Single,
    Multiple,
}

pub struct ActionBarStrings {
    pub aria_label: std::sync::Arc<str>,
    pub clear_label: std::sync::Arc<str>,
    pub selection_empty_label: std::sync::Arc<str>,
    pub selection_single_label: std::sync::Arc<str>,
    pub selection_multiple_template: std::sync::Arc<str>,
}

impl ActionBarStrings {
    pub fn selection_label(&self, count: usize) -> String;
}

pub struct ActionBarMotion {
    pub enabled: bool,
    pub spring: ui_motion::spring::SpringConfig,
    pub hidden_translate_px: f64,
    pub hidden_opacity: f64,
}

impl ActionBarMotion {
    pub fn disabled() -> Self;
}

pub fn sanitize_motion(motion: ActionBarMotion) -> ActionBarMotion;

pub fn ActionBar(
    selected_count: Option<leptos::prelude::Signal<usize>>,
    default_selected_count: Option<usize>,
    on_selected_count_change: Option<leptos::prelude::Callback<usize>>,
    on_clear_selection: Option<leptos::prelude::Callback<()>>,
    position: ActionBarPosition,
    is_force_visible: bool,
    aria_label: Option<String>,
    clear_label: Option<String>,
    selection_text: Option<String>,
    lang: Option<String>,
    dir: Option<ui_headless::A11yDirection>,
    motion: ActionBarMotion,
    class_name: Option<String>,
    children: Option<leptos::children::Children>,
) -> impl leptos::prelude::IntoView;
