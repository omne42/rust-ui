use crate::a11y::{A11yDirection, locale_attrs};
use crate::button::{ButtonElement, ButtonHandlers, ButtonOptions, use_button};
use crate::press::OnPress;
use leptos::prelude::*;

#[derive(Clone)]
pub struct SwatchOptions {
    pub is_disabled: bool,
    pub is_decorative: bool,
    pub is_mixed_value: bool,
    pub is_selected: Signal<bool>,
    pub aria_label: Option<String>,
    pub on_press: Option<OnPress>,
    pub lang: Option<String>,
    pub dir: Option<A11yDirection>,
}

#[derive(Clone)]
pub struct SwatchHandlers {
    pub button: ButtonHandlers,
}

#[derive(Clone)]
pub struct SwatchAttrs {
    pub role: Option<&'static str>,
    pub tabindex: Option<i32>,
    pub aria_disabled: Option<&'static str>,
    pub aria_pressed: Signal<Option<&'static str>>,
    pub aria_checked: Option<&'static str>,
    pub aria_hidden: Option<&'static str>,
    pub aria_label: Option<String>,
    pub lang: Option<String>,
    pub dir: Option<&'static str>,
}

#[derive(Clone)]
pub struct SwatchState {
    pub is_pressed: ReadSignal<bool>,
    pub is_interactive: bool,
}

#[derive(Clone)]
pub struct SwatchAria {
    pub state: SwatchState,
    pub handlers: SwatchHandlers,
    pub attrs: SwatchAttrs,
}

pub fn use_swatch(options: SwatchOptions) -> SwatchAria {
    let SwatchOptions {
        is_disabled,
        is_decorative,
        is_mixed_value,
        is_selected,
        aria_label,
        on_press,
        lang,
        dir,
    } = options;

    let is_interactive = !is_disabled && !is_decorative && !is_mixed_value;
    let locale = locale_attrs(lang, dir);

    let button = use_button(ButtonOptions {
        is_disabled: !is_interactive,
        on_press,
        element: ButtonElement::Custom,
    });

    let aria_pressed = Signal::derive(move || {
        (!is_decorative && !is_mixed_value).then_some(if is_selected.get() {
            "true"
        } else {
            "false"
        })
    });

    SwatchAria {
        state: SwatchState {
            is_pressed: button.is_pressed,
            is_interactive,
        },
        handlers: SwatchHandlers {
            button: button.handlers,
        },
        attrs: SwatchAttrs {
            role: (!is_decorative).then_some("button"),
            tabindex: is_interactive.then_some(0),
            aria_disabled: is_disabled.then_some("true"),
            aria_pressed,
            aria_checked: is_mixed_value.then_some("mixed"),
            aria_hidden: is_decorative.then_some("true"),
            aria_label: (!is_decorative).then_some(aria_label).flatten(),
            lang: locale.lang,
            dir: locale.dir,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    };

    #[test]
    fn swatch_contract_maps_aria_and_locale_attrs() {
        let (selected, set_selected) = signal(false);
        let swatch = use_swatch(SwatchOptions {
            is_disabled: false,
            is_decorative: false,
            is_mixed_value: false,
            is_selected: selected.into(),
            aria_label: Some("Accent".to_string()),
            on_press: None,
            lang: Some("  zh-CN ".to_string()),
            dir: Some(A11yDirection::Rtl),
        });

        assert_eq!(swatch.attrs.role, Some("button"));
        assert_eq!(swatch.attrs.tabindex, Some(0));
        assert_eq!(swatch.attrs.aria_disabled, None);
        assert_eq!(swatch.attrs.aria_checked, None);
        assert_eq!(swatch.attrs.aria_hidden, None);
        assert_eq!(swatch.attrs.aria_label.as_deref(), Some("Accent"));
        assert_eq!(swatch.attrs.lang.as_deref(), Some("zh-CN"));
        assert_eq!(swatch.attrs.dir, Some("rtl"));
        assert_eq!(swatch.attrs.aria_pressed.get_untracked(), Some("false"));
        assert!(swatch.state.is_interactive);

        set_selected.set(true);
        assert_eq!(swatch.attrs.aria_pressed.get_untracked(), Some("true"));
    }

    #[test]
    fn swatch_contract_disables_interaction_for_mixed_and_decorative_states() {
        let (selected, _set_selected) = signal(true);
        let mixed = use_swatch(SwatchOptions {
            is_disabled: false,
            is_decorative: false,
            is_mixed_value: true,
            is_selected: selected.into(),
            aria_label: Some("Mixed".to_string()),
            on_press: None,
            lang: None,
            dir: None,
        });

        assert!(!mixed.state.is_interactive);
        assert_eq!(mixed.attrs.tabindex, None);
        assert_eq!(mixed.attrs.aria_pressed.get_untracked(), None);
        assert_eq!(mixed.attrs.aria_checked, Some("mixed"));
        assert_eq!(mixed.attrs.aria_hidden, None);

        let decorative = use_swatch(SwatchOptions {
            is_disabled: false,
            is_decorative: true,
            is_mixed_value: false,
            is_selected: selected.into(),
            aria_label: Some("Decorative".to_string()),
            on_press: None,
            lang: None,
            dir: None,
        });

        assert!(!decorative.state.is_interactive);
        assert_eq!(decorative.attrs.role, None);
        assert_eq!(decorative.attrs.tabindex, None);
        assert_eq!(decorative.attrs.aria_label, None);
        assert_eq!(decorative.attrs.aria_hidden, Some("true"));
    }

    #[test]
    fn swatch_contract_delegates_press_to_button_press_model() {
        let (selected, _set_selected) = signal(false);
        let pressed = Arc::new(AtomicUsize::new(0));
        let pressed2 = Arc::clone(&pressed);
        let swatch = use_swatch(SwatchOptions {
            is_disabled: false,
            is_decorative: false,
            is_mixed_value: false,
            is_selected: selected.into(),
            aria_label: Some("Accent".to_string()),
            on_press: Some(Callback::new(move |_| {
                pressed2.fetch_add(1, Ordering::SeqCst);
            })),
            lang: None,
            dir: None,
        });

        let prevented = swatch
            .handlers
            .button
            .press
            .on_key_down
            .run(" ".to_string());
        assert!(prevented);
        assert_eq!(pressed.load(Ordering::SeqCst), 0);

        let prevented = swatch.handlers.button.press.on_key_up.run(" ".to_string());
        assert!(prevented);
        assert_eq!(pressed.load(Ordering::SeqCst), 1);
    }
}
