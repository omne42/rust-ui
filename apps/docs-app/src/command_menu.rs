use crate::search_index::search;
use leptos::{ev, html, prelude::*};
use ui_components::{Button, ButtonSize, ButtonVariant, Dialog, DialogSize, SearchInputButton};
use ui_layout::{Flex, FlexAlign, FlexDirection, FlexGap};

fn truncate_chars(input: &str, max_chars: usize) -> String {
    let input = input.trim();
    if input.is_empty() {
        return String::new();
    }

    let mut out = String::new();
    let mut iter = input.chars();
    for _ in 0..max_chars {
        match iter.next() {
            Some(ch) => out.push(ch),
            None => return out,
        }
    }

    if iter.next().is_some() {
        out.push('…');
    }

    out
}

fn eq_ascii_case_insensitive(a: &[u8], b: &[u8]) -> bool {
    a.eq_ignore_ascii_case(b)
}

fn find_ascii_case_insensitive(haystack: &str, needle: &str) -> Option<usize> {
    let needle = needle.trim();
    if needle.is_empty() {
        return Some(0);
    }

    let needle_bytes = needle.as_bytes();
    if needle_bytes.len() > haystack.len() {
        return None;
    }

    let haystack_bytes = haystack.as_bytes();
    for (start, _) in haystack.char_indices() {
        let end = start.saturating_add(needle_bytes.len());
        if end > haystack_bytes.len() {
            break;
        }
        if eq_ascii_case_insensitive(&haystack_bytes[start..end], needle_bytes) {
            return Some(start);
        }
    }

    None
}

fn create_snippet(content: &str, query: &str) -> Option<String> {
    let content = content.trim();
    if content.is_empty() {
        return None;
    }

    let query = query.trim();
    if query.is_empty() {
        return Some(truncate_chars(content, 160));
    }

    let Some(match_byte) = find_ascii_case_insensitive(content, query) else {
        return Some(truncate_chars(content, 160));
    };

    let mut char_starts = Vec::new();
    char_starts.extend(content.char_indices().map(|(idx, _)| idx));
    char_starts.push(content.len());

    let Ok(match_char) = char_starts.binary_search(&match_byte) else {
        return Some(truncate_chars(content, 160));
    };

    let needle_chars = query.chars().count();
    let max_chars = char_starts.len().saturating_sub(1);

    let start_char = match_char.saturating_sub(40);
    let end_char = (match_char + needle_chars + 40).min(max_chars);

    let start_byte = char_starts.get(start_char).copied().unwrap_or(0);
    let end_byte = char_starts.get(end_char).copied().unwrap_or(content.len());

    let slice = content.get(start_byte..end_byte).unwrap_or(content);

    let mut out = String::new();
    if start_char > 0 {
        out.push('…');
    }
    out.push_str(slice.trim());
    if end_char < max_chars {
        out.push('…');
    }

    Some(out)
}

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
    drop(el.focus());
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

        let record_title = record.title.clone();
        let record_subtitle = record.subtitle.clone();
        let record_route_label = record.route_label.clone();
        let record_content = record.content.clone();

        let snippet = Signal::derive(move || {
            let q = query.get();
            let q = q.trim();
            if q.is_empty() {
                return None;
            }
            create_snippet(&record_content, q)
        });

        view! {
            <li class="docs-command-menu__item">
                <Button
                    variant=ButtonVariant::Ghost
                    size=ButtonSize::Sm
                    class_name="docs-command-menu__button".to_string()
                    on_press=on_press
                    aria_label=record_title.clone()
                >
                    <div
                        class="docs-command-menu__button-inner"
                        data-active=move || is_active.get().then_some("true")
                        on:mouseenter=move |_| on_mouse_enter.run(())
                    >
                        <Flex
                            direction=FlexDirection::Column
                            align=FlexAlign::Start
                            gap=FlexGap::Xs
                            class_name="docs-command-menu__label".to_string()
                        >
                            <Flex align=FlexAlign::Baseline gap=FlexGap::Sm class_name="docs-command-menu__topline".to_string()>
                                <span class="docs-command-menu__title">{record_title.clone()}</span>
                                <span class="docs-command-menu__subtitle">{record_subtitle.clone()}</span>
                            </Flex>
                            {move || snippet.get().map(|snippet| view! {
                                <span class="docs-command-menu__snippet">{snippet}</span>
                            })}
                        </Flex>
                        <code class="docs-command-menu__route">{record_route_label.clone()}</code>
                    </div>
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
                <Flex direction=FlexDirection::Column gap=FlexGap::Sm class_name="docs-command-menu__body".to_string()>
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
                </Flex>
            </Dialog>
        </div>
    }
}

#[cfg(test)]
#[path = "test/command_menu.rs"]
mod tests;
