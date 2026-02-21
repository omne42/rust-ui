use crate::a11y::{A11yDirection, locale_attrs};
use ui_state_primitives::file_trigger::FileTriggerState;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct FileTriggerHandlers;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FileTriggerAttrs {
    pub lang: Option<String>,
    pub dir: Option<&'static str>,
    pub data_state: &'static str,
    pub data_disabled: Option<&'static str>,
    pub data_enabled: Option<&'static str>,
    pub input_tabindex: i32,
    pub input_aria_hidden: &'static str,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FileTriggerSemanticState {
    pub state: &'static str,
    pub is_disabled: bool,
    pub is_enabled: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FileTriggerContract {
    pub attrs: FileTriggerAttrs,
    pub handlers: FileTriggerHandlers,
    pub state: FileTriggerSemanticState,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FileTriggerOptions {
    pub state: FileTriggerState,
    pub lang: Option<String>,
    pub dir: Option<A11yDirection>,
}

pub fn use_file_trigger(options: FileTriggerOptions) -> FileTriggerContract {
    let locale = locale_attrs(options.lang, options.dir);

    FileTriggerContract {
        attrs: FileTriggerAttrs {
            lang: locale.lang,
            dir: locale.dir,
            data_state: options.state.state_attr,
            data_disabled: options.state.is_disabled.then_some("true"),
            data_enabled: options.state.is_enabled.then_some("true"),
            input_tabindex: -1,
            input_aria_hidden: "true",
        },
        handlers: FileTriggerHandlers,
        state: FileTriggerSemanticState {
            state: options.state.state_attr,
            is_disabled: options.state.is_disabled,
            is_enabled: options.state.is_enabled,
        },
    }
}

#[cfg(test)]
#[path = "test/file_trigger.rs"]
mod tests;
