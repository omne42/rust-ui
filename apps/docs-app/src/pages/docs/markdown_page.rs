use leptos::prelude::*;

#[component]
pub fn MarkdownPage(markdown: &'static str) -> impl IntoView {
    let rendered = crate::markdown::render_markdown(markdown);

    if let Some(toc) = crate::toc::use_docs_toc() {
        toc.set_items(rendered.toc);
    }

    let html = StoredValue::new(rendered.html);

    view! {
        <section class="docs-card docs-prose">
            <div inner_html=move || html.get_value()></div>
        </section>
    }
}
