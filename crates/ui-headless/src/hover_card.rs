use leptos::prelude::*;

#[cfg(all(feature = "web", target_arch = "wasm32"))]
use std::time::Duration;

const DEFAULT_OPEN_DELAY_MS: u64 = 140;
const DEFAULT_CLOSE_DELAY_MS: u64 = 180;

#[cfg(all(feature = "web", target_arch = "wasm32"))]
type HoverCardTimeoutHandle = TimeoutHandle;

#[cfg(all(test, not(target_arch = "wasm32")))]
type HoverCardTimeoutHandle = test_timers::TestTimeoutHandle;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HoverCardTriggerOptions {
    pub is_disabled: bool,
    pub open_delay_ms: u64,
    pub close_delay_ms: u64,
}

impl Default for HoverCardTriggerOptions {
    fn default() -> Self {
        Self {
            is_disabled: false,
            open_delay_ms: DEFAULT_OPEN_DELAY_MS,
            close_delay_ms: DEFAULT_CLOSE_DELAY_MS,
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
    pub is_open: ReadSignal<bool>,
    pub dismiss: Callback<()>,
}

#[derive(Clone)]
pub struct HoverCardTriggerAria {
    pub state: HoverCardTriggerState,
    pub handlers: HoverCardTriggerHandlers,
}

pub fn use_hover_card_trigger(options: HoverCardTriggerOptions) -> HoverCardTriggerAria {
    let trigger_hover = crate::use_hover(crate::HoverOptions {
        is_disabled: options.is_disabled,
    });
    let panel_hover = crate::use_hover(crate::HoverOptions {
        is_disabled: options.is_disabled,
    });
    let trigger_focus = crate::use_focus_within(crate::FocusWithinOptions {
        is_disabled: options.is_disabled,
    });
    let panel_focus = crate::use_focus_within(crate::FocusWithinOptions {
        is_disabled: options.is_disabled,
    });

    let global_focus_visible = crate::use_focus_visible()
        .map(|state| state.is_focus_visible())
        .unwrap_or_else(|| signal(false).0);
    let on_trigger_focus_in = trigger_focus.handlers.on_focus_in;
    let on_trigger_focus_out = trigger_focus.handlers.on_focus_out;

    let (is_open, set_open) = signal(false);
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
            set_open.set(false);
        })
    };

    let timers_for_effect = timers.clone();
    Effect::new(move |_| {
        if options.is_disabled {
            timers_for_effect.clear();
            set_dismissed.set(false);
            set_open.set(false);
            return;
        }

        let intent_open = wants_open.get();

        if !intent_open {
            set_dismissed.set(false);
        }

        if is_dismissed.get_untracked() && intent_open {
            timers_for_effect.clear();
            set_open.set(false);
            return;
        }

        if intent_open {
            if is_open.get_untracked() {
                timers_for_effect.clear();
                return;
            }
            timers_for_effect.open(options.open_delay_ms, set_open);
            return;
        }

        if !is_open.get_untracked() {
            timers_for_effect.clear();
            return;
        }

        timers_for_effect.close(options.close_delay_ms, set_open);
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

    fn open(&self, delay_ms: u64, set_open: WriteSignal<bool>) {
        self.clear_close();

        #[cfg(all(feature = "web", target_arch = "wasm32"))]
        {
            if delay_ms == 0 {
                self.clear_open();
                set_open.set(true);
                return;
            }

            if self.open_handle.get_value().is_some() {
                return;
            }

            let open_handle = self.open_handle;
            let Ok(handle) = set_timeout_with_handle(
                move || {
                    open_handle.set_value(None);
                    set_open.set(true);
                },
                Duration::from_millis(delay_ms),
            ) else {
                set_open.set(true);
                return;
            };

            self.open_handle.set_value(Some(handle));
        }

        #[cfg(all(test, not(target_arch = "wasm32")))]
        {
            if delay_ms == 0 {
                self.clear_open();
                set_open.set(true);
                return;
            }

            if self.open_handle.get_value().is_some() {
                return;
            }

            let open_handle = self.open_handle;
            let handle = test_timers::set_timeout(delay_ms, move || {
                open_handle.set_value(None);
                set_open.set(true);
            });
            self.open_handle.set_value(Some(handle));
        }

        #[cfg(not(any(
            all(feature = "web", target_arch = "wasm32"),
            all(test, not(target_arch = "wasm32"))
        )))]
        {
            let _unused_delay_ms = delay_ms;
            set_open.set(true);
        }
    }

    fn close(&self, delay_ms: u64, set_open: WriteSignal<bool>) {
        self.clear_open();

        #[cfg(all(feature = "web", target_arch = "wasm32"))]
        {
            if delay_ms == 0 {
                self.clear_close();
                set_open.set(false);
                return;
            }

            if self.close_handle.get_value().is_some() {
                return;
            }

            let close_handle = self.close_handle;
            let Ok(handle) = set_timeout_with_handle(
                move || {
                    close_handle.set_value(None);
                    set_open.set(false);
                },
                Duration::from_millis(delay_ms),
            ) else {
                set_open.set(false);
                return;
            };

            self.close_handle.set_value(Some(handle));
        }

        #[cfg(all(test, not(target_arch = "wasm32")))]
        {
            if delay_ms == 0 {
                self.clear_close();
                set_open.set(false);
                return;
            }

            if self.close_handle.get_value().is_some() {
                return;
            }

            let close_handle = self.close_handle;
            let handle = test_timers::set_timeout(delay_ms, move || {
                close_handle.set_value(None);
                set_open.set(false);
            });
            self.close_handle.set_value(Some(handle));
        }

        #[cfg(not(any(
            all(feature = "web", target_arch = "wasm32"),
            all(test, not(target_arch = "wasm32"))
        )))]
        {
            let _unused_delay_ms = delay_ms;
            set_open.set(false);
        }
    }
}

impl Default for HoverCardTimers {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod test_timers {
    use std::cell::RefCell;

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct TestTimeoutHandle {
        id: u64,
    }

    impl TestTimeoutHandle {
        pub fn clear(self) {
            TEST_SCHEDULER.with(|scheduler| scheduler.borrow_mut().cancel(self.id));
        }
    }

    struct Task {
        id: u64,
        due_ms: u64,
        callback: Option<Box<dyn FnOnce()>>,
    }

    struct Scheduler {
        now_ms: u64,
        next_id: u64,
        tasks: Vec<Task>,
    }

    impl Scheduler {
        fn new() -> Self {
            Self {
                now_ms: 0,
                next_id: 1,
                tasks: Vec::new(),
            }
        }

        fn set_timeout(
            &mut self,
            delay_ms: u64,
            callback: impl FnOnce() + 'static,
        ) -> TestTimeoutHandle {
            let id = self.next_id;
            self.next_id = self.next_id.saturating_add(1);
            let due_ms = self.now_ms.saturating_add(delay_ms);
            self.tasks.push(Task {
                id,
                due_ms,
                callback: Some(Box::new(callback)),
            });
            TestTimeoutHandle { id }
        }

        fn cancel(&mut self, id: u64) {
            self.tasks.retain(|task| task.id != id);
        }

        fn take_due(&mut self) -> Vec<Box<dyn FnOnce()>> {
            let now_ms = self.now_ms;
            let mut callbacks = Vec::new();
            self.tasks.retain_mut(|task| {
                if task.due_ms <= now_ms {
                    if let Some(callback) = task.callback.take() {
                        callbacks.push(callback);
                    }
                    false
                } else {
                    true
                }
            });
            callbacks
        }
    }

    thread_local! {
        static TEST_SCHEDULER: RefCell<Scheduler> = RefCell::new(Scheduler::new());
    }

    pub fn set_timeout(delay_ms: u64, callback: impl FnOnce() + 'static) -> TestTimeoutHandle {
        TEST_SCHEDULER.with(|scheduler| scheduler.borrow_mut().set_timeout(delay_ms, callback))
    }

    pub fn advance_by(delta_ms: u64) {
        TEST_SCHEDULER.with(|scheduler| {
            let mut scheduler = scheduler.borrow_mut();
            scheduler.now_ms = scheduler.now_ms.saturating_add(delta_ms);
        });

        loop {
            let callbacks = TEST_SCHEDULER.with(|scheduler| scheduler.borrow_mut().take_due());
            if callbacks.is_empty() {
                break;
            }
            for callback in callbacks {
                callback();
            }
        }
    }

    pub fn reset() {
        TEST_SCHEDULER.with(|scheduler| *scheduler.borrow_mut() = Scheduler::new());
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests;
