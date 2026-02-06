use super::component_catalog;
use leptos::prelude::*;
use ui_components::Snippet;

#[component]
pub fn ComponentPage(
    title: &'static str,
    slug: &'static str,
    group: &'static str,
    #[prop(optional)] description: &'static str,
    children: Children,
) -> impl IntoView {
    let description = (!description.trim().is_empty()).then_some(description);

    let (prev, next) = {
        let catalog = component_catalog();
        let current = catalog.iter().position(|doc| doc.slug == slug);
        current
            .map(|idx| {
                let prev = idx
                    .checked_sub(1)
                    .and_then(|prev_idx| catalog.get(prev_idx).copied());
                let next = catalog.get(idx.saturating_add(1)).copied();
                (prev, next)
            })
            .unwrap_or((None, None))
    };

    let import_text = format!("use ui_components::{title};");

    view! {
        <section class="docs-card docs-prose docs-page-header">
            <div class="docs-page-header__top">
                <div>
                    <h2 class="docs-page-title">{title}</h2>
                    {description.map(|description| view! {
                        <p class="docs-page-description">{description}</p>
                    })}
                </div>
                <div class="docs-page-header__actions">
                    {prev.map(|doc| view! {
                        <a class="docs-page-nav" href=format!("#/components/{}", doc.slug)>
                            "← " {doc.name}
                        </a>
                    })}
                    {next.map(|doc| view! {
                        <a class="docs-page-nav" href=format!("#/components/{}", doc.slug)>
                            {doc.name} " →"
                        </a>
                    })}
                    <a class="docs-page-back" href="#/components">
                        "All components"
                    </a>
                </div>
            </div>
            <div class="docs-page-meta">
                <div class="docs-page-meta__left">
                    <span class="docs-page-group">{group}</span>
                    <code class="docs-page-slug">{slug}</code>
                </div>
                <Snippet
                    text=import_text
                    label="Import".to_string()
                    copyable=true
                    class_name="docs-page-import".to_string()
                />
            </div>
        </section>

        {children()}
    }
}
