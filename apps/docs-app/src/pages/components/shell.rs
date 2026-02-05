use leptos::prelude::*;

#[component]
pub fn ComponentPage(
    title: &'static str,
    slug: &'static str,
    group: &'static str,
    #[prop(optional)] description: &'static str,
    children: Children,
) -> impl IntoView {
    let description = (!description.trim().is_empty()).then_some(description);

    view! {
        <section class="docs-card docs-prose docs-page-header">
            <div class="docs-page-header__top">
                <div>
                    <h2 class="docs-page-title">{title}</h2>
                    {description.map(|description| view! {
                        <p class="docs-page-description">{description}</p>
                    })}
                </div>
                <a class="docs-page-back" href="#/components">
                    "All components"
                </a>
            </div>
            <div class="docs-page-meta">
                <span class="docs-page-group">{group}</span>
                <code class="docs-page-slug">{slug}</code>
            </div>
        </section>

        {children()}
    }
}
