use leptos::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SidebarMenuKeyboardAttrs {
    pub aria_keyshortcuts: Option<String>,
    pub shortcut_source_attr: &'static str,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SidebarMenuKeyDownInput {
    pub key: String,
    pub ctrl_key: bool,
    pub meta_key: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SidebarMenuKeyAction {
    None,
    FocusFirst,
    MoveNext,
    MovePrevious,
    Home,
    End,
    Activate,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SidebarMenuKeyboardState {
    pub is_disabled: bool,
    pub has_shortcut_key: bool,
    pub shortcut_source_attr: &'static str,
}

#[derive(Clone)]
pub struct SidebarMenuKeyboardHandlers {
    pub on_key_down: Callback<SidebarMenuKeyDownInput, SidebarMenuKeyAction>,
}

#[derive(Clone)]
pub struct SidebarMenuKeyboardContract {
    pub attrs: SidebarMenuKeyboardAttrs,
    pub handlers: SidebarMenuKeyboardHandlers,
    pub state: SidebarMenuKeyboardState,
}

#[derive(Clone)]
pub struct SidebarMenuKeyboardOptions {
    pub is_disabled: bool,
    pub shortcut_key: Option<String>,
}

fn shortcut_hint(shortcut_key: Option<&str>) -> Option<String> {
    shortcut_key.map(|shortcut_key| format!("Ctrl+{shortcut_key}"))
}

fn is_activation_key(key: &str) -> bool {
    key == "Enter" || key == " " || key == "Space" || key == "Spacebar"
}

pub fn resolve_sidebar_menu_key_action(
    input: &SidebarMenuKeyDownInput,
    shortcut_key: Option<&str>,
    is_disabled: bool,
) -> SidebarMenuKeyAction {
    if is_disabled {
        return SidebarMenuKeyAction::None;
    }

    if let Some(shortcut_key) = shortcut_key
        && (input.ctrl_key || input.meta_key)
        && input.key.eq_ignore_ascii_case(shortcut_key)
    {
        return SidebarMenuKeyAction::FocusFirst;
    }

    match input.key.as_str() {
        "ArrowDown" => SidebarMenuKeyAction::MoveNext,
        "ArrowUp" => SidebarMenuKeyAction::MovePrevious,
        "Home" => SidebarMenuKeyAction::Home,
        "End" => SidebarMenuKeyAction::End,
        key if is_activation_key(key) => SidebarMenuKeyAction::Activate,
        _ => SidebarMenuKeyAction::None,
    }
}

pub fn use_sidebar_menu_keyboard(
    options: SidebarMenuKeyboardOptions,
) -> SidebarMenuKeyboardContract {
    let state = SidebarMenuKeyboardState {
        is_disabled: options.is_disabled,
        has_shortcut_key: options.shortcut_key.is_some(),
        shortcut_source_attr: if options.shortcut_key.is_some() {
            "provided"
        } else {
            "none"
        },
    };

    let attrs = SidebarMenuKeyboardAttrs {
        aria_keyshortcuts: shortcut_hint(options.shortcut_key.as_deref()),
        shortcut_source_attr: state.shortcut_source_attr,
    };

    let shortcut_key = options.shortcut_key;
    let is_disabled = options.is_disabled;
    let handlers = SidebarMenuKeyboardHandlers {
        on_key_down: Callback::new(move |input: SidebarMenuKeyDownInput| {
            resolve_sidebar_menu_key_action(&input, shortcut_key.as_deref(), is_disabled)
        }),
    };

    SidebarMenuKeyboardContract {
        attrs,
        handlers,
        state,
    }
}

#[cfg(test)]
#[path = "test/sidebar_menu.rs"]
mod tests;
