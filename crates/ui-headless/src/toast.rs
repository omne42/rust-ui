use crate::a11y::{A11yDirection, LiveRegionPriority, live_region_attrs, locale_attrs};
use leptos::{ev, prelude::*};

#[derive(Clone)]
pub struct ToastA11yOptions {
    pub is_open: Signal<bool>,
    pub priority: LiveRegionPriority,
    pub lang: Option<String>,
    pub dir: Option<A11yDirection>,
    pub on_dismiss_request: Callback<()>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ToastA11yAttrs {
    pub role: &'static str,
    pub aria_live: &'static str,
    pub aria_atomic: &'static str,
    pub aria_keyshortcuts: &'static str,
    pub lang: Option<String>,
    pub dir: Option<&'static str>,
}

#[derive(Clone)]
pub struct ToastA11yHandlers {
    pub on_key_down: Callback<ev::KeyboardEvent>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ToastA11yState {
    pub priority: LiveRegionPriority,
    pub priority_attr: &'static str,
}

#[derive(Clone)]
pub struct ToastA11yContract {
    pub attrs: ToastA11yAttrs,
    pub handlers: ToastA11yHandlers,
    pub state: ToastA11yState,
}

pub fn toast_priority_attr(priority: LiveRegionPriority) -> &'static str {
    match priority {
        LiveRegionPriority::Polite => "polite",
        LiveRegionPriority::Assertive => "assertive",
    }
}

pub fn should_dismiss_toast_on_escape(
    key: &str,
    is_open: bool,
    is_composing: bool,
    default_prevented: bool,
) -> bool {
    key == "Escape" && is_open && !is_composing && !default_prevented
}

pub fn use_toast_a11y(options: ToastA11yOptions) -> ToastA11yContract {
    let locale = locale_attrs(options.lang, options.dir);
    let live_region = live_region_attrs(options.priority);

    let is_open = options.is_open;
    let on_dismiss_request = options.on_dismiss_request;
    let handlers = ToastA11yHandlers {
        on_key_down: Callback::new(move |ev: ev::KeyboardEvent| {
            #[cfg(target_arch = "wasm32")]
            let is_composing = ev.is_composing();
            #[cfg(not(target_arch = "wasm32"))]
            let is_composing = false;

            if !should_dismiss_toast_on_escape(
                &ev.key(),
                is_open.get_untracked(),
                is_composing,
                ev.default_prevented(),
            ) {
                return;
            }

            ev.stop_propagation();
            ev.prevent_default();
            on_dismiss_request.run(());
        }),
    };

    ToastA11yContract {
        attrs: ToastA11yAttrs {
            role: live_region.role,
            aria_live: live_region.aria_live,
            aria_atomic: "true",
            aria_keyshortcuts: "Escape",
            lang: locale.lang,
            dir: locale.dir,
        },
        handlers,
        state: ToastA11yState {
            priority: options.priority,
            priority_attr: toast_priority_attr(options.priority),
        },
    }
}

#[cfg(test)]
#[path = "test/toast.rs"]
mod tests;
