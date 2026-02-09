use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use ui_components::{Breadcrumb, BreadcrumbItem};

pub(super) fn breadcrumb() -> AnyView {
    let items = vec![
        BreadcrumbItem {
            label: "Home".to_string(),
            href: Some("#/docs/welcome".to_string()),
        },
        BreadcrumbItem {
            label: "Components".to_string(),
            href: Some("#/components".to_string()),
        },
        BreadcrumbItem {
            label: "Breadcrumb".to_string(),
            href: None,
        },
    ];

    let label_only_items = vec![
        BreadcrumbItem {
            label: "Library".to_string(),
            href: None,
        },
        BreadcrumbItem {
            label: "UI".to_string(),
            href: None,
        },
        BreadcrumbItem {
            label: "Current".to_string(),
            href: None,
        },
    ];
    let empty_items = Vec::<BreadcrumbItem>::new();

    let code = r##"let items = vec![
  BreadcrumbItem { label: "Home".to_string(), href: Some("#/docs/welcome".to_string()) },
  BreadcrumbItem { label: "Components".to_string(), href: Some("#/components".to_string()) },
  BreadcrumbItem { label: "Breadcrumb".to_string(), href: None },
];
<Breadcrumb items=items />"##;

    let states_code = r#"<Breadcrumb
  items=vec![
    BreadcrumbItem { label: "Library".to_string(), href: None },
    BreadcrumbItem { label: "UI".to_string(), href: None },
    BreadcrumbItem { label: "Current".to_string(), href: None },
  ]
  aria_label="Label-only trail".to_string()
/>
<Breadcrumb items=empty_items aria_label="Empty trail".to_string() />"#;

    view! {
        <ComponentPage
            title="Breadcrumb"
            slug="breadcrumb"
            group="Collections"
            description="Shadcn-compatible breadcrumb navigation with current-page semantics and Spectrum-style root state attrs."
        >
            <Playground title="Trail" code=code>
                <Breadcrumb items=items />
            </Playground>

            <Playground title="Label-Only + Empty" code=states_code>
                <div class="docs-row">
                    <div class="docs-stack">
                        <Breadcrumb
                            items=label_only_items
                            aria_label="Label-only trail".to_string()
                        />
                        <span class="ui-muted">"all labels (no links)"</span>
                    </div>
                    <div class="docs-stack">
                        <Breadcrumb
                            items=empty_items
                            aria_label="Empty trail".to_string()
                        />
                        <span class="ui-muted">"empty trail (0 items)"</span>
                    </div>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
