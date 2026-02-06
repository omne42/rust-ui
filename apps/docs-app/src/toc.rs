use leptos::prelude::*;
use std::collections::BTreeMap;

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
}

pub fn provide_docs_toc() -> DocsToc {
    let toc = DocsToc {
        items: RwSignal::new(Vec::new()),
        used_ids: RwSignal::new(BTreeMap::new()),
        next_fallback_id: RwSignal::new(0),
    };
    provide_context(toc);
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

    pub fn clear(self) {
        self.items.set(Vec::new());
        self.used_ids.set(BTreeMap::new());
        self.next_fallback_id.set(0);
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
                title: title.trim().to_string(),
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

    view! {
        <Show when=move || !items.get().is_empty()>
            <div class="docs-toc__inner">
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
                                crate::route::route_section(&route.get()) == Some(id_for_active.as_str())
                            });

                            view! {
                                <li class="docs-toc__item" data-level=level.to_string() data-active=move || is_active.get().then_some("true")>
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
            </div>
        </Show>
    }
    .into_any()
}
