use leptos::prelude::*;
use std::collections::BTreeMap;
use ui_layout::{Flex, FlexDirection, FlexGap};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TocItem {
    pub id: String,
    pub title: String,
    pub level: u8,
}

#[derive(Clone, Copy)]
pub struct DocsToc {
    items: RwSignal<Vec<TocItem>>,
    used_ids: RwSignal<BTreeMap<String, usize>>,
    next_fallback_id: RwSignal<u32>,
    active_id: RwSignal<Option<String>>,
}

pub fn provide_docs_toc() -> DocsToc {
    let toc = DocsToc {
        items: RwSignal::new(Vec::new()),
        used_ids: RwSignal::new(BTreeMap::new()),
        next_fallback_id: RwSignal::new(0),
        active_id: RwSignal::new(None),
    };
    provide_context(toc);

    attach_scroll_spy(toc);

    toc
}

pub fn use_docs_toc() -> Option<DocsToc> {
    use_context::<DocsToc>()
}

fn slugify_id(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    let mut prev_dash = false;

    for ch in input.chars() {
        if ch.is_ascii_alphanumeric() {
            out.push(ch.to_ascii_lowercase());
            prev_dash = false;
        } else if (ch.is_ascii_whitespace() || matches!(ch, '-' | '_' | '.'))
            && !out.is_empty()
            && !prev_dash
        {
            out.push('-');
            prev_dash = true;
        }
    }

    while out.ends_with('-') {
        out.pop();
    }

    out
}

impl DocsToc {
    pub fn items(self) -> ReadSignal<Vec<TocItem>> {
        self.items.read_only()
    }

    pub fn active_id(self) -> ReadSignal<Option<String>> {
        self.active_id.read_only()
    }

    pub fn set_active(self, id: Option<String>) {
        let id = id.filter(|value| !value.trim().is_empty());
        if self.active_id.get_untracked().as_deref() == id.as_deref() {
            return;
        }
        self.active_id.set(id);
    }

    pub fn clear(self) {
        self.items.set(Vec::new());
        self.used_ids.set(BTreeMap::new());
        self.next_fallback_id.set(0);
        self.active_id.set(None);
    }

    pub fn set_items(self, items: Vec<TocItem>) {
        self.items.set(items);
    }

    pub fn register(self, title: &str, level: u8) -> String {
        let slug = slugify_id(title);

        let mut used = self.used_ids.get_untracked();
        let id_base = if slug.is_empty() {
            let fallback = self.next_fallback_id.get_untracked();
            self.next_fallback_id.set(fallback.saturating_add(1));
            format!("section-{fallback}")
        } else {
            slug
        };

        let counter = used.entry(id_base.clone()).or_insert(0);
        let id = if *counter == 0 {
            id_base.clone()
        } else {
            format!("{id_base}-{}", *counter + 1)
        };
        *counter += 1;
        self.used_ids.set(used);

        self.items.update(|items| {
            items.push(TocItem {
                id: id.clone(),
                title: title.trim().into(),
                level,
            });
        });

        id
    }
}

#[component]
pub fn DocsTocPanel(route: ReadSignal<String>, navigate: Callback<String>) -> AnyView {
    let Some(toc) = use_docs_toc() else {
        return ().into_any();
    };

    let items = toc.items();
    let active_id = toc.active_id();

    view! {
        <Show when=move || !items.get().is_empty()>
            <Flex direction=FlexDirection::Column gap=FlexGap::Sm class_name="docs-toc__inner".to_string()>
                <div class="docs-toc__title">"On this page"</div>
                <ul class="docs-toc__list">
                    <For
                        each=move || items.get()
                        key=|item| item.id.clone()
                        children=move |item| {
                            let id = item.id.clone();
                            let id_for_active = id.clone();
                            let id_for_click = id.clone();
                            let title = item.title.clone();
                            let level = item.level;
                            let is_active = Signal::derive(move || {
                                active_id.get().as_deref() == Some(id_for_active.as_str())
                            });

                            view! {
                                <li class="docs-toc__item" data-level=level data-active=move || is_active.get().then_some("true")>
                                    <a
                                        href="#"
                                        on:click=move |ev| {
                                            ev.prevent_default();
                                            navigate.run(crate::route::route_with_section(&route.get(), &id_for_click));
                                        }
                                    >
                                        {title}
                                    </a>
                                </li>
                            }
                        }
                    />
                </ul>
            </Flex>
        </Show>
    }
    .into_any()
}

#[cfg(target_arch = "wasm32")]
fn attach_scroll_spy(toc: DocsToc) {
    use gloo_events::EventListener;
    use leptos::wasm_bindgen::{JsCast, closure::Closure};
    use std::{cell::Cell, rc::Rc};

    let Some(window) = web_sys::window() else {
        return;
    };

    let scheduled = Rc::new(Cell::new(false));
    let schedule_update = Rc::new({
        let window = window.clone();
        let scheduled = Rc::clone(&scheduled);
        move || {
            if scheduled.get() {
                return;
            }
            scheduled.set(true);

            let callback = Closure::once_into_js({
                let window = window.clone();
                let scheduled = Rc::clone(&scheduled);
                move || {
                    scheduled.set(false);
                    update_active_from_scroll(&window, toc);
                }
            });

            let Some(callback) = callback.dyn_ref::<js_sys::Function>() else {
                return;
            };
            drop(window.request_animation_frame(callback));
        }
    });

    let scroll_listener = StoredValue::new_local(None::<EventListener>);
    let resize_listener = StoredValue::new_local(None::<EventListener>);

    scroll_listener.set_value(Some(EventListener::new(&window, "scroll", {
        let schedule_update = Rc::clone(&schedule_update);
        move |_| schedule_update()
    })));

    resize_listener.set_value(Some(EventListener::new(&window, "resize", {
        let schedule_update = Rc::clone(&schedule_update);
        move |_| schedule_update()
    })));

    Effect::new({
        let window = window.clone();
        move |_| {
            let items = toc.items.get();
            if items.is_empty() {
                toc.set_active(None);
                return;
            }
            update_active_from_scroll(&window, toc);
        }
    });

    on_cleanup(move || {
        scroll_listener.set_value(None);
        resize_listener.set_value(None);
    });
}

#[cfg(not(target_arch = "wasm32"))]
fn attach_scroll_spy(_toc: DocsToc) {}

#[cfg(target_arch = "wasm32")]
fn update_active_from_scroll(window: &web_sys::Window, toc: DocsToc) {
    let Some(document) = window.document() else {
        return;
    };

    let items = toc.items.get_untracked();
    if items.is_empty() {
        toc.set_active(None);
        return;
    }

    let offset_px = 120.0;
    let mut last_seen = None::<String>;

    for item in &items {
        let Some(el) = document.get_element_by_id(&item.id) else {
            continue;
        };

        let rect = el.get_bounding_client_rect();
        if rect.top() <= offset_px {
            last_seen = Some(item.id.clone());
        } else {
            break;
        }
    }

    if last_seen.is_none() {
        last_seen = Some(items[0].id.clone());
    }

    toc.set_active(last_seen);
}
