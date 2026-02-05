pub mod components;
pub mod docs;
pub mod nav;

use leptos::prelude::*;

fn not_found(route: String) -> AnyView {
    view! {
        <section class="docs-card docs-prose">
            <h2>"Not found"</h2>
            <p>
                "Unknown route: " <code>{route}</code>
            </p>
        </section>
    }
    .into_any()
}

pub fn route_view(route: String) -> AnyView {
    match route.as_str() {
        "docs/welcome" => docs::Welcome().into_any(),
        "docs/rules" => docs::Rules().into_any(),
        "components" => components::ComponentsIndex().into_any(),
        other => {
            if let Some(slug) = other.strip_prefix("components/") {
                if let Some(page) = components::component_page(slug) {
                    page
                } else {
                    not_found(route)
                }
            } else {
                not_found(route)
            }
        }
    }
}
