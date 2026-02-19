mod pages;
mod shell;

use leptos::prelude::*;
use ui_components::{SegmentedControl, SegmentedControlSize};
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
mod tests {
    use super::*;
    use leptos::prelude::Owner;
    use std::{collections::HashSet, fs, path::Path};

    fn component_module_slugs() -> Vec<String> {
        let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
        let lib_rs = manifest_dir.join("../../crates/ui-components/src/lib.rs");
        let source = fs::read_to_string(&lib_rs)
            .unwrap_or_else(|err| panic!("failed to read {}: {err}", lib_rs.display()));

        source
            .lines()
            .filter_map(|line| {
                let line = line.trim();
                line.strip_prefix("pub mod ")
                    .and_then(|rest| rest.strip_suffix(';'))
                    .map(|module| module.replace('_', "-"))
            })
            .collect()
    }

    fn expected_doc_slugs(module_slug: &str) -> &'static [&'static str] {
        match module_slug {
            "button-flip" => &["flip-button"],
            "button-search-input" => &["search-input-button"],
            "button-share" => &["share-button"],
            "button-theme-toggle" => &["theme-toggle-button"],
            "direction" => &["direction-provider"],
            "number" => &["static-number", "sliding-number"],
            "ripple" => &["motion-ripple"],
            "root" => &["ui-root"],
            "layout" => &["flex", "grid"],
            "group" => &["field-group"],
            "overlays" => &["overlay", "popover", "modal", "tray"],
            "overlay-arrow" => &["icon", "popover"],
            "collection" => &["item"],
            "color" => &[
                "color-picker",
                "color-field",
                "color-area",
                "color-slider",
                "color-wheel",
                "color-swatch",
                "color-editor",
                "color-swatch-picker",
            ],
            "area" => &["color-area"],
            "editor" => &["color-editor"],
            "handle" => &["color-handle"],
            "loupe" => &["color-loupe"],
            "swatch-picker" => &["color-swatch-picker"],
            "thumb" => &["color-thumb"],
            "wheel" => &["color-wheel"],
            "field-form" => &["field"],
            "list" => &["list", "list-item", "list-section"],
            "selection-indicator" => &["list-item", "menu-item"],
            "shared-element-transition" => &["view"],
            "virtualizer" => &["scroll-area"],
            "hidden-date-input" => &["date-input-group"],
            "dnd" => &["drop-zone", "file-trigger"],
            "drag-and-drop" => &["drop-zone", "file-trigger"],
            "theme-dark" => &["ui-root"],
            "theme-default" => &["ui-root"],
            "theme-express" => &["ui-root"],
            "theme-light" => &["ui-root"],
            "example-theme" => &["ui-root"],
            "spinbutton" => &["number-field"],
            "text-input" => &["input"],
            "toast" => &["toast-viewport"],
            "toolbar" => &["action-bar"],
            "ai-space" => &["accordion"],
            "active-highlight" => &[],
            _ => &[],
        }
    }

    #[test]
    fn component_catalog_covers_public_component_modules() {
        let doc_slugs: HashSet<&str> = component_catalog().iter().map(|doc| doc.slug).collect();
        let mut missing = Vec::new();

        for module_slug in component_module_slugs() {
            let mapped = expected_doc_slugs(&module_slug);
            if mapped.is_empty() && module_slug == "active-highlight" {
                continue;
            }

            let covered = if mapped.is_empty() {
                doc_slugs.contains(module_slug.as_str())
            } else {
                mapped.iter().any(|slug| doc_slugs.contains(*slug))
            };

            if !covered {
                let expected = if mapped.is_empty() {
                    module_slug.clone()
                } else {
                    mapped.join(" | ")
                };
                missing.push(format!("{module_slug} -> {expected}"));
            }
        }

        assert!(
            missing.is_empty(),
            "docs catalog is missing component module coverage:\n{}",
            missing.join("\n")
        );
    }

    #[test]
    fn every_component_doc_page_renders_at_least_one_playground() {
        drop(any_spawner::Executor::init_futures_executor());
        for doc in component_catalog().iter().copied() {
            Owner::new().with(|| {
                let _toc = crate::toc::provide_docs_toc();
                let registry = crate::playground::provide_playground_registry();

                drop((doc.page)());
                let titles = registry.titles().get_untracked();
                assert!(
                    !titles.is_empty(),
                    "component page `{}` is missing a <Playground>",
                    doc.slug
                );
            });
        }
    }
}
