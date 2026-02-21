use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use ui_components::{Breadcrumb, BreadcrumbItem, SegmentedControl, SegmentedControlSize, Switch};

const BREADCRUMB_DOC_IMPORTS: &str =
    "use leptos::prelude::*;\nuse ui_components::{Breadcrumb, BreadcrumbItem};";

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

    let stream_mode_options = vec![
        "Snapshot".to_string(),
        "Streaming (fallback=snapshot)".to_string(),
    ];
    let (stream_mode_index, set_stream_mode_index) = signal(Some(0_usize));
    let requested_stream_mode = Signal::derive(move || {
        if stream_mode_index.get().unwrap_or(0) == 0 {
            "snapshot"
        } else {
            "streaming"
        }
    });

    let hello_world_items = vec![
        BreadcrumbItem {
            label: "Home".to_string(),
            href: Some("#/docs/welcome".to_string()),
        },
        BreadcrumbItem {
            label: "Breadcrumb".to_string(),
            href: None,
        },
    ];
    let hello_world_items_for_hello = hello_world_items.clone();
    let hello_world_items_for_state_matrix = hello_world_items.clone();
    let hello_world_items_for_stream = hello_world_items.clone();
    let hello_world_code = Signal::derive(move || {
        r##"let items = vec![
  BreadcrumbItem { label: "Home".to_string(), href: Some("#/docs/welcome".to_string()) },
  BreadcrumbItem { label: "Breadcrumb".to_string(), href: None },
];

<Breadcrumb items=items />"##
            .to_string()
    });

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
            snippet.push("  aria_label=\"Documentation navigation\".into()".to_string());
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
            "/* components/breadcrumb/src/styles.rs */\n{}",
            ui_components::breadcrumb::styles::CSS
        )
    });

    let state_matrix_code = Signal::derive(move || {
        r##"let linked_items = vec![
  BreadcrumbItem { label: "Home".to_string(), href: Some("#/docs/welcome".to_string()) },
  BreadcrumbItem { label: "Components".to_string(), href: Some("#/components".to_string()) },
  BreadcrumbItem { label: "Breadcrumb".to_string(), href: None },
];
let label_only_items = vec![
  BreadcrumbItem { label: "Library".to_string(), href: None },
  BreadcrumbItem { label: "UI".to_string(), href: None },
  BreadcrumbItem { label: "Current".to_string(), href: None },
];
let empty_items: Vec<BreadcrumbItem> = Vec::new();

<Breadcrumb items=linked_items aria_label="Linked trail".to_string() />
<Breadcrumb items=label_only_items aria_label="Label-only trail".to_string() />
<Breadcrumb items=empty_items aria_label="Empty trail".to_string() />"##
            .to_string()
    });

    let controlled_uncontrolled_na_code = Signal::derive(move || {
        r##"// Breadcrumb has no internal mutable axis.
// Controlled/uncontrolled triads are N/A.
let items = vec![
  BreadcrumbItem { label: "Docs".to_string(), href: Some("#/docs".to_string()) },
  BreadcrumbItem { label: "Breadcrumb".to_string(), href: None },
];

<Breadcrumb items=items />"##
            .to_string()
    });

    let streaming_snapshot_code = Signal::derive(move || {
        r##"// Snapshot baseline: render the final trail in one pass.
let items = vec![
  BreadcrumbItem { label: "Docs".to_string(), href: Some("#/docs".to_string()) },
  BreadcrumbItem { label: "Breadcrumb".to_string(), href: None },
];

<Breadcrumb items=items />

// Streaming is optional for Breadcrumb; fallback remains snapshot.
// Inspect markers: data-ui-render-mode / data-ui-stream-support / data-ui-stream-fallback."##
            .to_string()
    });

    let source_first_code = Signal::derive(move || {
        r##"let items = vec![
  BreadcrumbItem { label: "Home".to_string(), href: Some("#/docs/welcome".to_string()) },
  BreadcrumbItem { label: "Components".to_string(), href: Some("#/components".to_string()) },
  BreadcrumbItem { label: "Breadcrumb".to_string(), href: None },
];

<Breadcrumb
  items=items
  aria_label="Documentation navigation".to_string()
  separator="/".to_string()
/>"##
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
                title="Hello World"
                code_signal=hello_world_code
                code_imports=BREADCRUMB_DOC_IMPORTS.to_string()
            >
                <Breadcrumb items=hello_world_items_for_hello />
            </Playground>

            <Playground
                title="Trail"
                code_signal=code
                code_imports=BREADCRUMB_DOC_IMPORTS.to_string()
                test_css_source=test_css_source
                test_source_path="/root/autodl-tmp/zjj/p/rust-ui/components/breadcrumb/src/styles.rs".to_string()
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

            <Playground
                title="State Matrix (Linked / Label-only / Empty)"
                code_signal=state_matrix_code
                code_imports=BREADCRUMB_DOC_IMPORTS.to_string()
            >
                <div class="docs-row" data-slot="breadcrumb-state-matrix">
                    <div class="docs-stack" data-slot="breadcrumb-state-linked">
                        <Breadcrumb
                            items=hello_world_items_for_state_matrix
                            aria_label="Linked trail".to_string()
                        />
                        <span class="ui-muted">"linked items + current page"</span>
                    </div>
                    <div class="docs-stack" data-slot="breadcrumb-state-label-only">
                        <Breadcrumb
                            items=label_only_items
                            aria_label="Label-only trail".to_string()
                        />
                        <span class="ui-muted">"all labels (no links)"</span>
                    </div>
                    <div class="docs-stack" data-slot="breadcrumb-state-empty">
                        <Breadcrumb
                            items=empty_items
                            aria_label="Empty trail".to_string()
                        />
                        <span class="ui-muted">"empty trail (0 items)"</span>
                    </div>
                </div>
            </Playground>

            <Playground
                title="Controlled vs Uncontrolled (N/A)"
                code_signal=controlled_uncontrolled_na_code
                code_imports=BREADCRUMB_DOC_IMPORTS.to_string()
            >
                <div class="docs-stack docs-stack--tight" data-slot="breadcrumb-controlled-uncontrolled-na">
                    <div class="ui-muted">
                        "Breadcrumb has no internal controlled/uncontrolled runtime axis."
                    </div>
                    <div class="ui-muted">
                        "App state can replace `items` directly; component always renders snapshot input."
                    </div>
                </div>
            </Playground>

            <Playground
                title="Streaming / Snapshot Contract"
                description="Breadcrumb 非正文阅读面：Streaming optional，fallback 固定 snapshot。"
                code_signal=streaming_snapshot_code
                code_imports=BREADCRUMB_DOC_IMPORTS.to_string()
            >
                <div
                    class="docs-stack docs-stack--tight"
                    data-slot="breadcrumb-streaming-contract"
                    data-requested-stream-mode=move || requested_stream_mode.get()
                >
                    <div class="docs-row">
                        <SegmentedControl
                            id_base="docs-breadcrumb-stream-mode".to_string()
                            options=stream_mode_options.clone()
                            selected_index=stream_mode_index
                            set_selected_index=set_stream_mode_index
                            size=SegmentedControlSize::Sm
                            aria_label="Breadcrumb requested stream mode".to_string()
                        />
                    </div>
                    <Breadcrumb
                        items=hello_world_items_for_stream
                        aria_label="Streaming contract breadcrumb".to_string()
                    />
                    <span class="ui-muted">
                        "requested mode: "
                        {move || requested_stream_mode.get()}
                    </span>
                    <span class="ui-muted">
                        "effective markers: data-ui-render-mode=snapshot data-ui-stream-support=optional data-ui-stream-fallback=snapshot data-ui-output-status=verified"
                    </span>
                </div>
            </Playground>

            <Playground
                title="Source-first Starter (Copy-Paste Ready)"
                description="Show code 面板默认可复制可运行片段；示例自动补全 imports，并给出真实源码与依赖前提。"
                code_signal=source_first_code
                code_imports=BREADCRUMB_DOC_IMPORTS.to_string()
            >
                <div class="docs-stack docs-stack--tight" data-slot="breadcrumb-source-first">
                    <div class="docs-stack docs-stack--tight" data-slot="breadcrumb-source-first-contract">
                        <h3>"Source-first / Copy-Paste Ready Contract"</h3>
                        <div class="ui-muted">
                            "Open "
                            <code>"Show code"</code>
                            " then use the copy button. Snippets prepend imports automatically."
                        </div>
                        <div class="ui-muted">
                            "docs entry: apps/docs-app/src/pages/components/pages/collections_breadcrumb.rs::breadcrumb"
                        </div>
                    </div>

                    <div
                        class="docs-stack docs-stack--tight"
                        data-slot="breadcrumb-source-first-dependency-baseline"
                    >
                        <div class="docs-search__label">"Dependency baseline (Cargo.toml)"</div>
                        <code>
                            "ui-components = { default-features = false, features = [\"component-breadcrumb\", \"inject-css\"] }"
                        </code>
                    </div>

                    <div class="docs-stack docs-stack--tight" data-slot="breadcrumb-source-paths">
                        <div class="docs-search__label">"Source paths"</div>
                        <div class="ui-muted">"components/breadcrumb/src/mod.rs"</div>
                        <div class="ui-muted">"components/breadcrumb/src/logic.rs"</div>
                        <div class="ui-muted">"components/breadcrumb/src/view.rs"</div>
                        <div class="ui-muted">"components/breadcrumb/src/styles.rs"</div>
                    </div>

                    <div class="ui-muted" data-slot="breadcrumb-source-prerequisites">
                        "Feature prerequisites: component-breadcrumb (inject-css optional for runtime style injection)."
                    </div>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
