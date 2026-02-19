use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use std::collections::BTreeSet;
use std::sync::Arc;
use ui_components::{
    Accordion, AccordionItem, AccordionSelectionMode, AccordionStreamingProjection,
    AccordionVariant, AiOutputStatus, AiRenderMode, AiSpace, Autocomplete, BreadcrumbItem,
    Breadcrumbs, ComboBox, Disclosure, DropdownMenu, DropdownMenuMotion, List, Menu, MenuItemKind,
    MenuTrigger, Pagination, SegmentedControl, SegmentedControlSize, Select, Switch, Tabs,
    TabsKeyboardActivation, Tag, TagGroup, open_set, project_streaming_accordion_markup,
};
use ui_headless::PopoverPlacement;

#[cfg(target_arch = "wasm32")]
const ACCORDION_WORKBENCH_STORAGE_KEY: &str = "docs:accordion:workbench:open";

#[cfg(target_arch = "wasm32")]
fn encode_open_set(indices: &BTreeSet<usize>) -> String {
    indices
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join(",")
}

#[cfg(target_arch = "wasm32")]
fn decode_open_set(raw: &str) -> BTreeSet<usize> {
    raw.split(',')
        .filter_map(|part| part.trim().parse::<usize>().ok())
        .collect()
}

#[cfg(target_arch = "wasm32")]
fn load_workbench_open() -> Option<BTreeSet<usize>> {
    let storage = web_sys::window().and_then(|window| window.local_storage().ok().flatten())?;
    let raw = storage
        .get_item(ACCORDION_WORKBENCH_STORAGE_KEY)
        .ok()
        .flatten()?;
    Some(decode_open_set(&raw))
}

#[cfg(not(target_arch = "wasm32"))]
fn load_workbench_open() -> Option<BTreeSet<usize>> {
    None
}

#[cfg(target_arch = "wasm32")]
fn save_workbench_open(indices: &BTreeSet<usize>) {
    if let Some(storage) =
        web_sys::window().and_then(|window| window.local_storage().ok().flatten())
    {
        drop(storage.set_item(ACCORDION_WORKBENCH_STORAGE_KEY, &encode_open_set(indices)));
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn save_workbench_open(_indices: &BTreeSet<usize>) {}

#[cfg(target_arch = "wasm32")]
fn clear_workbench_open() {
    if let Some(storage) =
        web_sys::window().and_then(|window| window.local_storage().ok().flatten())
    {
        drop(storage.remove_item(ACCORDION_WORKBENCH_STORAGE_KEY));
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn clear_workbench_open() {}

#[cfg(target_arch = "wasm32")]
const TABS_WORKBENCH_STORAGE_KEY: &str = "docs:tabs:workbench:selected";

#[cfg(target_arch = "wasm32")]
fn load_tabs_workbench_selected() -> Option<usize> {
    let storage = web_sys::window().and_then(|window| window.local_storage().ok().flatten())?;
    let raw = storage
        .get_item(TABS_WORKBENCH_STORAGE_KEY)
        .ok()
        .flatten()?;
    raw.parse::<usize>().ok()
}

#[cfg(not(target_arch = "wasm32"))]
fn load_tabs_workbench_selected() -> Option<usize> {
    None
}

#[cfg(target_arch = "wasm32")]
fn save_tabs_workbench_selected(selected_index: usize) {
    if let Some(storage) =
        web_sys::window().and_then(|window| window.local_storage().ok().flatten())
    {
        let selected_index_attr = selected_index.to_string();
        drop(storage.set_item(TABS_WORKBENCH_STORAGE_KEY, &selected_index_attr));
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn save_tabs_workbench_selected(_selected_index: usize) {}

#[cfg(target_arch = "wasm32")]
fn clear_tabs_workbench_selected() {
    if let Some(storage) =
        web_sys::window().and_then(|window| window.local_storage().ok().flatten())
    {
        drop(storage.remove_item(TABS_WORKBENCH_STORAGE_KEY));
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn clear_tabs_workbench_selected() {}

const ACCORDION_STREAMING_ITEM_LABELS: [&str; 3] = ["Chunk #1", "Chunk #2", "Chunk #3"];
const ACCORDION_STREAMING_TOTAL_ITEMS: usize = ACCORDION_STREAMING_ITEM_LABELS.len();
const ACCORDION_STREAMING_FULL_CODE: &str = r#"<Accordion
  id_base="docs-accordion-ai-stream".to_string()
  selection_mode=AccordionSelectionMode::Multiple
  variant=AccordionVariant::Splitted
>
    <AccordionItem label="Chunk #1">"First completed item from AI output."</AccordionItem>
    <AccordionItem label="Chunk #2">"Second completed item, mounted incrementally."</AccordionItem>
    <AccordionItem label="Chunk #3">"Final completed item."</AccordionItem>
</Accordion>"#;
const ACCORDION_STREAMING_MANUAL_STEP: usize = 20;
const ACCORDION_STREAMING_AUTO_STEP: usize = 2;
const ACCORDION_STREAMING_AUTO_INTERVAL_MS: u64 = 100;
const ACCORDION_STREAMING_AUTO_RESET_DELAY_MS: u64 = 3000;

fn count_chars(input: &str) -> usize {
    input.chars().count()
}

fn take_chars(input: &str, count: usize) -> String {
    input.chars().take(count).collect()
}

fn derive_item_open(open: ReadSignal<BTreeSet<usize>>, key: usize) -> Signal<bool> {
    Signal::derive(move || open.get().contains(&key))
}

fn on_item_open_change(set_open: WriteSignal<BTreeSet<usize>>, key: usize) -> Callback<bool> {
    Callback::new(move |is_open: bool| {
        set_open.update(|open| {
            if is_open {
                open.insert(key);
            } else {
                open.remove(&key);
            }
        });
    })
}

fn snapshot_open(indices: &BTreeSet<usize>, visible_items: usize) -> BTreeSet<usize> {
    indices
        .iter()
        .copied()
        .filter(|index| *index < visible_items)
        .collect()
}

fn escape_rust_string_literal(input: &str) -> String {
    input.replace('\\', "\\\\").replace('"', "\\\"")
}

fn build_streaming_snapshot_code(
    projection: &AccordionStreamingProjection,
    open: &BTreeSet<usize>,
    _mode: AiRenderMode,
    _status: AiOutputStatus,
) -> String {
    if !projection.has_root_open || projection.items.is_empty() {
        return String::new();
    }

    let visible_items = projection.items.len().min(ACCORDION_STREAMING_TOTAL_ITEMS);
    let open = snapshot_open(open, visible_items);
    let mut out = String::new();

    out.push_str("<Accordion\n");
    out.push_str("  id_base=\"docs-accordion-ai-stream\".into()\n");
    out.push_str("  selection_mode=AccordionSelectionMode::Multiple\n");
    out.push_str("  variant=AccordionVariant::Splitted\n");
    out.push_str(">\n");

    for (key, item) in projection
        .items
        .iter()
        .take(ACCORDION_STREAMING_TOTAL_ITEMS)
        .enumerate()
    {
        let label = escape_rust_string_literal(&item.label);
        let text = escape_rust_string_literal(&item.text);
        let is_open = open.contains(&key);
        out.push_str(&format!(
            "  <AccordionItem key={key} label=\"{label}\".into() default_open={is_open}>\"{text}\"</AccordionItem>\n"
        ));
    }

    out.push_str("</Accordion>");
    out
}

fn compose_streaming_demo_code(
    input_code: &str,
    projection: &AccordionStreamingProjection,
    open: &BTreeSet<usize>,
    mode: AiRenderMode,
    status: AiOutputStatus,
) -> String {
    let snapshot = build_streaming_snapshot_code(projection, open, mode, status);
    if snapshot.is_empty() {
        input_code.into()
    } else if input_code.trim().is_empty() {
        format!("SNAPSHOT\n{snapshot}")
    } else {
        format!("STREAMING_INPUT\n{input_code}\n\nSNAPSHOT\n{snapshot}")
    }
}

pub(super) fn breadcrumbs() -> AnyView {
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
            label: "Breadcrumbs".to_string(),
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
    let items_compare = items.clone();
    let label_only_items_compare = label_only_items.clone();
    let empty_items_compare = empty_items.clone();
    let (workbench_include_links, set_workbench_include_links) = signal(true);
    let (workbench_empty, set_workbench_empty) = signal(false);
    let (workbench_long_trail, set_workbench_long_trail) = signal(false);
    let (workbench_custom_aria, set_workbench_custom_aria) = signal(false);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);

    let workbench_items = Signal::derive(move || {
        if workbench_empty.get() {
            return Vec::<BreadcrumbItem>::new();
        }

        if workbench_long_trail.get() {
            let mut long_items = vec![
                BreadcrumbItem {
                    label: "Home".to_string(),
                    href: Some("#/docs".to_string()),
                },
                BreadcrumbItem {
                    label: "Design".to_string(),
                    href: Some("#/docs/design".to_string()),
                },
                BreadcrumbItem {
                    label: "Tokens".to_string(),
                    href: Some("#/docs/design/tokens".to_string()),
                },
                BreadcrumbItem {
                    label: "Navigation".to_string(),
                    href: Some("#/docs/design/tokens/navigation".to_string()),
                },
                BreadcrumbItem {
                    label: "Current".to_string(),
                    href: None,
                },
            ];

            if !workbench_include_links.get() {
                let non_last = long_items.len().saturating_sub(1);
                for item in long_items.iter_mut().take(non_last) {
                    item.href = None;
                }
            }

            return long_items;
        }

        if workbench_include_links.get() {
            vec![
                BreadcrumbItem {
                    label: "Home".to_string(),
                    href: Some("#/docs/welcome".to_string()),
                },
                BreadcrumbItem {
                    label: "Collections".to_string(),
                    href: Some("#/components/collections".to_string()),
                },
                BreadcrumbItem {
                    label: "Breadcrumbs".to_string(),
                    href: None,
                },
            ]
        } else {
            vec![
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
            ]
        }
    });

    let code = Signal::derive(move || {
        r##"let items = vec![
  BreadcrumbItem { label: "Home".to_string(), href: Some("#/docs/welcome".to_string()) },
  BreadcrumbItem { label: "Components".to_string(), href: Some("#/components".to_string()) },
  BreadcrumbItem { label: "Breadcrumbs".to_string(), href: None },
];
<Breadcrumbs items=items />"##
            .to_string()
    });

    let states_code = Signal::derive(move || {
        r#"<Breadcrumbs
  items=vec![
    BreadcrumbItem { label: "Library".to_string(), href: None },
    BreadcrumbItem { label: "UI".to_string(), href: None },
    BreadcrumbItem { label: "Current".to_string(), href: None },
  ]
  aria_label="Label-only trail".to_string()
/>
<Breadcrumbs items=Vec::<BreadcrumbItem>::new() aria_label="Empty trail".to_string() />"#
            .to_string()
    });
    let workbench_code = Signal::derive(move || {
        let include_links = workbench_include_links.get();
        let use_empty = workbench_empty.get();
        let use_long_trail = workbench_long_trail.get();
        let use_custom_aria = workbench_custom_aria.get();
        let use_custom_class = workbench_custom_class.get();
        let mut lines: Vec<String> = vec!["<Breadcrumbs".into()];

        if use_empty {
            lines.push("  items=Vec::<BreadcrumbItem>::new()".into());
        } else if use_long_trail {
            lines.push("  items=vec![".into());
            lines.push(format!(
                "    BreadcrumbItem {{ label: \"Home\".to_string(), href: {} }},",
                if include_links {
                    "Some(\"#/docs\".to_string())"
                } else {
                    "None"
                }
            ));
            lines.push(format!(
                "    BreadcrumbItem {{ label: \"Design\".to_string(), href: {} }},",
                if include_links {
                    "Some(\"#/docs/design\".to_string())"
                } else {
                    "None"
                }
            ));
            lines.push(format!(
                "    BreadcrumbItem {{ label: \"Tokens\".to_string(), href: {} }},",
                if include_links {
                    "Some(\"#/docs/design/tokens\".to_string())"
                } else {
                    "None"
                }
            ));
            lines.push(format!(
                "    BreadcrumbItem {{ label: \"Navigation\".to_string(), href: {} }},",
                if include_links {
                    "Some(\"#/docs/design/tokens/navigation\".to_string())"
                } else {
                    "None"
                }
            ));
            lines.push("    BreadcrumbItem { label: \"Current\".to_string(), href: None },".into());
            lines.push("  ]".into());
        } else {
            lines.push("  items=vec![".into());
            lines.push(format!(
                "    BreadcrumbItem {{ label: \"Home\".to_string(), href: {} }},",
                if include_links {
                    "Some(\"#/docs/welcome\".to_string())"
                } else {
                    "None"
                }
            ));
            lines.push(format!(
                "    BreadcrumbItem {{ label: \"Collections\".to_string(), href: {} }},",
                if include_links {
                    "Some(\"#/components/collections\".to_string())"
                } else {
                    "None"
                }
            ));
            lines.push(
                "    BreadcrumbItem { label: \"Breadcrumbs\".to_string(), href: None },".into(),
            );
            lines.push("  ]".into());
        }

        if use_custom_aria {
            lines.push("  aria_label=\"Docs breadcrumb trail\".to_string()".into());
        }
        if use_custom_class {
            lines.push("  class_name=\"docs-breadcrumbs-custom\".to_string()".into());
        }

        lines.push("/>".into());
        lines.join("\n")
    });
    let test_css_source = Signal::derive(move || {
        format!(
            "/* crates/ui-components/src/breadcrumbs/styles.rs */\n{}",
            ui_components::breadcrumbs::styles::CSS
        )
    });
    let actual_config = Signal::derive(move || {
        let items = workbench_items.get();
        let item_count = items.len();
        let has_links = items
            .iter()
            .enumerate()
            .any(|(index, item)| index + 1 < item_count && item.href.is_some());
        let data_state = if item_count == 0 {
            "empty"
        } else if has_links {
            "with-links"
        } else {
            "label-only"
        };
        let aria_source = if workbench_custom_aria.get() {
            "custom"
        } else {
            "default"
        };
        let class_source = if workbench_custom_class.get() {
            "custom"
        } else {
            "default"
        };
        let class_name = if workbench_custom_class.get() {
            "ui-breadcrumbs docs-breadcrumbs-custom"
        } else {
            "ui-breadcrumbs"
        };

        format!(
            "BreadcrumbsActualConfig {{\n  item_count: {item_count},\n  has_links: {has_links},\n  data_state: \"{data_state}\",\n  aria_source: \"{aria_source}\",\n  class_source: \"{class_source}\",\n  class: \"{class_name}\",\n}}"
        )
    });

    view! {
        <ComponentPage
            title="Breadcrumbs"
            slug="breadcrumbs"
            group="Collections"
            description="Breadcrumb nav with current-page semantics and baseline-style root state attrs."
        >
            <Playground title="Trail" code_signal=code>
                <Breadcrumbs items=items.clone() />
            </Playground>

            <Playground title="Label-Only + Empty" code_signal=states_code>
                <div class="docs-row">
                    <div class="docs-stack">
                        <Breadcrumbs
                            items=label_only_items.clone()
                            aria_label="Label-only trail".to_string()
                        />
                        <span class="ui-muted">"all labels (no links)"</span>
                    </div>
                    <div class="docs-stack">
                        <Breadcrumbs items=empty_items.clone() aria_label="Empty trail".to_string() />
                        <span class="ui-muted">"empty trail (0 items)"</span>
                    </div>
                </div>
            </Playground>

            <Playground
                title="Workbench (Display + Config + Code + CSS Test)"
                description="展示区提供当前配置与多场景对比；Config/Code/CSS Test 区用于契约回归。"
                code_signal=workbench_code
                test_css_source=test_css_source
                test_source_path="/root/autodl-tmp/zjj/p/rust-ui/crates/ui-components/src/breadcrumbs/styles.rs".to_string()
                test_config_signal=actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="breadcrumbs-workbench-controls">
                        <div class="docs-row">
                            <ui_components::Button
                                variant=ui_components::ButtonVariant::Secondary
                                on_press=Callback::new(move |_| {
                                    set_workbench_include_links.update(|v| *v = !*v)
                                })
                            >
                                {move || if workbench_include_links.get() {
                                    "Links: on"
                                } else {
                                    "Links: off"
                                }}
                            </ui_components::Button>
                            <ui_components::Button
                                variant=ui_components::ButtonVariant::Secondary
                                on_press=Callback::new(move |_| {
                                    set_workbench_empty.update(|v| *v = !*v)
                                })
                            >
                                {move || if workbench_empty.get() {
                                    "Empty: on"
                                } else {
                                    "Empty: off"
                                }}
                            </ui_components::Button>
                        </div>

                        <div class="docs-row">
                            <ui_components::Button
                                variant=ui_components::ButtonVariant::Secondary
                                on_press=Callback::new(move |_| {
                                    set_workbench_long_trail.update(|v| *v = !*v)
                                })
                            >
                                {move || if workbench_long_trail.get() {
                                    "Trail: long"
                                } else {
                                    "Trail: default"
                                }}
                            </ui_components::Button>
                            <ui_components::Button
                                variant=ui_components::ButtonVariant::Secondary
                                on_press=Callback::new(move |_| {
                                    set_workbench_custom_aria.update(|v| *v = !*v)
                                })
                            >
                                {move || if workbench_custom_aria.get() {
                                    "Aria: custom"
                                } else {
                                    "Aria: default"
                                }}
                            </ui_components::Button>
                            <ui_components::Button
                                variant=ui_components::ButtonVariant::Secondary
                                on_press=Callback::new(move |_| {
                                    set_workbench_custom_class.update(|v| *v = !*v)
                                })
                            >
                                {move || if workbench_custom_class.get() {
                                    "Class: custom"
                                } else {
                                    "Class: default"
                                }}
                            </ui_components::Button>
                        </div>
                    </div>
                }
            >
                <div class="docs-stack" data-slot="breadcrumbs-workbench-preview">
                    <div class="docs-stack docs-stack--tight">
                        <span class="ui-muted">"当前配置"</span>
                        {move || {
                            let items = workbench_items.get();
                            let aria_label = if workbench_custom_aria.get() {
                                "Docs breadcrumb trail".to_string()
                            } else {
                                String::new()
                            };
                            let class_name = if workbench_custom_class.get() {
                                "docs-breadcrumbs-custom".to_string()
                            } else {
                                String::new()
                            };

                            view! {
                                <Breadcrumbs
                                    items=items
                                    aria_label=aria_label
                                    class_name=class_name
                                />
                            }
                        }}
                    </div>

                    <div class="docs-row">
                        <div class="docs-stack docs-stack--tight" style="min-width: 16rem;">
                            <span class="ui-muted">"对比：With links"</span>
                            <Breadcrumbs items=items_compare />
                        </div>
                        <div class="docs-stack docs-stack--tight" style="min-width: 16rem;">
                            <span class="ui-muted">"对比：Label-only"</span>
                            <Breadcrumbs
                                items=label_only_items_compare
                                aria_label="Label-only trail".to_string()
                            />
                        </div>
                        <div class="docs-stack docs-stack--tight" style="min-width: 16rem;">
                            <span class="ui-muted">"对比：Empty"</span>
                            <Breadcrumbs
                                items=empty_items_compare
                                aria_label="Empty trail".to_string()
                            />
                        </div>
                    </div>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn accordion() -> AnyView {
    let (open_multi, set_open_multi) = signal(open_set([0]));
    let open_multi_0 = derive_item_open(open_multi, 0);
    let open_multi_1 = derive_item_open(open_multi, 1);
    let open_multi_2 = derive_item_open(open_multi, 2);
    let on_multi_0_change = on_item_open_change(set_open_multi, 0);
    let on_multi_1_change = on_item_open_change(set_open_multi, 1);
    let on_multi_2_change = on_item_open_change(set_open_multi, 2);

    let (open_single, set_open_single) = signal(open_set([1]));
    let open_single_0 = derive_item_open(open_single, 0);
    let open_single_1 = derive_item_open(open_single, 1);
    let open_single_2 = derive_item_open(open_single, 2);
    let on_single_0_change = on_item_open_change(set_open_single, 0);
    let on_single_1_change = on_item_open_change(set_open_single, 1);
    let on_single_2_change = on_item_open_change(set_open_single, 2);

    let persisted_workbench_open = load_workbench_open();
    let (workbench_open, set_workbench_open) = signal(
        persisted_workbench_open
            .clone()
            .unwrap_or_else(|| open_set([0])),
    );
    let workbench_open_0 = derive_item_open(workbench_open, 0);
    let workbench_open_1 = derive_item_open(workbench_open, 1);
    let workbench_open_2 = derive_item_open(workbench_open, 2);
    let on_workbench_0_change = on_item_open_change(set_workbench_open, 0);
    let on_workbench_1_change = on_item_open_change(set_workbench_open, 1);
    let on_workbench_2_change = on_item_open_change(set_workbench_open, 2);
    let (workbench_multiple_mode, set_workbench_multiple_mode) = signal(true);
    let (workbench_disable_second, set_workbench_disable_second) = signal(false);
    let (workbench_persist_state, set_workbench_persist_state) =
        signal(persisted_workbench_open.is_some());
    let (workbench_radius_px, set_workbench_radius_px) = signal(12_u16);
    let (workbench_hover_alpha, set_workbench_hover_alpha) = signal(10_u16);
    let snapshot_mode = Signal::derive(|| AiRenderMode::Snapshot);
    let verified_output = Signal::derive(|| AiOutputStatus::Verified);

    let streaming_total_items = ACCORDION_STREAMING_TOTAL_ITEMS;
    let streaming_total_chars = count_chars(ACCORDION_STREAMING_FULL_CODE);
    let (streaming_chars, set_streaming_chars) = signal(0_usize);
    let (streaming_auto, set_streaming_auto) = signal(false);
    let (streaming_open_set, set_streaming_open_set) = signal(open_set([]));
    let streaming_open_0 = derive_item_open(streaming_open_set, 0);
    let streaming_open_1 = derive_item_open(streaming_open_set, 1);
    let streaming_open_2 = derive_item_open(streaming_open_set, 2);
    let on_streaming_0_change = on_item_open_change(set_streaming_open_set, 0);
    let on_streaming_1_change = on_item_open_change(set_streaming_open_set, 1);
    let on_streaming_2_change = on_item_open_change(set_streaming_open_set, 2);

    let streaming_chars_clamped =
        Signal::derive(move || streaming_chars.get().min(streaming_total_chars));
    let streaming_input_code = Signal::derive(move || {
        take_chars(ACCORDION_STREAMING_FULL_CODE, streaming_chars_clamped.get())
    });
    let streaming_projection =
        Signal::derive(move || project_streaming_accordion_markup(&streaming_input_code.get()));
    let streaming_is_complete = Signal::derive(move || streaming_projection.get().is_complete());
    let streaming_mode = Signal::derive(move || {
        if streaming_is_complete.get() {
            AiRenderMode::Snapshot
        } else {
            AiRenderMode::Streaming
        }
    });
    let streaming_output_status = Signal::derive(move || {
        if streaming_is_complete.get() {
            AiOutputStatus::Verified
        } else {
            AiOutputStatus::Draft
        }
    });
    let streaming_has_root_open = Signal::derive(move || streaming_projection.get().has_root_open);
    let streaming_rendered_items = Signal::derive(move || streaming_projection.get().items.len());
    let streaming_visible_item_count = Signal::derive(move || {
        streaming_rendered_items
            .get()
            .min(ACCORDION_STREAMING_TOTAL_ITEMS)
    });
    let streaming_item_1_text = Signal::derive(move || {
        streaming_projection
            .get()
            .items
            .first()
            .map(|item| item.text.clone())
            .unwrap_or_default()
    });
    let streaming_item_2_text = Signal::derive(move || {
        streaming_projection
            .get()
            .items
            .get(1)
            .map(|item| item.text.clone())
            .unwrap_or_default()
    });
    let streaming_item_3_text = Signal::derive(move || {
        streaming_projection
            .get()
            .items
            .get(2)
            .map(|item| item.text.clone())
            .unwrap_or_default()
    });
    let streaming_update_disabled =
        Signal::derive(move || streaming_chars_clamped.get() >= streaming_total_chars);
    let streaming_code = Signal::derive(move || {
        compose_streaming_demo_code(
            &streaming_input_code.get(),
            &streaming_projection.get(),
            &streaming_open_set.get(),
            streaming_mode.get(),
            streaming_output_status.get(),
        )
    });

    Effect::new(move |_| {
        let visible = streaming_visible_item_count.get();
        set_streaming_open_set.update(|open_set| {
            open_set.retain(|index| *index < visible);
        });
    });

    let auto_timeout = StoredValue::new_local(None::<TimeoutHandle>);
    Effect::new(move |_| {
        if let Some(handle) = auto_timeout.get_value() {
            handle.clear();
        }
        auto_timeout.set_value(None);

        if !streaming_auto.get() {
            return;
        }

        let current = streaming_chars_clamped.get();
        let delay_ms = if current >= streaming_total_chars {
            ACCORDION_STREAMING_AUTO_RESET_DELAY_MS
        } else {
            ACCORDION_STREAMING_AUTO_INTERVAL_MS
        };

        if let Ok(handle) = set_timeout_with_handle(
            move || {
                if current >= streaming_total_chars {
                    set_streaming_chars.set(0);
                    set_streaming_open_set.set(open_set([]));
                } else {
                    set_streaming_chars.update(|value| {
                        *value = (*value + ACCORDION_STREAMING_AUTO_STEP).min(streaming_total_chars)
                    });
                }
            },
            std::time::Duration::from_millis(delay_ms),
        ) {
            auto_timeout.set_value(Some(handle));
        }
    });

    on_cleanup(move || {
        if let Some(handle) = auto_timeout.get_value() {
            handle.clear();
        }
    });

    Effect::new(move |_| {
        let open = workbench_open.get();
        if workbench_persist_state.get() {
            save_workbench_open(&open);
        } else {
            clear_workbench_open();
        }
    });

    let hello_code = Signal::derive(move || {
        r#"<Accordion variant=AccordionVariant::Light>
  <AccordionItem label="First">"Panel 1"</AccordionItem>
  <AccordionItem label="Second">"Panel 2"</AccordionItem>
</Accordion>"#
            .to_string()
    });

    let code = Signal::derive(move || {
        r#"let (open, set_open) = signal(open_set([0]));
let item_0_open = Signal::derive(move || open.get().contains(&0));
let item_1_open = Signal::derive(move || open.get().contains(&1));
let item_2_open = Signal::derive(move || open.get().contains(&2));

<Accordion
  id_base="accordion".to_string()
  selection_mode=AccordionSelectionMode::Multiple
  variant=AccordionVariant::Shadow
>
  <AccordionItem key=0 label="First" open=item_0_open on_open_change=on_item_open_change(set_open, 0)><div>"Panel 1"</div></AccordionItem>
  <AccordionItem key=1 label="Second" open=item_1_open on_open_change=on_item_open_change(set_open, 1)><div>"Panel 2"</div></AccordionItem>
  <AccordionItem key=2 label="Third" open=item_2_open on_open_change=on_item_open_change(set_open, 2)><div>"Panel 3"</div></AccordionItem>
</Accordion>"#.to_string()
    });

    let states_code = Signal::derive(move || {
        r#"let (open, set_open) = signal(open_set([1]));
let item_0_open = Signal::derive(move || open.get().contains(&0));
let item_1_open = Signal::derive(move || open.get().contains(&1));
let item_2_open = Signal::derive(move || open.get().contains(&2));

<Accordion
  id_base="accordion-single".to_string()
  selection_mode=AccordionSelectionMode::Single
  variant=AccordionVariant::Bordered
  disallow_empty_selection=true
>
  <AccordionItem key=0 label="Overview" open=item_0_open on_open_change=on_item_open_change(set_open, 0)><div>"Overview"</div></AccordionItem>
  <AccordionItem key=1 label="Details" open=item_1_open on_open_change=on_item_open_change(set_open, 1)><div>"Details"</div></AccordionItem>
  <AccordionItem key=2 label="History" open=item_2_open on_open_change=on_item_open_change(set_open, 2) is_disabled=true><div>"History"</div></AccordionItem>
</Accordion>"#.to_string()
    });

    let workbench_code = Signal::derive(move || {
        r#"let saved = load_workbench_open();
let (open, set_open) = signal(saved.unwrap_or_else(|| open_set([0])));
let item_0_open = Signal::derive(move || open.get().contains(&0));
let item_1_open = Signal::derive(move || open.get().contains(&1));
// style knobs update CSS variables in-place (no wasm rebuild)
<Accordion id_base="accordion-workbench".to_string()>
  <AccordionItem key=0 label="Profile" open=item_0_open on_open_change=on_item_open_change(set_open, 0)><div>"Profile panel"</div></AccordionItem>
  <AccordionItem key=1 label="Security" open=item_1_open on_open_change=on_item_open_change(set_open, 1)><div>"Security panel"</div></AccordionItem>
</Accordion>"#.to_string()
    });

    view! {
        <ComponentPage
            title="Accordion"
            slug="accordion"
            group="Collections"
            description="Multi-panel disclosure with roving tabindex, baseline-level spring motion, and baseline-style root state attrs."
        >
            <Playground
                title="Hello World (Uncontrolled)"
                description="Zero wiring path: no controlled state and no headless/state-primitives setup needed."
                code_signal=hello_code
                >
                    <div class="docs-stack">
                        <AiSpace mode=snapshot_mode output_status=verified_output>
                            <Accordion variant=AccordionVariant::Light>
                                <AccordionItem label="First">"Panel 1 content"</AccordionItem>
                                <AccordionItem label="Second">"Panel 2 content"</AccordionItem>
                            </Accordion>
                        </AiSpace>
                        <span class="ui-muted">"minimal default path"</span>
                    </div>
            </Playground>

            <Playground title="Multiple + Controlled" code_signal=code>
                <div class="docs-stack">
                    <AiSpace mode=snapshot_mode output_status=verified_output>
                        <Accordion
                            id_base="docs-accordion".to_string()
                            selection_mode=AccordionSelectionMode::Multiple
                            variant=AccordionVariant::Shadow
                        >
                            <AccordionItem key=0 label="First" open=open_multi_0 on_open_change=on_multi_0_change>
                                <div class="docs-stack">
                                    <div>"Panel 1 content"</div>
                                    <div class="ui-muted">"Press Enter/Space or click the triggers."</div>
                                </div>
                            </AccordionItem>
                            <AccordionItem key=1 label="Second" open=open_multi_1 on_open_change=on_multi_1_change>
                                <div class="docs-stack">
                                    <div>"Panel 2 content"</div>
                                    <div class="ui-muted">"Arrow keys move focus between triggers."</div>
                                </div>
                            </AccordionItem>
                            <AccordionItem key=2 label="Third" open=open_multi_2 on_open_change=on_multi_2_change>
                                <div class="docs-stack">
                                    <div>"Panel 3 content"</div>
                                    <div class="ui-muted">"Multiple mode allows multiple open panels."</div>
                                </div>
                            </AccordionItem>
                        </Accordion>
                    </AiSpace>
                    <span class="ui-muted">
                        "open: "
                        {move || {
                            let open = open_multi.get().iter().copied().collect::<Vec<_>>();
                            format!("{open:?}")
                        }}
                    </span>
                </div>
            </Playground>

            <Playground title="Single + Disabled" code_signal=states_code>
                <div class="docs-stack">
                    <AiSpace mode=snapshot_mode output_status=verified_output>
                        <Accordion
                            id_base="docs-accordion-single".to_string()
                            selection_mode=AccordionSelectionMode::Single
                            variant=AccordionVariant::Bordered
                            disallow_empty_selection=true
                        >
                            <AccordionItem key=0 label="Overview" open=open_single_0 on_open_change=on_single_0_change>
                                <div class="docs-stack">
                                    <div>"Overview content"</div>
                                    <div class="ui-muted">"Single mode keeps at most one panel open."</div>
                                </div>
                            </AccordionItem>
                            <AccordionItem key=1 label="Details" open=open_single_1 on_open_change=on_single_1_change>
                                <div class="docs-stack">
                                    <div>"Details content"</div>
                                    <div class="ui-muted">"Selection is fully controlled by `open`."</div>
                                </div>
                            </AccordionItem>
                            <AccordionItem key=2 label="History" open=open_single_2 on_open_change=on_single_2_change is_disabled=true>
                                <div class="docs-stack">
                                    <div>"History content"</div>
                                    <div class="ui-muted">"This trigger is disabled and skipped by roving focus."</div>
                                </div>
                            </AccordionItem>
                        </Accordion>
                    </AiSpace>
                    <span class="ui-muted">
                        "single open: "
                        {move || {
                            let open = open_single.get().iter().copied().collect::<Vec<_>>();
                            format!("{open:?}")
                        }}
                    </span>
                    <span class="ui-muted">"disabled index: 2"</span>
                </div>
            </Playground>

            <Playground
                title="Streaming Output (AI Space)"
                code_signal=streaming_code
                description="Character streaming starts from <Accordion>. Update adds +20 chars each time; Auto adds +2 chars every 0.1s. After completion, wait 3s, reset, and loop."
                code_imports=String::new()
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="accordion-streaming-controls">
                        <div class="docs-row">
                            <button
                                type="button"
                                on:click=move |_| {
                                    set_streaming_chars.set(0);
                                    set_streaming_open_set.set(open_set([]));
                                }
                            >
                                "Reset"
                            </button>
                            <button
                                type="button"
                                prop:disabled=streaming_update_disabled
                                on:click=move |_| {
                                    set_streaming_chars.update(|value| {
                                        *value = (*value + ACCORDION_STREAMING_MANUAL_STEP)
                                            .min(streaming_total_chars)
                                    });
                                }
                            >
                                "Update"
                            </button>
                            <label class="docs-search__label">
                                <input
                                    type="checkbox"
                                    prop:checked=move || streaming_auto.get()
                                    on:change=move |ev| set_streaming_auto.set(event_target_checked(&ev))
                                />
                                " Auto"
                            </label>
                        </div>
                    </div>
                }
            >
                <div class="docs-stack" data-slot="accordion-streaming-demo">
                    <span class="ui-muted">
                        "mode: "
                        {move || streaming_mode.get().as_str()}
                        " | status: "
                        {move || streaming_output_status.get().as_str()}
                        " | chars: "
                        {move || streaming_chars_clamped.get()}
                        "/"
                        {streaming_total_chars}
                        " | items rendered: "
                        {move || streaming_rendered_items.get()}
                        "/"
                        {streaming_total_items}
                    </span>
                    <AiSpace mode=streaming_mode output_status=streaming_output_status>
                        <div class="docs-card" data-slot="accordion-streaming-canvas">
                            <Show when=move || streaming_has_root_open.get()>
                                <Show when=move || streaming_visible_item_count.get() == 1>
                                    <Accordion
                                        id_base="docs-accordion-ai-stream".to_string()
                                        selection_mode=AccordionSelectionMode::Multiple
                                        variant=AccordionVariant::Splitted
                                    >
                                        <AccordionItem
                                            key=0
                                            label=ACCORDION_STREAMING_ITEM_LABELS[0].to_string()
                                            open=streaming_open_0
                                            on_open_change=on_streaming_0_change
                                        >
                                            <div>{move || streaming_item_1_text.get()}</div>
                                        </AccordionItem>
                                    </Accordion>
                                </Show>
                                <Show when=move || streaming_visible_item_count.get() == 2>
                                    <Accordion
                                        id_base="docs-accordion-ai-stream".to_string()
                                        selection_mode=AccordionSelectionMode::Multiple
                                        variant=AccordionVariant::Splitted
                                    >
                                        <AccordionItem
                                            key=0
                                            label=ACCORDION_STREAMING_ITEM_LABELS[0].to_string()
                                            open=streaming_open_0
                                            on_open_change=on_streaming_0_change
                                        >
                                            <div>{move || streaming_item_1_text.get()}</div>
                                        </AccordionItem>
                                        <AccordionItem
                                            key=1
                                            label=ACCORDION_STREAMING_ITEM_LABELS[1].to_string()
                                            open=streaming_open_1
                                            on_open_change=on_streaming_1_change
                                        >
                                            <div>{move || streaming_item_2_text.get()}</div>
                                        </AccordionItem>
                                    </Accordion>
                                </Show>
                                <Show when=move || streaming_visible_item_count.get() == 3>
                                    <Accordion
                                        id_base="docs-accordion-ai-stream".to_string()
                                        selection_mode=AccordionSelectionMode::Multiple
                                        variant=AccordionVariant::Splitted
                                    >
                                        <AccordionItem
                                            key=0
                                            label=ACCORDION_STREAMING_ITEM_LABELS[0].to_string()
                                            open=streaming_open_0
                                            on_open_change=on_streaming_0_change
                                        >
                                            <div>{move || streaming_item_1_text.get()}</div>
                                        </AccordionItem>
                                        <AccordionItem
                                            key=1
                                            label=ACCORDION_STREAMING_ITEM_LABELS[1].to_string()
                                            open=streaming_open_1
                                            on_open_change=on_streaming_1_change
                                        >
                                            <div>{move || streaming_item_2_text.get()}</div>
                                        </AccordionItem>
                                        <AccordionItem
                                            key=2
                                            label=ACCORDION_STREAMING_ITEM_LABELS[2].to_string()
                                            open=streaming_open_2
                                            on_open_change=on_streaming_2_change
                                        >
                                            <div>{move || streaming_item_3_text.get()}</div>
                                        </AccordionItem>
                                    </Accordion>
                                </Show>
                            </Show>
                        </div>
                    </AiSpace>
                </div>
            </Playground>

            <Playground
                title="Workbench (Isolated Canvas + Optional Persist)"
                description="Tune CSS variables live, keep interaction context, and optionally persist open state."
                code_signal=workbench_code
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="accordion-workbench-controls">
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_multiple_mode.get()
                                on:change=move |ev| set_workbench_multiple_mode.set(event_target_checked(&ev))
                            />
                            " Multiple mode"
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_disable_second.get()
                                on:change=move |ev| set_workbench_disable_second.set(event_target_checked(&ev))
                            />
                            " Disable item #1"
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_persist_state.get()
                                on:change=move |ev| set_workbench_persist_state.set(event_target_checked(&ev))
                            />
                            " Persist open state (optional)"
                        </label>
                        <label class="docs-search__label">
                            "Radius "
                            <input
                                type="range"
                                min="8"
                                max="24"
                                prop:value=move || workbench_radius_px.get().to_string()
                                on:input=move |ev| {
                                    if let Ok(next) = event_target_value(&ev).parse::<u16>() {
                                        set_workbench_radius_px.set(next.clamp(8, 24));
                                    }
                                }
                            />
                        </label>
                        <label class="docs-search__label">
                            "Hover alpha "
                            <input
                                type="range"
                                min="5"
                                max="30"
                                prop:value=move || workbench_hover_alpha.get().to_string()
                                on:input=move |ev| {
                                    if let Ok(next) = event_target_value(&ev).parse::<u16>() {
                                        set_workbench_hover_alpha.set(next.clamp(5, 30));
                                    }
                                }
                            />
                        </label>
                    </div>
                }
            >
                <div class="docs-stack" data-slot="accordion-workbench">
                    <span class="ui-muted">
                        "persist: "
                        {move || if workbench_persist_state.get() { "on" } else { "off" }}
                        " | open: "
                        {move || {
                            let open = workbench_open.get().iter().copied().collect::<Vec<_>>();
                            format!("{open:?}")
                        }}
                    </span>
                    {move || {
                        let selection_mode = if workbench_multiple_mode.get() {
                            AccordionSelectionMode::Multiple
                        } else {
                            AccordionSelectionMode::Single
                        };
                        let disallow_empty_selection =
                            selection_mode == AccordionSelectionMode::Single;
                        let disable_security = workbench_disable_second.get();
                        let radius = workbench_radius_px.get();
                        let alpha = f32::from(workbench_hover_alpha.get()) / 100.0;
                        let workbench_style = format!(
                            "--ui-radius-md: {radius}px; --ui-accordion-trigger-hover-bg: rgba(0, 111, 238, {alpha});"
                        );
                        view! {
                            <div
                                class="docs-card"
                                data-slot="accordion-workbench-canvas"
                                style=workbench_style
                            >
                                <AiSpace mode=snapshot_mode output_status=verified_output>
                                    <Accordion
                                        id_base="docs-accordion-workbench".to_string()
                                        selection_mode=selection_mode
                                        variant=AccordionVariant::Splitted
                                        disallow_empty_selection=disallow_empty_selection
                                    >
                                        <AccordionItem
                                            key=0
                                            label="Profile"
                                            open=workbench_open_0
                                            on_open_change=on_workbench_0_change
                                        >
                                            <div>"Profile panel content"</div>
                                        </AccordionItem>
                                        <AccordionItem
                                            key=1
                                            label="Security".to_string()
                                            open=workbench_open_1
                                            on_open_change=on_workbench_1_change
                                            is_disabled=disable_security
                                        >
                                            <div>"Security panel content"</div>
                                        </AccordionItem>
                                        <AccordionItem
                                            key=2
                                            label="Notifications"
                                            open=workbench_open_2
                                            on_open_change=on_workbench_2_change
                                        >
                                            <div>"Notifications panel content"</div>
                                        </AccordionItem>
                                    </Accordion>
                                </AiSpace>
                            </div>
                        }
                    }}
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn disclosure() -> AnyView {
    let (open, set_open) = signal(true);
    let on_open_change = Callback::new(move |next: bool| set_open.set(next));
    let (workbench_open, set_workbench_open) = signal(true);
    let (workbench_controlled, set_workbench_controlled) = signal(true);
    let (workbench_disabled, set_workbench_disabled) = signal(false);
    let (workbench_custom_motion, set_workbench_custom_motion) = signal(false);
    let workbench_on_open_change = Callback::new(move |next: bool| set_workbench_open.set(next));

    let code = Signal::derive(move || {
        r#"let (open, set_open) = signal(true);
let on_open_change = Callback::new(move |next: bool| set_open.set(next));
<Disclosure
  id_base="disc".to_string()
  label="Details".to_string()
  open=open
  on_open_change=on_open_change
>
  <div>"Hidden content"</div>
</Disclosure>"#
            .to_string()
    });

    let workbench_code = Signal::derive(move || {
        let controlled = workbench_controlled.get();
        let disabled = workbench_disabled.get();
        let custom_motion = workbench_custom_motion.get();
        let default_open = workbench_open.get();

        let mut lines = vec![
            "let (open, set_open) = signal(true);".to_string(),
            "let on_open_change = Callback::new(move |next: bool| set_open.set(next));".to_string(),
            "<Disclosure".to_string(),
            "  id_base=\"docs-disclosure-workbench\".into()".to_string(),
            "  label=\"Workbench details\".into()".to_string(),
        ];

        if controlled {
            lines.push("  open=open".to_string());
            lines.push("  on_open_change=on_open_change".to_string());
        } else {
            lines.push(format!("  default_open={default_open}"));
        }
        if disabled {
            lines.push("  disabled=true".to_string());
        }
        if custom_motion {
            lines.push("  motion=DisclosureMotion { open_rotation_deg: 135.0, panel_offset_y_px: 10.0, ..DisclosureMotion::default() }".to_string());
        }

        lines.extend([
            ">".to_string(),
            "  <div>\"Workbench disclosure content\"</div>".to_string(),
            "</Disclosure>".to_string(),
        ]);

        lines.join("\n")
    });

    let disclosure_test_css_source = Signal::derive(move || {
        format!(
            "/* crates/ui-components/src/disclosure/styles.rs */\n{}",
            ui_components::disclosure::styles::CSS
        )
    });

    let disclosure_actual_config = Signal::derive(move || {
        let controlled = workbench_controlled.get();
        let disabled = workbench_disabled.get();
        let custom_motion = workbench_custom_motion.get();
        let open_value = workbench_open.get();

        format!(
            "DisclosureActualConfig {{\n  open: {open_value},\n  disabled: {disabled},\n  control_mode: \"{}\",\n  default_open_source: \"{}\",\n  motion_source: \"{}\",\n  expected_root_attrs: [\"data-open-control-mode\", \"data-default-open-source\", \"data-motion-source\"],\n}}",
            if controlled {
                "controlled"
            } else {
                "uncontrolled"
            },
            if controlled {
                "implicit-default"
            } else {
                "prop"
            },
            if custom_motion { "custom" } else { "default" },
        )
    });

    let states_code = Signal::derive(move || {
        r#"<Disclosure
  id_base="disc-disabled".to_string()
  label="Disabled details".to_string()
  default_open=false
  disabled=true
>
  <div>"Disabled content"</div>
</Disclosure>"#
            .to_string()
    });

    view! {
        <ComponentPage
            title="Disclosure"
            slug="disclosure"
            group="Collections"
            description="Single disclosure panel with baseline-level spring motion and baseline-style root state attrs."
        >
            <Playground title="Controlled" code_signal=code>
                <div class="docs-stack">
                    <Disclosure
                        id_base="docs-disclosure".to_string()
                        label="Details".to_string()
                        open=open.into()
                        on_open_change=on_open_change
                    >
                        <div class="docs-stack">
                            <div>"Hidden content"</div>
                            <div class="ui-muted">"Uses the same open-state contract as overlays."</div>
                        </div>
                    </Disclosure>
                    <span class="ui-muted">
                        "open: "
                        {move || open.get()}
                    </span>
                </div>
            </Playground>

            <Playground title="Disabled" code_signal=states_code>
                <div class="docs-stack">
                    <Disclosure
                        id_base="docs-disclosure-disabled".to_string()
                        label="Disabled details".to_string()
                        default_open=false
                        disabled=true
                    >
                        <div class="docs-stack">
                            <div>"Disabled content"</div>
                            <div class="ui-muted">"Disabled disclosure keeps trigger non-interactive."</div>
                        </div>
                    </Disclosure>
                    <span class="ui-muted">"disabled: true"</span>
                </div>
            </Playground>

            <Playground
                title="Workbench (Display + Config + Code + CSS Test)"
                code_signal=workbench_code
                test_css_source=disclosure_test_css_source
                test_source_path="/root/autodl-tmp/zjj/p/rust-ui/crates/ui-components/src/disclosure/styles.rs".to_string()
                test_config_signal=disclosure_actual_config
                description="Disclosure workbench: 对比展示 + config + code + scoped CSS test."
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight">
                        <ui_components::Switch checked=workbench_controlled set_checked=set_workbench_controlled>
                            "Controlled mode"
                        </ui_components::Switch>
                        <ui_components::Switch checked=workbench_disabled set_checked=set_workbench_disabled>
                            "Disabled"
                        </ui_components::Switch>
                        <ui_components::Switch checked=workbench_custom_motion set_checked=set_workbench_custom_motion>
                            "Custom motion"
                        </ui_components::Switch>
                        <ui_components::Switch checked=workbench_open set_checked=set_workbench_open>
                            "Open state (for controlled/default_open)"
                        </ui_components::Switch>
                    </div>
                }
            >
                {move || {
                    let controlled = workbench_controlled.get();
                    let disabled = workbench_disabled.get();
                    let custom_motion = workbench_custom_motion.get();
                    let motion = if custom_motion {
                        ui_components::DisclosureMotion {
                            open_rotation_deg: 135.0,
                            panel_offset_y_px: 10.0,
                            ..ui_components::DisclosureMotion::default()
                        }
                    } else {
                        ui_components::DisclosureMotion::default()
                    };

                    view! {
                        <div class="docs-stack">
                            <div class="docs-row">
                                <div class="docs-card">
                                    <h4>"Configured Disclosure"</h4>
                                    {if controlled {
                                        view! {
                                            <Disclosure
                                                id_base="docs-disclosure-workbench".to_string()
                                                label="Workbench details".to_string()
                                                open=workbench_open.into()
                                                on_open_change=workbench_on_open_change
                                                disabled=disabled
                                                motion=motion
                                            >
                                                <div class="docs-stack">
                                                    <div>"Configured content"</div>
                                                    <div class="ui-muted">"Tracks controlled/uncontrolled + motion source attrs."</div>
                                                </div>
                                            </Disclosure>
                                        }
                                            .into_any()
                                    } else {
                                        view! {
                                            <Disclosure
                                                id_base="docs-disclosure-workbench".to_string()
                                                label="Workbench details".to_string()
                                                default_open=workbench_open.get()
                                                disabled=disabled
                                                motion=motion
                                            >
                                                <div class="docs-stack">
                                                    <div>"Configured content"</div>
                                                    <div class="ui-muted">"Uncontrolled path uses default_open source marker."</div>
                                                </div>
                                            </Disclosure>
                                        }
                                            .into_any()
                                    }}
                                </div>

                                <div class="docs-card">
                                    <h4>"Reference Disclosure"</h4>
                                    <Disclosure
                                        id_base="docs-disclosure-reference".to_string()
                                        label="Reference details".to_string()
                                        default_open=true
                                    >
                                        <div class="docs-stack">
                                            <div>"Reference content"</div>
                                            <div class="ui-muted">"Baseline uncontrolled + default motion."</div>
                                        </div>
                                    </Disclosure>
                                </div>
                            </div>
                        </div>
                    }
                }}
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn tabs() -> AnyView {
    let manual_labels = vec!["Profile", "Billing", "Team"];
    let workbench_labels = vec!["Overview", "Details", "Settings"];

    let (selected_auto, set_selected_auto) = signal(0_usize);
    let on_auto_change = Callback::new(move |index: usize| set_selected_auto.set(index));

    let (selected_manual, set_selected_manual) = signal(1_usize);
    let on_manual_change = Callback::new(move |index: usize| set_selected_manual.set(index));

    let persisted_tabs_workbench_selected = load_tabs_workbench_selected();
    let (tabs_workbench_selected, set_tabs_workbench_selected) =
        signal(persisted_tabs_workbench_selected.unwrap_or(0_usize));
    let on_tabs_workbench_change =
        Callback::new(move |index: usize| set_tabs_workbench_selected.set(index));
    let (tabs_workbench_manual_mode, set_tabs_workbench_manual_mode) = signal(false);
    let (tabs_workbench_disable_settings, set_tabs_workbench_disable_settings) = signal(false);
    let (tabs_workbench_persist_state, set_tabs_workbench_persist_state) =
        signal(persisted_tabs_workbench_selected.is_some());

    Effect::new(move |_| {
        let selected_index = tabs_workbench_selected.get();
        if tabs_workbench_persist_state.get() {
            save_tabs_workbench_selected(selected_index);
        } else {
            clear_tabs_workbench_selected();
        }
    });

    let hello_world_code = Signal::derive(move || {
        r#"<Tabs labels=vec!["Overview", "Details", "Settings"] id_base="tabs".to_string()>
  <div>"Overview panel"</div>
  <div>"Details panel"</div>
  <div>"Settings panel"</div>
</Tabs>"#
            .to_string()
    });

    let code = Signal::derive(move || {
        r#"let (selected, set_selected) = signal(0_usize);
let on_change = Callback::new(move |next: usize| set_selected.set(next));
<Tabs
  labels=vec!["Overview", "Details", "Settings"]
  id_base="tabs".to_string()
  selected_index=selected
  on_selection_change=on_change
  keyboard_activation=TabsKeyboardActivation::Automatic
>
  <div>"Overview panel"</div>
  <div>"Details panel"</div>
  <div>"Settings panel"</div>
</Tabs>"#
            .to_string()
    });

    let states_code = Signal::derive(move || {
        r#"let (selected, set_selected) = signal(1_usize);
let on_change = Callback::new(move |next: usize| set_selected.set(next));
<Tabs
  labels=vec!["Profile", "Billing", "Team"]
  id_base="tabs-manual".to_string()
  keyboard_activation=TabsKeyboardActivation::Manual
  selected_index=selected
  on_selection_change=on_change
  disabled_indices=vec![2]
>
  <div>"Profile panel"</div>
  <div>"Billing panel"</div>
  <div>"Team panel"</div>
</Tabs>"#
            .to_string()
    });

    let workbench_code = Signal::derive(move || {
        r#"let saved = load_tabs_workbench_selected();
let (selected, set_selected) = signal(saved.unwrap_or(0_usize));
let on_change = Callback::new(move |next: usize| set_selected.set(next));
// Workbench keeps interaction context and can optionally persist selected index.
<Tabs
  labels=vec!["Overview", "Details", "Settings"]
  id_base="tabs-workbench".to_string()
  selected_index=selected
  on_selection_change=on_change
  keyboard_activation=TabsKeyboardActivation::Automatic
>
  <div>"Overview panel"</div>
  <div>"Details panel"</div>
  <div>"Settings panel"</div>
</Tabs>"#
            .to_string()
    });

    view! {
        <ComponentPage
            title="Tabs"
            slug="tabs"
            group="Collections"
            description="Tabs with roving tabindex, spring indicator motion, and default-theme visual baseline hierarchy."
        >
            <Playground
                title="Hello World (Uncontrolled)"
                description="Zero-wiring default path for beginners: no controlled state setup required."
                code_signal=hello_world_code
            >
                <div class="docs-stack">
                    <Tabs
                        labels=vec!["Overview", "Details", "Settings"]
                        id_base="docs-tabs-hello".to_string()
                    >
                        <div class="docs-stack">
                            <div>"Overview"</div>
                            <div class="ui-muted">"Start here: default selection is managed internally."</div>
                        </div>
                        <div class="docs-stack">
                            <div>"Details"</div>
                            <div class="ui-muted">"No state machine wiring required for common usage."</div>
                        </div>
                        <div class="docs-stack">
                            <div>"Settings"</div>
                            <div class="ui-muted">"Upgrade to controlled mode only when needed."</div>
                        </div>
                    </Tabs>
                    <span class="ui-muted">
                        "Beginner path first; advanced controls follow below."
                    </span>
                </div>
            </Playground>

            <Playground title="Automatic + Controlled" code_signal=code>
                <div class="docs-stack">
                    <Tabs
                        labels=vec!["Overview", "Details", "Settings"]
                        id_base="docs-tabs".to_string()
                        selected_index=selected_auto
                        on_selection_change=on_auto_change
                        keyboard_activation=TabsKeyboardActivation::Automatic
                    >
                        <div class="docs-stack">
                            <div>"Overview"</div>
                            <div class="ui-muted">"Arrow keys move + select in automatic mode."</div>
                        </div>
                        <div class="docs-stack">
                            <div>"Details"</div>
                            <div class="ui-muted">"Selection change is controlled by signal callback."</div>
                        </div>
                        <div class="docs-stack">
                            <div>"Settings"</div>
                            <div class="ui-muted">"Indicator motion stays spring-driven."</div>
                        </div>
                    </Tabs>
                    <span class="ui-muted">
                        "selected: "
                        {move || selected_auto.get()}
                    </span>
                    <span class="ui-muted">
                        "Default theme baseline: clear hierarchy, layered contrast, and explicit hover/focus feedback."
                    </span>
                </div>
            </Playground>

            <Playground title="Manual + Disabled" code_signal=states_code>
                <div class="docs-stack">
                    <Tabs
                        labels=manual_labels
                        id_base="docs-tabs-manual".to_string()
                        selected_index=selected_manual
                        on_selection_change=on_manual_change
                        keyboard_activation=TabsKeyboardActivation::Manual
                        disabled_indices=vec![2]
                    >
                        <div class="docs-stack">
                            <div>"Profile"</div>
                            <div class="ui-muted">"Manual mode: focus moves first, Enter/Space commits."</div>
                        </div>
                        <div class="docs-stack">
                            <div>"Billing"</div>
                            <div class="ui-muted">"Current selected index reflects committed tab."</div>
                        </div>
                        <div class="docs-stack">
                            <div>"Team"</div>
                            <div class="ui-muted">"This tab is disabled and skipped by roving focus."</div>
                        </div>
                    </Tabs>
                    <span class="ui-muted">
                        "manual selected: "
                        {move || selected_manual.get()}
                    </span>
                    <span class="ui-muted">"disabled tab index: 2"</span>
                </div>
            </Playground>

            <Playground
                title="Workbench (Isolated Canvas + Optional Persist)"
                description="Tune keyboard/disabled semantics while preserving context, with optional selected-index persistence."
                code_signal=workbench_code
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="tabs-workbench-controls">
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || tabs_workbench_manual_mode.get()
                                on:change=move |ev| set_tabs_workbench_manual_mode.set(event_target_checked(&ev))
                            />
                            " Manual keyboard activation"
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || tabs_workbench_disable_settings.get()
                                on:change=move |ev| set_tabs_workbench_disable_settings.set(event_target_checked(&ev))
                            />
                            " Disable \"Settings\" tab"
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || tabs_workbench_persist_state.get()
                                on:change=move |ev| set_tabs_workbench_persist_state.set(event_target_checked(&ev))
                            />
                            " Persist selected index (optional)"
                        </label>
                    </div>
                }
            >
                <div class="docs-stack" data-slot="tabs-workbench">
                    <span class="ui-muted">
                        "persist selected: "
                        {move || if tabs_workbench_persist_state.get() { "on" } else { "off" }}
                    </span>
                    <span class="ui-muted">
                        "workbench selected: "
                        {move || tabs_workbench_selected.get()}
                    </span>
                    <div class="docs-card" data-slot="tabs-workbench-canvas">
                        {move || {
                            let disabled_indices = if tabs_workbench_disable_settings.get() {
                                vec![2]
                            } else {
                                Vec::new()
                            };
                            let selected_index = tabs_workbench_selected;
                            let on_selection_change = on_tabs_workbench_change;

                            if tabs_workbench_manual_mode.get() {
                                view! {
                                    <Tabs
                                        labels=workbench_labels.clone()
                                        id_base="docs-tabs-workbench".to_string()
                                        selected_index=selected_index
                                        on_selection_change=on_selection_change
                                        keyboard_activation=TabsKeyboardActivation::Manual
                                        disabled_indices=disabled_indices
                                    >
                                        <div class="docs-stack">
                                            <div>"Overview"</div>
                                            <div class="ui-muted">"Keep context while toggling keyboard mode."</div>
                                        </div>
                                        <div class="docs-stack">
                                            <div>"Details"</div>
                                            <div class="ui-muted">"Selection stays controlled by workbench signal."</div>
                                        </div>
                                        <div class="docs-stack">
                                            <div>"Settings"</div>
                                            <div class="ui-muted">"Optional disabled state stays inspectable via markers."</div>
                                        </div>
                                    </Tabs>
                                }
                                .into_any()
                            } else {
                                view! {
                                    <Tabs
                                        labels=workbench_labels.clone()
                                        id_base="docs-tabs-workbench".to_string()
                                        selected_index=selected_index
                                        on_selection_change=on_selection_change
                                        keyboard_activation=TabsKeyboardActivation::Automatic
                                        disabled_indices=disabled_indices
                                    >
                                        <div class="docs-stack">
                                            <div>"Overview"</div>
                                            <div class="ui-muted">"Keep context while toggling keyboard mode."</div>
                                        </div>
                                        <div class="docs-stack">
                                            <div>"Details"</div>
                                            <div class="ui-muted">"Selection stays controlled by workbench signal."</div>
                                        </div>
                                        <div class="docs-stack">
                                            <div>"Settings"</div>
                                            <div class="ui-muted">"Optional disabled state stays inspectable via markers."</div>
                                        </div>
                                    </Tabs>
                                }
                                .into_any()
                            }
                        }}
                    </div>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn list() -> AnyView {
    let showcase_items: Arc<[String]> = vec![
        "Overview".to_string(),
        "Billing".to_string(),
        "Integrations".to_string(),
        "Audit Logs".to_string(),
    ]
    .into();
    let disabled_items: Arc<[String]> = vec![
        "Overview".to_string(),
        "Billing".to_string(),
        "Integrations".to_string(),
    ]
    .into();
    let empty_items: Arc<[String]> = Vec::<String>::new().into();

    let (showcase_selected_default, set_showcase_selected_default) = signal(Some(0_usize));
    let (showcase_selected_unsynced, set_showcase_selected_unsynced) = signal(Some(1_usize));
    let (showcase_selected_disabled, set_showcase_selected_disabled) = signal(Some(0_usize));
    let (showcase_selected_empty, set_showcase_selected_empty) = signal(None::<usize>);

    let workbench_items: Arc<[String]> = vec![
        "Overview".to_string(),
        "Billing".to_string(),
        "Integrations".to_string(),
        "Audit Logs".to_string(),
        "Security".to_string(),
    ]
    .into();
    let (workbench_selected, set_workbench_selected) = signal(Some(1_usize));
    let (workbench_sync_active, set_workbench_sync_active) = signal(true);
    let (workbench_disable_last, set_workbench_disable_last) = signal(true);
    let (workbench_root_disabled, set_workbench_root_disabled) = signal(false);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);

    let showcase_code = Signal::derive(move || {
        r#"let items: Arc<[String]> = vec![
  "Overview".to_string(),
  "Billing".to_string(),
  "Integrations".to_string(),
  "Audit Logs".to_string(),
].into();

let (selected_a, set_selected_a) = signal(Some(0_usize));
let (selected_b, set_selected_b) = signal(Some(1_usize));
let (selected_c, set_selected_c) = signal(Some(0_usize));
let (selected_empty, set_selected_empty) = signal(None::<usize>);

<List
  id_base="list-default".to_string()
  items=items.clone()
  selected_index=selected_a
  set_selected_index=set_selected_a
  aria_label="Default list".to_string()
  disabled_indices=vec![2]
/>
<List
  id_base="list-unsynced".to_string()
  items=items
  selected_index=selected_b
  set_selected_index=set_selected_b
  aria_label="Unsynced list".to_string()
  sync_active_index_to_selected=false
/>
<List
  id_base="list-disabled".to_string()
  items=vec!["Overview".to_string(), "Billing".to_string(), "Integrations".to_string()].into()
  selected_index=selected_c
  set_selected_index=set_selected_c
  aria_label="Disabled list".to_string()
  disabled=true
/>
<List
  id_base="list-empty".to_string()
  items=Vec::<String>::new().into()
  selected_index=selected_empty
  set_selected_index=set_selected_empty
  aria_label="Empty list".to_string()
/>"#
        .to_string()
    });

    let workbench_code = Signal::derive(move || {
        let sync_active = workbench_sync_active.get();
        let root_disabled = workbench_root_disabled.get();
        let disable_last = workbench_disable_last.get();
        let custom_class = workbench_custom_class.get();

        let mut lines = vec![
            "let (selected, set_selected) = signal(Some(1_usize));".to_string(),
            "<List".to_string(),
            "  id_base=\"docs-list-workbench\".into()".to_string(),
            "  items=vec![".to_string(),
            "    \"Overview\".into(),".to_string(),
            "    \"Billing\".into(),".to_string(),
            "    \"Integrations\".into(),".to_string(),
            "    \"Audit Logs\".into(),".to_string(),
            "    \"Security\".into(),".to_string(),
            "  ].into()".to_string(),
            "  selected_index=selected".to_string(),
            "  set_selected_index=set_selected".to_string(),
            "  aria_label=\"List workbench\".into()".to_string(),
        ];

        if !sync_active {
            lines.push("  sync_active_index_to_selected=false".to_string());
        }
        if root_disabled {
            lines.push("  disabled=true".to_string());
        }
        if disable_last {
            lines.push("  disabled_indices=vec![4]".to_string());
        }
        if custom_class {
            lines.push("  class_name=\"docs-list-workbench--custom\".into()".to_string());
        }

        lines.push("/>".to_string());
        lines.join("\n")
    });

    let workbench_test_css = Signal::derive(move || {
        format!(
            "/* crates/ui-components/src/list/styles.rs */\n{}\n\n/* ListItem contract */\n{}\n\n/* ListSection contract */\n{}",
            ui_components::list::styles::CSS,
            ui_components::list::styles::ITEM_CSS,
            ui_components::list::styles::SECTION_CSS,
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        let selected = workbench_selected.get();
        let sync_active = workbench_sync_active.get();
        let root_disabled = workbench_root_disabled.get();
        let disable_last = workbench_disable_last.get();
        let custom_class = workbench_custom_class.get();

        let mut class = vec!["ui-listbox".to_string()];
        if custom_class {
            class.push("docs-list-workbench--custom".to_string());
        }
        if root_disabled {
            class.push("data-disabled=true".to_string());
        }
        if disable_last {
            class.push("data-has-disabled-options=true".to_string());
        }

        format!(
            "ListWorkbenchConfig {{\n  selected_index: {selected:?},\n  sync_active_index_to_selected: {sync_active},\n  disabled_root: {root_disabled},\n  disabled_indices: {},\n  custom_class: {custom_class},\n  class: \"{}\",\n}}",
            if disable_last { "vec![4]" } else { "vec![]" },
            class.join(" ")
        )
    });

    view! {
        <ComponentPage
            title="List"
            slug="list"
            group="Collections"
            description="List primitive with centralized root-state markers and optional active-index sync controls."
        >
            <Playground
                title="展示：多场景对比"
                description="同一套 List 在默认、unsynced、disabled root、empty 四种状态下的行为对比。"
                code_signal=showcase_code
            >
                <div class="docs-row" data-slot="list-showcase">
                    <div class="docs-stack" style="min-width: 220px;">
                        <span class="ui-muted">"default + disabled option"</span>
                        <List
                            id_base="docs-list-default".to_string()
                            items=showcase_items.clone()
                            selected_index=showcase_selected_default
                            set_selected_index=set_showcase_selected_default
                            aria_label="Default list".to_string()
                            disabled_indices=vec![2]
                        />
                        <span class="ui-muted">
                            "selected: "
                            {move || showcase_selected_default.get().map(|value| value.to_string()).unwrap_or_else(|| "None".to_string())}
                        </span>
                    </div>

                    <div class="docs-stack" style="min-width: 220px;">
                        <span class="ui-muted">"unsynced active index"</span>
                        <List
                            id_base="docs-list-unsynced".to_string()
                            items=showcase_items.clone()
                            selected_index=showcase_selected_unsynced
                            set_selected_index=set_showcase_selected_unsynced
                            aria_label="Unsynced list".to_string()
                            sync_active_index_to_selected=false
                        />
                        <span class="ui-muted">
                            "selected: "
                            {move || showcase_selected_unsynced.get().map(|value| value.to_string()).unwrap_or_else(|| "None".to_string())}
                        </span>
                    </div>

                    <div class="docs-stack" style="min-width: 220px;">
                        <span class="ui-muted">"disabled root"</span>
                        <List
                            id_base="docs-list-disabled".to_string()
                            items=disabled_items
                            selected_index=showcase_selected_disabled
                            set_selected_index=set_showcase_selected_disabled
                            aria_label="Disabled list".to_string()
                            disabled=true
                        />
                        <span class="ui-muted">
                            "selected: "
                            {move || showcase_selected_disabled.get().map(|value| value.to_string()).unwrap_or_else(|| "None".to_string())}
                        </span>
                    </div>

                    <div class="docs-stack" style="min-width: 220px;">
                        <span class="ui-muted">"empty list"</span>
                        <List
                            id_base="docs-list-empty".to_string()
                            items=empty_items
                            selected_index=showcase_selected_empty
                            set_selected_index=set_showcase_selected_empty
                            aria_label="Empty list".to_string()
                        />
                        <span class="ui-muted">
                            "selected: "
                            {move || showcase_selected_empty.get().map(|value| value.to_string()).unwrap_or_else(|| "None".to_string())}
                        </span>
                    </div>
                </div>
            </Playground>

            <Playground
                title="Workbench（展示 + Config + Code + CSS Test）"
                description="按钮式 workbench：单画布调参，支持 settings / code / css-test 面板联动。"
                code_signal=workbench_code
                test_css_source=workbench_test_css
                test_source_path="/root/autodl-tmp/zjj/p/rust-ui/crates/ui-components/src/list/styles.rs".to_string()
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="list-workbench-controls">
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_sync_active.get()
                                on:change=move |ev| set_workbench_sync_active.set(event_target_checked(&ev))
                            />
                            " Sync active index to selected"
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_disable_last.get()
                                on:change=move |ev| set_workbench_disable_last.set(event_target_checked(&ev))
                            />
                            " Disable last option"
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_root_disabled.get()
                                on:change=move |ev| set_workbench_root_disabled.set(event_target_checked(&ev))
                            />
                            " Disable root"
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_custom_class.get()
                                on:change=move |ev| set_workbench_custom_class.set(event_target_checked(&ev))
                            />
                            " Custom class marker"
                        </label>
                    </div>
                }
            >
                <div class="docs-stack" data-slot="list-workbench" style="width: min(100%, 420px);">
                    <span class="ui-muted">
                        "selected: "
                        {move || workbench_selected.get().map(|value| value.to_string()).unwrap_or_else(|| "None".to_string())}
                        " · disabled indices: "
                        {move || if workbench_disable_last.get() { "[4]" } else { "[]" }}
                    </span>
                    {move || {
                        let disable_last = workbench_disable_last.get();
                        let root_disabled = workbench_root_disabled.get();
                        let sync_active = workbench_sync_active.get();
                        let custom_class = workbench_custom_class.get();

                        let class_name = if custom_class {
                            "docs-list-workbench--custom".to_string()
                        } else {
                            String::new()
                        };
                        let disabled_indices = if disable_last { vec![4] } else { Vec::new() };

                        view! {
                            <div class="docs-card" data-slot="list-workbench-canvas">
                                <List
                                    id_base="docs-list-workbench".to_string()
                                    items=workbench_items.clone()
                                    selected_index=workbench_selected
                                    set_selected_index=set_workbench_selected
                                    aria_label="List workbench".to_string()
                                    sync_active_index_to_selected=sync_active
                                    disabled=root_disabled
                                    disabled_indices=disabled_indices
                                    class_name=class_name
                                />
                            </div>
                        }
                        .into_any()
                    }}
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn menu() -> AnyView {
    let items: Arc<[String]> = vec![
        "New file".to_string(),
        "Share with team".to_string(),
        "Sort ascending".to_string(),
    ]
    .into();
    let disabled_items: Arc<[String]> = vec![
        "Duplicate".to_string(),
        "Move".to_string(),
        "Archive".to_string(),
    ]
    .into();
    let empty_items: Arc<[String]> = Vec::<String>::new().into();

    let (last, set_last) = signal(None::<usize>);
    let (share_checked, set_share_checked) = signal(true);
    let (sort_ascending, set_sort_ascending) = signal(true);

    let on_action = Callback::new(move |index: usize| {
        set_last.set(Some(index));
        match index {
            1 => set_share_checked.update(|value| *value = !*value),
            2 => set_sort_ascending.update(|value| *value = !*value),
            _ => {}
        }
    });

    let noop_action = Callback::new(|_: usize| {});

    let code = Signal::derive(move || {
        r#"let on_action = Callback::new(move |_: usize| {});

<Menu
  id_base="menu".to_string()
  items=vec!["New file".to_string(), "Share with team".to_string()].into()
  on_action=on_action
  aria_label="File actions".to_string()
  item_kinds=vec![
    MenuItemKind::Action,
    MenuItemKind::Checkbox { is_checked: Signal::derive(|| true) },
  ]
/>"#
        .to_string()
    });

    let states_code = Signal::derive(move || {
        r#"let on_action = Callback::new(move |_: usize| {});

<Menu
  id_base="menu-disabled".to_string()
  items=vec!["Duplicate".to_string(), "Move".to_string(), "Archive".to_string()].into()
  on_action=on_action
  aria_label="Disabled menu".to_string()
  disabled=true
/>
<Menu
  id_base="menu-empty".to_string()
  items=Vec::<String>::new().into()
  on_action=Callback::new(move |_: usize| {})
  aria_label="Empty menu".to_string()
/>"#
        .to_string()
    });

    view! {
        <ComponentPage
            title="Menu"
            slug="menu"
            group="Collections"
            description="ARIA menu with action / checkbox / radio roles, active-highlight motion, and baseline-style root state attrs."
        >
            <Playground title="Kinds + Selection" code_signal=code>
                <div class="docs-stack">
                    <Menu
                        id_base="docs-menu".to_string()
                        items=items
                        on_action=on_action
                        aria_label="File actions".to_string()
                        item_kinds=vec![
                            MenuItemKind::Action,
                            MenuItemKind::Checkbox {
                                is_checked: Signal::derive(move || share_checked.get()),
                            },
                            MenuItemKind::Radio {
                                is_checked: Signal::derive(move || sort_ascending.get()),
                            },
                        ]
                    />
                    <span class="ui-muted">
                        "last action: "
                        {move || last.get().map(|value| value.to_string()).unwrap_or_else(|| "None".to_string())}
                    </span>
                    <span class="ui-muted">
                        "share checked: "
                        {move || share_checked.get()}
                        " · sort ascending: "
                        {move || sort_ascending.get()}
                    </span>
                </div>
            </Playground>

            <Playground title="Disabled + Empty" code_signal=states_code>
                <div class="docs-row">
                    <div class="docs-stack">
                        <Menu
                            id_base="docs-menu-disabled".to_string()
                            items=disabled_items
                            on_action=noop_action
                            aria_label="Disabled menu".to_string()
                            disabled=true
                        />
                        <span class="ui-muted">"disabled menu (no action)"</span>
                    </div>

                    <div class="docs-stack">
                        <Menu
                            id_base="docs-menu-empty".to_string()
                            items=empty_items
                            on_action=noop_action
                            aria_label="Empty menu".to_string()
                        />
                        <span class="ui-muted">"empty menu (0 items)"</span>
                    </div>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn menu_trigger() -> AnyView {
    let default_items = vec![
        "Profile".to_string(),
        "Settings".to_string(),
        "Log out".to_string(),
    ];
    let controlled_items = vec![
        "Rename".to_string(),
        "Duplicate".to_string(),
        "Archive".to_string(),
    ];
    let disabled_items = vec!["Copy".to_string(), "Move".to_string()];
    let empty_items: Vec<String> = Vec::new();

    let (last, set_last) = signal(None::<usize>);
    let on_action = Callback::new(move |index: usize| set_last.set(Some(index)));

    let (controlled_open_raw, set_controlled_open_raw) = signal(false);
    let controlled_open: Signal<bool> = Signal::derive(move || controlled_open_raw.get());
    let on_open_change = Callback::new(move |next: bool| set_controlled_open_raw.set(next));

    let code = Signal::derive(move || {
        r#"<MenuTrigger
  id_base="trigger".to_string()
  items=vec!["Duplicate".to_string(), "Move".to_string(), "Archive".to_string()]
  on_action=Callback::new(move |_: usize| {})
>
  "Open menu"
</MenuTrigger>"#
            .to_string()
    });

    let controlled_code = Signal::derive(move || {
        r#"let (open, set_open) = signal(false);

<MenuTrigger
  id_base="trigger-controlled".to_string()
  items=vec!["Duplicate".to_string(), "Move".to_string(), "Archive".to_string()]
  on_action=Callback::new(move |_: usize| {})
  close_on_action=false
  disabled_indices=vec![1]
  open=Signal::derive(move || open.get())
  on_open_change=Callback::new(move |next| set_open.set(next))
>
  "Controlled"
</MenuTrigger>"#
            .to_string()
    });

    let disabled_code = Signal::derive(move || {
        r#"<MenuTrigger
  id_base="trigger-disabled".to_string()
  items=vec!["Duplicate".to_string(), "Move".to_string(), "Archive".to_string()]
  on_action=Callback::new(move |_: usize| {})
  disabled=true
>
  "Disabled"
</MenuTrigger>
<MenuTrigger
  id_base="trigger-empty".to_string()
  items=Vec::<String>::new()
  on_action=Callback::new(move |_: usize| {})
>
  "Empty"
</MenuTrigger>"#
            .to_string()
    });
    let (workbench_open_raw, set_workbench_open_raw) = signal(false);
    let workbench_open: Signal<bool> = Signal::derive(move || workbench_open_raw.get());
    let on_workbench_open_change =
        Callback::new(move |next: bool| set_workbench_open_raw.set(next));
    let (workbench_last, set_workbench_last) = signal(None::<usize>);
    let on_workbench_action =
        Callback::new(move |index: usize| set_workbench_last.set(Some(index)));
    let (workbench_close_on_action, set_workbench_close_on_action) = signal(true);
    let (workbench_disabled, set_workbench_disabled) = signal(false);
    let (workbench_disable_second, set_workbench_disable_second) = signal(false);
    let (workbench_top_end, set_workbench_top_end) = signal(false);
    let (workbench_custom_label, set_workbench_custom_label) = signal(false);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);
    let workbench_code = Signal::derive(move || {
        let close_on_action = workbench_close_on_action.get();
        let disabled = workbench_disabled.get();
        let disable_second = workbench_disable_second.get();
        let top_end = workbench_top_end.get();
        let custom_label = workbench_custom_label.get();
        let custom_class = workbench_custom_class.get();

        let mut snippet = vec![
            "let (open, set_open) = signal(false);".to_string(),
            "<MenuTrigger".to_string(),
            "  id_base=\"docs-menu-trigger-workbench\".into()".to_string(),
            "  items=vec![\"Profile\".into(), \"Settings\".into(), \"Archive\".into()]".to_string(),
            "  on_action=Callback::new(move |_: usize| {})".to_string(),
            "  open=Signal::derive(move || open.get())".to_string(),
            "  on_open_change=Callback::new(move |next| set_open.set(next))".to_string(),
        ];
        if !close_on_action {
            snippet.push("  close_on_action=false".to_string());
        }
        if disabled {
            snippet.push("  disabled=true".to_string());
        }
        if disable_second {
            snippet.push("  disabled_indices=vec![1]".to_string());
        }
        if top_end {
            snippet.push("  placement=PopoverPlacement::TopEnd".to_string());
        }
        if custom_label {
            snippet.push("  aria_label=\"Workbench menu trigger\".into()".to_string());
        }
        if custom_class {
            snippet.push("  class_name=\"docs-menu-trigger-workbench\".into()".to_string());
        }
        snippet.extend([
            ">".to_string(),
            "  \"Workbench\"".to_string(),
            "</MenuTrigger>".to_string(),
        ]);
        snippet.join("\n")
    });
    let workbench_test_css_source = Signal::derive(move || {
        format!(
            "/* crates/ui-components/src/menu/trigger/styles.rs */\n{}",
            ui_components::menu_trigger::styles::CSS
        )
    });
    let workbench_actual_config = Signal::derive(move || {
        let placement = if workbench_top_end.get() {
            PopoverPlacement::TopEnd
        } else {
            PopoverPlacement::BottomStart
        };
        let disabled_indices = if workbench_disable_second.get() {
            vec![1]
        } else {
            Vec::new()
        };

        format!(
            "MenuTriggerActualConfig {{\n  open: {},\n  close_on_action: {},\n  disabled: {},\n  disabled_indices: {:?},\n  placement: PopoverPlacement::{:?},\n  custom_aria_label: {},\n  custom_class_name: {},\n  last_action: {},\n}}",
            workbench_open_raw.get(),
            workbench_close_on_action.get(),
            workbench_disabled.get(),
            disabled_indices,
            placement,
            workbench_custom_label.get(),
            workbench_custom_class.get(),
            workbench_last
                .get()
                .map(|value| value.to_string())
                .unwrap_or_else(|| "None".to_string())
        )
    });

    view! {
        <ComponentPage
            title="MenuTrigger"
            slug="menu-trigger"
            group="Collections"
            description="Button-triggered menu surface with baseline state attrs and controlled/uncontrolled close-strategy semantics."
        >
            <Playground title="Default" code_signal=code>
                <div class="docs-row">
                    <MenuTrigger
                        id_base="docs-menu-trigger".to_string()
                        items=default_items
                        on_action=on_action
                        item_kinds=vec![
                            MenuItemKind::Action,
                            MenuItemKind::Action,
                            MenuItemKind::Action,
                        ]
                    >
                        "Open menu"
                    </MenuTrigger>
                    <span class="ui-muted">
                        "last: "
                        {move || last.get().map(|v| v.to_string()).unwrap_or_else(|| "None".to_string())}
                    </span>
                </div>
            </Playground>

            <Playground title="Controlled + persistent open" code_signal=controlled_code>
                <div class="docs-stack">
                    <MenuTrigger
                        id_base="docs-menu-trigger-controlled".to_string()
                        items=controlled_items
                        on_action=on_action
                        close_on_action=false
                        disabled_indices=vec![1]
                        open=controlled_open
                        on_open_change=on_open_change
                        item_kinds=vec![
                            MenuItemKind::Action,
                            MenuItemKind::Action,
                            MenuItemKind::Action,
                        ]
                    >
                        "Controlled"
                    </MenuTrigger>
                    <span class="ui-muted">
                        "open: "
                        {move || controlled_open_raw.get()}
                    </span>
                </div>
            </Playground>

            <Playground title="Disabled + Empty" code_signal=disabled_code>
                <div class="docs-row">
                    <MenuTrigger
                        id_base="docs-menu-trigger-disabled".to_string()
                        items=disabled_items
                        on_action=on_action
                        disabled=true
                        item_kinds=vec![MenuItemKind::Action, MenuItemKind::Action]
                    >
                        "Disabled"
                    </MenuTrigger>

                    <MenuTrigger
                        id_base="docs-menu-trigger-empty".to_string()
                        items=empty_items
                        on_action=on_action
                    >
                        "Empty"
                    </MenuTrigger>
                </div>
            </Playground>

            <Playground
                title="Interactive Playground (Display / Config / Code / CSS Test)"
                code_signal=workbench_code
                test_css_source=workbench_test_css_source
                test_source_path="crates/ui-components/src/menu/trigger/styles.rs".to_string()
                test_config_signal=workbench_actual_config
                description="展示区用于 current 与 baseline 对比；Config/Code/CSS Test 区用于行为和样式契约验证。"
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="menu-trigger-config-controls">
                        <button
                            type="button"
                            on:click=move |_| set_workbench_close_on_action.update(|value| *value = !*value)
                        >
                            "Toggle close_on_action"
                        </button>
                        <button
                            type="button"
                            on:click=move |_| set_workbench_disabled.update(|value| *value = !*value)
                        >
                            "Toggle disabled"
                        </button>
                        <button
                            type="button"
                            on:click=move |_| set_workbench_disable_second.update(|value| *value = !*value)
                        >
                            "Toggle disabled item #1"
                        </button>
                        <button
                            type="button"
                            on:click=move |_| set_workbench_top_end.update(|value| *value = !*value)
                        >
                            "Toggle placement (bottom-start/top-end)"
                        </button>
                        <button
                            type="button"
                            on:click=move |_| set_workbench_custom_label.update(|value| *value = !*value)
                        >
                            "Toggle custom aria label"
                        </button>
                        <button
                            type="button"
                            on:click=move |_| set_workbench_custom_class.update(|value| *value = !*value)
                        >
                            "Toggle custom class"
                        </button>
                        <p class="ui-muted" data-slot="menu-trigger-config-summary">
                            {move || {
                                format!(
                                    "config: open={} close_on_action={} disabled={} placement={} custom_label={} custom_class={}",
                                    workbench_open_raw.get(),
                                    workbench_close_on_action.get(),
                                    workbench_disabled.get(),
                                    if workbench_top_end.get() {
                                        "top-end"
                                    } else {
                                        "bottom-start"
                                    },
                                    workbench_custom_label.get(),
                                    workbench_custom_class.get()
                                )
                            }}
                        </p>
                    </div>
                }
            >
                {move || {
                    let placement = if workbench_top_end.get() {
                        PopoverPlacement::TopEnd
                    } else {
                        PopoverPlacement::BottomStart
                    };
                    let disabled_indices = if workbench_disable_second.get() {
                        vec![1]
                    } else {
                        Vec::new()
                    };
                    let aria_label = if workbench_custom_label.get() {
                        "Workbench menu trigger".to_string()
                    } else {
                        String::new()
                    };
                    let class_name = if workbench_custom_class.get() {
                        "docs-menu-trigger-workbench".to_string()
                    } else {
                        String::new()
                    };

                    view! {
                        <div class="docs-stack" data-slot="menu-trigger-workbench-display">
                            <span class="ui-muted">
                                "display: current config vs baseline"
                            </span>
                            <div class="docs-row">
                                <div class="docs-stack">
                                    <span class="ui-muted">"Current"</span>
                                    <MenuTrigger
                                        id_base="docs-menu-trigger-workbench".to_string()
                                        items=vec![
                                            "Profile".to_string(),
                                            "Settings".to_string(),
                                            "Archive".to_string(),
                                        ]
                                        on_action=on_workbench_action
                                        close_on_action=workbench_close_on_action.get()
                                        disabled=workbench_disabled.get()
                                        disabled_indices=disabled_indices
                                        open=workbench_open
                                        on_open_change=on_workbench_open_change
                                        placement=placement
                                        aria_label=aria_label
                                        class_name=class_name
                                        item_kinds=vec![
                                            MenuItemKind::Action,
                                            MenuItemKind::Action,
                                            MenuItemKind::Action,
                                        ]
                                    >
                                        "Workbench"
                                    </MenuTrigger>
                                    <span class="ui-muted">
                                        "open: "
                                        {workbench_open_raw.get()}
                                        " · last: "
                                        {workbench_last
                                            .get()
                                            .map(|value| value.to_string())
                                            .unwrap_or_else(|| "None".to_string())}
                                    </span>
                                </div>

                                <div class="docs-stack">
                                    <span class="ui-muted">"Baseline"</span>
                                    <MenuTrigger
                                        id_base="docs-menu-trigger-workbench-baseline".to_string()
                                        items=vec![
                                            "Profile".to_string(),
                                            "Settings".to_string(),
                                            "Archive".to_string(),
                                        ]
                                        on_action=on_action
                                        item_kinds=vec![
                                            MenuItemKind::Action,
                                            MenuItemKind::Action,
                                            MenuItemKind::Action,
                                        ]
                                    >
                                        "Baseline"
                                    </MenuTrigger>
                                </div>
                            </div>
                        </div>
                    }
                }}
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn select() -> AnyView {
    let hello_items = vec!["Apple".to_string(), "Banana".to_string()];
    let (hello_selected, set_hello_selected) = signal(None::<usize>);

    let items = vec![
        "Apple".to_string(),
        "Banana".to_string(),
        "Cherry".to_string(),
        "Durian".to_string(),
    ];
    let (selected, set_selected) = signal(Some(1_usize));

    let (controlled_open_raw, set_controlled_open_raw) = signal(false);
    let controlled_open: Signal<bool> = controlled_open_raw.into();
    let on_open_change = Callback::new(move |next: bool| set_controlled_open_raw.set(next));

    let disabled_indices = vec![3_usize];
    let disabled_option_count = disabled_indices.len();
    let has_selection = Signal::derive(move || selected.get().is_some());

    let disabled_items = vec!["Oak".to_string(), "Pine".to_string(), "Birch".to_string()];
    let (disabled_selected, set_disabled_selected) = signal(Some(0_usize));

    let empty_items: Vec<String> = Vec::new();
    let (empty_selected, set_empty_selected) = signal(None::<usize>);

    let hello_code = Signal::derive(move || {
        r#"let (selected, set_selected) = signal(None::<usize>);
<Select id_base="select-hello".to_string() items=vec!["Apple".to_string(), "Banana".to_string()] selected_index=selected set_selected_index=set_selected />"#.to_string()
    });

    let code = Signal::derive(move || {
        r#"let (selected, set_selected) = signal(Some(1_usize));
let (open, set_open) = signal(false);
let on_open_change = Callback::new(move |next: bool| set_open.set(next));

<Select
  id_base="fruit".to_string()
  items=vec!["Apple".to_string(), "Banana".to_string(), "Cherry".to_string(), "Durian".to_string()]
  selected_index=selected
  set_selected_index=set_selected
  open=open
  on_open_change=on_open_change
  disabled_indices=vec![3]
/>"#
        .to_string()
    });

    let states_code = Signal::derive(move || {
        r#"let (selected, set_selected) = signal(Some(0_usize));
let (empty_selected, set_empty_selected) = signal(None::<usize>);

<Select
  id_base="select-disabled".to_string()
  items=vec!["Oak".to_string(), "Pine".to_string(), "Birch".to_string()]
  selected_index=selected
  set_selected_index=set_selected
  is_disabled=true
/>
<Select
  id_base="select-empty".to_string()
  items=Vec::<String>::new()
  selected_index=empty_selected
  set_selected_index=set_empty_selected
  placeholder="No options".to_string()
/>"#
        .to_string()
    });

    view! {
        <ComponentPage
            title="Select"
            slug="select"
            group="Collections"
            description="Select with controlled open state, listbox semantics, and baseline-style root state attrs."
        >
            <Playground title="Hello World" code_signal=hello_code>
                <div class="docs-stack">
                    <Select
                        id_base="docs-select-hello".to_string()
                        items=hello_items
                        selected_index=hello_selected
                        set_selected_index=set_hello_selected
                    />
                    <span class="ui-muted">
                        "Start here: default Select wiring with only items + selected signals."
                    </span>
                </div>
            </Playground>

            <Playground title="Controlled Open + Selection" code_signal=code>
                <div class="docs-stack">
                    <Select
                        id_base="docs-select-controlled".to_string()
                        items=items
                        selected_index=selected
                        set_selected_index=set_selected
                        open=controlled_open
                        on_open_change=on_open_change
                        disabled_indices=disabled_indices
                    />
                    <div class="docs-row">
                        <ui_components::Button
                            variant=ui_components::ButtonVariant::Secondary
                            on_press=Callback::new(move |_| {
                                set_controlled_open_raw.update(|value| *value = !*value);
                            })
                        >
                            "Toggle open"
                        </ui_components::Button>
                        <span class="ui-muted">
                            "open: "
                            {move || controlled_open_raw.get()}
                        </span>
                    </div>
                    <span class="ui-muted">
                        "selected: "
                        {move || selected.get().map(|value| value.to_string()).unwrap_or_else(|| "None".to_string())}
                        " · has selection: "
                        {move || has_selection.get()}
                        " · disabled options: "
                        {disabled_option_count}
                    </span>
                </div>
            </Playground>

            <Playground title="Disabled + Empty" code_signal=states_code>
                <div class="docs-row">
                    <div class="docs-stack">
                        <Select
                            id_base="docs-select-disabled".to_string()
                            items=disabled_items
                            selected_index=disabled_selected
                            set_selected_index=set_disabled_selected
                            is_disabled=true
                        />
                        <span class="ui-muted">
                            "disabled selected: "
                            {move || disabled_selected.get().map(|value| value.to_string()).unwrap_or_else(|| "None".to_string())}
                        </span>
                    </div>

                    <div class="docs-stack">
                        <Select
                            id_base="docs-select-empty".to_string()
                            items=empty_items
                            selected_index=empty_selected
                            set_selected_index=set_empty_selected
                            placeholder="No options".to_string()
                        />
                        <span class="ui-muted">
                            "empty selected: "
                            {move || empty_selected.get().map(|value| value.to_string()).unwrap_or_else(|| "None".to_string())}
                        </span>
                    </div>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
pub(super) fn combo_box() -> AnyView {
    let showcase_items = vec![
        "Rust".to_string(),
        "TypeScript".to_string(),
        "Go".to_string(),
        "Python".to_string(),
        "Zig".to_string(),
    ];
    let disabled_items = vec!["Alpha".to_string(), "Beta".to_string(), "Gamma".to_string()];
    let empty_items: Vec<String> = Vec::new();

    let (selected, set_selected) = signal(Some(1_usize));
    let (invalid, set_invalid) = signal(false);

    let (controlled_selected, set_controlled_selected) = signal(Some(2_usize));
    let (controlled_open_raw, set_controlled_open_raw) = signal(false);
    let controlled_open: Signal<bool> = Signal::derive(move || controlled_open_raw.get());
    let on_open_change = Callback::new(move |next: bool| set_controlled_open_raw.set(next));

    let (disabled_selected, set_disabled_selected) = signal(Some(0_usize));
    let (empty_selected, set_empty_selected) = signal(None::<usize>);

    let workbench_items = vec![
        "Rust".to_string(),
        "TypeScript".to_string(),
        "Go".to_string(),
        "Python".to_string(),
        "Zig".to_string(),
    ];
    let (workbench_selected, set_workbench_selected) = signal(Some(1_usize));
    let (workbench_invalid, set_workbench_invalid) = signal(false);
    let (workbench_disabled, set_workbench_disabled) = signal(false);
    let (workbench_disable_last, set_workbench_disable_last) = signal(true);
    let (workbench_controlled_open, set_workbench_controlled_open) = signal(false);
    let (workbench_use_controlled_open, set_workbench_use_controlled_open) = signal(true);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);

    let showcase_code = Signal::derive(move || {
        r#"let items = vec![
  "Rust".to_string(),
  "TypeScript".to_string(),
  "Go".to_string(),
  "Python".to_string(),
  "Zig".to_string(),
];

let (selected_default, set_selected_default) = signal(Some(1_usize));
let (invalid, set_invalid) = signal(false);
let (selected_controlled, set_selected_controlled) = signal(Some(2_usize));
let (open, set_open) = signal(false);
let (selected_disabled, set_selected_disabled) = signal(Some(0_usize));
let (selected_empty, set_selected_empty) = signal(None::<usize>);

<ComboBox
  id_base="combo-default".to_string()
  label="Default".to_string()
  items=items.clone()
  selected_index=selected_default
  set_selected_index=set_selected_default
  disabled_indices=vec![4]
  description="Pick one runtime language".to_string()
  error="Language is required".to_string()
  is_invalid=Signal::derive(move || invalid.get())
/>
<ComboBox
  id_base="combo-controlled".to_string()
  label="Controlled open".to_string()
  items=items.clone()
  selected_index=selected_controlled
  set_selected_index=set_selected_controlled
  is_open=Signal::derive(move || open.get())
  on_open_change=Callback::new(move |next| set_open.set(next))
/>
<ComboBox
  id_base="combo-disabled".to_string()
  label="Disabled".to_string()
  items=vec!["Alpha".to_string(), "Beta".to_string(), "Gamma".to_string()]
  selected_index=selected_disabled
  set_selected_index=set_selected_disabled
  is_disabled=true
/>
<ComboBox
  id_base="combo-empty".to_string()
  label="Empty".to_string()
  items=Vec::<String>::new()
  selected_index=selected_empty
  set_selected_index=set_selected_empty
  placeholder="No options".to_string()
/>"#
        .to_string()
    });

    let workbench_code = Signal::derive(move || {
        let invalid = workbench_invalid.get();
        let disabled = workbench_disabled.get();
        let disable_last = workbench_disable_last.get();
        let use_controlled_open = workbench_use_controlled_open.get();
        let custom_class = workbench_custom_class.get();

        let mut lines = vec![
            "let (selected, set_selected) = signal(Some(1_usize));".to_string(),
            "let (open, set_open) = signal(false);".to_string(),
            "<ComboBox".to_string(),
            "  id_base=\"docs-combo-box-workbench\".into()".to_string(),
            "  label=\"Language\".into()".to_string(),
            "  items=vec![".to_string(),
            "    \"Rust\".into(),".to_string(),
            "    \"TypeScript\".into(),".to_string(),
            "    \"Go\".into(),".to_string(),
            "    \"Python\".into(),".to_string(),
            "    \"Zig\".into(),".to_string(),
            "  ]".to_string(),
            "  selected_index=selected".to_string(),
            "  set_selected_index=set_selected".to_string(),
            "  description=\"Pick one runtime language\".into()".to_string(),
            "  error=\"Language is required\".into()".to_string(),
        ];

        if invalid {
            lines.push("  is_invalid=Signal::derive(move || true)".to_string());
        }
        if disabled {
            lines.push("  is_disabled=true".to_string());
        }
        if disable_last {
            lines.push("  disabled_indices=vec![4]".to_string());
        }
        if use_controlled_open {
            lines.push("  is_open=Signal::derive(move || open.get())".to_string());
            lines
                .push("  on_open_change=Callback::new(move |next| set_open.set(next))".to_string());
        }
        if custom_class {
            lines.push("  class_name=\"docs-combo-box-workbench--custom\".into()".to_string());
        }

        lines.push("/>".to_string());
        lines.join("\n")
    });

    let workbench_test_css = Signal::derive(move || {
        format!(
            "/* crates/ui-components/src/combo_box/styles.rs */\n{}",
            ui_components::combo_box::styles::CSS,
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        let selected = workbench_selected.get();
        let invalid = workbench_invalid.get();
        let disabled = workbench_disabled.get();
        let disable_last = workbench_disable_last.get();
        let open = workbench_controlled_open.get();
        let use_controlled_open = workbench_use_controlled_open.get();
        let custom_class = workbench_custom_class.get();

        let mut class = vec!["ui-combo-box".to_string()];
        if custom_class {
            class.push("docs-combo-box-workbench--custom".to_string());
        }
        if invalid {
            class.push("ui-combo-box--invalid".to_string());
        }
        if use_controlled_open {
            class.push("ui-combo-box--controlled".to_string());
        }

        format!(
            "ComboBoxWorkbenchConfig {{\n  selected_index: {selected:?},\n  is_invalid: {invalid},\n  is_disabled: {disabled},\n  disabled_indices: {},\n  controlled_open_enabled: {use_controlled_open},\n  controlled_open_state: {open},\n  custom_class: {custom_class},\n  class: \"{}\",\n}}",
            if disable_last { "vec![4]" } else { "vec![]" },
            class.join(" ")
        )
    });

    view! {
        <ComponentPage
            title="ComboBox"
            slug="combo-box"
            group="Collections"
            description="Combobox with input + listbox + popover, baseline-style root attrs, and baseline-level panel/highlight motion."
        >
            <Playground
                title="展示：多场景对比"
                description="同一套 ComboBox 在校验、受控 open、禁用、空数据四种状态下的对比展示。"
                code_signal=showcase_code
            >
                <div class="docs-row" data-slot="combo-box-showcase">
                    <div class="docs-stack" style="min-width: 260px; width: min(100%, 320px);">
                        <span class="ui-muted">"validation + disabled option"</span>
                        <ComboBox
                            id_base="docs-combo-box".to_string()
                            label="Language".to_string()
                            items=showcase_items.clone()
                            selected_index=selected
                            set_selected_index=set_selected
                            disabled_indices=vec![4]
                            description="Pick one runtime language".to_string()
                            error="Language is required".to_string()
                            is_invalid=Signal::derive(move || invalid.get())
                        />
                        <div class="docs-row">
                            <ui_components::Button
                                variant=ui_components::ButtonVariant::Secondary
                                on_press=Callback::new(move |_| set_invalid.update(|value| *value = !*value))
                            >
                                {move || if invalid.get() { "Clear invalid" } else { "Mark invalid" }}
                            </ui_components::Button>
                            <span class="ui-muted">
                                "selected: "
                                {move || selected.get().map(|value| value.to_string()).unwrap_or_else(|| "None".to_string())}
                            </span>
                        </div>
                    </div>

                    <div class="docs-stack" style="min-width: 260px; width: min(100%, 320px);">
                        <span class="ui-muted">"controlled open"</span>
                        <ComboBox
                            id_base="docs-combo-box-controlled".to_string()
                            label="Controlled language".to_string()
                            items=showcase_items.clone()
                            selected_index=controlled_selected
                            set_selected_index=set_controlled_selected
                            is_open=controlled_open
                            on_open_change=on_open_change
                            disabled_indices=vec![4]
                            description="Open state is externally controlled".to_string()
                        />
                        <div class="docs-row">
                            <ui_components::Button
                                variant=ui_components::ButtonVariant::Secondary
                                on_press=Callback::new(move |_| {
                                    set_controlled_open_raw.update(|value| *value = !*value)
                                })
                            >
                                "Toggle open"
                            </ui_components::Button>
                            <span class="ui-muted">
                                "open: "
                                {move || controlled_open_raw.get()}
                                " · selected: "
                                {move || controlled_selected.get().map(|value| value.to_string()).unwrap_or_else(|| "None".to_string())}
                            </span>
                        </div>
                    </div>

                    <div class="docs-stack" style="min-width: 260px; width: min(100%, 320px);">
                        <span class="ui-muted">"disabled root"</span>
                        <ComboBox
                            id_base="docs-combo-box-disabled".to_string()
                            label="Disabled language".to_string()
                            items=disabled_items
                            selected_index=disabled_selected
                            set_selected_index=set_disabled_selected
                            is_disabled=true
                        />
                        <span class="ui-muted">
                            "disabled selected: "
                            {move || disabled_selected.get().map(|value| value.to_string()).unwrap_or_else(|| "None".to_string())}
                        </span>
                    </div>

                    <div class="docs-stack" style="min-width: 260px; width: min(100%, 320px);">
                        <span class="ui-muted">"empty items"</span>
                        <ComboBox
                            id_base="docs-combo-box-empty".to_string()
                            label="Empty language list".to_string()
                            items=empty_items
                            selected_index=empty_selected
                            set_selected_index=set_empty_selected
                            placeholder="No options".to_string()
                        />
                        <span class="ui-muted">
                            "empty selected: "
                            {move || empty_selected.get().map(|value| value.to_string()).unwrap_or_else(|| "None".to_string())}
                        </span>
                    </div>
                </div>
            </Playground>

            <Playground
                title="Workbench（展示 + Config + Code + CSS Test）"
                description="按钮式 workbench：单画布调参，支持 settings / code / css-test 面板联动。"
                code_signal=workbench_code
                test_css_source=workbench_test_css
                test_source_path="/root/autodl-tmp/zjj/p/rust-ui/crates/ui-components/src/combo_box/styles.rs".to_string()
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="combo-box-workbench-controls">
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_invalid.get()
                                on:change=move |ev| set_workbench_invalid.set(event_target_checked(&ev))
                            />
                            " Invalid"
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_disabled.get()
                                on:change=move |ev| set_workbench_disabled.set(event_target_checked(&ev))
                            />
                            " Disabled root"
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_disable_last.get()
                                on:change=move |ev| set_workbench_disable_last.set(event_target_checked(&ev))
                            />
                            " Disable last option"
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_use_controlled_open.get()
                                on:change=move |ev| set_workbench_use_controlled_open.set(event_target_checked(&ev))
                            />
                            " Controlled open"
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_custom_class.get()
                                on:change=move |ev| set_workbench_custom_class.set(event_target_checked(&ev))
                            />
                            " Custom class marker"
                        </label>
                    </div>
                }
            >
                <div class="docs-stack" data-slot="combo-box-workbench" style="width: min(100%, 420px);">
                    <div class="docs-row">
                        <ui_components::Button
                            variant=ui_components::ButtonVariant::Secondary
                            on_press=Callback::new(move |_| {
                                set_workbench_controlled_open.update(|value| *value = !*value)
                            })
                        >
                            "Toggle open"
                        </ui_components::Button>
                        <span class="ui-muted">
                            "open: "
                            {move || workbench_controlled_open.get()}
                            " · selected: "
                            {move || workbench_selected.get().map(|value| value.to_string()).unwrap_or_else(|| "None".to_string())}
                        </span>
                    </div>

                    {move || {
                        let invalid = workbench_invalid.get();
                        let disabled = workbench_disabled.get();
                        let disable_last = workbench_disable_last.get();
                        let use_controlled_open = workbench_use_controlled_open.get();
                        let custom_class = workbench_custom_class.get();
                        let controlled_open: Signal<bool> =
                            Signal::derive(move || workbench_controlled_open.get());
                        let on_open_change =
                            Callback::new(move |next: bool| set_workbench_controlled_open.set(next));

                        let class_name = if custom_class {
                            "docs-combo-box-workbench--custom".to_string()
                        } else {
                            String::new()
                        };
                        let disabled_indices = if disable_last { vec![4] } else { Vec::new() };

                        if use_controlled_open {
                            view! {
                                <div class="docs-card" data-slot="combo-box-workbench-canvas">
                                    <ComboBox
                                        id_base="docs-combo-box-workbench".to_string()
                                        label="Language".to_string()
                                        items=workbench_items.clone()
                                        selected_index=workbench_selected
                                        set_selected_index=set_workbench_selected
                                        description="Pick one runtime language".to_string()
                                        error="Language is required".to_string()
                                        is_invalid=Signal::derive(move || invalid)
                                        is_disabled=disabled
                                        disabled_indices=disabled_indices
                                        is_open=controlled_open
                                        on_open_change=on_open_change
                                        class_name=class_name.clone()
                                    />
                                </div>
                            }
                            .into_any()
                        } else {
                            view! {
                                <div class="docs-card" data-slot="combo-box-workbench-canvas">
                                    <ComboBox
                                        id_base="docs-combo-box-workbench".to_string()
                                        label="Language".to_string()
                                        items=workbench_items.clone()
                                        selected_index=workbench_selected
                                        set_selected_index=set_workbench_selected
                                        description="Pick one runtime language".to_string()
                                        error="Language is required".to_string()
                                        is_invalid=Signal::derive(move || invalid)
                                        is_disabled=disabled
                                        disabled_indices=disabled_indices
                                        class_name=class_name
                                    />
                                </div>
                            }
                            .into_any()
                        }
                    }}
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn autocomplete() -> AnyView {
    let hello_items = vec![
        "San Francisco".to_string(),
        "Seattle".to_string(),
        "Shanghai".to_string(),
    ];
    let (hello_selected, set_hello_selected) = signal(None::<usize>);
    let items = vec![
        "San Francisco".to_string(),
        "Seattle".to_string(),
        "Shanghai".to_string(),
        "Shenzhen".to_string(),
        "Singapore".to_string(),
    ];
    let controlled_items = vec![
        "San Francisco".to_string(),
        "Seattle".to_string(),
        "Shanghai".to_string(),
        "Shenzhen".to_string(),
        "Singapore".to_string(),
    ];
    let (selected, set_selected) = signal(Some(1_usize));
    let (invalid, set_invalid) = signal(false);

    let (controlled_selected, set_controlled_selected) = signal(Some(2_usize));
    let (controlled_open_raw, set_controlled_open_raw) = signal(false);
    let controlled_open: Signal<bool> = Signal::derive(move || controlled_open_raw.get());
    let on_open_change = Callback::new(move |next: bool| set_controlled_open_raw.set(next));

    let disabled_items = vec![
        "Berlin".to_string(),
        "Boston".to_string(),
        "Brisbane".to_string(),
    ];
    let (disabled_selected, set_disabled_selected) = signal(Some(0_usize));

    let empty_items: Vec<String> = Vec::new();
    let (empty_selected, set_empty_selected) = signal(None::<usize>);

    let hello_code = Signal::derive(move || {
        r#"let (selected, set_selected) = signal(None::<usize>);

<Autocomplete
  id_base="city".to_string()
  label="City".to_string()
  items=vec!["Sydney".to_string(), "Melbourne".to_string()]
  selected_index=selected
  set_selected_index=set_selected
/>"#
        .to_string()
    });

    let code = Signal::derive(move || {
        r#"let (selected, set_selected) = signal(Some(1_usize));
let (invalid, set_invalid) = signal(false);

<Autocomplete
  id_base="city".to_string()
  label="City".to_string()
  items=vec![
    "Sydney".to_string(),
    "Melbourne".to_string(),
    "Perth".to_string(),
    "Brisbane".to_string(),
  ]
  selected_index=selected
  set_selected_index=set_selected
  disabled_indices=vec![3]
  description="Search and pick one city".to_string()
  error="City is required".to_string()
  is_invalid=Signal::derive(move || invalid.get())
/>"#
        .to_string()
    });

    let controlled_code = Signal::derive(move || {
        r#"let (selected, set_selected) = signal(Some(2_usize));
let (open, set_open) = signal(false);

<Autocomplete
  id_base="city-controlled".to_string()
  label="Controlled city".to_string()
  items=vec![
    "Sydney".to_string(),
    "Melbourne".to_string(),
    "Perth".to_string(),
    "Brisbane".to_string(),
  ]
  selected_index=selected
  set_selected_index=set_selected
  is_open=Signal::derive(move || open.get())
  on_open_change=Callback::new(move |next| set_open.set(next))
  disabled_indices=vec![3]
/>"#
        .to_string()
    });

    let states_code = Signal::derive(move || {
        r#"let (selected, set_selected) = signal(Some(0_usize));
let (empty_selected, set_empty_selected) = signal(None::<usize>);

<Autocomplete
  id_base="city-disabled".to_string()
  label="Disabled city".to_string()
  items=vec!["Sydney".to_string(), "Melbourne".to_string(), "Perth".to_string()]
  selected_index=selected
  set_selected_index=set_selected
  is_disabled=true
/>
<Autocomplete
  id_base="city-empty".to_string()
  label="Empty city list".to_string()
  items=Vec::<String>::new()
  selected_index=empty_selected
  set_selected_index=set_empty_selected
  placeholder="No options".to_string()
/>"#
        .to_string()
    });

    view! {
        <ComponentPage
            title="Autocomplete"
            slug="autocomplete"
            group="Collections"
            description="Combobox-like autocomplete with baseline-style root attrs, controlled/uncontrolled open state, and baseline-level active highlight motion."
        >
            <Playground title="Hello World" code_signal=hello_code>
                <div class="docs-stack" data-slot="autocomplete-hello-world">
                    <Autocomplete
                        id_base="docs-autocomplete-hello".to_string()
                        label="City".to_string()
                        items=hello_items
                        selected_index=hello_selected
                        set_selected_index=set_hello_selected
                    />
                    <span class="ui-muted" data-slot="autocomplete-hello-selected">
                        "selected: "
                        {move || hello_selected.get().map(|value| value.to_string()).unwrap_or_else(|| "None".to_string())}
                    </span>
                </div>
            </Playground>

            <Playground title="Selection + Validation" code_signal=code>
                <div class="docs-stack" data-slot="autocomplete-validation-playground">
                    <Autocomplete
                        id_base="docs-autocomplete".to_string()
                        label="City".to_string()
                        items=items
                        selected_index=selected
                        set_selected_index=set_selected
                        disabled_indices=vec![3]
                        description="Search and pick one city".to_string()
                        error="City is required".to_string()
                        is_invalid=Signal::derive(move || invalid.get())
                        placeholder="Type…".to_string()
                    />
                    <div class="docs-row">
                        <ui_components::Button
                            variant=ui_components::ButtonVariant::Secondary
                            on_press=Callback::new(move |_| set_invalid.update(|value| *value = !*value))
                        >
                            {move || if invalid.get() { "Clear invalid" } else { "Mark invalid" }}
                        </ui_components::Button>
                        <span class="ui-muted">
                            "selected: "
                            {move || selected.get().map(|value| value.to_string()).unwrap_or_else(|| "None".to_string())}
                        </span>
                    </div>
                </div>
            </Playground>

            <Playground title="Controlled Open State" code_signal=controlled_code>
                <div class="docs-stack" data-slot="autocomplete-controlled-playground">
                    <Autocomplete
                        id_base="docs-autocomplete-controlled".to_string()
                        label="Controlled city".to_string()
                        items=controlled_items
                        selected_index=controlled_selected
                        set_selected_index=set_controlled_selected
                        is_open=controlled_open
                        on_open_change=on_open_change
                        disabled_indices=vec![3]
                        description="Open state is externally controlled".to_string()
                    />
                    <span class="ui-muted" data-slot="autocomplete-controlled-open">
                        "open: "
                        {move || controlled_open_raw.get()}
                    </span>
                    <span class="ui-muted" data-slot="autocomplete-controlled-selected">
                        "selected: "
                        {move || controlled_selected.get().map(|value| value.to_string()).unwrap_or_else(|| "None".to_string())}
                    </span>
                </div>
            </Playground>

            <Playground title="Disabled + Empty" code_signal=states_code>
                <div class="docs-row" data-slot="autocomplete-states-playground">
                    <div class="docs-stack" data-slot="autocomplete-disabled-playground">
                        <Autocomplete
                            id_base="docs-autocomplete-disabled".to_string()
                            label="Disabled city".to_string()
                            items=disabled_items
                            selected_index=disabled_selected
                            set_selected_index=set_disabled_selected
                            is_disabled=true
                        />
                        <span class="ui-muted" data-slot="autocomplete-disabled-selected">
                            "disabled selected: "
                            {move || disabled_selected.get().map(|value| value.to_string()).unwrap_or_else(|| "None".to_string())}
                        </span>
                    </div>

                    <div class="docs-stack" data-slot="autocomplete-empty-playground">
                        <Autocomplete
                            id_base="docs-autocomplete-empty".to_string()
                            label="Empty city list".to_string()
                            items=empty_items
                            selected_index=empty_selected
                            set_selected_index=set_empty_selected
                            placeholder="No options".to_string()
                        />
                        <span class="ui-muted" data-slot="autocomplete-empty-selected">
                            "empty selected: "
                            {move || empty_selected.get().map(|value| value.to_string()).unwrap_or_else(|| "None".to_string())}
                        </span>
                    </div>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn dropdown_menu() -> AnyView {
    let default_items = vec![
        "Duplicate".to_string(),
        "Move".to_string(),
        "Archive".to_string(),
    ];
    let controlled_items = vec![
        "Rename".to_string(),
        "Move".to_string(),
        "Share".to_string(),
    ];
    let disabled_items = vec!["Duplicate".to_string(), "Archive".to_string()];
    let empty_items: Vec<String> = Vec::new();

    let (last, set_last) = signal(None::<usize>);
    let on_action = Callback::new(move |index: usize| set_last.set(Some(index)));

    let (controlled_open_raw, set_controlled_open_raw) = signal(false);
    let controlled_open: Signal<bool> = Signal::derive(move || controlled_open_raw.get());
    let on_open_change = Callback::new(move |next: bool| set_controlled_open_raw.set(next));
    let (interactive_last, set_interactive_last) = signal(None::<usize>);
    let on_interactive_action =
        Callback::new(move |index: usize| set_interactive_last.set(Some(index)));
    let (interactive_item_mode, set_interactive_item_mode) = signal(Some(0_usize));
    let (interactive_disabled, set_interactive_disabled) = signal(false);
    let (interactive_close_on_action, set_interactive_close_on_action) = signal(true);
    let (interactive_controlled, set_interactive_controlled) = signal(false);
    let (interactive_with_disabled_items, set_interactive_with_disabled_items) = signal(false);
    let (interactive_custom_class, set_interactive_custom_class) = signal(false);
    let (interactive_custom_motion, set_interactive_custom_motion) = signal(false);
    let (interactive_open_raw, set_interactive_open_raw) = signal(false);
    let interactive_open: Signal<bool> = Signal::derive(move || interactive_open_raw.get());
    let on_interactive_open_change =
        Callback::new(move |next: bool| set_interactive_open_raw.set(next));
    let item_mode_options = vec![
        "3 items".to_string(),
        "2 items".to_string(),
        "empty".to_string(),
    ];

    let interactive_code = Signal::derive(move || {
        let item_mode = interactive_item_mode.get().unwrap_or(0);
        let disabled = interactive_disabled.get();
        let close_on_action = interactive_close_on_action.get();
        let controlled = interactive_controlled.get();
        let with_disabled_items = interactive_with_disabled_items.get();
        let custom_class = interactive_custom_class.get();
        let custom_motion = interactive_custom_motion.get();

        let items_code = match item_mode {
            1 => "vec![\"Rename\".into(), \"Share\".into()]".to_string(),
            2 => "Vec::<String>::new()".to_string(),
            _ => "vec![\"Duplicate\".into(), \"Move\".into(), \"Archive\".into()]".to_string(),
        };

        let mut lines = vec![
            format!("let items = {items_code};"),
            "".to_string(),
            "<DropdownMenu".to_string(),
            "  id_base=\"docs-dropdown-interactive\".into()".to_string(),
            "  items=items".to_string(),
            "  on_action=Callback::new(move |index: usize| { /* ... */ })".to_string(),
        ];

        if disabled {
            lines.push("  disabled=true".to_string());
        }
        if !close_on_action {
            lines.push("  close_on_action=false".to_string());
        }
        if controlled {
            lines.push("  open=Signal::derive(move || open.get())".to_string());
            lines
                .push("  on_open_change=Callback::new(move |next| set_open.set(next))".to_string());
        }
        if with_disabled_items {
            lines.push("  disabled_indices=vec![1]".to_string());
        }
        if custom_class {
            lines.push("  class_name=\"docs-dropdown-custom\".into()".to_string());
        }
        if custom_motion {
            lines.push("  motion=DropdownMenuMotion {".to_string());
            lines.push("    popover: PopoverMotion {".to_string());
            lines.push("      initial_scale: 0.96,".to_string());
            lines.push("      offset_y_px: 14.0,".to_string());
            lines.push("      ..PopoverMotion::default()".to_string());
            lines.push("    },".to_string());
            lines.push("  }".to_string());
        }
        lines.push(">".to_string());
        lines.push("  \"Actions\"".to_string());
        lines.push("</DropdownMenu>".to_string());
        lines.join("\n")
    });

    let interactive_test_css = Signal::derive(move || {
        format!(
            "/* crates/ui-components/src/menu/dropdown_menu/styles.rs */\n{}",
            ui_components::dropdown_menu::styles::CSS
        )
    });

    let interactive_config = Signal::derive(move || {
        let item_count = match interactive_item_mode.get().unwrap_or(0) {
            1 => 2,
            2 => 0,
            _ => 3,
        };
        let motion_source = if interactive_custom_motion.get() {
            "custom"
        } else {
            "default"
        };
        let class_name = if interactive_custom_class.get() {
            "\"docs-dropdown-custom\""
        } else {
            "None"
        };
        format!(
            "DropdownMenuActualConfig {{\n  item_count: {item_count},\n  disabled: {},\n  close_on_action: {},\n  controlled: {},\n  open: {},\n  disabled_indices: {},\n  class_name: {class_name},\n  motion_source: \"{motion_source}\",\n}}",
            interactive_disabled.get(),
            interactive_close_on_action.get(),
            interactive_controlled.get(),
            interactive_open_raw.get(),
            if interactive_with_disabled_items.get() {
                "[1]"
            } else {
                "[]"
            },
        )
    });

    let code = Signal::derive(move || {
        r#"<DropdownMenu
  id_base="dd".to_string()
  items=vec!["Duplicate".to_string(), "Move".to_string(), "Archive".to_string()]
  on_action=Callback::new(move |_: usize| {})
>
  "Open"
</DropdownMenu>"#
            .to_string()
    });

    let controlled_code = Signal::derive(move || {
        r#"let (open, set_open) = signal(false);

<DropdownMenu
  id_base="dd-controlled".to_string()
  items=vec!["Rename".to_string(), "Move".to_string(), "Share".to_string()]
  on_action=Callback::new(move |_: usize| {})
  open=Signal::derive(move || open.get())
  on_open_change=Callback::new(move |next| set_open.set(next))
  close_on_action=false
  disabled_indices=vec![1]
>
  "Persistent"
</DropdownMenu>"#
            .to_string()
    });

    let disabled_code = Signal::derive(move || {
        r#"<DropdownMenu
  id_base="dd-disabled".to_string()
  items=vec!["Duplicate".to_string(), "Archive".to_string()]
  on_action=Callback::new(move |_: usize| {})
  disabled=true
>
  "Disabled"
</DropdownMenu>
<DropdownMenu
  id_base="dd-empty".to_string()
  items=Vec::<String>::new()
  on_action=Callback::new(move |_: usize| {})
>
  "Empty"
</DropdownMenu>"#
            .to_string()
    });

    view! {
        <ComponentPage
            title="DropdownMenu"
            slug="dropdown-menu"
            group="Collections"
            description="Button trigger that opens a Menu in a Popover with baseline-style root attrs, controlled/uncontrolled state, and persistent-open action handling."
        >
            <Playground
                title="Interactive Playground"
                description="Display + Config + Code + CSS Test: tune close strategy, control mode, and state markers."
                code_signal=interactive_code
                test_css_source=interactive_test_css
                test_source_path="crates/ui-components/src/menu/dropdown_menu/styles.rs".to_string()
                test_config_signal=interactive_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight">
                        <div class="docs-search__label">"Items"</div>
                        <SegmentedControl
                            id_base="docs-dropdown-item-mode".to_string()
                            options=item_mode_options.clone()
                            selected_index=interactive_item_mode
                            set_selected_index=set_interactive_item_mode
                            size=SegmentedControlSize::Sm
                            aria_label="Dropdown item mode".to_string()
                        />

                        <Switch checked=interactive_disabled set_checked=set_interactive_disabled>
                            "Disabled trigger"
                        </Switch>
                        <Switch
                            checked=interactive_close_on_action
                            set_checked=set_interactive_close_on_action
                        >
                            "Close on action"
                        </Switch>
                        <Switch checked=interactive_controlled set_checked=set_interactive_controlled>
                            "Controlled open"
                        </Switch>
                        <Switch
                            checked=interactive_with_disabled_items
                            set_checked=set_interactive_with_disabled_items
                        >
                            "Disabled index = [1]"
                        </Switch>
                        <Switch
                            checked=interactive_custom_class
                            set_checked=set_interactive_custom_class
                        >
                            "Custom class"
                        </Switch>
                        <Switch
                            checked=interactive_custom_motion
                            set_checked=set_interactive_custom_motion
                        >
                            "Custom motion"
                        </Switch>
                    </div>
                }
            >
                {move || {
                    let item_mode = interactive_item_mode.get().unwrap_or(0);
                    let items = match item_mode {
                        1 => vec!["Rename".to_string(), "Share".to_string()],
                        2 => Vec::<String>::new(),
                        _ => vec![
                            "Duplicate".to_string(),
                            "Move".to_string(),
                            "Archive".to_string(),
                        ],
                    };
                    let disabled_indices = if interactive_with_disabled_items.get() {
                        vec![1]
                    } else {
                        Vec::new()
                    };
                    let item_kinds = if items.is_empty() {
                        Vec::new()
                    } else {
                        vec![MenuItemKind::Action; items.len()]
                    };
                    let motion = if interactive_custom_motion.get() {
                        DropdownMenuMotion {
                            popover: ui_components::PopoverMotion {
                                initial_scale: 0.96,
                                offset_y_px: 14.0,
                                ..ui_components::PopoverMotion::default()
                            },
                        }
                    } else {
                        DropdownMenuMotion::default()
                    };
                    let class_name = if interactive_custom_class.get() {
                        "docs-dropdown-custom".to_string()
                    } else {
                        String::new()
                    };

                    if interactive_controlled.get() {
                        view! {
                            <div class="docs-stack docs-stack--tight">
                                <DropdownMenu
                                    id_base="docs-dropdown-interactive".to_string()
                                    items=items
                                    on_action=on_interactive_action
                                    disabled=interactive_disabled.get()
                                    disabled_indices=disabled_indices
                                    item_kinds=item_kinds
                                    close_on_action=interactive_close_on_action.get()
                                    open=interactive_open
                                    on_open_change=on_interactive_open_change
                                    motion=motion
                                    class_name=class_name.clone()
                                >
                                    "Interactive"
                                </DropdownMenu>
                                <span class="ui-muted">
                                    "last: "
                                    {move || interactive_last.get().map(|v| v.to_string()).unwrap_or_else(|| "None".to_string())}
                                    " · open: "
                                    {move || interactive_open_raw.get()}
                                </span>
                            </div>
                        }
                            .into_any()
                    } else {
                        view! {
                            <div class="docs-stack docs-stack--tight">
                                <DropdownMenu
                                    id_base="docs-dropdown-interactive".to_string()
                                    items=items
                                    on_action=on_interactive_action
                                    disabled=interactive_disabled.get()
                                    disabled_indices=disabled_indices
                                    item_kinds=item_kinds
                                    close_on_action=interactive_close_on_action.get()
                                    motion=motion
                                    class_name=class_name
                                >
                                    "Interactive"
                                </DropdownMenu>
                                <span class="ui-muted">
                                    "last: "
                                    {move || interactive_last.get().map(|v| v.to_string()).unwrap_or_else(|| "None".to_string())}
                                </span>
                            </div>
                        }
                            .into_any()
                    }
                }}
            </Playground>

            <Playground title="Default" code_signal=code>
                <div class="docs-row">
                    <DropdownMenu
                        id_base="docs-dropdown".to_string()
                        items=default_items
                        on_action=on_action
                        item_kinds=vec![
                            MenuItemKind::Action,
                            MenuItemKind::Action,
                            MenuItemKind::Action,
                        ]
                    >
                        "Open"
                    </DropdownMenu>
                    <span class="ui-muted">
                        "last: "
                        {move || last.get().map(|v| v.to_string()).unwrap_or_else(|| "None".to_string())}
                    </span>
                </div>
            </Playground>

            <Playground title="Controlled + Persistent Open" code_signal=controlled_code>
                <div class="docs-stack">
                    <DropdownMenu
                        id_base="docs-dropdown-controlled".to_string()
                        items=controlled_items
                        on_action=on_action
                        open=controlled_open
                        on_open_change=on_open_change
                        close_on_action=false
                        disabled_indices=vec![1]
                        item_kinds=vec![
                            MenuItemKind::Action,
                            MenuItemKind::Action,
                            MenuItemKind::Action,
                        ]
                    >
                        "Controlled"
                    </DropdownMenu>
                    <span class="ui-muted">
                        "open: "
                        {move || controlled_open_raw.get()}
                    </span>
                    <span class="ui-muted">"close_on_action: false (select keeps popover open)"</span>
                </div>
            </Playground>

            <Playground title="Disabled + Empty" code_signal=disabled_code>
                <div class="docs-row">
                    <DropdownMenu
                        id_base="docs-dropdown-disabled".to_string()
                        items=disabled_items
                        on_action=on_action
                        disabled=true
                        item_kinds=vec![MenuItemKind::Action, MenuItemKind::Action]
                    >
                        "Disabled"
                    </DropdownMenu>

                    <DropdownMenu
                        id_base="docs-dropdown-empty".to_string()
                        items=empty_items
                        on_action=on_action
                    >
                        "Empty"
                    </DropdownMenu>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
pub(super) fn pagination() -> AnyView {
    let (page, set_page) = signal(1_usize);
    let (last_change, set_last_change) = signal(None::<usize>);
    let on_change = Callback::new(move |next: usize| set_last_change.set(Some(next)));

    let (compact_page, set_compact_page) = signal(8_usize);
    let (wide_page, set_wide_page) = signal(8_usize);
    let (code_page, set_code_page) = signal(3_usize);
    let (css_page, set_css_page) = signal(5_usize);

    let (first_page, set_first_page) = signal(1_usize);
    let (middle_page, set_middle_page) = signal(6_usize);
    let (last_page, set_last_page) = signal(12_usize);
    let (disabled_page, set_disabled_page) = signal(1_usize);
    let (empty_page, set_empty_page) = signal(1_usize);

    let display_code = Signal::derive(move || {
        r#"let (page, set_page) = signal(1_usize);
let on_change = Callback::new(move |next: usize| { /* ... */ });
<Pagination total_pages=12 page=page set_page=set_page siblings=1 boundaries=1 on_change=on_change />"#.to_string()
    });

    let config_code = Signal::derive(move || {
        r#"<Pagination total_pages=20 page=compact_page set_page=set_compact_page siblings=0 boundaries=1 />
<Pagination total_pages=20 page=wide_page set_page=set_wide_page siblings=2 boundaries=2 />"#.to_string()
    });

    let code_example = Signal::derive(move || {
        r#"let (page, set_page) = signal(3_usize);
let on_change = Callback::new(move |next: usize| { /* sync route/query */ });
<Pagination total_pages=9 page=page set_page=set_page on_change=on_change />"#
            .to_string()
    });

    let css_test_code = Signal::derive(move || {
        r#"<Pagination
  total_pages=10
  page=css_page
  set_page=set_css_page
  class_name="docs-pagination-custom".to_string()
/>"#
        .to_string()
    });

    let states_code = Signal::derive(move || {
        r#"<Pagination total_pages=12 page=first_page set_page=set_first_page />
<Pagination total_pages=12 page=middle_page set_page=set_middle_page />
<Pagination total_pages=12 page=last_page set_page=set_last_page />
<Pagination total_pages=1 page=disabled_page set_page=set_disabled_page disabled=true />
<Pagination total_pages=0 page=empty_page set_page=set_empty_page />"#
            .to_string()
    });

    view! {
        <ComponentPage
            title="Pagination"
            slug="pagination"
            group="Collections"
            description="Pagination control with display/config/code/css-test/state-matrix playgrounds and baseline-style state attrs."
        >
            <Playground title="展示 Display" code_signal=display_code>
                <div class="docs-stack" data-slot="pagination-display-playground">
                    <Pagination
                        total_pages=12
                        page=page
                        set_page=set_page
                        siblings=1
                        boundaries=1
                        on_change=on_change
                    />
                    <span class="ui-muted">"page: " {move || page.get()}</span>
                    <span class="ui-muted">
                        "last change: "
                        {move || last_change.get().map(|value| value.to_string()).unwrap_or_else(|| "None".to_string())}
                    </span>
                </div>
            </Playground>

            <Playground title="Config 配置对比" code_signal=config_code>
                <div class="docs-row" data-slot="pagination-config-playground">
                    <div class="docs-stack" data-slot="pagination-config-compact">
                        <Pagination
                            total_pages=20
                            page=compact_page
                            set_page=set_compact_page
                            siblings=0
                            boundaries=1
                        />
                        <span class="ui-muted">
                            "compact config (siblings=0 boundaries=1): "
                            {move || compact_page.get()}
                        </span>
                    </div>

                    <div class="docs-stack" data-slot="pagination-config-wide">
                        <Pagination
                            total_pages=20
                            page=wide_page
                            set_page=set_wide_page
                            siblings=2
                            boundaries=2
                        />
                        <span class="ui-muted">
                            "wide config (siblings=2 boundaries=2): "
                            {move || wide_page.get()}
                        </span>
                    </div>
                </div>
            </Playground>

            <Playground title="Code 代码示例" code_signal=code_example>
                <div class="docs-stack" data-slot="pagination-code-playground">
                    <Pagination
                        total_pages=9
                        page=code_page
                        set_page=set_code_page
                        on_change=on_change
                    />
                    <span class="ui-muted">
                        "code sample page: "
                        {move || code_page.get()}
                    </span>
                </div>
            </Playground>

            <Playground title="CSS Test" code_signal=css_test_code>
                <div class="docs-row" data-slot="pagination-css-test-playground">
                    <div class="docs-stack" data-slot="pagination-css-test-custom">
                        <Pagination
                            total_pages=10
                            page=css_page
                            set_page=set_css_page
                            class_name="docs-pagination-custom".to_string()
                        />
                        <span class="ui-muted">
                            "custom class: docs-pagination-custom"
                        </span>
                    </div>

                    <div class="docs-stack" data-slot="pagination-css-test-default">
                        <Pagination
                            total_pages=10
                            page=css_page
                            set_page=set_css_page
                        />
                        <span class="ui-muted">"default style (for comparison)"</span>
                    </div>
                </div>
            </Playground>

            <Playground title="状态对比 State Matrix" code_signal=states_code>
                <div class="docs-row" data-slot="pagination-states-playground">
                    <div class="docs-stack" data-slot="pagination-state-first">
                        <Pagination total_pages=12 page=first_page set_page=set_first_page />
                        <span class="ui-muted">"first page: " {move || first_page.get()}</span>
                    </div>

                    <div class="docs-stack" data-slot="pagination-state-middle">
                        <Pagination total_pages=12 page=middle_page set_page=set_middle_page />
                        <span class="ui-muted">"middle page: " {move || middle_page.get()}</span>
                    </div>

                    <div class="docs-stack" data-slot="pagination-state-last">
                        <Pagination total_pages=12 page=last_page set_page=set_last_page />
                        <span class="ui-muted">"last page: " {move || last_page.get()}</span>
                    </div>

                    <div class="docs-stack" data-slot="pagination-state-disabled">
                        <Pagination
                            total_pages=1
                            page=disabled_page
                            set_page=set_disabled_page
                            disabled=true
                        />
                        <span class="ui-muted">
                            "disabled page: "
                            {move || disabled_page.get()}
                        </span>
                    </div>

                    <div class="docs-stack" data-slot="pagination-state-empty">
                        <Pagination
                            total_pages=0
                            page=empty_page
                            set_page=set_empty_page
                        />
                        <span class="ui-muted">
                            "empty page signal: "
                            {move || empty_page.get()}
                        </span>
                    </div>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
pub(super) fn tag_group() -> AnyView {
    let (hello_tags, _set_hello_tags) = signal(vec![
        Tag::new("tag-rust", "Rust"),
        Tag::new("tag-leptos", "Leptos"),
    ]);
    let (removable_tags, set_removable_tags) = signal(vec![
        Tag::new("tag-rust", "Rust"),
        Tag::new("tag-leptos", "Leptos"),
        Tag::disabled("tag-a11y", "Accessibility"),
    ]);

    let on_remove_removable = Callback::new(move |tag: Tag| {
        set_removable_tags.update(|list| list.retain(|item| item.id != tag.id));
    });

    let removable_count = Signal::derive(move || removable_tags.get().len());
    let removable_has_disabled =
        Signal::derive(move || removable_tags.get().iter().any(|tag| tag.disabled));

    let (validation_tags, set_validation_tags) = signal(vec![
        Tag::new("tag-required", "Required"),
        Tag::new("tag-baseline", "Baseline"),
    ]);

    let on_remove_validation = Callback::new(move |tag: Tag| {
        set_validation_tags.update(|list| list.retain(|item| item.id != tag.id));
    });

    let validation_invalid = Signal::derive(move || validation_tags.get().is_empty());
    let validation_required = Signal::derive(|| true);

    let (disabled_tags, _set_disabled_tags) = signal(vec![
        Tag::new("tag-motion", "Motion"),
        Tag::new("tag-tokens", "Tokens"),
    ]);
    let (empty_tags, _set_empty_tags) = signal(Vec::<Tag>::new());

    let hello_world_code = Signal::derive(|| {
        r#"<TagGroup
  tags=Signal::derive(|| vec![
    Tag::new("tag-rust", "Rust"),
    Tag::new("tag-leptos", "Leptos"),
  ])
  label="Tech tags".to_string()
/>"#
        .to_string()
    });

    let code = Signal::derive(move || {
        r#"let (tags, set_tags) = signal(vec![
  Tag::new("tag-rust", "Rust"),
  Tag::disabled("tag-a11y", "Accessibility"),
]);
let on_remove = Callback::new(move |tag: Tag| {
  set_tags.update(|list| list.retain(|item| item.id != tag.id));
});
<TagGroup tags=tags on_remove=on_remove label="Framework tags".to_string() />"#
            .to_string()
    });

    let states_code = Signal::derive(move || {
        r#"let (tags, set_tags) = signal(vec![
  Tag::new("tag-rust", "Rust"),
  Tag::disabled("tag-a11y", "Accessibility"),
]);
let on_remove = Callback::new(move |tag: Tag| {
  set_tags.update(|list| list.retain(|item| item.id != tag.id));
});
let invalid = Signal::derive(move || tags.get().is_empty());

<TagGroup
  tags=tags
  on_remove=on_remove
  label="Required tags".to_string()
  description="Remove all tags to trigger invalid state".to_string()
  error="At least one tag is required".to_string()
  invalid=invalid
  required=Signal::derive(|| true)
/>"#
        .to_string()
    });

    let disabled_empty_code = Signal::derive(move || {
        r#"<TagGroup
  tags=disabled_tags
  disabled=true
  label="Disabled tags".to_string()
  description="All chips are non-removable when disabled".to_string()
/>
<TagGroup
  tags=empty_tags
  label="Empty tags".to_string()
  description="No tags currently selected".to_string()
  error="At least one tag is required".to_string()
  invalid=Signal::derive(|| true)
  required=Signal::derive(|| true)
/>"#
        .to_string()
    });

    view! {
        <ComponentPage
            title="TagGroup"
            slug="tag-group"
            group="Collections"
            description="Tag list with removable chips, validation semantics, and baseline-style root state attrs."
        >
            <Playground
                title="Hello World"
                code_signal=hello_world_code
                test_source_path="crates/ui-components/src/tag/group/view.rs".to_string()
            >
                <TagGroup
                    tags=hello_tags
                    label="Tech tags".to_string()
                />
            </Playground>

            <Playground
                title="Removable + State"
                code_signal=code
                test_source_path="crates/ui-components/src/tag/group/view.rs".to_string()
            >
                <div class="docs-stack">
                    <TagGroup
                        tags=removable_tags
                        on_remove=on_remove_removable
                        label="Framework tags".to_string()
                        description="Remove any non-disabled tag".to_string()
                    />
                    <span class="ui-muted">
                        "count: "
                        {move || removable_count.get()}
                    </span>
                    <span class="ui-muted">
                        "has disabled tags: "
                        {move || removable_has_disabled.get()}
                    </span>
                </div>
            </Playground>

            <Playground
                title="Validation + Required"
                code_signal=states_code
                test_source_path="crates/ui-components/src/tag/group/view.rs".to_string()
            >
                <div class="docs-stack">
                    <TagGroup
                        tags=validation_tags
                        on_remove=on_remove_validation
                        label="Required tags".to_string()
                        description="Remove all tags to trigger invalid state".to_string()
                        error="At least one tag is required".to_string()
                        invalid=validation_invalid
                        required=validation_required
                    />
                    <span class="ui-muted">
                        "invalid: "
                        {move || validation_invalid.get()}
                    </span>
                </div>
            </Playground>

            <Playground
                title="Disabled + Empty"
                code_signal=disabled_empty_code
                test_source_path="crates/ui-components/src/tag/group/view.rs".to_string()
            >
                <div class="docs-row">
                    <div class="docs-stack">
                        <TagGroup
                            tags=disabled_tags
                            disabled=true
                            label="Disabled tags".to_string()
                            description="All chips are non-removable when disabled".to_string()
                        />
                        <span class="ui-muted">"disabled: true"</span>
                    </div>

                    <div class="docs-stack">
                        <TagGroup
                            tags=empty_tags
                            label="Empty tags".to_string()
                            description="No tags currently selected".to_string()
                            error="At least one tag is required".to_string()
                            invalid=Signal::derive(|| true)
                            required=Signal::derive(|| true)
                        />
                        <span class="ui-muted">"empty: true"</span>
                    </div>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
