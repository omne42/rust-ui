use leptos::prelude::*;

const MAX_EVENTS: usize = 200;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum UiTraceEventKind {
    OpenChange {
        open: bool,
    },
    Inspect {
        tag: String,
        data_slot: Option<String>,
    },
    Note {
        message: String,
    },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UiTraceEvent {
    pub ts_ms: u64,
    pub component: &'static str,
    pub kind: UiTraceEventKind,
}

#[derive(Clone, Copy)]
pub struct UiTrace {
    enabled: bool,
    events: RwSignal<Vec<UiTraceEvent>>,
}

pub fn provide_ui_trace(enabled: bool) -> UiTrace {
    let trace = UiTrace {
        enabled,
        events: RwSignal::new(Vec::new()),
    };
    provide_context(trace);
    trace
}

pub fn use_ui_trace() -> Option<UiTrace> {
    use_context::<UiTrace>()
}

impl UiTrace {
    pub fn enabled(self) -> bool {
        self.enabled
    }

    pub fn events(self) -> ReadSignal<Vec<UiTraceEvent>> {
        self.events.read_only()
    }

    pub fn emit(self, component: &'static str, kind: UiTraceEventKind) {
        if !self.enabled {
            return;
        }

        let event = UiTraceEvent {
            ts_ms: now_ms(),
            component,
            kind,
        };

        self.events.update(|events| {
            events.push(event);
            if events.len() > MAX_EVENTS {
                let overflow = events.len().saturating_sub(MAX_EVENTS);
                events.drain(0..overflow);
            }
        });
    }
}

fn now_ms() -> u64 {
    #[cfg(target_arch = "wasm32")]
    {
        leptos::web_sys::window()
            .and_then(|window| window.performance())
            .map(|perf| perf.now().floor() as u64)
            .unwrap_or(0)
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        use std::time::{SystemTime, UNIX_EPOCH};
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_millis() as u64)
            .unwrap_or(0)
    }
}
