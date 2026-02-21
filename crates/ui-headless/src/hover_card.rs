use leptos::{ev, prelude::*};
use ui_state_primitives::hover_card::{
    DEFAULT_CLOSE_DELAY_MS as DEFAULT_HOVER_CARD_CLOSE_DELAY_MS,
    DEFAULT_OPEN_DELAY_MS as DEFAULT_HOVER_CARD_OPEN_DELAY_MS,
};

#[cfg(all(feature = "web", target_arch = "wasm32"))]
use std::time::Duration;

const DEFAULT_OPEN_DELAY_MS: u64 = DEFAULT_HOVER_CARD_OPEN_DELAY_MS;
const DEFAULT_CLOSE_DELAY_MS: u64 = DEFAULT_HOVER_CARD_CLOSE_DELAY_MS;

#[cfg(all(feature = "web", target_arch = "wasm32"))]
type HoverCardTimeoutHandle = TimeoutHandle;

#[cfg(all(test, not(target_arch = "wasm32")))]
type HoverCardTimeoutHandle = test_timers::TestTimeoutHandle;

#[derive(Clone)]
pub struct HoverCardTriggerOptions {
    pub is_disabled: bool,
    pub open_delay_ms: u64,
    pub close_delay_ms: u64,
    pub open: Option<Signal<bool>>,
    pub default_open: Option<bool>,
    pub on_open_change: Option<Callback<bool>>,
}

impl Default for HoverCardTriggerOptions {
    fn default() -> Self {
        Self {
            is_disabled: false,
            open_delay_ms: DEFAULT_OPEN_DELAY_MS,
            close_delay_ms: DEFAULT_CLOSE_DELAY_MS,
            open: None,
            default_open: None,
            on_open_change: None,
        }
    }
}

#[derive(Clone)]
pub struct HoverCardTriggerHandlers {
    pub on_trigger_pointer_enter: Callback<()>,
    pub on_trigger_pointer_leave: Callback<()>,
    pub on_trigger_focus_in: Callback<()>,
    pub on_trigger_focus_out: Callback<()>,
    pub on_panel_pointer_enter: Callback<()>,
    pub on_panel_pointer_leave: Callback<()>,
    pub on_panel_focus_in: Callback<()>,
    pub on_panel_focus_out: Callback<()>,
}

#[derive(Clone)]
pub struct HoverCardTriggerState {
    pub is_open: Signal<bool>,
    pub dismiss: Callback<()>,
}

#[derive(Clone)]
pub struct HoverCardTriggerAria {
    pub state: HoverCardTriggerState,
    pub handlers: HoverCardTriggerHandlers,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HoverCardDismissAttrs {
    pub aria_keyshortcuts: &'static str,
}

#[derive(Clone)]
pub struct HoverCardDismissHandlers {
    pub on_key_down: Callback<ev::KeyboardEvent>,
}

#[derive(Clone)]
pub struct HoverCardDismissState {
    pub is_open: Signal<bool>,
}

#[derive(Clone)]
pub struct HoverCardDismissA11y {
    pub attrs: HoverCardDismissAttrs,
    pub handlers: HoverCardDismissHandlers,
    pub state: HoverCardDismissState,
}

#[derive(Clone)]
pub struct HoverCardDismissOptions {
    pub is_open: Signal<bool>,
    pub dismiss: Callback<()>,
}

pub fn should_dismiss_on_escape(key: &str, is_open: bool, is_composing: bool) -> bool {
    key == "Escape" && is_open && !is_composing
}

pub fn use_hover_card_dismiss(options: HoverCardDismissOptions) -> HoverCardDismissA11y {
    let HoverCardDismissOptions { is_open, dismiss } = options;

    HoverCardDismissA11y {
        attrs: HoverCardDismissAttrs {
            aria_keyshortcuts: "Escape",
        },
        state: HoverCardDismissState { is_open },
        handlers: HoverCardDismissHandlers {
            on_key_down: Callback::new(move |ev: ev::KeyboardEvent| {
                #[cfg(target_arch = "wasm32")]
                let is_composing = ev.is_composing();
                #[cfg(not(target_arch = "wasm32"))]
                let is_composing = false;

                if !should_dismiss_on_escape(&ev.key(), is_open.get_untracked(), is_composing) {
                    return;
                }

                ev.stop_propagation();
                ev.prevent_default();
                dismiss.run(());
            }),
        },
    }
}

#[derive(Clone)]
pub struct HoverCardFocusA11yOptions {
    pub hover_card_id: StoredValue<String>,
    pub is_open: Signal<bool>,
    pub on_focus_in: Callback<()>,
    pub on_focus_out: Callback<()>,
}

#[derive(Clone)]
pub struct HoverCardFocusA11yAttrs {
    pub manages_aria_describedby: bool,
}

#[derive(Clone)]
pub struct HoverCardFocusA11yHandlers {
    pub on_focus_in: Callback<ev::FocusEvent>,
    pub on_focus_out: Callback<ev::FocusEvent>,
}

#[derive(Clone)]
pub struct HoverCardFocusA11yState {
    pub is_open: Signal<bool>,
}

#[derive(Clone)]
pub struct HoverCardFocusA11y {
    pub attrs: HoverCardFocusA11yAttrs,
    pub handlers: HoverCardFocusA11yHandlers,
    pub state: HoverCardFocusA11yState,
}

pub fn use_hover_card_focus_a11y(options: HoverCardFocusA11yOptions) -> HoverCardFocusA11y {
    let HoverCardFocusA11yOptions {
        hover_card_id,
        is_open,
        on_focus_in,
        on_focus_out,
    } = options;

    #[cfg(not(target_arch = "wasm32"))]
    let _unused_focus_a11y = (&hover_card_id, is_open);
    #[cfg(target_arch = "wasm32")]
    let focus_target = StoredValue::new_local(None::<leptos::web_sys::Element>);

    #[cfg(target_arch = "wasm32")]
    on_cleanup(move || {
        if let Some(target) = focus_target.get_value() {
            drop(target.remove_attribute("aria-describedby"));
        }
    });

    #[cfg(target_arch = "wasm32")]
    Effect::new(move |_| {
        let open_now = is_open.get();
        let Some(target) = focus_target.get_value() else {
            return;
        };

        let id = hover_card_id.with_value(|id| id.clone());
        if open_now {
            drop(target.set_attribute("aria-describedby", &id));
        } else {
            drop(target.remove_attribute("aria-describedby"));
        }
    });

    let on_focus_in = Callback::new(move |ev: ev::FocusEvent| {
        on_focus_in.run(());

        #[cfg(not(target_arch = "wasm32"))]
        let _unused_event = &ev;
        #[cfg(target_arch = "wasm32")]
        {
            use leptos::wasm_bindgen::JsCast;

            if let Some(target) = focus_target.get_value() {
                drop(target.remove_attribute("aria-describedby"));
            }

            let Some(target) = ev.target() else {
                focus_target.set_value(None);
                return;
            };

            let Ok(target) = target.dyn_into::<leptos::web_sys::Element>() else {
                focus_target.set_value(None);
                return;
            };

            if is_open.get_untracked() {
                let id = hover_card_id.with_value(|id| id.clone());
                drop(target.set_attribute("aria-describedby", &id));
            }

            focus_target.set_value(Some(target));
        }
    });

    let on_focus_out = Callback::new(move |ev: ev::FocusEvent| {
        #[cfg(not(target_arch = "wasm32"))]
        let _unused_event = &ev;
        #[cfg(target_arch = "wasm32")]
        {
            if let Some(target) = focus_target.get_value() {
                drop(target.remove_attribute("aria-describedby"));
            }
            focus_target.set_value(None);
        }

        on_focus_out.run(());
    });

    HoverCardFocusA11y {
        attrs: HoverCardFocusA11yAttrs {
            manages_aria_describedby: true,
        },
        state: HoverCardFocusA11yState { is_open },
        handlers: HoverCardFocusA11yHandlers {
            on_focus_in,
            on_focus_out,
        },
    }
}

pub fn use_hover_card_trigger(options: HoverCardTriggerOptions) -> HoverCardTriggerAria {
    let HoverCardTriggerOptions {
        is_disabled,
        open_delay_ms,
        close_delay_ms,
        open,
        default_open,
        on_open_change,
    } = options;

    let open_state =
        crate::use_controllable_open_state_traced("hover_card", open, default_open, on_open_change);

    let trigger_hover = crate::use_hover(crate::HoverOptions { is_disabled });
    let panel_hover = crate::use_hover(crate::HoverOptions { is_disabled });
    let trigger_focus = crate::use_focus_within(crate::FocusWithinOptions { is_disabled });
    let panel_focus = crate::use_focus_within(crate::FocusWithinOptions { is_disabled });

    let global_focus_visible = crate::use_focus_visible()
        .map(|state| state.is_focus_visible())
        .unwrap_or_else(|| signal(false).0);
    let on_trigger_focus_in = trigger_focus.handlers.on_focus_in;
    let on_trigger_focus_out = trigger_focus.handlers.on_focus_out;

    let is_open = open_state.open;
    let request_open_change = open_state.request_open_change;
    let (is_dismissed, set_dismissed) = signal(false);
    let timers = HoverCardTimers::new();

    let wants_open = Signal::derive(move || {
        trigger_hover.is_hovered.get()
            || panel_hover.is_hovered.get()
            || trigger_focus.is_focus_within.get()
            || panel_focus.is_focus_within.get()
    });

    let dismiss = {
        let timers = timers.clone();
        Callback::new(move |_| {
            set_dismissed.set(true);
            timers.clear();
            request_open_change.run(false);
        })
    };

    let timers_for_effect = timers.clone();
    let request_open_change_for_effect = request_open_change;
    Effect::new(move |_| {
        if is_disabled {
            timers_for_effect.clear();
            set_dismissed.set(false);
            request_open_change_for_effect.run(false);
            return;
        }

        let intent_open = wants_open.get();

        if !intent_open {
            set_dismissed.set(false);
        }

        if is_dismissed.get_untracked() && intent_open {
            timers_for_effect.clear();
            request_open_change_for_effect.run(false);
            return;
        }

        if intent_open {
            if is_open.get_untracked() {
                timers_for_effect.clear();
                return;
            }
            timers_for_effect.open(open_delay_ms, request_open_change_for_effect);
            return;
        }

        if !is_open.get_untracked() {
            timers_for_effect.clear();
            return;
        }

        timers_for_effect.close(close_delay_ms, request_open_change_for_effect);
    });

    on_cleanup({
        let timers = timers.clone();
        move || timers.clear()
    });

    HoverCardTriggerAria {
        state: HoverCardTriggerState { is_open, dismiss },
        handlers: HoverCardTriggerHandlers {
            on_trigger_pointer_enter: trigger_hover.handlers.on_pointer_enter,
            on_trigger_pointer_leave: trigger_hover.handlers.on_pointer_leave,
            on_trigger_focus_in: Callback::new(move |_| {
                if !global_focus_visible.get_untracked() {
                    return;
                }
                on_trigger_focus_in.run(());
            }),
            on_trigger_focus_out,
            on_panel_pointer_enter: panel_hover.handlers.on_pointer_enter,
            on_panel_pointer_leave: panel_hover.handlers.on_pointer_leave,
            on_panel_focus_in: panel_focus.handlers.on_focus_in,
            on_panel_focus_out: panel_focus.handlers.on_focus_out,
        },
    }
}

#[derive(Clone)]
struct HoverCardTimers {
    #[cfg(any(
        all(feature = "web", target_arch = "wasm32"),
        all(test, not(target_arch = "wasm32"))
    ))]
    open_handle: StoredValue<Option<HoverCardTimeoutHandle>, LocalStorage>,
    #[cfg(any(
        all(feature = "web", target_arch = "wasm32"),
        all(test, not(target_arch = "wasm32"))
    ))]
    close_handle: StoredValue<Option<HoverCardTimeoutHandle>, LocalStorage>,
}

impl HoverCardTimers {
    fn new() -> Self {
        #[cfg(any(
            all(feature = "web", target_arch = "wasm32"),
            all(test, not(target_arch = "wasm32"))
        ))]
        {
            Self {
                open_handle: StoredValue::new_local(None),
                close_handle: StoredValue::new_local(None),
            }
        }

        #[cfg(not(any(
            all(feature = "web", target_arch = "wasm32"),
            all(test, not(target_arch = "wasm32"))
        )))]
        {
            Self {}
        }
    }

    fn clear(&self) {
        self.clear_open();
        self.clear_close();
    }

    fn clear_open(&self) {
        #[cfg(any(
            all(feature = "web", target_arch = "wasm32"),
            all(test, not(target_arch = "wasm32"))
        ))]
        {
            if let Some(handle) = self.open_handle.get_value() {
                handle.clear();
            }
            self.open_handle.set_value(None);
        }
    }

    fn clear_close(&self) {
        #[cfg(any(
            all(feature = "web", target_arch = "wasm32"),
            all(test, not(target_arch = "wasm32"))
        ))]
        {
            if let Some(handle) = self.close_handle.get_value() {
                handle.clear();
            }
            self.close_handle.set_value(None);
        }
    }

    fn open(&self, delay_ms: u64, request_open_change: Callback<bool>) {
        self.clear_close();

        #[cfg(all(feature = "web", target_arch = "wasm32"))]
        {
            if delay_ms == 0 {
                self.clear_open();
                request_open_change.run(true);
                return;
            }

            if self.open_handle.get_value().is_some() {
                return;
            }

            let open_handle = self.open_handle;
            let Ok(handle) = set_timeout_with_handle(
                move || {
                    open_handle.set_value(None);
                    request_open_change.run(true);
                },
                Duration::from_millis(delay_ms),
            ) else {
                request_open_change.run(true);
                return;
            };

            self.open_handle.set_value(Some(handle));
        }

        #[cfg(all(test, not(target_arch = "wasm32")))]
        {
            if delay_ms == 0 {
                self.clear_open();
                request_open_change.run(true);
                return;
            }

            if self.open_handle.get_value().is_some() {
                return;
            }

            let open_handle = self.open_handle;
            let handle = test_timers::set_timeout(delay_ms, move || {
                open_handle.set_value(None);
                request_open_change.run(true);
            });
            self.open_handle.set_value(Some(handle));
        }

        #[cfg(not(any(
            all(feature = "web", target_arch = "wasm32"),
            all(test, not(target_arch = "wasm32"))
        )))]
        {
            let _unused_delay_ms = delay_ms;
            request_open_change.run(true);
        }
    }

    fn close(&self, delay_ms: u64, request_open_change: Callback<bool>) {
        self.clear_open();

        #[cfg(all(feature = "web", target_arch = "wasm32"))]
        {
            if delay_ms == 0 {
                self.clear_close();
                request_open_change.run(false);
                return;
            }

            if self.close_handle.get_value().is_some() {
                return;
            }

            let close_handle = self.close_handle;
            let Ok(handle) = set_timeout_with_handle(
                move || {
                    close_handle.set_value(None);
                    request_open_change.run(false);
                },
                Duration::from_millis(delay_ms),
            ) else {
                request_open_change.run(false);
                return;
            };

            self.close_handle.set_value(Some(handle));
        }

        #[cfg(all(test, not(target_arch = "wasm32")))]
        {
            if delay_ms == 0 {
                self.clear_close();
                request_open_change.run(false);
                return;
            }

            if self.close_handle.get_value().is_some() {
                return;
            }

            let close_handle = self.close_handle;
            let handle = test_timers::set_timeout(delay_ms, move || {
                close_handle.set_value(None);
                request_open_change.run(false);
            });
            self.close_handle.set_value(Some(handle));
        }

        #[cfg(not(any(
            all(feature = "web", target_arch = "wasm32"),
            all(test, not(target_arch = "wasm32"))
        )))]
        {
            let _unused_delay_ms = delay_ms;
            request_open_change.run(false);
        }
    }
}

impl Default for HoverCardTimers {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
#[path = "test/hover_card_timers.rs"]
mod test_timers;

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests;
