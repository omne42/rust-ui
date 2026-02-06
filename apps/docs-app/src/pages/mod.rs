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
    let path = crate::route::route_path(&route);

    if let Some(page) = docs::doc_page(path) {
        return page;
    }

    match path {
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
