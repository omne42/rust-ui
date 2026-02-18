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
mod tests {
    use super::*;
    use std::sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    };

    fn interactive_state() -> UnderlayPartState {
        UnderlayPartState {
            slot: ui_state_primitives::underlay::UnderlaySlot::Root,
            slot_attr: "underlay",
            base_class: "ui-underlay",
            state_attr: "open",
            tone_attr: "scrim",
            close_mode_attr: "interactive",
            open_attr: Some("true"),
            transparent_attr: None,
            disabled_attr: None,
            interactive_attr: Some("true"),
            is_open: true,
            is_transparent: false,
            is_disabled: false,
            is_interactive: true,
            has_custom_transparent: false,
            has_custom_disabled: false,
            has_custom_close_handler: true,
            has_custom_class_name: false,
            transparent_source_attr: "default",
            disabled_source_attr: "default",
            close_source_attr: "custom",
            class_source_attr: "default",
        }
    }

    #[test]
    fn underlay_click_calls_on_close_when_interactive() {
        let (state_signal, _set_state_signal) = signal(interactive_state());

        let called = Arc::new(AtomicUsize::new(0));
        let called2 = Arc::clone(&called);

        let underlay = use_underlay(UnderlayOptions {
            state: state_signal.into(),
            on_close: Some(Callback::new(move |_| {
                called2.fetch_add(1, Ordering::SeqCst);
            })),
            lang: Some("  zh-CN ".to_string()),
            dir: Some(A11yDirection::Rtl),
        });

        assert_eq!(underlay.attrs.role, "presentation");
        assert_eq!(underlay.attrs.aria_hidden, "true");
        assert_eq!(underlay.attrs.tabindex, -1);
        assert_eq!(underlay.attrs.lang.as_deref(), Some("zh-CN"));
        assert_eq!(underlay.attrs.dir, Some("rtl"));

        assert!(underlay.state.is_open.get_untracked());
        assert!(underlay.state.is_interactive.get_untracked());
        assert!(!underlay.state.is_disabled.get_untracked());

        underlay.handlers.on_click.run(());
        assert_eq!(called.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn underlay_click_is_ignored_when_not_interactive() {
        let mut state = interactive_state();
        state.is_interactive = false;
        state.close_mode_attr = "static";
        state.interactive_attr = None;
        state.has_custom_close_handler = false;
        state.close_source_attr = "default";
        state.state_attr = "disabled";
        state.is_disabled = true;

        let (state_signal, _set_state_signal) = signal(state);

        let called = Arc::new(AtomicUsize::new(0));
        let called2 = Arc::clone(&called);

        let underlay = use_underlay(UnderlayOptions {
            state: state_signal.into(),
            on_close: Some(Callback::new(move |_| {
                called2.fetch_add(1, Ordering::SeqCst);
            })),
            lang: None,
            dir: None,
        });

        underlay.handlers.on_click.run(());
        assert_eq!(called.load(Ordering::SeqCst), 0);
        assert!(underlay.state.is_disabled.get_untracked());
        assert!(!underlay.state.is_interactive.get_untracked());
    }
}
