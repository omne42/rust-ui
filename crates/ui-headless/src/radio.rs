use crate::a11y::{A11yDirection, locale_attrs};
use crate::radio_group::{RadioGroupHandlers, RadioGroupOptions, use_radio_group};
use leptos::prelude::*;

#[derive(Clone)]
pub struct RadioOptions {
    pub group: RadioGroupOptions,
    pub lang: Option<String>,
    pub dir: Option<A11yDirection>,
}

#[derive(Clone)]
pub struct RadioAttrs {
    pub role: &'static str,
    pub aria_disabled: Option<&'static str>,
    pub lang: Option<String>,
    pub dir: Option<&'static str>,
}

#[derive(Clone)]
pub struct RadioState {
    pub active_index: ReadSignal<usize>,
    pub selected_index: ReadSignal<Option<usize>>,
    pub radio_id: Callback<usize, String>,
}

#[derive(Clone)]
pub struct RadioContract {
    pub attrs: RadioAttrs,
    pub handlers: RadioGroupHandlers,
    pub state: RadioState,
}

pub fn use_radio(options: RadioOptions) -> RadioContract {
    let RadioOptions { group, lang, dir } = options;
    let locale = locale_attrs(lang, dir);
    let group = use_radio_group(group);

    RadioContract {
        attrs: RadioAttrs {
            role: group.attrs.role,
            aria_disabled: group.attrs.aria_disabled,
            lang: locale.lang,
            dir: locale.dir,
        },
        handlers: group.handlers,
        state: RadioState {
            active_index: group.active_index,
            selected_index: group.selected_index,
            radio_id: group.radio_id,
        },
    }
}

#[cfg(test)]
#[path = "test/radio.rs"]
mod tests;
