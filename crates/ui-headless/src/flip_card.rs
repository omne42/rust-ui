use crate::a11y::{A11yDirection, locale_attrs};
use crate::focus_ring::{FocusRingHandlers, FocusRingOptions, use_focus_ring};
use leptos::prelude::*;

#[derive(Clone)]
pub struct FlipCardOptions {
    pub is_disabled: bool,
    pub is_flipped: Signal<bool>,
    pub request_is_flipped_change: Callback<bool>,
    pub flip_on_hover: bool,
    pub lang: Option<String>,
    pub dir: Option<A11yDirection>,
}

#[derive(Clone)]
pub struct FlipCardHandlers {
    pub on_click: Callback<()>,
    pub on_key_down: Callback<(String, bool), bool>,
    pub on_pointer_enter: Callback<()>,
    pub on_pointer_leave: Callback<()>,
    pub on_focus: Callback<()>,
    pub on_blur: Callback<()>,
}

#[derive(Clone)]
pub struct FlipCardAttrs {
    pub role: &'static str,
    pub tabindex: i32,
    pub aria_pressed: Signal<bool>,
    pub aria_disabled: Option<&'static str>,
    pub lang: Option<String>,
    pub dir: Option<&'static str>,
}

#[derive(Clone)]
pub struct FlipCardState {
    pub is_flipped: Signal<bool>,
    pub is_hovered: Signal<bool>,
    pub is_focused: ReadSignal<bool>,
    pub is_focus_visible: Memo<bool>,
}

#[derive(Clone)]
pub struct FlipCardA11y {
    pub attrs: FlipCardAttrs,
    pub handlers: FlipCardHandlers,
    pub state: FlipCardState,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlipCardKeyDownResult {
    Ignored,
    ToggleAndPreventDefault,
}

fn is_space_key(key: &str) -> bool {
    matches!(key, " " | "Space" | "Spacebar")
}

pub fn resolve_flip_card_key_down(
    key: &str,
    is_disabled: bool,
    is_composing: bool,
) -> FlipCardKeyDownResult {
    if is_disabled || is_composing {
        return FlipCardKeyDownResult::Ignored;
    }

    if key == "Enter" || is_space_key(key) {
        FlipCardKeyDownResult::ToggleAndPreventDefault
    } else {
        FlipCardKeyDownResult::Ignored
    }
}

fn toggle(is_flipped: Signal<bool>, request_is_flipped_change: Callback<bool>, is_disabled: bool) {
    if is_disabled {
        return;
    }
    request_is_flipped_change.run(!is_flipped.get_untracked());
}

pub fn use_flip_card(options: FlipCardOptions) -> FlipCardA11y {
    let FlipCardOptions {
        is_disabled,
        is_flipped,
        request_is_flipped_change,
        flip_on_hover,
        lang,
        dir,
    } = options;

    let locale = locale_attrs(lang, dir);
    let (is_pointer_hovered, set_pointer_hovered) = signal(false);
    let focus_ring = use_focus_ring(FocusRingOptions { is_disabled });
    let is_hovered =
        Signal::derive(move || is_pointer_hovered.get() || focus_ring.is_focused.get());

    let on_click =
        { Callback::new(move |_| toggle(is_flipped, request_is_flipped_change, is_disabled)) };

    let on_key_down = {
        Callback::new(move |(key, is_composing): (String, bool)| {
            if resolve_flip_card_key_down(&key, is_disabled, is_composing)
                == FlipCardKeyDownResult::ToggleAndPreventDefault
            {
                toggle(is_flipped, request_is_flipped_change, is_disabled);
                true
            } else {
                false
            }
        })
    };

    let on_pointer_enter = {
        Callback::new(move |_| {
            if is_disabled {
                return;
            }

            set_pointer_hovered.set(true);
            if flip_on_hover {
                request_is_flipped_change.run(true);
            }
        })
    };

    let on_pointer_leave = {
        Callback::new(move |_| {
            set_pointer_hovered.set(false);
            if flip_on_hover {
                request_is_flipped_change.run(false);
            }
        })
    };

    let focus_ring_handlers: FocusRingHandlers = focus_ring.handlers.clone();

    FlipCardA11y {
        attrs: FlipCardAttrs {
            role: "button",
            tabindex: if is_disabled { -1 } else { 0 },
            aria_pressed: is_flipped,
            aria_disabled: is_disabled.then_some("true"),
            lang: locale.lang,
            dir: locale.dir,
        },
        handlers: FlipCardHandlers {
            on_click,
            on_key_down,
            on_pointer_enter,
            on_pointer_leave,
            on_focus: focus_ring_handlers.on_focus,
            on_blur: focus_ring_handlers.on_blur,
        },
        state: FlipCardState {
            is_flipped,
            is_hovered,
            is_focused: focus_ring.is_focused,
            is_focus_visible: focus_ring.is_focus_visible,
        },
    }
}

#[cfg(test)]
#[path = "test/flip_card.rs"]
mod tests;
