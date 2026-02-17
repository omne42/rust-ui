use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use std::collections::BTreeSet;
use std::sync::Arc;
use ui_components::{
    Accordion, AccordionItem, AccordionSelectionMode, AccordionStreamingProjection,
    AccordionVariant, AiOutputStatus, AiRenderMode, AiSpace, Autocomplete, BreadcrumbItem,
    Breadcrumbs, ComboBox, Disclosure, DropdownMenu, List, Menu, MenuItemKind, MenuTrigger,
    Pagination, Select, Tabs, TabsKeyboardActivation, Tag, TagGroup, open_set,
    project_streaming_accordion_markup,
};

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
        let _ = storage.set_item(ACCORDION_WORKBENCH_STORAGE_KEY, &encode_open_set(indices));
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn save_workbench_open(_indices: &BTreeSet<usize>) {}

#[cfg(target_arch = "wasm32")]
fn clear_workbench_open() {
    if let Some(storage) =
        web_sys::window().and_then(|window| window.local_storage().ok().flatten())
    {
        let _ = storage.remove_item(ACCORDION_WORKBENCH_STORAGE_KEY);
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn clear_workbench_open() {}

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
    out.push_str("  id_base=\"docs-accordion-ai-stream\".to_string()\n");
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
            "  <AccordionItem key={key} label=\"{label}\".to_string() default_open={is_open}>\"{text}\"</AccordionItem>\n"
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
        input_code.to_string()
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

    view! {
        <ComponentPage
            title="Breadcrumbs"
            slug="breadcrumbs"
            group="Collections"
            description="Breadcrumb nav with current-page semantics and baseline-style root state attrs."
        >
            <Playground title="Trail" code_signal=code>
                <Breadcrumbs items=items />
            </Playground>

            <Playground title="Label-Only + Empty" code_signal=states_code>
                <div class="docs-row">
                    <div class="docs-stack">
                        <Breadcrumbs items=label_only_items aria_label="Label-only trail".to_string() />
                        <span class="ui-muted">"all labels (no links)"</span>
                    </div>
                    <div class="docs-stack">
                        <Breadcrumbs items=empty_items aria_label="Empty trail".to_string() />
                        <span class="ui-muted">"empty trail (0 items)"</span>
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
</Accordion>"#
            .to_string()
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
</Accordion>"#
            .to_string()
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
</Accordion>"#
            .to_string()
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
                        {move || streaming_chars_clamped.get().to_string()}
                        "/"
                        {streaming_total_chars.to_string()}
                        " | items rendered: "
                        {move || streaming_rendered_items.get().to_string()}
                        "/"
                        {streaming_total_items.to_string()}
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
                        {move || open.get().to_string()}
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
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn tabs() -> AnyView {
    let labels = vec!["Overview", "Details", "Settings"];
    let manual_labels = vec!["Profile", "Billing", "Team"];

    let (selected_auto, set_selected_auto) = signal(0_usize);
    let on_auto_change = Callback::new(move |index: usize| set_selected_auto.set(index));

    let (selected_manual, set_selected_manual) = signal(1_usize);
    let on_manual_change = Callback::new(move |index: usize| set_selected_manual.set(index));

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

    view! {
        <ComponentPage
            title="Tabs"
            slug="tabs"
            group="Collections"
            description="Tabs with roving tabindex, baseline-level indicator motion, and baseline-style root state attrs."
        >
            <Playground title="Automatic + Controlled" code_signal=code>
                <div class="docs-stack">
                    <Tabs
                        labels=labels
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
                        {move || selected_auto.get().to_string()}
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
                        {move || selected_manual.get().to_string()}
                    </span>
                    <span class="ui-muted">"disabled tab index: 2"</span>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn list() -> AnyView {
    let items: Arc<[String]> = vec![
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
    let (selected, set_selected) = signal(Some(0_usize));
    let (disabled_selected, set_disabled_selected) = signal(Some(0_usize));

    let code = Signal::derive(move || {
        r#"let items: Arc<[String]> = vec![
  "Overview".to_string(),
  "Billing".to_string(),
  "Integrations".to_string(),
].into();
let (selected, set_selected) = signal(Some(0_usize));

<List
  id_base="settings-nav".to_string()
  items=items
  selected_index=selected
  set_selected_index=set_selected
  aria_label="Settings navigation".to_string()
  disabled_indices=vec![2]
/>"#
        .to_string()
    });

    let states_code = Signal::derive(move || {
        r#"<List
  id_base="settings-nav-disabled".to_string()
  items=vec![
    "Overview".to_string(),
    "Billing".to_string(),
    "Integrations".to_string(),
  ].into()
  selected_index=disabled_selected
  set_selected_index=set_disabled_selected
  disabled=true
  sync_active_index_to_selected=false
  aria_label="Disabled navigation".to_string()
/>"#
        .to_string()
    });

    view! {
        <ComponentPage
            title="List"
            slug="list"
            group="Collections"
            description="List primitive with centralized root-state markers and optional active-index sync controls."
        >
            <Playground title="Selection + Disabled Item" code_signal=code>
                <div class="docs-stack">
                    <List
                        id_base="docs-list".to_string()
                        items=items
                        selected_index=selected
                        set_selected_index=set_selected
                        aria_label="Settings navigation".to_string()
                        disabled_indices=vec![2]
                    />
                    <span class="ui-muted">
                        "selected: "
                        {move || selected.get().map(|value| value.to_string()).unwrap_or_else(|| "None".to_string())}
                    </span>
                </div>
            </Playground>

            <Playground title="Disabled Root + Unsynced Active Index" code_signal=states_code>
                <List
                    id_base="docs-list-disabled".to_string()
                    items=disabled_items
                    selected_index=disabled_selected
                    set_selected_index=set_disabled_selected
                    disabled=true
                    sync_active_index_to_selected=false
                    aria_label="Disabled navigation".to_string()
                />
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
                        {move || share_checked.get().to_string()}
                        " · sort ascending: "
                        {move || sort_ascending.get().to_string()}
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
                        {move || controlled_open_raw.get().to_string()}
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
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn select() -> AnyView {
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
  disabled=true
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
                            {move || controlled_open_raw.get().to_string()}
                        </span>
                    </div>
                    <span class="ui-muted">
                        "selected: "
                        {move || selected.get().map(|value| value.to_string()).unwrap_or_else(|| "None".to_string())}
                        " · has selection: "
                        {move || has_selection.get().to_string()}
                        " · disabled options: "
                        {disabled_option_count.to_string()}
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
                            disabled=true
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
    let items = vec![
        "Rust".to_string(),
        "TypeScript".to_string(),
        "Go".to_string(),
        "Python".to_string(),
        "Zig".to_string(),
    ];
    let controlled_items = vec![
        "Rust".to_string(),
        "TypeScript".to_string(),
        "Go".to_string(),
        "Python".to_string(),
        "Zig".to_string(),
    ];
    let (selected, set_selected) = signal(Some(1_usize));
    let (invalid, set_invalid) = signal(false);

    let (controlled_selected, set_controlled_selected) = signal(Some(2_usize));
    let (controlled_open_raw, set_controlled_open_raw) = signal(false);
    let controlled_open: Signal<bool> = Signal::derive(move || controlled_open_raw.get());
    let on_open_change = Callback::new(move |next: bool| set_controlled_open_raw.set(next));

    let disabled_items = vec!["Alpha".to_string(), "Beta".to_string(), "Gamma".to_string()];
    let (disabled_selected, set_disabled_selected) = signal(Some(0_usize));

    let empty_items: Vec<String> = Vec::new();
    let (empty_selected, set_empty_selected) = signal(None::<usize>);

    let code = Signal::derive(move || {
        r#"let (selected, set_selected) = signal(Some(1_usize));
let (invalid, set_invalid) = signal(false);

<ComboBox
  id_base="lang".to_string()
  label="Language".to_string()
  items=vec![
    "Rust".to_string(),
    "TypeScript".to_string(),
    "Go".to_string(),
    "Python".to_string(),
    "Swift".to_string(),
  ]
  selected_index=selected
  set_selected_index=set_selected
  disabled_indices=vec![4]
  description="Pick one runtime language".to_string()
  error="Language is required".to_string()
  is_invalid=Signal::derive(move || invalid.get())
/>"#
        .to_string()
    });

    let controlled_code = Signal::derive(move || {
        r#"let (selected, set_selected) = signal(Some(2_usize));
let (open, set_open) = signal(false);

<ComboBox
  id_base="lang-controlled".to_string()
  label="Controlled language".to_string()
  items=vec![
    "Rust".to_string(),
    "TypeScript".to_string(),
    "Go".to_string(),
    "Python".to_string(),
    "Swift".to_string(),
  ]
  selected_index=selected
  set_selected_index=set_selected
  is_open=Signal::derive(move || open.get())
  on_open_change=Callback::new(move |next| set_open.set(next))
  disabled_indices=vec![4]
/>"#
        .to_string()
    });

    let states_code = Signal::derive(move || {
        r#"let (selected, set_selected) = signal(Some(0_usize));
let (empty_selected, set_empty_selected) = signal(None::<usize>);

<ComboBox
  id_base="lang-disabled".to_string()
  label="Disabled language".to_string()
  items=vec!["Alpha".to_string(), "Beta".to_string(), "Gamma".to_string()]
  selected_index=selected
  set_selected_index=set_selected
  is_disabled=true
/>
<ComboBox
  id_base="lang-empty".to_string()
  label="Empty language list".to_string()
  items=Vec::<String>::new()
  selected_index=empty_selected
  set_selected_index=set_empty_selected
  placeholder="No options".to_string()
/>"#
        .to_string()
    });

    view! {
        <ComponentPage
            title="ComboBox"
            slug="combo-box"
            group="Collections"
            description="Combobox with input + listbox + popover, baseline-style root attrs, and baseline-level panel/highlight motion."
        >
            <Playground title="Selection + Validation" code_signal=code>
                <div class="docs-stack">
                    <ComboBox
                        id_base="docs-combo-box".to_string()
                        label="Language".to_string()
                        items=items
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
            </Playground>

            <Playground title="Controlled Open State" code_signal=controlled_code>
                <div class="docs-stack">
                    <ComboBox
                        id_base="docs-combo-box-controlled".to_string()
                        label="Controlled language".to_string()
                        items=controlled_items
                        selected_index=controlled_selected
                        set_selected_index=set_controlled_selected
                        is_open=controlled_open
                        on_open_change=on_open_change
                        disabled_indices=vec![4]
                        description="Open state is externally controlled".to_string()
                    />
                    <span class="ui-muted">
                        "open: "
                        {move || controlled_open_raw.get().to_string()}
                    </span>
                    <span class="ui-muted">
                        "selected: "
                        {move || controlled_selected.get().map(|value| value.to_string()).unwrap_or_else(|| "None".to_string())}
                    </span>
                </div>
            </Playground>

            <Playground title="Disabled + Empty" code_signal=states_code>
                <div class="docs-row">
                    <div class="docs-stack">
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

                    <div class="docs-stack">
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
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn autocomplete() -> AnyView {
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
            <Playground title="Selection + Validation" code_signal=code>
                <div class="docs-stack">
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
                <div class="docs-stack">
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
                    <span class="ui-muted">
                        "open: "
                        {move || controlled_open_raw.get().to_string()}
                    </span>
                    <span class="ui-muted">
                        "selected: "
                        {move || controlled_selected.get().map(|value| value.to_string()).unwrap_or_else(|| "None".to_string())}
                    </span>
                </div>
            </Playground>

            <Playground title="Disabled + Empty" code_signal=states_code>
                <div class="docs-row">
                    <div class="docs-stack">
                        <Autocomplete
                            id_base="docs-autocomplete-disabled".to_string()
                            label="Disabled city".to_string()
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
                        <Autocomplete
                            id_base="docs-autocomplete-empty".to_string()
                            label="Empty city list".to_string()
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
                        {move || controlled_open_raw.get().to_string()}
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

    let (disabled_page, set_disabled_page) = signal(1_usize);
    let (empty_page, set_empty_page) = signal(1_usize);

    let code = Signal::derive(move || {
        r#"let (page, set_page) = signal(1_usize);
let on_change = Callback::new(move |next: usize| { /* ... */ });
<Pagination total_pages=12 page=page set_page=set_page siblings=1 boundaries=1 on_change=on_change />"#.to_string()
    });

    let states_code = Signal::derive(move || {
        r#"<Pagination total_pages=1 page=disabled_page set_page=set_disabled_page disabled=true />
<Pagination total_pages=0 page=empty_page set_page=set_empty_page />"#
            .to_string()
    });

    view! {
        <ComponentPage
            title="Pagination"
            slug="pagination"
            group="Collections"
            description="Pagination control with sibling/boundary range logic and baseline-style state attrs."
        >
            <Playground title="Pages + on_change" code_signal=code>
                <div class="docs-stack">
                    <Pagination
                        total_pages=12
                        page=page
                        set_page=set_page
                        siblings=1
                        boundaries=1
                        on_change=on_change
                    />
                    <span class="ui-muted">"page: " {move || page.get().to_string()}</span>
                    <span class="ui-muted">
                        "last change: "
                        {move || last_change.get().map(|value| value.to_string()).unwrap_or_else(|| "None".to_string())}
                    </span>
                </div>
            </Playground>

            <Playground title="Disabled + Empty" code_signal=states_code>
                <div class="docs-row">
                    <div class="docs-stack">
                        <Pagination
                            total_pages=1
                            page=disabled_page
                            set_page=set_disabled_page
                            disabled=true
                        />
                        <span class="ui-muted">
                            "disabled page: "
                            {move || disabled_page.get().to_string()}
                        </span>
                    </div>

                    <div class="docs-stack">
                        <Pagination
                            total_pages=0
                            page=empty_page
                            set_page=set_empty_page
                        />
                        <span class="ui-muted">
                            "empty page signal: "
                            {move || empty_page.get().to_string()}
                        </span>
                    </div>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
pub(super) fn tag_group() -> AnyView {
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
            <Playground title="Removable + State" code_signal=code>
                <div class="docs-stack">
                    <TagGroup
                        tags=removable_tags
                        on_remove=on_remove_removable
                        label="Framework tags".to_string()
                        description="Remove any non-disabled tag".to_string()
                    />
                    <span class="ui-muted">
                        "count: "
                        {move || removable_count.get().to_string()}
                    </span>
                    <span class="ui-muted">
                        "has disabled tags: "
                        {move || removable_has_disabled.get().to_string()}
                    </span>
                </div>
            </Playground>

            <Playground title="Validation + Required" code_signal=states_code>
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
                        {move || validation_invalid.get().to_string()}
                    </span>
                </div>
            </Playground>

            <Playground title="Disabled + Empty" code_signal=disabled_empty_code>
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
