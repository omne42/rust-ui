use crate::a11y::{A11yDirection, locale_attrs};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CommandInputAttrs {
    pub role: &'static str,
    pub aria_autocomplete: &'static str,
    pub aria_expanded: &'static str,
    pub lang: Option<String>,
    pub dir: Option<&'static str>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CommandOptionA11yInput {
    pub is_disabled: bool,
    pub is_selected: bool,
    pub is_focused: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CommandOptionA11yAttrs {
    pub role: &'static str,
    pub data_state: &'static str,
    pub aria_selected: Option<&'static str>,
    pub aria_disabled: Option<&'static str>,
    pub data_selected: Option<&'static str>,
    pub data_focused: Option<&'static str>,
    pub data_disabled: Option<&'static str>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CommandInputKeyDownResult {
    Ignored,
    DelegatedToListBox,
    ClearedQuery,
}

pub fn command_input_attrs(lang: Option<String>, dir: Option<A11yDirection>) -> CommandInputAttrs {
    let locale = locale_attrs(lang, dir);

    CommandInputAttrs {
        role: "combobox",
        aria_autocomplete: "list",
        aria_expanded: "true",
        lang: locale.lang,
        dir: locale.dir,
    }
}

pub fn command_option_a11y_attrs(input: CommandOptionA11yInput) -> CommandOptionA11yAttrs {
    let data_state = if input.is_disabled {
        "disabled"
    } else if input.is_selected {
        "selected"
    } else if input.is_focused {
        "focused"
    } else {
        "idle"
    };

    CommandOptionA11yAttrs {
        role: "option",
        data_state,
        aria_selected: input.is_selected.then_some("true"),
        aria_disabled: input.is_disabled.then_some("true"),
        data_selected: input.is_selected.then_some("true"),
        data_focused: input.is_focused.then_some("true"),
        data_disabled: input.is_disabled.then_some("true"),
    }
}

pub fn resolve_command_input_key_down(key: &str, has_query: bool) -> CommandInputKeyDownResult {
    if key == "Escape" && has_query {
        return CommandInputKeyDownResult::ClearedQuery;
    }

    let should_delegate = matches!(key, "ArrowDown" | "ArrowUp" | "Home" | "End" | "Enter");
    if should_delegate {
        CommandInputKeyDownResult::DelegatedToListBox
    } else {
        CommandInputKeyDownResult::Ignored
    }
}

#[cfg(test)]
#[path = "test/command.rs"]
mod tests;
