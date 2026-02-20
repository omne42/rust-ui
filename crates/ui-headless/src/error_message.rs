use crate::a11y::{A11yDirection, LiveRegionPriority, live_region_attrs, locale_attrs};
use ui_state_primitives::error_message::ErrorMessageState;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct ErrorMessageHandlers;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ErrorMessageAttrs {
    pub role: &'static str,
    pub aria_live: &'static str,
    pub aria_label: String,
    pub aria_disabled: Option<&'static str>,
    pub lang: Option<String>,
    pub dir: Option<&'static str>,
    pub data_tone: &'static str,
    pub data_state: &'static str,
    pub data_disabled: Option<&'static str>,
    pub data_truncate: Option<&'static str>,
    pub data_message_source: &'static str,
    pub data_aria_source: &'static str,
    pub data_custom_class: Option<&'static str>,
    pub data_class_source: &'static str,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ErrorMessageSemanticState {
    pub tone: &'static str,
    pub state: &'static str,
    pub message_source: &'static str,
    pub aria_source: &'static str,
    pub class_source: &'static str,
    pub is_disabled: bool,
    pub is_truncated: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ErrorMessageContract {
    pub attrs: ErrorMessageAttrs,
    pub handlers: ErrorMessageHandlers,
    pub state: ErrorMessageSemanticState,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ErrorMessageOptions {
    pub state: ErrorMessageState,
    pub aria_label: String,
    pub lang: Option<String>,
    pub dir: Option<A11yDirection>,
}

pub fn use_error_message(options: ErrorMessageOptions) -> ErrorMessageContract {
    let locale = locale_attrs(options.lang, options.dir);
    let live_region = live_region_attrs(LiveRegionPriority::Assertive);
    let state = options.state;

    ErrorMessageContract {
        attrs: ErrorMessageAttrs {
            role: live_region.role,
            aria_live: live_region.aria_live,
            aria_label: options.aria_label,
            aria_disabled: state.is_disabled.then_some("true"),
            lang: locale.lang,
            dir: locale.dir,
            data_tone: state.tone_attr,
            data_state: state.data_state_attr,
            data_disabled: state.is_disabled.then_some("true"),
            data_truncate: state.is_truncated.then_some("true"),
            data_message_source: state.message_source_attr,
            data_aria_source: state.aria_source_attr,
            data_custom_class: state.has_custom_class_name.then_some("true"),
            data_class_source: state.class_source_attr,
        },
        handlers: ErrorMessageHandlers,
        state: ErrorMessageSemanticState {
            tone: state.tone_attr,
            state: state.data_state_attr,
            message_source: state.message_source_attr,
            aria_source: state.aria_source_attr,
            class_source: state.class_source_attr,
            is_disabled: state.is_disabled,
            is_truncated: state.is_truncated,
            has_custom_class_name: state.has_custom_class_name,
        },
    }
}

#[cfg(test)]
#[path = "test/error_message.rs"]
mod tests;
