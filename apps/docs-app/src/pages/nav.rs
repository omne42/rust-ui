use crate::pages::components::component_catalog;
use leptos::prelude::*;

const GROUP_ORDER: &[&str] = &[
    "Actions",
    "Forms",
    "Layout",
    "Display",
    "Files",
    "Collections",
    "Overlays",
];

#[component]
pub fn DocsNav(route: ReadSignal<String>, navigate: Callback<String>) -> impl IntoView {
    let (filter, set_filter) = signal(String::new());
    let filtered = Memo::new(move |_| {
        let q = filter.get().to_lowercase();
        if q.trim().is_empty() {
            return Vec::new();
        }
        component_catalog()
            .iter()
            .copied()
            .filter(|doc| {
                doc.name.to_lowercase().contains(&q) || doc.slug.to_lowercase().contains(&q)
            })
            .take(24)
            .collect::<Vec<_>>()
    });

    view! {
        <div class="docs-nav__inner">
            <div class="docs-nav-section">
                <div class="docs-nav-title">"Docs"</div>
                <a
                    href="#/docs/welcome"
                    data-active=move || (route.get() == "docs/welcome").then_some("true")
                    on:click=move |ev| {
                        ev.prevent_default();
                        navigate.run("docs/welcome".to_string());
                    }
                >
                    "Welcome"
                </a>
                <a
                    href="#/docs/rules"
                    data-active=move || (route.get() == "docs/rules").then_some("true")
                    on:click=move |ev| {
                        ev.prevent_default();
                        navigate.run("docs/rules".to_string());
                    }
                >
                    "Rules"
                </a>
            </div>

            <div class="docs-nav-section">
                <div class="docs-nav-title">"Components"</div>
                <a
                    href="#/components"
                    data-active=move || (route.get() == "components").then_some("true")
                    on:click=move |ev| {
                        ev.prevent_default();
                        navigate.run("components".to_string());
                    }
                >
                    "All components"
                </a>

                <div class="docs-nav-search">
                    <input
                        class="docs-nav-search__input"
                        type="search"
                        placeholder="Search components…"
                        prop:value=move || filter.get()
                        on:input=move |ev| set_filter.set(event_target_value(&ev))
                    />
                </div>

                <Show when=move || !filtered.get().is_empty()>
                    <div class="docs-nav-search-results">
                        <For
                            each=move || filtered.get()
                            key=|doc| doc.slug
                            children=move |doc| {
                                let href = format!("#/components/{}", doc.slug);
                                view! {
                                    <a
                                        href=href
                                        data-active=move || (route.get() == format!("components/{}", doc.slug)).then_some("true")
                                    >
                                        {doc.name}
                                    </a>
                                }
                            }
                        />
                    </div>
                </Show>

                <Show when=move || filter.get().trim().is_empty()>
                    <For
                        each=move || GROUP_ORDER.iter().copied()
                        key=|group| *group
                        children=move |group| {
                            let items = component_catalog()
                                .iter()
                                .copied()
                                .filter(|doc| doc.group == group)
                                .collect::<Vec<_>>();

                            view! {
                                <div class="docs-nav-section">
                                    <div class="docs-nav-title">{group}</div>
                                    <For
                                        each=move || items.clone()
                                        key=|doc| doc.slug
                                        children=move |doc| {
                                            view! {
                                                <a
                                                    href=format!("#/components/{}", doc.slug)
                                                    data-active=move || (route.get() == format!("components/{}", doc.slug)).then_some("true")
                                                >
                                                    {doc.name}
                                                </a>
                                            }
                                        }
                                    />
                                </div>
                            }
                        }
                    />
                </Show>
            </div>
        </div>
    }
}
