use crate::search_index::search;
use leptos::{ev, html, prelude::*};
use ui_components::{Button, ButtonSize, ButtonVariant, Dialog, DialogSize, SearchInputButton};

#[cfg(target_arch = "wasm32")]
fn detect_meta_key_label() -> &'static str {
    let Some(window) = web_sys::window() else {
        return "Ctrl";
    };
    let Ok(platform) = window.navigator().platform() else {
        return "Ctrl";
    };
    let platform = platform.to_lowercase();
    if platform.contains("mac")
        || platform.contains("iphone")
        || platform.contains("ipad")
        || platform.contains("ipod")
    {
        "⌘"
    } else {
        "Ctrl"
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn detect_meta_key_label() -> &'static str {
    "Ctrl"
}

#[cfg(target_arch = "wasm32")]
fn focus_input(input_ref: &NodeRef<html::Input>) {
    let Some(el) = input_ref.get_untracked() else {
        return;
    };
    let _ = el.focus();
}

#[cfg(not(target_arch = "wasm32"))]
fn focus_input(_input_ref: &NodeRef<html::Input>) {}

#[component]
pub fn DocsCommandMenu(navigate: Callback<String>) -> impl IntoView {
    let (open, set_open) = signal(false);
    let (query, set_query) = signal(String::new());
    let (active_index, set_active_index) = signal(0usize);

    let input_ref: NodeRef<html::Input> = NodeRef::new();

    let meta_key_label = StoredValue::new(detect_meta_key_label().to_string());

    let all_records = StoredValue::new(crate::search_index::build_records());

    let filtered = Memo::new(move |_| {
        let q = query.get();
        all_records.with_value(|records| search(records, &q, 18))
    });

    let indexed = Memo::new(move |_| filtered.get().into_iter().enumerate().collect::<Vec<_>>());

    let close = Callback::new(move |_| {
        set_open.set(false);
        set_query.set(String::new());
        set_active_index.set(0);
    });

    let on_open: Callback<()> = Callback::new(move |_| {
        set_open.set(true);
        set_active_index.set(0);
    });

    let on_select = Callback::new(move |idx: usize| {
        let route = all_records
            .with_value(|records| records.get(idx).map(|record| record.route.clone()))
            .unwrap_or_default();

        if route.trim().is_empty() {
            return;
        }

        close.run(());
        navigate.run(route);
    });

    #[cfg(target_arch = "wasm32")]
    {
        use gloo_events::EventListener;
        use leptos::wasm_bindgen::JsCast;

        let listener = StoredValue::new_local(None::<EventListener>);
        if let Some(window) = web_sys::window() {
            let set_open_for_listener = set_open.clone();
            listener.set_value(Some(EventListener::new(&window, "keydown", move |event| {
                let Some(event) = event.dyn_ref::<web_sys::KeyboardEvent>() else {
                    return;
                };

                let key = event.key();
                if (key == "k" || key == "K") && (event.meta_key() || event.ctrl_key()) {
                    event.prevent_default();
                    set_open_for_listener.update(|value| *value = !*value);
                }
            })));
        }

        on_cleanup(move || listener.set_value(None));
    }

    Effect::new(move |_| {
        if open.get() {
            focus_input(&input_ref);
        }
    });

    let on_input = Callback::new(move |ev: ev::Event| {
        set_query.set(event_target_value(&ev));
        set_active_index.set(0);
    });

    let on_input_key_down = Callback::new(move |ev: ev::KeyboardEvent| {
        let key = ev.key();
        let len = filtered.get().len();
        if key == "ArrowDown" {
            if len == 0 {
                return;
            }
            ev.prevent_default();
            set_active_index.update(|idx| *idx = (*idx + 1).min(len.saturating_sub(1)));
        } else if key == "ArrowUp" {
            if len == 0 {
                return;
            }
            ev.prevent_default();
            set_active_index.update(|idx| *idx = idx.saturating_sub(1));
        } else if key == "Enter" {
            if len == 0 {
                return;
            }
            ev.prevent_default();
            if let Some(idx) = filtered.get().get(active_index.get()).copied() {
                on_select.run(idx);
            }
        } else if key == "Escape" {
            close.run(());
        }
    });

    let render_row = move |(idx, record_idx): (usize, usize)| {
        let Some(record) = all_records.with_value(|records| records.get(record_idx).cloned())
        else {
            return ().into_any();
        };

        let is_active = Signal::derive(move || active_index.get() == idx);
        let on_press = Callback::new(move |_| on_select.run(record_idx));
        let on_mouse_enter = Callback::new(move |_| set_active_index.set(idx));

        view! {
            <li class="docs-command-menu__item">
                <Button
                    variant=ButtonVariant::Ghost
                    size=ButtonSize::Sm
                    class_name="docs-command-menu__button".to_string()
                    on_press=on_press
                    aria_label=record.title.to_string()
                >
                    <span
                        class="docs-command-menu__button-inner"
                        data-active=move || is_active.get().then_some("true")
                        on:mouseenter=move |_| on_mouse_enter.run(())
                    >
                        <span class="docs-command-menu__label">
                            <span class="docs-command-menu__title">{record.title.clone()}</span>
                            <span class="docs-command-menu__subtitle">{record.subtitle.clone()}</span>
                        </span>
                        <code class="docs-command-menu__route">{record.route_label.clone()}</code>
                    </span>
                </Button>
            </li>
        }
        .into_any()
    };

    view! {
        <div class="docs-command-menu" data-slot="docs-command-menu">
            <SearchInputButton
                placeholder="Search docs…".to_string()
                compact_placeholder="Search…".to_string()
                meta_key_label=meta_key_label.get_value()
                key_label="K".to_string()
                aria_label="Search docs".to_string()
                class_name="docs-command-menu__trigger".to_string()
                on_press=on_open
            />

            <Dialog
                open=open.into()
                on_close=close
                id_base="docs-command-menu".to_string()
                title="Search".to_string()
                description="Type to filter pages and components. Use ↑/↓ and Enter.".to_string()
                size=DialogSize::Sm
                show_close_button=true
            >
                <div class="docs-command-menu__body">
                    <input
                        class="docs-command-menu__input"
                        type="search"
                        placeholder="Button, Select, Overlay…"
                        node_ref=input_ref
                        prop:value=move || query.get()
                        on:input=move |ev| on_input.run(ev)
                        on:keydown=move |ev| on_input_key_down.run(ev)
                    />

                    <div class="docs-command-menu__results" data-slot="docs-command-menu-results">
                        <Show when=move || filtered.get().is_empty()>
                            <div class="docs-command-menu__empty">
                                "No matching pages."
                            </div>
                        </Show>

                        <Show when=move || !filtered.get().is_empty()>
                            <ul class="docs-command-menu__list">
                                <For
                                    each=move || indexed.get()
                                    key=move |(_, record_idx)| {
                                        all_records
                                            .with_value(|records| {
                                                records
                                                    .get(*record_idx)
                                                    .map(|record| record.key.clone())
                                            })
                                            .unwrap_or_else(|| record_idx.to_string())
                                    }
                                    children=render_row
                                />
                            </ul>
                        </Show>
                    </div>
                </div>
            </Dialog>
        </div>
    }
}
