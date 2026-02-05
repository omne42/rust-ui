use crate::markdown::markdown_to_html;
use leptos::prelude::*;

const RULES_MD: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../docs/RULES_ZH.md"
));

#[component]
pub fn Rules() -> impl IntoView {
    let html = StoredValue::new(markdown_to_html(RULES_MD));
    view! {
        <section class="docs-card docs-prose">
            <div inner_html=move || html.get_value()></div>
        </section>
    }
}
