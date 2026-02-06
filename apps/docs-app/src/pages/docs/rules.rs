use leptos::prelude::*;

const RULES_MD: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../docs/RULES_ZH.md"
));

#[component]
pub fn Rules() -> impl IntoView {
    let rendered = crate::markdown::render_markdown(RULES_MD);
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
