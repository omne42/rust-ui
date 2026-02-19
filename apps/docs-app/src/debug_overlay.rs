use leptos::prelude::*;

pub const CSS: &str = r#"
.ui-debug-overlay {
  position: fixed;
  right: 12px;
  bottom: 12px;
  z-index: 2147483000;
  font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, "Liberation Mono",
    "Courier New", monospace;
  font-size: 12px;
  line-height: 1.4;
  color: var(--ui-fg);
}

.ui-debug-overlay__button {
  appearance: none;
  border: 1px solid var(--ui-border);
  background: var(--ui-bg);
  color: var(--ui-fg);
  padding: 6px 10px;
  border-radius: 10px;
  cursor: pointer;
}

.ui-debug-overlay__button:hover {
  background: var(--ui-bg-muted);
}

.ui-debug-overlay__panel {
  margin-top: 8px;
  width: min(520px, calc(100vw - 24px));
  max-height: min(60vh, 520px);
  overflow: auto;
  border: 1px solid var(--ui-border);
  background: var(--ui-bg);
  border-radius: 12px;
  box-shadow: 0 18px 40px rgba(0, 0, 0, 0.18);
  padding: 10px;
}

.ui-debug-overlay__section-title {
  font-size: 11px;
  letter-spacing: 0.02em;
  text-transform: uppercase;
  color: var(--ui-fg-muted);
  margin: 10px 0 6px;
}

.ui-debug-overlay__events {
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.ui-debug-overlay__event {
  border: 1px solid var(--ui-border);
  border-radius: 10px;
  padding: 6px 8px;
  background: var(--ui-bg);
}

.ui-debug-overlay__event-meta {
  display: flex;
  gap: 8px;
  flex-wrap: wrap;
  font-size: 11px;
  color: var(--ui-fg-muted);
}

.ui-debug-overlay__event-body {
  margin-top: 4px;
  word-break: break-word;
}

.ui-debug-overlay__row {
  display: grid;
  grid-template-columns: 150px 1fr;
  gap: 8px;
  padding: 4px 0;
  border-bottom: 1px dashed var(--ui-border);
}

.ui-debug-overlay__row:last-child {
  border-bottom: none;
}

.ui-debug-overlay__key {
  color: var(--ui-fg-muted);
  word-break: break-word;
}

.ui-debug-overlay__value {
  word-break: break-word;
}
"#;

#[cfg(target_arch = "wasm32")]
#[derive(Clone, Debug, Default, PartialEq, Eq)]
struct UiDebugSnapshot {
    tag: String,
    id: Option<String>,
    class_name: Option<String>,
    role: Option<String>,
    data_attrs: Vec<(String, String)>,
    aria_attrs: Vec<(String, String)>,
}

#[component]
pub fn UiDebugOverlay(#[prop(optional)] enabled: bool) -> AnyView {
    if !enabled {
        return ().into_view().into_any();
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        ().into_view().into_any()
    }

    #[cfg(target_arch = "wasm32")]
    {
        use leptos::wasm_bindgen::JsCast;

        let trace = ui_headless::use_ui_trace();
        let trace = StoredValue::new(trace);

        let (is_open, set_is_open) = signal(false);
        let (snapshot, set_snapshot) = signal::<Option<UiDebugSnapshot>>(None);

        Effect::new(move |_| {
            let Some(document) = leptos::web_sys::window().and_then(|window| window.document())
            else {
                return;
            };

            let set_snapshot = set_snapshot.clone();
            let trace = trace.get_value();
            let listener = leptos::wasm_bindgen::closure::Closure::wrap(Box::new(
                move |ev: leptos::web_sys::Event| {
                    let Some(target) = ev.target() else { return };
                    let Ok(el) = target.dyn_into::<leptos::web_sys::Element>() else {
                        return;
                    };
                    let snapshot = snapshot_from_element(&el);
                    set_snapshot.set(Some(snapshot.clone()));
                    if let Some(trace) = trace {
                        trace.emit(
                            "debug-overlay",
                            ui_headless::UiTraceEventKind::Inspect {
                                tag: snapshot.tag.clone(),
                                data_slot: snapshot
                                    .data_attrs
                                    .iter()
                                    .find_map(|(k, v)| (k == "data-slot").then(|| v.clone())),
                            },
                        );
                    }
                },
            )
                as Box<dyn FnMut(leptos::web_sys::Event)>);

            drop(document.add_event_listener_with_callback(
                "pointerdown",
                listener.as_ref().unchecked_ref(),
            ));
            listener.forget();
        });

        let toggle = move |_| set_is_open.update(|open| *open = !*open);

        view! {
            <div class="ui-debug-overlay" data-slot="ui-debug-overlay" data-open=move || is_open.get().then_some("true")>
                <button class="ui-debug-overlay__button" type="button" on:click=toggle>
                    {move || if is_open.get() { "Debug (on)" } else { "Debug" }}
                </button>

                <Show when=move || is_open.get()>
                    <div class="ui-debug-overlay__panel" data-slot="ui-debug-overlay-panel">
                        <div class="ui-debug-overlay__section-title">"Inspect"</div>
                        {move || snapshot.get().map(render_snapshot)}

                        <div class="ui-debug-overlay__section-title">"Events"</div>
                        {move || trace.get_value().map(render_events).unwrap_or_else(|| view! {
                            <div class="ui-muted" data-slot="ui-debug-overlay-events-disabled">
                                "Trace context not available."
                            </div>
                        }.into_any())}
                    </div>
                </Show>
            </div>
        }
        .into_any()
    }
}

#[cfg(target_arch = "wasm32")]
fn snapshot_from_element(el: &leptos::web_sys::Element) -> UiDebugSnapshot {
    let tag = el.tag_name();
    let id = el.get_attribute("id").and_then(non_empty);
    let class_name = el.get_attribute("class").and_then(non_empty);
    let role = el.get_attribute("role").and_then(non_empty);

    let mut data_attrs = Vec::new();
    let mut aria_attrs = Vec::new();

    let attrs = el.attributes();
    for idx in 0..attrs.length() {
        let Some(attr) = attrs.item(idx) else {
            continue;
        };
        let name = attr.name();
        let value = attr.value();
        if name.starts_with("data-") {
            data_attrs.push((name, value));
        } else if name == "role" || name.starts_with("aria-") {
            aria_attrs.push((name, value));
        }
    }

    data_attrs.sort_by(|a, b| a.0.cmp(&b.0));
    aria_attrs.sort_by(|a, b| a.0.cmp(&b.0));

    UiDebugSnapshot {
        tag,
        id,
        class_name,
        role,
        data_attrs,
        aria_attrs,
    }
}

#[cfg(target_arch = "wasm32")]
fn non_empty(value: String) -> Option<String> {
    let trimmed = value.trim();
    (!trimmed.is_empty()).then(|| trimmed.into())
}

#[cfg(target_arch = "wasm32")]
fn render_snapshot(snapshot: UiDebugSnapshot) -> impl IntoView {
    let UiDebugSnapshot {
        tag,
        id,
        class_name,
        role,
        data_attrs,
        aria_attrs,
    } = snapshot;

    let id = id.unwrap_or_else(|| "∅".to_string());
    let class_name = class_name.unwrap_or_else(|| "∅".to_string());
    let role = role.unwrap_or_else(|| "∅".to_string());

    view! {
        <div data-slot="ui-debug-overlay-snapshot">
            <div class="ui-debug-overlay__row" data-slot="ui-debug-overlay-row">
                <div class="ui-debug-overlay__key">"tag"</div>
                <div class="ui-debug-overlay__value">{tag}</div>
            </div>
            <div class="ui-debug-overlay__row" data-slot="ui-debug-overlay-row">
                <div class="ui-debug-overlay__key">"id"</div>
                <div class="ui-debug-overlay__value">{id}</div>
            </div>
            <div class="ui-debug-overlay__row" data-slot="ui-debug-overlay-row">
                <div class="ui-debug-overlay__key">"class"</div>
                <div class="ui-debug-overlay__value">{class_name}</div>
            </div>
            <div class="ui-debug-overlay__row" data-slot="ui-debug-overlay-row">
                <div class="ui-debug-overlay__key">"role"</div>
                <div class="ui-debug-overlay__value">{role}</div>
            </div>

            <div class="ui-debug-overlay__row" data-slot="ui-debug-overlay-row">
                <div class="ui-debug-overlay__key">"data-*"</div>
                <div class="ui-debug-overlay__value">
                    {data_attrs.into_iter().map(|(k, v)| view! { <div><span class="ui-muted">{k}</span>{": "}{v}</div> }).collect_view()}
                </div>
            </div>

            <div class="ui-debug-overlay__row" data-slot="ui-debug-overlay-row">
                <div class="ui-debug-overlay__key">"aria-*"</div>
                <div class="ui-debug-overlay__value">
                    {aria_attrs.into_iter().map(|(k, v)| view! { <div><span class="ui-muted">{k}</span>{": "}{v}</div> }).collect_view()}
                </div>
            </div>
        </div>
    }
}

#[cfg(target_arch = "wasm32")]
fn render_events(trace: ui_headless::UiTrace) -> AnyView {
    let events = trace.events();
    view! {
        <div class="ui-debug-overlay__events" data-slot="ui-debug-overlay-events">
            {move || {
                let events = events.get();
                events
                    .into_iter()
                    .rev()
                    .take(40)
                    .map(|event| render_event(event))
                    .collect_view()
            }}
        </div>
    }
    .into_any()
}

#[cfg(target_arch = "wasm32")]
fn render_event(event: ui_headless::UiTraceEvent) -> AnyView {
    let component = event.component;
    let ts_ms = event.ts_ms;
    let (kind_label, body, kind_attr) = match event.kind {
        ui_headless::UiTraceEventKind::OpenChange { open } => {
            ("open-change", format!("open={open}"), "open-change")
        }
        ui_headless::UiTraceEventKind::Inspect { tag, data_slot } => (
            "inspect",
            format!(
                "tag={tag}, data-slot={}",
                data_slot.unwrap_or_else(|| "∅".to_string())
            ),
            "inspect",
        ),
        ui_headless::UiTraceEventKind::Note { message } => ("note", message, "note"),
    };

    view! {
        <div
            class="ui-debug-overlay__event"
            data-slot="ui-debug-overlay-event"
            data-component=component
            data-kind=kind_attr
        >
            <div class="ui-debug-overlay__event-meta">
                <span>{format!("{ts_ms}ms")}</span>
                <span>{component}</span>
                <span>{kind_label}</span>
            </div>
            <div class="ui-debug-overlay__event-body">{body}</div>
        </div>
    }
    .into_any()
}
