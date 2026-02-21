use crate::a11y::{A11yDirection, locale_attrs};
use ui_state_primitives::native_select::{
    NativeSelectOptionResolved, NativeSelectState, find_index_by_value,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct NativeSelectHandlers;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NativeSelectAttrs {
    pub aria_label: String,
    pub aria_invalid: Option<&'static str>,
    pub disabled: bool,
    pub required: bool,
    pub lang: Option<String>,
    pub dir: Option<&'static str>,
    pub data_slot: &'static str,
    pub data_aria_source: &'static str,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NativeSelectSemanticState {
    pub is_disabled: bool,
    pub is_invalid: bool,
    pub is_required: bool,
    pub visual_state: &'static str,
    pub aria_source: &'static str,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NativeSelectContract {
    pub attrs: NativeSelectAttrs,
    pub handlers: NativeSelectHandlers,
    pub state: NativeSelectSemanticState,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NativeSelectOptions {
    pub state: NativeSelectState,
    pub aria_label: String,
    pub lang: Option<String>,
    pub dir: Option<A11yDirection>,
}

pub fn use_native_select(options: NativeSelectOptions) -> NativeSelectContract {
    let locale = locale_attrs(options.lang, options.dir);

    NativeSelectContract {
        attrs: NativeSelectAttrs {
            aria_label: options.aria_label,
            aria_invalid: options.state.is_invalid.then_some("true"),
            disabled: options.state.control_disabled,
            required: options.state.is_required,
            lang: locale.lang,
            dir: locale.dir,
            data_slot: "native-select-control",
            data_aria_source: options.state.aria_source_attr,
        },
        handlers: NativeSelectHandlers,
        state: NativeSelectSemanticState {
            is_disabled: options.state.control_disabled,
            is_invalid: options.state.is_invalid,
            is_required: options.state.is_required,
            visual_state: options.state.data_state_attr,
            aria_source: options.state.aria_source_attr,
        },
    }
}

pub fn resolve_native_select_change_index(
    value: &str,
    options: &[NativeSelectOptionResolved],
) -> Option<usize> {
    let normalized = value.trim();
    if normalized.is_empty() {
        None
    } else {
        find_index_by_value(normalized, options)
    }
}

#[cfg(test)]
#[path = "test/native_select.rs"]
mod tests;
