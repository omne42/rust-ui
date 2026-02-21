pub struct BreadcrumbItem {
    pub label: String,
    pub href: Option<String>,
}

pub enum BreadcrumbComponentSchemaVersion {
    V1,
}

pub struct BreadcrumbComponentSpec {
    pub schema_version: BreadcrumbComponentSchemaVersion,
}

pub fn Breadcrumb(
    items: Vec<BreadcrumbItem>,
    aria_label: Option<String>,
    class_name: Option<String>,
    separator: Option<String>,
    lang: Option<String>,
    dir: Option<ui_headless::A11yDirection>,
) -> impl leptos::prelude::IntoView;
