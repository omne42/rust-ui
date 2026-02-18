use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use ui_components::{Breadcrumb, BreadcrumbItem, SegmentedControl, SegmentedControlSize, Switch};

pub(super) fn breadcrumb() -> AnyView {
    let scenario_options = vec![
        "trail".to_string(),
        "label_only".to_string(),
        "empty".to_string(),
    ];
    let (scenario_index, set_scenario_index) = signal(Some(0_usize));

    let items = Signal::derive(move || match scenario_index.get().unwrap_or(0) {
        1 => vec![
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
        ],
        2 => Vec::new(),
        _ => vec![
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
        ],
    });
    let (custom_aria_label, set_custom_aria_label) = signal(false);

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

    let code = Signal::derive(move || {
        let items_literal = match scenario_index.get().unwrap_or(0) {
            1 => {
                r##"vec![
  BreadcrumbItem { label: "Library".to_string(), href: None },
  BreadcrumbItem { label: "UI".to_string(), href: None },
  BreadcrumbItem { label: "Current".to_string(), href: None },
]"##
            }
            2 => "Vec::new()",
            _ => {
                r##"vec![
  BreadcrumbItem { label: "Home".to_string(), href: Some("#/docs/welcome".to_string()) },
  BreadcrumbItem { label: "Components".to_string(), href: Some("#/components".to_string()) },
  BreadcrumbItem { label: "Breadcrumb".to_string(), href: None },
]"##
            }
        };

        let mut snippet = vec![
            format!("let items = {items_literal};"),
            String::new(),
            "<Breadcrumb".to_string(),
            "  items=items".to_string(),
        ];

        if custom_aria_label.get() {
            snippet.push("  aria_label=\"Documentation navigation\".to_string()".to_string());
        }

        snippet.extend(["/>".to_string()]);

        snippet.join("\n")
    });

    let scenario_name = Signal::derive(move || match scenario_index.get().unwrap_or(0) {
        1 => "label_only",
        2 => "empty",
        _ => "trail",
    });

    let actual_config = Signal::derive(move || {
        let resolved_items = items.get();
        let linked_items = resolved_items
            .iter()
            .filter(|item| item.href.is_some())
            .count();

        format!(
            "BreadcrumbActualConfig {{\n  scenario: \"{}\",\n  item_count: {},\n  linked_item_count: {},\n  has_custom_aria_label: {},\n  class: \"ui-breadcrumb\",\n}}",
            scenario_name.get(),
            resolved_items.len(),
            linked_items,
            custom_aria_label.get()
        )
    });

    let test_css_source = Signal::derive(move || {
        format!(
            "/* crates/ui-components/src/breadcrumb/styles.rs */\n{}",
            ui_components::breadcrumb::styles::CSS
        )
    });

    let states_code = Signal::derive(move || {
        r#"<Breadcrumb
  items=vec![
    BreadcrumbItem { label: "Library".to_string(), href: None },
    BreadcrumbItem { label: "UI".to_string(), href: None },
    BreadcrumbItem { label: "Current".to_string(), href: None },
  ]
  aria_label="Label-only trail".to_string()
/>
<Breadcrumb items=empty_items aria_label="Empty trail".to_string() />"#
            .to_string()
    });

    view! {
        <ComponentPage
            title="Breadcrumb"
            slug="breadcrumb"
            group="Collections"
            description="baseline-compatible breadcrumb navigation with current-page semantics and baseline-style root state attrs."
        >
            <Playground
                title="Trail"
                code_signal=code
                test_css_source=test_css_source
                test_source_path="/root/autodl-tmp/zjj/p/rust-ui/crates/ui-components/src/breadcrumb/styles.rs".to_string()
                test_config_signal=actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight">
                        <div class="docs-search__label">"Scenario"</div>
                        <SegmentedControl
                            id_base="docs-breadcrumb-scenario".to_string()
                            options=scenario_options.clone()
                            selected_index=scenario_index
                            set_selected_index=set_scenario_index
                            size=SegmentedControlSize::Sm
                            aria_label="Breadcrumb scenario".to_string()
                        />

                        <Switch checked=custom_aria_label set_checked=set_custom_aria_label>
                            "Custom aria label"
                        </Switch>
                    </div>
                }
            >
                {move || {
                    let items = items.get();

                    if custom_aria_label.get() {
                        view! {
                            <Breadcrumb
                                items=items
                                aria_label="Documentation navigation".to_string()
                            />
                        }
                            .into_any()
                    } else {
                        view! { <Breadcrumb items=items /> }.into_any()
                    }
                }}
            </Playground>

            <Playground title="Label-Only + Empty" code_signal=states_code>
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
