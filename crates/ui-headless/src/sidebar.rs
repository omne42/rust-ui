use crate::a11y::{A11yDirection, aria_expanded, locale_attrs};
use leptos::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SidebarRootAttrs {
    pub role: &'static str,
    pub aria_label: String,
    pub aria_keyshortcuts: Option<String>,
    pub lang: Option<String>,
    pub dir: Option<&'static str>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SidebarKeyDownInput {
    pub key: String,
    pub ctrl_key: bool,
    pub meta_key: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SidebarRootState {
    pub is_disabled: bool,
    pub has_shortcut_key: bool,
    pub shortcut_source_attr: &'static str,
}

#[derive(Clone)]
pub struct SidebarRootHandlers {
    pub on_key_down: Callback<SidebarKeyDownInput, bool>,
}

#[derive(Clone)]
pub struct SidebarRootContract {
    pub attrs: SidebarRootAttrs,
    pub handlers: SidebarRootHandlers,
    pub state: SidebarRootState,
}

#[derive(Clone)]
pub struct SidebarRootOptions {
    pub is_disabled: bool,
    pub shortcut_key: Option<String>,
    pub aria_label: String,
    pub lang: Option<String>,
    pub dir: Option<A11yDirection>,
    pub on_shortcut_toggle: Option<Callback<()>>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SidebarToggleButtonA11yAttrs {
    pub aria_disabled: Option<&'static str>,
    pub aria_expanded: Signal<&'static str>,
    pub aria_label: String,
    pub lang: Option<String>,
    pub dir: Option<&'static str>,
}

#[derive(Clone)]
pub struct SidebarToggleButtonA11yOptions {
    pub is_disabled: bool,
    pub aria_label: String,
    pub lang: Option<String>,
    pub dir: Option<A11yDirection>,
}

pub fn shortcut_hint(shortcut_key: Option<&str>) -> Option<String> {
    shortcut_key.map(|shortcut_key| format!("Ctrl+{shortcut_key}"))
}

pub fn should_toggle_for_shortcut(
    input: &SidebarKeyDownInput,
    shortcut_key: Option<&str>,
    is_disabled: bool,
) -> bool {
    if is_disabled {
        return false;
    }

    let Some(shortcut_key) = shortcut_key else {
        return false;
    };

    if !(input.ctrl_key || input.meta_key) {
        return false;
    }

    input.key.eq_ignore_ascii_case(shortcut_key)
}

pub fn use_sidebar_root(options: SidebarRootOptions) -> SidebarRootContract {
    let locale = locale_attrs(options.lang, options.dir);
    let state = SidebarRootState {
        is_disabled: options.is_disabled,
        has_shortcut_key: options.shortcut_key.is_some(),
        shortcut_source_attr: if options.shortcut_key.is_some() {
            "provided"
        } else {
            "none"
        },
    };

    let attrs = SidebarRootAttrs {
        role: "complementary",
        aria_label: options.aria_label,
        aria_keyshortcuts: shortcut_hint(options.shortcut_key.as_deref()),
        lang: locale.lang.clone(),
        dir: locale.dir,
    };

    let shortcut_key = options.shortcut_key;
    let is_disabled = options.is_disabled;
    let on_shortcut_toggle = options.on_shortcut_toggle;
    let handlers = SidebarRootHandlers {
        on_key_down: Callback::new(move |input: SidebarKeyDownInput| {
            if should_toggle_for_shortcut(&input, shortcut_key.as_deref(), is_disabled) {
                if let Some(on_shortcut_toggle) = on_shortcut_toggle.as_ref() {
                    on_shortcut_toggle.run(());
                }
                return true;
            }
            false
        }),
    };

    SidebarRootContract {
        attrs,
        handlers,
        state,
    }
}

pub fn sidebar_toggle_button_a11y_attrs(
    open: Signal<bool>,
    options: SidebarToggleButtonA11yOptions,
) -> SidebarToggleButtonA11yAttrs {
    let locale = locale_attrs(options.lang, options.dir);

    SidebarToggleButtonA11yAttrs {
        aria_disabled: options.is_disabled.then_some("true"),
        aria_expanded: aria_expanded(open),
        aria_label: options.aria_label,
        lang: locale.lang,
        dir: locale.dir,
    }
}

#[cfg(test)]
#[path = "test/sidebar.rs"]
mod tests;
