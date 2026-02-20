use leptos::{ev, html, prelude::*};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

#[cfg(all(feature = "web", target_arch = "wasm32"))]
use std::time::Duration;

const DEFAULT_DELAY_MS: u64 = 1500;
const DEFAULT_CLOSE_DELAY_MS: u64 = 500;
const TOOLTIP_COOLDOWN_MS: u64 = 500;

#[cfg(all(feature = "web", target_arch = "wasm32"))]
type TooltipTimeoutHandle = TimeoutHandle;

#[cfg(all(test, not(target_arch = "wasm32")))]
type TooltipTimeoutHandle = test_timers::TestTimeoutHandle;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum TooltipTriggerMode {
    /// Open on hover and on focus-visible.
    #[default]
    Hover,
    /// Open only on focus-visible.
    Focus,
}

#[derive(Clone)]
pub struct TooltipTriggerOptions {
    pub is_disabled: bool,
    pub delay_ms: u64,
    pub close_delay_ms: u64,
    pub trigger: TooltipTriggerMode,
    pub should_close_on_press: bool,
    pub open: Option<Signal<bool>>,
    pub default_open: Option<bool>,
    pub on_open_change: Option<Callback<bool>>,
}

impl Default for TooltipTriggerOptions {
    fn default() -> Self {
        Self {
            is_disabled: false,
            delay_ms: DEFAULT_DELAY_MS,
            close_delay_ms: DEFAULT_CLOSE_DELAY_MS,
            trigger: TooltipTriggerMode::Hover,
            should_close_on_press: true,
            open: None,
            default_open: None,
            on_open_change: None,
        }
    }
}

#[derive(Clone)]
pub struct TooltipTriggerHandlers {
    pub on_pointer_enter: Callback<()>,
    pub on_pointer_leave: Callback<()>,
    pub on_focus: Callback<()>,
    pub on_blur: Callback<()>,
    pub on_pointer_down: Callback<()>,
    pub on_key_down: Callback<String>,
}

#[derive(Clone)]
pub struct TooltipTriggerState {
    id: String,
    delay_ms: u64,
    close_delay_ms: u64,
    open: Signal<bool>,
    request_open_change: Callback<bool>,
    timers: TooltipTimers,
}

impl TooltipTriggerState {
    pub fn is_open(&self) -> Signal<bool> {
        self.open
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn open(&self, immediate: bool) {
        if immediate || self.delay_ms == 0 || self.timers.is_close_pending() {
            self.show_tooltip();
            return;
        }

        let others = TOOLTIP_GLOBAL.with(|global| global.borrow_mut().take_other_hides(&self.id));
        for hide in others {
            hide(true);
        }

        let should_show_immediately = TOOLTIP_GLOBAL.with(|global| {
            let mut global = global.borrow_mut();
            global.ensure_entry(self.id.clone(), self.hide_fn());

            if self.open.get_untracked() {
                return false;
            }

            if global.warmup_pending() || global.warmed_up {
                return true;
            }

            global.schedule_warmup(self.id.clone(), self.delay_ms, {
                let state = self.clone();
                move || {
                    TOOLTIP_GLOBAL.with(|global| global.borrow_mut().warmed_up = true);
                    state.show_tooltip();
                }
            });

            false
        });

        if should_show_immediately {
            self.show_tooltip();
        }
    }

    pub fn close(&self, immediate: bool) {
        let close_delay_ms = self.close_delay_ms;
        let id = self.id.clone();

        TOOLTIP_GLOBAL.with(|global| {
            let mut global = global.borrow_mut();
            global.clear_warmup();

            if !global.warmed_up {
                return;
            }

            global.clear_cooldown();
            global.schedule_cooldown(TOOLTIP_COOLDOWN_MS.max(close_delay_ms), move || {
                TOOLTIP_GLOBAL.with(|global| {
                    let mut global = global.borrow_mut();
                    global.tooltips.remove(&id);
                    global.warmed_up = false;
                    global.clear_cooldown();
                });
            });
        });

        self.timers
            .close(immediate, close_delay_ms, self.request_open_change);
    }

    fn show_tooltip(&self) {
        let others = TOOLTIP_GLOBAL.with(|global| {
            let mut global = global.borrow_mut();
            global.clear_warmup();
            global.clear_cooldown();
            global.warmed_up = true;

            global.take_other_hides(&self.id)
        });

        for hide in others {
            hide(true);
        }

        TOOLTIP_GLOBAL.with(|global| {
            let mut global = global.borrow_mut();
            global.ensure_entry(self.id.clone(), self.hide_fn());
        });

        self.timers.clear_close();
        self.request_open_change.run(true);
    }

    fn hide_fn(&self) -> Rc<dyn Fn(bool)> {
        let state = self.clone();
        Rc::new(move |immediate| state.close(immediate))
    }
}

#[derive(Clone)]
pub struct TooltipTriggerAria {
    pub state: TooltipTriggerState,
    pub handlers: TooltipTriggerHandlers,
}

#[derive(Clone)]
pub struct TooltipFocusHandlers {
    pub on_focus_in: Callback<ev::FocusEvent>,
    pub on_focus_out: Callback<ev::FocusEvent>,
}

#[derive(Clone)]
pub struct TooltipFocusA11yOptions {
    pub anchor_ref: NodeRef<html::Span>,
    pub tooltip_id: StoredValue<String>,
    pub is_open: Signal<bool>,
    pub on_focus: Callback<()>,
    pub on_blur: Callback<()>,
}

pub fn use_tooltip_focus_a11y(options: TooltipFocusA11yOptions) -> TooltipFocusHandlers {
    let TooltipFocusA11yOptions {
        anchor_ref,
        tooltip_id,
        is_open,
        on_focus,
        on_blur,
    } = options;

    #[cfg(not(target_arch = "wasm32"))]
    let _unused_focus_a11y = (&anchor_ref, &tooltip_id, is_open);
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

        let id = tooltip_id.with_value(|id| id.clone());
        if open_now {
            drop(target.set_attribute("aria-describedby", &id));
        } else {
            drop(target.remove_attribute("aria-describedby"));
        }
    });

    let on_focus_in = Callback::new(move |ev: ev::FocusEvent| {
        on_focus.run(());

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
                let id = tooltip_id.with_value(|id| id.clone());
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
            use leptos::wasm_bindgen::JsCast;

            if let Some(target) = focus_target.get_value() {
                drop(target.remove_attribute("aria-describedby"));
            }
            focus_target.set_value(None);

            let leaving = match anchor_ref.get_untracked() {
                Some(anchor) => {
                    let anchor_el: leptos::web_sys::Element = anchor.unchecked_into();
                    match ev.related_target() {
                        Some(related) => match related.dyn_into::<leptos::web_sys::Node>() {
                            Ok(node) => !anchor_el.contains(Some(&node)),
                            Err(_) => true,
                        },
                        None => true,
                    }
                }
                None => true,
            };

            if !leaving {
                return;
            }
        }

        on_blur.run(());
    });

    TooltipFocusHandlers {
        on_focus_in,
        on_focus_out,
    }
}

pub fn use_tooltip_trigger(
    id: Option<String>,
    options: TooltipTriggerOptions,
) -> TooltipTriggerAria {
    let TooltipTriggerOptions {
        is_disabled,
        delay_ms,
        close_delay_ms,
        trigger,
        should_close_on_press,
        open,
        default_open,
        on_open_change,
    } = options;

    let id = id.unwrap_or_else(next_tooltip_id);
    let open_state =
        crate::use_controllable_open_state_traced("tooltip", open, default_open, on_open_change);
    let timers = TooltipTimers::new();
    let state = TooltipTriggerState {
        id,
        delay_ms,
        close_delay_ms,
        open: open_state.open,
        request_open_change: open_state.request_open_change,
        timers,
    };

    let (is_focused, set_focused) = signal(false);

    let global_focus_visible = crate::use_focus_visible()
        .map(|state| state.is_focus_visible())
        .unwrap_or_else(|| signal(false).0);

    let on_pointer_enter = {
        let state = state.clone();
        Callback::new(move |_| {
            if is_disabled || matches!(trigger, TooltipTriggerMode::Focus) {
                return;
            }
            state.open(is_focused.get_untracked());
        })
    };

    let on_pointer_leave = {
        let state = state.clone();
        Callback::new(move |_| {
            if is_disabled || matches!(trigger, TooltipTriggerMode::Focus) {
                return;
            }
            set_focused.set(false);
            state.close(false);
        })
    };

    let on_focus = {
        let state = state.clone();
        Callback::new(move |_| {
            if is_disabled {
                return;
            }
            if !global_focus_visible.get_untracked() {
                return;
            }
            set_focused.set(true);
            state.open(true);
        })
    };

    let on_blur = {
        let state = state.clone();
        Callback::new(move |_| {
            set_focused.set(false);
            state.close(true);
        })
    };

    let on_press_start = {
        let state = state.clone();
        Callback::new(move |_| {
            if !should_close_on_press {
                return;
            }
            set_focused.set(false);
            state.close(true);
        })
    };

    let on_pointer_down = on_press_start;

    let on_key_down = {
        Callback::new(move |key: String| {
            drop(key);
            on_press_start.run(());
        })
    };

    #[cfg(all(feature = "web", target_arch = "wasm32"))]
    attach_escape_listener(state.clone());

    on_cleanup({
        let id = state.id.clone();
        let timers = state.timers.clone();
        move || {
            timers.clear_close();
            TOOLTIP_GLOBAL.with(|global| {
                let mut global = global.borrow_mut();
                global.tooltips.remove(&id);
                #[cfg(any(
                    all(feature = "web", target_arch = "wasm32"),
                    all(test, not(target_arch = "wasm32"))
                ))]
                {
                    if global.warmup_owner.as_deref() == Some(id.as_str()) {
                        global.clear_warmup();
                    }
                }
            });
        }
    });

    TooltipTriggerAria {
        state,
        handlers: TooltipTriggerHandlers {
            on_pointer_enter,
            on_pointer_leave,
            on_focus,
            on_blur,
            on_pointer_down,
            on_key_down,
        },
    }
}

#[cfg(all(feature = "web", target_arch = "wasm32"))]
fn attach_escape_listener(state: TooltipTriggerState) {
    use send_wrapper::SendWrapper;
    use wasm_bindgen::JsCast;
    use wasm_bindgen::closure::Closure;

    Effect::new(move |_| {
        if !state.open.get() {
            return;
        }

        let Some(window) = web_sys::window() else {
            return;
        };
        let Some(document) = window.document() else {
            return;
        };

        let target: SendWrapper<web_sys::EventTarget> = SendWrapper::new(document.into());

        let keydown: SendWrapper<Closure<dyn FnMut(web_sys::KeyboardEvent)>> = SendWrapper::new({
            let state = state.clone();
            Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
                if event.key() == "Escape" {
                    event.stop_propagation();
                    state.close(true);
                }
            }) as Box<dyn FnMut(_)>)
        });

        drop(target.add_event_listener_with_callback_and_bool(
            "keydown",
            keydown.as_ref().unchecked_ref(),
            true,
        ));

        on_cleanup(move || {
            drop(target.remove_event_listener_with_callback_and_bool(
                "keydown",
                keydown.as_ref().unchecked_ref(),
                true,
            ));
        });
    });
}

fn next_tooltip_id() -> String {
    TOOLTIP_GLOBAL.with(|global| global.borrow_mut().alloc_id())
}

thread_local! {
    static TOOLTIP_GLOBAL: RefCell<TooltipGlobal> = RefCell::new(TooltipGlobal::new());
}

struct TooltipGlobal {
    next_id: u64,
    warmed_up: bool,
    tooltips: HashMap<String, Rc<dyn Fn(bool)>>,
    #[cfg(any(
        all(feature = "web", target_arch = "wasm32"),
        all(test, not(target_arch = "wasm32"))
    ))]
    warmup_handle: Option<TooltipTimeoutHandle>,
    #[cfg(any(
        all(feature = "web", target_arch = "wasm32"),
        all(test, not(target_arch = "wasm32"))
    ))]
    cooldown_handle: Option<TooltipTimeoutHandle>,
    #[cfg(any(
        all(feature = "web", target_arch = "wasm32"),
        all(test, not(target_arch = "wasm32"))
    ))]
    warmup_owner: Option<String>,
}

impl TooltipGlobal {
    fn new() -> Self {
        Self {
            next_id: 1,
            warmed_up: false,
            tooltips: HashMap::new(),
            #[cfg(any(
                all(feature = "web", target_arch = "wasm32"),
                all(test, not(target_arch = "wasm32"))
            ))]
            warmup_handle: None,
            #[cfg(any(
                all(feature = "web", target_arch = "wasm32"),
                all(test, not(target_arch = "wasm32"))
            ))]
            cooldown_handle: None,
            #[cfg(any(
                all(feature = "web", target_arch = "wasm32"),
                all(test, not(target_arch = "wasm32"))
            ))]
            warmup_owner: None,
        }
    }

    fn alloc_id(&mut self) -> String {
        let id = self.next_id;
        self.next_id = self.next_id.saturating_add(1);
        format!("ui-tooltip-{id}")
    }

    fn ensure_entry(&mut self, id: String, hide: Rc<dyn Fn(bool)>) {
        self.tooltips.insert(id, hide);
    }

    fn take_other_hides(&mut self, id: &str) -> Vec<Rc<dyn Fn(bool)>> {
        let mut hides = Vec::new();
        let mut keys = Vec::new();

        for (key, hide) in &self.tooltips {
            if key == id {
                continue;
            }
            hides.push(hide.clone());
            keys.push(key.clone());
        }

        for key in keys {
            self.tooltips.remove(&key);
        }

        hides
    }

    fn warmup_pending(&self) -> bool {
        #[cfg(any(
            all(feature = "web", target_arch = "wasm32"),
            all(test, not(target_arch = "wasm32"))
        ))]
        {
            self.warmup_handle.is_some()
        }
        #[cfg(not(any(
            all(feature = "web", target_arch = "wasm32"),
            all(test, not(target_arch = "wasm32"))
        )))]
        {
            false
        }
    }

    fn clear_warmup(&mut self) {
        #[cfg(any(
            all(feature = "web", target_arch = "wasm32"),
            all(test, not(target_arch = "wasm32"))
        ))]
        {
            if let Some(handle) = self.warmup_handle.take() {
                handle.clear();
            }
            self.warmup_owner = None;
        }
    }

    fn clear_cooldown(&mut self) {
        #[cfg(any(
            all(feature = "web", target_arch = "wasm32"),
            all(test, not(target_arch = "wasm32"))
        ))]
        {
            if let Some(handle) = self.cooldown_handle.take() {
                handle.clear();
            }
        }
    }

    fn schedule_warmup(&mut self, owner: String, delay_ms: u64, callback: impl FnOnce() + 'static) {
        #[cfg(all(feature = "web", target_arch = "wasm32"))]
        {
            if self.warmup_handle.is_some() {
                return;
            }

            let Ok(handle) = set_timeout_with_handle(callback, Duration::from_millis(delay_ms))
            else {
                return;
            };
            self.warmup_owner = Some(owner);
            self.warmup_handle = Some(handle);
        }

        #[cfg(all(test, not(target_arch = "wasm32")))]
        {
            if self.warmup_handle.is_some() {
                return;
            }

            self.warmup_owner = Some(owner);
            self.warmup_handle = Some(test_timers::set_timeout(delay_ms, callback));
        }

        #[cfg(not(any(
            all(feature = "web", target_arch = "wasm32"),
            all(test, not(target_arch = "wasm32"))
        )))]
        {
            drop((delay_ms, owner));
            callback();
        }
    }

    fn schedule_cooldown(&mut self, delay_ms: u64, callback: impl FnOnce() + 'static) {
        #[cfg(all(feature = "web", target_arch = "wasm32"))]
        {
            let Ok(handle) = set_timeout_with_handle(callback, Duration::from_millis(delay_ms))
            else {
                return;
            };
            self.cooldown_handle = Some(handle);
        }

        #[cfg(all(test, not(target_arch = "wasm32")))]
        {
            self.cooldown_handle = Some(test_timers::set_timeout(delay_ms, callback));
        }

        #[cfg(not(any(
            all(feature = "web", target_arch = "wasm32"),
            all(test, not(target_arch = "wasm32"))
        )))]
        {
            let _unused_delay_ms = delay_ms;
            callback();
        }
    }
}

#[derive(Clone)]
struct TooltipTimers {
    #[cfg(any(
        all(feature = "web", target_arch = "wasm32"),
        all(test, not(target_arch = "wasm32"))
    ))]
    close_handle: StoredValue<Option<TooltipTimeoutHandle>, LocalStorage>,
}

impl TooltipTimers {
    fn new() -> Self {
        #[cfg(any(
            all(feature = "web", target_arch = "wasm32"),
            all(test, not(target_arch = "wasm32"))
        ))]
        {
            Self {
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

    fn is_close_pending(&self) -> bool {
        #[cfg(any(
            all(feature = "web", target_arch = "wasm32"),
            all(test, not(target_arch = "wasm32"))
        ))]
        {
            self.close_handle.get_value().is_some()
        }
        #[cfg(not(any(
            all(feature = "web", target_arch = "wasm32"),
            all(test, not(target_arch = "wasm32"))
        )))]
        {
            false
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

    fn close(&self, immediate: bool, close_delay_ms: u64, request_open_change: Callback<bool>) {
        #[cfg(all(feature = "web", target_arch = "wasm32"))]
        {
            if immediate || close_delay_ms == 0 {
                self.clear_close();
                request_open_change.run(false);
                return;
            }

            if self.close_handle.get_value().is_some() {
                return;
            }

            let Ok(handle) = set_timeout_with_handle(
                move || request_open_change.run(false),
                Duration::from_millis(close_delay_ms),
            ) else {
                request_open_change.run(false);
                return;
            };
            self.close_handle.set_value(Some(handle));
        }

        #[cfg(all(test, not(target_arch = "wasm32")))]
        {
            if immediate || close_delay_ms == 0 {
                self.clear_close();
                request_open_change.run(false);
                return;
            }

            if self.close_handle.get_value().is_some() {
                return;
            }

            let handle =
                test_timers::set_timeout(close_delay_ms, move || request_open_change.run(false));
            self.close_handle.set_value(Some(handle));
        }

        #[cfg(not(any(
            all(feature = "web", target_arch = "wasm32"),
            all(test, not(target_arch = "wasm32"))
        )))]
        {
            let _unused_close_cfg = (close_delay_ms, immediate);
            request_open_change.run(false);
        }
    }
}

impl Default for TooltipTimers {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
#[path = "test/tooltip_timers.rs"]
mod test_timers;

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests;
