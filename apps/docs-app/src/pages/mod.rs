pub mod components;
pub mod docs;
pub mod nav;

use leptos::prelude::*;
use ui_layout::{Card, Heading, HeadingLevel};

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
        <Card class_name="docs-prose".to_string()>
            <Heading level=HeadingLevel::H2>"Not found"</Heading>
            <p>
                "Unknown route: " <code>{route}</code>
            </p>
        </Card>
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
