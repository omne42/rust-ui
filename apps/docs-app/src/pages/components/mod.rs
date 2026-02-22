mod pages;
mod shell;

use leptos::prelude::*;
use ui::{SegmentedControl, SegmentedControlSize};
use ui_layout::{Card, Flex, FlexDirection, FlexGap, Heading, HeadingLevel};

const GROUP_ORDER: &[&str] = &[
    "Actions",
    "Forms",
    "Layout",
    "Display",
    "Files",
    "Collections",
    "Overlays",
];

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
    let (group_index, set_group_index) = signal::<Option<usize>>(Some(0));

    let group_filter = Memo::new(move |_| match group_index.get() {
        Some(0) | None => None,
        Some(index) => GROUP_ORDER.get(index.saturating_sub(1)).copied(),
    });

    let filtered = Memo::new(move |_| {
        let q = query.get().to_lowercase();
        let group_filter = group_filter.get();
        component_catalog()
            .iter()
            .copied()
            .filter(|doc| {
                if let Some(group_filter) = group_filter
                    && doc.group != group_filter
                {
                    return false;
                }

                if q.trim().is_empty() {
                    true
                } else {
                    doc.name.to_lowercase().contains(&q) || doc.slug.to_lowercase().contains(&q)
                }
            })
            .collect::<Vec<_>>()
    });

    let group_options: Vec<String> = std::iter::once("All".to_string())
        .chain(GROUP_ORDER.iter().copied().map(|group| group.into()))
        .collect();

    view! {
        <Card class_name="docs-prose".to_string()>
            <Heading level=HeadingLevel::H2>"Components"</Heading>
            <p>"Every public component should have at least one playground here."</p>
            <p class="docs-components-summary">
                {move || {
                    let count = filtered.with(|docs| docs.len());
                    if let Some(group) = group_filter.get() {
                        format!("{count} components · {group}")
                    } else {
                        format!("{count} components")
                    }
                }}
            </p>

            <Flex direction=FlexDirection::Column gap=FlexGap::Xs class_name="docs-search".to_string()>
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
            </Flex>

            <Flex direction=FlexDirection::Column gap=FlexGap::Xs class_name="docs-components-filter".to_string()>
                <div class="docs-search__label">"Group"</div>
                <div class="docs-scroll-x docs-scroll-x--segmented">
                    <SegmentedControl
                        id_base="docs-component-group-filter".to_string()
                        options=group_options.clone()
                        selected_index=group_index
                        set_selected_index=set_group_index
                        size=SegmentedControlSize::Sm
                        aria_label="Component group filter".to_string()
                    />
                </div>
            </Flex>
        </Card>

        <Card>
            <Show when=move || filtered.with(|docs| docs.is_empty())>
                <div class="docs-empty">
                    "No matching components."
                </div>
            </Show>

            <Show when=move || !filtered.with(|docs| docs.is_empty())>
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
            </Show>
        </Card>
    }
}

pub use shell::ComponentPage;

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
