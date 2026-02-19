pub mod components;
pub mod docs;
pub mod nav;

use leptos::prelude::*;

pub fn title_for_path(path: &str) -> String {
    if let Some(doc) = docs::docs_catalog().iter().find(|doc| doc.route == path) {
        return doc.title.into();
    }

    if path == "components" {
        return "Components".to_string();
    }

    if let Some(slug) = path.strip_prefix("components/")
        && let Some(doc) = components::component_catalog()
            .iter()
            .find(|doc| doc.slug == slug)
    {
        return doc.name.into();
    }

    "Not found".to_string()
}

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
