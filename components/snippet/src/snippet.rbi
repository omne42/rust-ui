pub type A11yDirection = ui_headless::A11yDirection;

pub const DEFAULT_COPY_LABEL: &str;
pub const DEFAULT_COPIED_LABEL: &str;

pub struct SnippetTextContract {
    pub copy_label: String,
    pub copied_label: String,
    pub copy_aria_label: String,
    pub copy_error_label: String,
}

pub struct SnippetTextFallbacks {
    pub copy_label: Option<String>,
    pub copied_label: Option<String>,
    pub copy_aria_label: Option<String>,
    pub copy_error_label: Option<String>,
}

pub struct SnippetLogicOptions {
    pub text: String,
    pub is_copyable: bool,
    pub is_copied: Option<leptos::prelude::Signal<bool>>,
    pub default_copied: Option<bool>,
    pub on_copied_change: Option<leptos::prelude::Callback<bool>>,
    pub on_copy_error: Option<leptos::prelude::Callback<()>>,
    pub lang: Option<String>,
    pub dir: Option<A11yDirection>,
}

pub struct SnippetLogic {
    pub copied: leptos::prelude::Signal<bool>,
    pub is_loading: leptos::prelude::ReadSignal<bool>,
    pub has_error: leptos::prelude::ReadSignal<bool>,
    pub is_copying: leptos::prelude::ReadSignal<bool>,
    pub has_copy_error: leptos::prelude::ReadSignal<bool>,
    pub copy: leptos::prelude::Callback<()>,
    pub retry_copy: leptos::prelude::Callback<()>,
    pub aria_busy: leptos::prelude::Signal<Option<&'static str>>,
    pub lang: Option<String>,
    pub dir: Option<&'static str>,
}

pub struct SnippetMotion {
    pub enabled: bool,
    pub spring: ui_motion::spring::SpringConfig,
    pub copied_scale: f64,
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String>;

pub fn resolve_text_contract(
    copy_label: Option<String>,
    copied_label: Option<String>,
    copy_aria_label: Option<String>,
    copy_error_label: Option<String>,
    fallbacks: SnippetTextFallbacks,
) -> SnippetTextContract;

pub fn use_snippet_logic_with_options(options: SnippetLogicOptions) -> SnippetLogic;

pub fn use_snippet_logic(text: String) -> SnippetLogic;

pub fn Snippet(
    text: String,
    label: Option<String>,
    is_copyable: Option<bool>,
    copyable: Option<bool>,
    copy_label: Option<String>,
    copied_label: Option<String>,
    copy_aria_label: Option<String>,
    copy_error_label: Option<String>,
    is_copied: Option<leptos::prelude::Signal<bool>>,
    copied: Option<leptos::prelude::Signal<bool>>,
    default_copied: Option<bool>,
    on_copied_change: Option<leptos::prelude::Callback<bool>>,
    motion: SnippetMotion,
    class_name: Option<String>,
    lang: Option<String>,
    dir: Option<A11yDirection>,
) -> impl leptos::prelude::IntoView;
