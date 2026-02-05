mod pages;
mod shell;

use leptos::prelude::*;

#[derive(Clone, Copy, Debug)]
pub struct ComponentDoc {
    pub name: &'static str,
    pub slug: &'static str,
    pub group: &'static str,
    pub page: fn() -> AnyView,
}

impl PartialEq for ComponentDoc {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.slug == other.slug && self.group == other.group
    }
}

impl Eq for ComponentDoc {}

pub fn component_catalog() -> &'static [ComponentDoc] {
    pages::CATALOG
}

pub fn component_page(slug: &str) -> Option<AnyView> {
    component_catalog()
        .iter()
        .find(|doc| doc.slug == slug)
        .map(|doc| (doc.page)())
}

#[component]
pub fn ComponentsIndex() -> impl IntoView {
    let (query, set_query) = signal(String::new());
    let filtered = Memo::new(move |_| {
        let q = query.get().to_lowercase();
        component_catalog()
            .iter()
            .copied()
            .filter(|doc| {
                if q.trim().is_empty() {
                    true
                } else {
                    doc.name.to_lowercase().contains(&q) || doc.slug.to_lowercase().contains(&q)
                }
            })
            .collect::<Vec<_>>()
    });

    view! {
        <section class="docs-card docs-prose">
            <h2>"Components"</h2>
            <p>
                "Every public component should have at least one playground here. Use search to find a component."
            </p>

            <div class="docs-search">
                <label class="docs-search__label">
                    "Search"
                </label>
                <input
                    class="docs-search__input"
                    type="search"
                    placeholder="Button, Select, Overlay…"
                    prop:value=move || query.get()
                    on:input=move |ev| set_query.set(event_target_value(&ev))
                />
            </div>
        </section>

        <section class="docs-card">
            <div class="docs-component-grid">
                <For
                    each=move || filtered.get()
                    key=|doc| doc.slug
                    children=move |doc| {
                        view! {
                            <a class="docs-component-tile" href=format!("#/components/{}", doc.slug)>
                                <div class="docs-component-tile__name">{doc.name}</div>
                                <div class="docs-component-tile__meta">
                                    <span class="docs-component-tile__group">{doc.group}</span>
                                    <code class="docs-component-tile__slug">{doc.slug}</code>
                                </div>
                            </a>
                        }
                    }
                />
            </div>
        </section>
    }
}

pub use shell::ComponentPage;
