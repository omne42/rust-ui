use crate::a11y::{A11yDirection, locale_attrs};
use crate::press::OnPress;
use leptos::prelude::*;
use ui_state_primitives::underlay::UnderlayPartState;

#[derive(Clone)]
pub struct UnderlayOptions {
    pub state: Signal<UnderlayPartState>,
    pub on_close: Option<OnPress>,
    pub lang: Option<String>,
    pub dir: Option<A11yDirection>,
}

#[derive(Clone)]
pub struct UnderlayHandlers {
    pub on_click: Callback<()>,
}

#[derive(Clone)]
pub struct UnderlayAttrs {
    pub role: &'static str,
    pub aria_hidden: &'static str,
    pub tabindex: i32,
    pub lang: Option<String>,
    pub dir: Option<&'static str>,
}

#[derive(Clone)]
pub struct UnderlayState {
    pub is_open: Signal<bool>,
    pub is_disabled: Signal<bool>,
    pub is_interactive: Signal<bool>,
}

#[derive(Clone)]
pub struct UnderlayA11y {
    pub attrs: UnderlayAttrs,
    pub handlers: UnderlayHandlers,
    pub state: UnderlayState,
}

pub fn use_underlay(options: UnderlayOptions) -> UnderlayA11y {
    let UnderlayOptions {
        state,
        on_close,
        lang,
        dir,
    } = options;
    let locale = locale_attrs(lang, dir);

    let is_open = Signal::derive(move || state.get().is_open);
    let is_disabled = Signal::derive(move || state.get().is_disabled);
    let is_interactive = Signal::derive(move || state.get().is_interactive);

    let on_click = Callback::new(move |_| {
        let state = state.get_untracked();
        if !state.is_interactive {
            return;
        }

        if let Some(on_close) = on_close.as_ref() {
            on_close.run(());
        }
    });

    UnderlayA11y {
        attrs: UnderlayAttrs {
            role: "presentation",
            aria_hidden: "true",
            tabindex: -1,
            lang: locale.lang,
            dir: locale.dir,
        },
        handlers: UnderlayHandlers { on_click },
        state: UnderlayState {
            is_open,
            is_disabled,
            is_interactive,
        },
    }
}

#[cfg(test)]
#[path = "test/underlay.rs"]
mod tests;
