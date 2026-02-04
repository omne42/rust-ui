use leptos::prelude::*;

#[derive(Clone)]
pub struct HoverCardTimers {
    #[cfg(target_arch = "wasm32")]
    open_handle: StoredValue<Option<TimeoutHandle>, LocalStorage>,
    #[cfg(target_arch = "wasm32")]
    close_handle: StoredValue<Option<TimeoutHandle>, LocalStorage>,
}

impl HoverCardTimers {
    pub fn new() -> Self {
        #[cfg(target_arch = "wasm32")]
        {
            Self {
                open_handle: StoredValue::new_local(None),
                close_handle: StoredValue::new_local(None),
            }
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            Self {}
        }
    }

    #[cfg(target_arch = "wasm32")]
    pub fn clear(&self) {
        if let Some(handle) = self.open_handle.get_value() {
            handle.clear();
        }
        self.open_handle.set_value(None);

        if let Some(handle) = self.close_handle.get_value() {
            handle.clear();
        }
        self.close_handle.set_value(None);
    }

    #[cfg(target_arch = "wasm32")]
    pub fn schedule_open(&self, delay_ms: u64, set_open: WriteSignal<bool>) {
        if let Some(handle) = self.open_handle.get_value() {
            handle.clear();
        }
        self.open_handle.set_value(None);

        let Ok(handle) = set_timeout_with_handle(
            move || set_open.set(true),
            std::time::Duration::from_millis(delay_ms),
        ) else {
            return;
        };
        self.open_handle.set_value(Some(handle));
    }

    #[cfg(target_arch = "wasm32")]
    pub fn schedule_close(&self, delay_ms: u64, set_open: WriteSignal<bool>) {
        if let Some(handle) = self.close_handle.get_value() {
            handle.clear();
        }
        self.close_handle.set_value(None);

        let Ok(handle) = set_timeout_with_handle(
            move || set_open.set(false),
            std::time::Duration::from_millis(delay_ms),
        ) else {
            return;
        };
        self.close_handle.set_value(Some(handle));
    }
}

impl Default for HoverCardTimers {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HoverCardIntent {
    Open,
    Close,
}

pub fn drive_open_state(
    intent: HoverCardIntent,
    open_delay_ms: u64,
    close_delay_ms: u64,
    set_open: WriteSignal<bool>,
    timers: &HoverCardTimers,
) {
    #[cfg(target_arch = "wasm32")]
    {
        timers.clear();
        match intent {
            HoverCardIntent::Open => timers.schedule_open(open_delay_ms, set_open),
            HoverCardIntent::Close => timers.schedule_close(close_delay_ms, set_open),
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (open_delay_ms, close_delay_ms, timers);
        match intent {
            HoverCardIntent::Open => set_open.set(true),
            HoverCardIntent::Close => set_open.set(false),
        }
    }
}
