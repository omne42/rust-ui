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
mod tests {
    use super::*;
    use crate::roving_tabindex::RovingOrientation;

    fn init_executor() {
        drop(any_spawner::Executor::init_futures_executor());
    }

    fn poll_effects() {
        any_spawner::Executor::poll_local();
    }

    #[test]
    fn use_radio_exposes_typed_attrs_handlers_state_with_locale() {
        init_executor();

        let (count, _set_count) = signal(3_usize);
        let (selected, set_selected) = signal(Some(0_usize));

        let radio = use_radio(RadioOptions {
            group: RadioGroupOptions {
                is_disabled: false,
                id_base: "rg".to_string(),
                orientation: RovingOrientation::Horizontal,
                item_count: count,
                selected_index: selected,
                set_selected_index: set_selected,
                on_change: None,
                is_item_disabled: None,
            },
            lang: Some("  zh-CN ".to_string()),
            dir: Some(A11yDirection::Rtl),
        });

        poll_effects();

        assert_eq!(radio.attrs.role, "radiogroup");
        assert_eq!(radio.attrs.aria_disabled, None);
        assert_eq!(radio.attrs.lang.as_deref(), Some("zh-CN"));
        assert_eq!(radio.attrs.dir, Some("rtl"));
        assert_eq!(radio.state.active_index.get_untracked(), 0);
        assert_eq!(radio.state.selected_index.get_untracked(), Some(0));
        assert_eq!(radio.state.radio_id.run(2), "rg-radio-2");
    }

    #[test]
    fn use_radio_delegates_keyboard_to_headless_roving_contract() {
        init_executor();

        let (count, _set_count) = signal(3_usize);
        let (selected, set_selected) = signal(Some(0_usize));

        let radio = use_radio(RadioOptions {
            group: RadioGroupOptions {
                is_disabled: false,
                id_base: "rg".to_string(),
                orientation: RovingOrientation::Horizontal,
                item_count: count,
                selected_index: selected,
                set_selected_index: set_selected,
                on_change: None,
                is_item_disabled: None,
            },
            lang: None,
            dir: None,
        });

        poll_effects();

        assert!(radio.handlers.on_key_down.run("ArrowRight".to_string()));
        assert_eq!(radio.state.selected_index.get_untracked(), Some(1));
        assert_eq!(radio.state.active_index.get_untracked(), 1);
    }
}
