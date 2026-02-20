use leptos::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UiPerfBudget {
    pub max_mount_ms: f64,
    pub max_update_ms: Option<f64>,
    pub max_heap_kb: Option<f64>,
}

impl UiPerfBudget {
    pub const fn mount_only(max_mount_ms: f64) -> Self {
        Self {
            max_mount_ms,
            max_update_ms: None,
            max_heap_kb: None,
        }
    }
}

#[component]
pub fn UiPerfProbe(
    #[prop(into)] name: String,
    #[prop(optional)] budget: Option<UiPerfBudget>,
    children: Children,
) -> impl IntoView {
    let name = StoredValue::new(name);
    let budget = StoredValue::new(budget);

    let (mount_ms, set_mount_ms) = signal::<Option<f64>>(None);

    #[cfg(target_arch = "wasm32")]
    {
        let start_ms = now_ms();
        Effect::new(move |_| {
            if mount_ms.get_untracked().is_some() {
                return;
            }
            set_mount_ms.set(Some(now_ms() - start_ms));
        });
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let _unused_set_mount_ms = set_mount_ms;
    }

    let violation = Signal::derive(move || {
        let mount_ms = mount_ms.get()?;
        let budget = budget.get_value()?;
        (mount_ms > budget.max_mount_ms).then_some("true")
    });

    view! {
        <div
            data-slot="ui-perf-probe"
            data-perf-name=name.get_value()
            data-perf-mount-ms=move || mount_ms.get().map(|v| format!("{v:.2}"))
            data-perf-budget-ms=budget.get_value().map(|b| format!("{:.2}", b.max_mount_ms))
            data-perf-budget-update-ms=budget
                .get_value()
                .and_then(|b| b.max_update_ms.map(|v| format!("{v:.2}")))
            data-perf-budget-heap-kb=budget
                .get_value()
                .and_then(|b| b.max_heap_kb.map(|v| format!("{v:.2}")))
            data-perf-violation=move || violation.get()
            data-perf-observability=move || {
                budget.get_value().map(|budget| {
                    if budget.max_update_ms.is_some() || budget.max_heap_kb.is_some() {
                        "mount-plus-budget"
                    } else {
                        "mount-only"
                    }
                })
            }
        >
            {children()}
        </div>
    }
}

#[cfg(target_arch = "wasm32")]
fn now_ms() -> f64 {
    leptos::web_sys::window()
        .and_then(|window| window.performance())
        .map(|perf| perf.now())
        .unwrap_or(0.0)
}
