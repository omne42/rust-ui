use super::playground_workbench::{bool_word, push_line_when, rust_string_literal};
use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use std::collections::BTreeSet;
use std::sync::Arc;
use ui::menu::MenuMotion;
use ui::{
    Accordion, AccordionItem, AccordionMotion, AccordionSelectionMode,
    AccordionStreamingProjection, AccordionVariant, AiOutputStatus, AiRenderMode, AiSpace,
    Autocomplete, ComboBox, Disclosure, DropdownMenu, DropdownMenuMotion, List, Menu, MenuItemKind,
    MenuItemSpec, MenuTrigger, Pagination, SegmentedControl, SegmentedControlSize, Select, Snippet,
    Switch, Tabs, TabsKeyboardActivation, Tag, TagGroup,
    accordion::{AccordionPanelLifecycleEvent, AccordionSlotProjection},
    open_set, project_streaming_accordion_markup,
};
use ui_headless::{A11yDirection, PopoverPlacement};

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

#[cfg(target_arch = "wasm32")]
const COMBO_BOX_WORKBENCH_STORAGE_KEY: &str = "docs:combo-box:workbench:selected";

#[cfg(target_arch = "wasm32")]
fn load_combo_box_workbench_selected() -> Option<usize> {
    let storage = web_sys::window().and_then(|window| window.local_storage().ok().flatten())?;
    let raw = storage
        .get_item(COMBO_BOX_WORKBENCH_STORAGE_KEY)
        .ok()
        .flatten()?;
    raw.parse::<usize>().ok()
}

#[cfg(not(target_arch = "wasm32"))]
fn load_combo_box_workbench_selected() -> Option<usize> {
    None
}

#[cfg(target_arch = "wasm32")]
fn save_combo_box_workbench_selected(selected_index: usize) {
    if let Some(storage) =
        web_sys::window().and_then(|window| window.local_storage().ok().flatten())
    {
        let selected_index_attr = selected_index.to_string();
        drop(storage.set_item(COMBO_BOX_WORKBENCH_STORAGE_KEY, &selected_index_attr));
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn save_combo_box_workbench_selected(_selected_index: usize) {}

#[cfg(target_arch = "wasm32")]
fn clear_combo_box_workbench_selected() {
    if let Some(storage) =
        web_sys::window().and_then(|window| window.local_storage().ok().flatten())
    {
        drop(storage.remove_item(COMBO_BOX_WORKBENCH_STORAGE_KEY));
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn clear_combo_box_workbench_selected() {}

#[cfg(target_arch = "wasm32")]
const AUTOCOMPLETE_WORKBENCH_STORAGE_KEY: &str = "docs:autocomplete:workbench:selected";

#[cfg(target_arch = "wasm32")]
fn load_autocomplete_workbench_selected() -> Option<usize> {
    let storage = web_sys::window().and_then(|window| window.local_storage().ok().flatten())?;
    let raw = storage
        .get_item(AUTOCOMPLETE_WORKBENCH_STORAGE_KEY)
        .ok()
        .flatten()?;
    raw.parse::<usize>().ok()
}

#[cfg(not(target_arch = "wasm32"))]
fn load_autocomplete_workbench_selected() -> Option<usize> {
    None
}

#[cfg(target_arch = "wasm32")]
fn save_autocomplete_workbench_selected(selected_index: usize) {
    if let Some(storage) =
        web_sys::window().and_then(|window| window.local_storage().ok().flatten())
    {
        let selected_index_attr = selected_index.to_string();
        drop(storage.set_item(AUTOCOMPLETE_WORKBENCH_STORAGE_KEY, &selected_index_attr));
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn save_autocomplete_workbench_selected(_selected_index: usize) {}

#[cfg(target_arch = "wasm32")]
fn clear_autocomplete_workbench_selected() {
    if let Some(storage) =
        web_sys::window().and_then(|window| window.local_storage().ok().flatten())
    {
        drop(storage.remove_item(AUTOCOMPLETE_WORKBENCH_STORAGE_KEY));
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn clear_autocomplete_workbench_selected() {}

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

fn derive_default_open_set(
    first_default_open: bool,
    second_default_open: bool,
    third_default_open: bool,
) -> BTreeSet<usize> {
    let mut open = open_set([]);
    if first_default_open {
        open.insert(0);
    }
    if second_default_open {
        open.insert(1);
    }
    if third_default_open {
        open.insert(2);
    }
    open
}

fn snapshot_open(indices: &BTreeSet<usize>, visible_items: usize) -> BTreeSet<usize> {
    indices
        .iter()
        .copied()
        .filter(|index| *index < visible_items)
        .collect()
}

fn open_set_literal(indices: &BTreeSet<usize>) -> String {
    let mut values = String::new();
    for (index, value) in indices.iter().enumerate() {
        if index > 0 {
            values.push_str(", ");
        }
        values.push_str(&value.to_string());
    }
    format!("open_set([{values}])")
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

    let workbench_variant_options = vec![
        "Light".to_string(),
        "Shadow".to_string(),
        "Bordered".to_string(),
        "Splitted".to_string(),
    ];
    let workbench_slot_projection_options = vec![
        "KeepAlive".to_string(),
        "Lazy".to_string(),
        "Eager".to_string(),
    ];
    let workbench_lang_options = vec![
        "auto".to_string(),
        "en".to_string(),
        "zh-CN".to_string(),
        "ar".to_string(),
    ];
    let workbench_dir_options = vec!["auto".to_string(), "ltr".to_string(), "rtl".to_string()];

    let (workbench_multiple_mode, set_workbench_multiple_mode) = signal(true);
    let (workbench_disallow_empty_selection, set_workbench_disallow_empty_selection) = signal(true);
    let (workbench_root_disabled, set_workbench_root_disabled) = signal(false);
    let (workbench_disable_second, set_workbench_disable_second) = signal(false);
    let (workbench_disable_third, set_workbench_disable_third) = signal(false);
    let (workbench_variant_index, set_workbench_variant_index) = signal(Some(3_usize));
    let (workbench_slot_projection_index, set_workbench_slot_projection_index) =
        signal(Some(0_usize));
    let (workbench_lang_index, set_workbench_lang_index) = signal(Some(0_usize));
    let (workbench_dir_index, set_workbench_dir_index) = signal(Some(0_usize));
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);
    let (workbench_custom_motion, set_workbench_custom_motion) = signal(false);
    let (workbench_id_base, set_workbench_id_base) = signal("docs-accordion-workbench".to_string());
    let (workbench_persist_state, set_workbench_persist_state) =
        signal(persisted_workbench_open.is_some());
    let (item_api_first_default_open, set_item_api_first_default_open) = signal(true);
    let (item_api_second_default_open, set_item_api_second_default_open) = signal(false);
    let (item_api_third_default_open, set_item_api_third_default_open) = signal(false);
    let item_api_default_open = Signal::derive(move || {
        derive_default_open_set(
            item_api_first_default_open.get(),
            item_api_second_default_open.get(),
            item_api_third_default_open.get(),
        )
    });
    let (item_api_open, set_item_api_open) = signal(item_api_default_open.get_untracked());
    let (item_api_attach_panel_lifecycle, set_item_api_attach_panel_lifecycle) = signal(true);
    let (item_api_lifecycle_events, set_item_api_lifecycle_events) = signal(Vec::<String>::new());
    let (item_api_remount_key, set_item_api_remount_key) = signal(0_u64);
    let on_item_api_0_change = on_item_open_change(set_item_api_open, 0);
    let on_item_api_1_change = on_item_open_change(set_item_api_open, 1);
    let on_item_api_2_change = on_item_open_change(set_item_api_open, 2);

    let on_item_api_panel_lifecycle_0 =
        Callback::new(move |event: AccordionPanelLifecycleEvent| {
            set_item_api_lifecycle_events.update(|events| {
                events.push(format!("key=0 {}", event.as_str()));
                if events.len() > 12 {
                    let stale = events.len() - 12;
                    events.drain(0..stale);
                }
            });
        });
    let on_item_api_panel_lifecycle_1 =
        Callback::new(move |event: AccordionPanelLifecycleEvent| {
            set_item_api_lifecycle_events.update(|events| {
                events.push(format!("key=1 {}", event.as_str()));
                if events.len() > 12 {
                    let stale = events.len() - 12;
                    events.drain(0..stale);
                }
            });
        });
    let on_item_api_panel_lifecycle_2 =
        Callback::new(move |event: AccordionPanelLifecycleEvent| {
            set_item_api_lifecycle_events.update(|events| {
                events.push(format!("key=2 {}", event.as_str()));
                if events.len() > 12 {
                    let stale = events.len() - 12;
                    events.drain(0..stale);
                }
            });
        });

    let workbench_variant =
        Signal::derive(move || match workbench_variant_index.get().unwrap_or(3) {
            0 => AccordionVariant::Light,
            1 => AccordionVariant::Shadow,
            2 => AccordionVariant::Bordered,
            _ => AccordionVariant::Splitted,
        });
    let workbench_slot_projection =
        Signal::derive(
            move || match workbench_slot_projection_index.get().unwrap_or(0) {
                1 => AccordionSlotProjection::Lazy,
                2 => AccordionSlotProjection::Eager,
                _ => AccordionSlotProjection::KeepAlive,
            },
        );
    let workbench_lang = Signal::derive(move || match workbench_lang_index.get().unwrap_or(0) {
        1 => Some("en".to_string()),
        2 => Some("zh-CN".to_string()),
        3 => Some("ar".to_string()),
        _ => None,
    });
    let workbench_dir = Signal::derive(move || match workbench_dir_index.get().unwrap_or(0) {
        1 => Some(ui::color::area::A11yDirection::Ltr),
        2 => Some(ui::color::area::A11yDirection::Rtl),
        _ => None,
    });
    let workbench_motion = Signal::derive(move || {
        if workbench_custom_motion.get() {
            AccordionMotion {
                indicator_open_rotation_deg: 108.0,
                panel_offset_y_px: 24.0,
                ..AccordionMotion::default()
            }
        } else {
            AccordionMotion::default()
        }
    });

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

    Effect::new(move |_| {
        let multiple_mode = workbench_multiple_mode.get();
        let disallow_empty = workbench_disallow_empty_selection.get();
        let disable_second = workbench_disable_second.get();
        let disable_third = workbench_disable_third.get();

        set_workbench_open.update(|open| {
            open.retain(|index| *index < 3);
            if disable_second {
                open.remove(&1);
            }
            if disable_third {
                open.remove(&2);
            }

            if !multiple_mode && open.len() > 1 {
                let keep = open.iter().next().copied();
                open.clear();
                if let Some(keep) = keep {
                    open.insert(keep);
                }
            }

            if disallow_empty && open.is_empty() {
                if !disable_second {
                    open.insert(1);
                } else if !disable_third {
                    open.insert(2);
                } else {
                    open.insert(0);
                }
            }
        });
    });

    let hello_code = Signal::derive(move || {
        r#"<Accordion variant=AccordionVariant::Light>
  <AccordionItem label="First">"Panel 1"</AccordionItem>
  <AccordionItem label="Second">"Panel 2"</AccordionItem>
</Accordion>"#
            .to_string()
    });

    let code = Signal::derive(move || {
        let open_literal = open_set_literal(&open_multi.get());
        format!(
            "let (open, set_open) = signal({open_literal});\n\
let item_0_open = Signal::derive(move || open.get().contains(&0));\n\
let item_1_open = Signal::derive(move || open.get().contains(&1));\n\
let item_2_open = Signal::derive(move || open.get().contains(&2));\n\
\n\
<Accordion\n\
  id_base=\"accordion\".to_string()\n\
  selection_mode=AccordionSelectionMode::Multiple\n\
  variant=AccordionVariant::Shadow\n\
>\n\
  <AccordionItem key=0 label=\"First\" open=item_0_open on_open_change=on_item_open_change(set_open, 0)><div>\"Panel 1\"</div></AccordionItem>\n\
  <AccordionItem key=1 label=\"Second\" open=item_1_open on_open_change=on_item_open_change(set_open, 1)><div>\"Panel 2\"</div></AccordionItem>\n\
  <AccordionItem key=2 label=\"Third\" open=item_2_open on_open_change=on_item_open_change(set_open, 2)><div>\"Panel 3\"</div></AccordionItem>\n\
</Accordion>"
        )
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

    let controlled_preview_actual_config = Signal::derive(move || {
        let open = open_multi.get().iter().copied().collect::<Vec<_>>();
        format!(
            "AccordionControlledPreviewConfig {{\n  id_base: Some(\"docs-accordion\"),\n  selection_mode: \"multiple\",\n  variant: \"shadow\",\n  open: {open:?},\n  on_open_change: \"per-item callback updates open set\",\n  on_panel_lifecycle: \"events=0 (not attached in controlled preview)\",\n}}"
        )
    });

    let workbench_code = Signal::derive(move || {
        let multiple_mode = workbench_multiple_mode.get();
        let disallow_empty_selection = workbench_disallow_empty_selection.get();
        let root_disabled = workbench_root_disabled.get();
        let disable_security = workbench_disable_second.get();
        let disable_third = workbench_disable_third.get();
        let variant = workbench_variant.get();
        let slot_projection = workbench_slot_projection.get();
        let lang = workbench_lang.get();
        let dir = workbench_dir.get();
        let custom_class = workbench_custom_class.get();
        let custom_motion = workbench_custom_motion.get();
        let id_base = workbench_id_base.get();
        let persist_state = workbench_persist_state.get();
        let open_literal = open_set_literal(&workbench_open.get());
        let selection_mode = if multiple_mode {
            "AccordionSelectionMode::Multiple"
        } else {
            "AccordionSelectionMode::Single"
        };
        let variant_literal = match variant {
            AccordionVariant::Light => "AccordionVariant::Light",
            AccordionVariant::Shadow => "AccordionVariant::Shadow",
            AccordionVariant::Bordered => "AccordionVariant::Bordered",
            AccordionVariant::Splitted => "AccordionVariant::Splitted",
        };
        let slot_projection_literal = match slot_projection {
            AccordionSlotProjection::Lazy => "AccordionSlotProjection::Lazy",
            AccordionSlotProjection::KeepAlive => "AccordionSlotProjection::KeepAlive",
            AccordionSlotProjection::Eager => "AccordionSlotProjection::Eager",
        };

        let mut lines = vec![
            format!("// persist_state={}", bool_word(persist_state)),
            if persist_state {
                "let saved = load_workbench_open();".to_string()
            } else {
                "// persist disabled; use current runtime snapshot".to_string()
            },
            if persist_state {
                format!("let (open, set_open) = signal(saved.unwrap_or_else(|| {open_literal}));")
            } else {
                format!("let (open, set_open) = signal({open_literal});")
            },
            "let item_0_open = Signal::derive(move || open.get().contains(&0));".to_string(),
            "let item_1_open = Signal::derive(move || open.get().contains(&1));".to_string(),
            "let item_2_open = Signal::derive(move || open.get().contains(&2));".to_string(),
            "<Accordion".to_string(),
            format!("  id_base={}.to_string()", rust_string_literal(&id_base)),
            format!("  selection_mode={selection_mode}"),
            format!("  variant={variant_literal}"),
            format!("  slot_projection={slot_projection_literal}"),
        ];
        push_line_when(
            &mut lines,
            disallow_empty_selection,
            "  disallow_empty_selection=true".to_string(),
        );
        push_line_when(&mut lines, root_disabled, "  is_disabled=true".to_string());
        if let Some(lang) = lang {
            lines.push(format!("  lang={}.to_string()", rust_string_literal(&lang)));
        }
        if let Some(dir) = dir {
            let dir_literal = match dir {
                ui::color::area::A11yDirection::Ltr => "ui::color::area::A11yDirection::Ltr",
                ui::color::area::A11yDirection::Rtl => "ui::color::area::A11yDirection::Rtl",
            };
            lines.push(format!("  dir={dir_literal}"));
        }
        push_line_when(
            &mut lines,
            custom_class,
            "  class_name=\"docs-accordion-workbench--custom\".to_string()".to_string(),
        );
        if custom_motion {
            lines.push(
                "  motion=AccordionMotion { indicator_open_rotation_deg: 108.0, panel_offset_y_px: 24.0, ..AccordionMotion::default() }"
                    .to_string(),
            );
        }
        lines.push(">".to_string());
        lines.push(
            "  <AccordionItem key=0 label=\"Profile\" open=item_0_open on_open_change=on_item_open_change(set_open, 0)><div>\"Profile panel\"</div></AccordionItem>".to_string(),
        );
        lines.push(format!(
            "  <AccordionItem key=1 label=\"Security\" open=item_1_open on_open_change=on_item_open_change(set_open, 1){}><div>\"Security panel\"</div></AccordionItem>",
            if disable_security { " is_disabled=true" } else { "" }
        ));
        lines.push(format!(
            "  <AccordionItem key=2 label=\"Notifications\" open=item_2_open on_open_change=on_item_open_change(set_open, 2){}><div>\"Notifications panel\"</div></AccordionItem>",
            if disable_third { " is_disabled=true" } else { "" }
        ));
        lines.push("</Accordion>".to_string());
        lines.join("\n")
    });

    let workbench_actual_config = Signal::derive(move || {
        let open = workbench_open.get().iter().copied().collect::<Vec<_>>();
        let selection_mode = if workbench_multiple_mode.get() {
            "multiple"
        } else {
            "single"
        };
        let variant = workbench_variant.get().as_str();
        let slot_projection = workbench_slot_projection.get().as_str();
        let dir = match workbench_dir.get() {
            Some(ui::color::area::A11yDirection::Ltr) => "ltr",
            Some(ui::color::area::A11yDirection::Rtl) => "rtl",
            _ => "auto",
        };
        let lang = workbench_lang.get();
        let class_name = if workbench_custom_class.get() {
            "docs-accordion-workbench--custom"
        } else {
            ""
        };
        let id_base = workbench_id_base.get();
        let second_open = open.contains(&1);
        let third_open = open.contains(&2);

        format!(
            "AccordionWorkbenchConfig {{\n  id_base: {id_base:?},\n  selection_mode: \"{selection_mode}\",\n  variant: \"{variant}\",\n  slot_projection: \"{slot_projection}\",\n  disallow_empty_selection: {},\n  is_disabled: {},\n  lang: {lang:?},\n  dir: \"{dir}\",\n  class_name: \"{class_name}\",\n  motion: \"{}\",\n  custom_motion: {},\n  persist_state: {},\n  open: {open:?},\n  items: [\n    {{ key: 0, label: \"Profile\", is_disabled: false, open: {}, default_open: false, on_open_change: true, on_panel_lifecycle: false }},\n    {{ key: 1, label: \"Security\", is_disabled: {}, open: {}, default_open: false, on_open_change: true, on_panel_lifecycle: false }},\n    {{ key: 2, label: \"Notifications\", is_disabled: {}, open: {}, default_open: false, on_open_change: true, on_panel_lifecycle: false }},\n  ],\n}}",
            if workbench_custom_motion.get() {
                "custom"
            } else {
                "default"
            },
            bool_word(workbench_disallow_empty_selection.get()),
            bool_word(workbench_root_disabled.get()),
            bool_word(workbench_custom_motion.get()),
            bool_word(workbench_persist_state.get()),
            bool_word(open.contains(&0)),
            bool_word(workbench_disable_second.get()),
            bool_word(second_open),
            bool_word(workbench_disable_third.get()),
            bool_word(third_open),
        )
    });

    let item_api_code = Signal::derive(move || {
        let default_open = item_api_default_open.get();
        let attach_panel_lifecycle = item_api_attach_panel_lifecycle.get();
        let first_default_open = default_open.contains(&0);
        let second_default_open = default_open.contains(&1);
        let third_default_open = default_open.contains(&2);

        let mut lines = vec![
            "let (observed_open, set_observed_open) = signal(open_set([]));".to_string(),
            "let on_item_0_change = on_item_open_change(set_observed_open, 0);".to_string(),
            "let on_item_1_change = on_item_open_change(set_observed_open, 1);".to_string(),
            "let on_item_2_change = on_item_open_change(set_observed_open, 2);".to_string(),
            "<Accordion".to_string(),
            "  id_base=\"docs-accordion-item-api\".to_string()".to_string(),
            "  selection_mode=AccordionSelectionMode::Multiple".to_string(),
            "  variant=AccordionVariant::Splitted".to_string(),
            ">".to_string(),
        ];

        lines.push(format!(
            "  <AccordionItem key=0 label=\"Profile\" default_open={first_default_open} on_open_change=on_item_0_change{}><div>\"Profile panel content\"</div></AccordionItem>",
            if attach_panel_lifecycle {
                " on_panel_lifecycle=on_panel_lifecycle_0"
            } else {
                ""
            }
        ));
        lines.push(format!(
            "  <AccordionItem key=1 label=\"Security\" default_open={second_default_open} on_open_change=on_item_1_change{}><div>\"Security panel content\"</div></AccordionItem>",
            if attach_panel_lifecycle {
                " on_panel_lifecycle=on_panel_lifecycle_1"
            } else {
                ""
            }
        ));
        lines.push(format!(
            "  <AccordionItem key=2 label=\"Notifications\" default_open={third_default_open} on_open_change=on_item_2_change{}><div>\"Notifications panel content\"</div></AccordionItem>",
            if attach_panel_lifecycle {
                " on_panel_lifecycle=on_panel_lifecycle_2"
            } else {
                ""
            }
        ));
        lines.push("</Accordion>".to_string());
        lines.join("\n")
    });

    let item_api_actual_config = Signal::derive(move || {
        let default_open = item_api_default_open
            .get()
            .iter()
            .copied()
            .collect::<Vec<_>>();
        let observed_open = item_api_open.get().iter().copied().collect::<Vec<_>>();
        let attach_panel_lifecycle = item_api_attach_panel_lifecycle.get();
        let lifecycle_events = item_api_lifecycle_events.get();
        let lifecycle_count = lifecycle_events.len();
        let remount_key = item_api_remount_key.get();

        format!(
            "AccordionItemApiConfig {{\n  remount_key: {remount_key},\n  selection_mode: \"multiple\",\n  default_open: {default_open:?},\n  observed_open: {observed_open:?},\n  attach_on_panel_lifecycle: {},\n  lifecycle_event_count: {lifecycle_count},\n  items: [\n    {{ key: 0, label: \"Profile\", is_disabled: false, open: \"uncontrolled\", default_open: {}, on_open_change: true, on_panel_lifecycle: {} }},\n    {{ key: 1, label: \"Security\", is_disabled: false, open: \"uncontrolled\", default_open: {}, on_open_change: true, on_panel_lifecycle: {} }},\n    {{ key: 2, label: \"Notifications\", is_disabled: false, open: \"uncontrolled\", default_open: {}, on_open_change: true, on_panel_lifecycle: {} }},\n  ],\n  lifecycle_events: {lifecycle_events:?},\n}}",
            bool_word(attach_panel_lifecycle),
            bool_word(default_open.contains(&0)),
            bool_word(attach_panel_lifecycle),
            bool_word(default_open.contains(&1)),
            bool_word(attach_panel_lifecycle),
            bool_word(default_open.contains(&2)),
            bool_word(attach_panel_lifecycle),
        )
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

            <Playground
                title="Multiple + Controlled"
                code_signal=code
                test_config_signal=controlled_preview_actual_config
                controls=move || view! {
                    <div class="docs-row" data-slot="accordion-controlled-preview-controls">
                        <button
                            type="button"
                            on:click=move |_| set_open_multi.set(open_set([0, 1, 2]))
                        >
                            "Open all"
                        </button>
                        <button
                            type="button"
                            on:click=move |_| set_open_multi.set(open_set([0]))
                        >
                            "Only first"
                        </button>
                        <button
                            type="button"
                            on:click=move |_| set_open_multi.set(open_set([]))
                        >
                            "Close all"
                        </button>
                    </div>
                }
            >
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

            <Playground title="State Matrix (Single + Disabled)" code_signal=states_code>
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
                description="Accordion API workbench: adjust root/item props and inspect real-time config/code feedback."
                code_signal=workbench_code
                test_config_signal=workbench_actual_config
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
                                prop:checked=move || workbench_disallow_empty_selection.get()
                                on:change=move |ev| set_workbench_disallow_empty_selection.set(event_target_checked(&ev))
                            />
                            " Disallow empty selection"
                        </label>
                        <div class="docs-search__label">"Variant"</div>
                        <SegmentedControl
                            id_base="docs-accordion-workbench-variant".to_string()
                            options=workbench_variant_options.clone()
                            selected_index=workbench_variant_index
                            set_selected_index=set_workbench_variant_index
                            size=SegmentedControlSize::Sm
                            aria_label="Accordion workbench variant".to_string()
                        />
                        <div class="docs-search__label">"Slot projection"</div>
                        <SegmentedControl
                            id_base="docs-accordion-workbench-slot-projection".to_string()
                            options=workbench_slot_projection_options.clone()
                            selected_index=workbench_slot_projection_index
                            set_selected_index=set_workbench_slot_projection_index
                            size=SegmentedControlSize::Sm
                            aria_label="Accordion workbench slot projection".to_string()
                        />
                        <div class="docs-search__label">"Lang"</div>
                        <SegmentedControl
                            id_base="docs-accordion-workbench-lang".to_string()
                            options=workbench_lang_options.clone()
                            selected_index=workbench_lang_index
                            set_selected_index=set_workbench_lang_index
                            size=SegmentedControlSize::Sm
                            aria_label="Accordion workbench language".to_string()
                        />
                        <div class="docs-search__label">"Direction"</div>
                        <SegmentedControl
                            id_base="docs-accordion-workbench-dir".to_string()
                            options=workbench_dir_options.clone()
                            selected_index=workbench_dir_index
                            set_selected_index=set_workbench_dir_index
                            size=SegmentedControlSize::Sm
                            aria_label="Accordion workbench direction".to_string()
                        />
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
                                prop:checked=move || workbench_disable_second.get()
                                on:change=move |ev| set_workbench_disable_second.set(event_target_checked(&ev))
                            />
                            " Disable item #1"
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_disable_third.get()
                                on:change=move |ev| set_workbench_disable_third.set(event_target_checked(&ev))
                            />
                            " Disable item #2"
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_custom_class.get()
                                on:change=move |ev| set_workbench_custom_class.set(event_target_checked(&ev))
                            />
                            " Custom class"
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_custom_motion.get()
                                on:change=move |ev| set_workbench_custom_motion.set(event_target_checked(&ev))
                            />
                            " Custom motion"
                        </label>
                        <label class="docs-search__label">
                            "id_base "
                            <input
                                type="text"
                                prop:value=move || workbench_id_base.get()
                                on:input=move |ev| set_workbench_id_base.set(event_target_value(&ev))
                            />
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_persist_state.get()
                                on:change=move |ev| set_workbench_persist_state.set(event_target_checked(&ev))
                            />
                            " Persist open state (optional)"
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
                    <span class="ui-muted" data-slot="accordion-workbench-summary">
                        "mode: "
                        {move || if workbench_multiple_mode.get() { "multiple" } else { "single" }}
                        " | variant: "
                        {move || workbench_variant.get().as_str()}
                        " | slot: "
                        {move || workbench_slot_projection.get().as_str()}
                        " | root_disabled: "
                        {move || workbench_root_disabled.get()}
                        " | disable_second: "
                        {move || workbench_disable_second.get()}
                        " | disable_third: "
                        {move || workbench_disable_third.get()}
                    </span>
                    <pre class="docs-code" data-slot="accordion-workbench-config-preview">
                        {move || workbench_actual_config.get()}
                    </pre>
                    {move || {
                        let selection_mode = if workbench_multiple_mode.get() {
                            AccordionSelectionMode::Multiple
                        } else {
                            AccordionSelectionMode::Single
                        };
                        let disallow_empty_selection = workbench_disallow_empty_selection.get();
                        let disable_security = workbench_disable_second.get();
                        let disable_notifications = workbench_disable_third.get();
                        let variant = workbench_variant.get();
                        let slot_projection = workbench_slot_projection.get();
                        let is_disabled = workbench_root_disabled.get();
                        let lang = workbench_lang.get().unwrap_or_default();
                        let dir = workbench_dir.get();
                        let motion = workbench_motion.get();
                        let class_name = if workbench_custom_class.get() {
                            "docs-accordion-workbench--custom".to_string()
                        } else {
                            String::new()
                        };
                        let id_base = workbench_id_base.get();
                        view! {
                            <div
                                class="docs-card"
                                data-slot="accordion-workbench-canvas"
                            >
                                <AiSpace mode=snapshot_mode output_status=verified_output>
                                    {match dir {
                                        Some(dir) => view! {
                                            <Accordion
                                                id_base=id_base.clone()
                                                selection_mode=selection_mode
                                                variant=variant
                                                disallow_empty_selection=disallow_empty_selection
                                                is_disabled=is_disabled
                                                lang=lang.clone()
                                                dir=dir
                                                slot_projection=slot_projection
                                                motion=motion
                                                class_name=class_name.clone()
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
                                                    is_disabled=disable_notifications
                                                >
                                                    <div>"Notifications panel content"</div>
                                                </AccordionItem>
                                            </Accordion>
                                        }
                                            .into_any(),
                                        None => view! {
                                            <Accordion
                                                id_base=id_base
                                                selection_mode=selection_mode
                                                variant=variant
                                                disallow_empty_selection=disallow_empty_selection
                                                is_disabled=is_disabled
                                                lang=lang
                                                slot_projection=slot_projection
                                                motion=motion
                                                class_name=class_name
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
                                                    is_disabled=disable_notifications
                                                >
                                                    <div>"Notifications panel content"</div>
                                                </AccordionItem>
                                            </Accordion>
                                        }
                                            .into_any(),
                                    }}
                                </AiSpace>
                            </div>
                        }
                    }}
                </div>
            </Playground>

            <Playground
                title="Item API Workbench (default_open + lifecycle)"
                description="Focused API coverage for AccordionItem defaults and lifecycle callbacks. Update config then remount to replay default_open initialization."
                code_signal=item_api_code
                test_config_signal=item_api_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="accordion-item-api-controls">
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || item_api_first_default_open.get()
                                on:change=move |ev| set_item_api_first_default_open.set(event_target_checked(&ev))
                            />
                            " Item #0 default_open"
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || item_api_second_default_open.get()
                                on:change=move |ev| set_item_api_second_default_open.set(event_target_checked(&ev))
                            />
                            " Item #1 default_open"
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || item_api_third_default_open.get()
                                on:change=move |ev| set_item_api_third_default_open.set(event_target_checked(&ev))
                            />
                            " Item #2 default_open"
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || item_api_attach_panel_lifecycle.get()
                                on:change=move |ev| set_item_api_attach_panel_lifecycle.set(event_target_checked(&ev))
                            />
                            " Attach on_panel_lifecycle"
                        </label>
                        <button
                            type="button"
                            data-slot="accordion-item-api-remount-button"
                            on:click=move |_| {
                                set_item_api_open.set(item_api_default_open.get_untracked());
                                set_item_api_lifecycle_events.set(Vec::new());
                                set_item_api_remount_key.update(|key| *key = key.saturating_add(1));
                            }
                        >
                            "Apply default_open + remount"
                        </button>
                    </div>
                }
            >
                <div class="docs-stack" data-slot="accordion-item-api-workbench">
                    <span class="ui-muted">
                        "remount_key: "
                        {move || item_api_remount_key.get()}
                        " | default_open: "
                        {move || {
                            let open = item_api_default_open.get().iter().copied().collect::<Vec<_>>();
                            format!("{open:?}")
                        }}
                        " | observed_open: "
                        {move || {
                            let open = item_api_open.get().iter().copied().collect::<Vec<_>>();
                            format!("{open:?}")
                        }}
                    </span>
                    <pre class="docs-code" data-slot="accordion-item-api-config-preview">
                        {move || item_api_actual_config.get()}
                    </pre>
                    <div class="docs-card" data-slot="accordion-item-api-canvas">
                        <For
                            each=move || vec![item_api_remount_key.get()]
                            key=|mount_key| *mount_key
                            children=move |_| {
                                let default_open = item_api_default_open.get();
                                let attach_panel_lifecycle = item_api_attach_panel_lifecycle.get();
                                let first_default_open = default_open.contains(&0);
                                let second_default_open = default_open.contains(&1);
                                let third_default_open = default_open.contains(&2);
                                view! {
                                    <Accordion
                                        id_base="docs-accordion-item-api".to_string()
                                        selection_mode=AccordionSelectionMode::Multiple
                                        variant=AccordionVariant::Splitted
                                    >
                                        {if attach_panel_lifecycle {
                                            view! {
                                                <AccordionItem
                                                    key=0
                                                    label="Profile"
                                                    default_open=first_default_open
                                                    on_open_change=on_item_api_0_change
                                                    on_panel_lifecycle=on_item_api_panel_lifecycle_0
                                                >
                                                    <div>"Profile panel content"</div>
                                                </AccordionItem>
                                            }
                                                .into_any()
                                        } else {
                                            view! {
                                                <AccordionItem
                                                    key=0
                                                    label="Profile"
                                                    default_open=first_default_open
                                                    on_open_change=on_item_api_0_change
                                                >
                                                    <div>"Profile panel content"</div>
                                                </AccordionItem>
                                            }
                                                .into_any()
                                        }}
                                        {if attach_panel_lifecycle {
                                            view! {
                                                <AccordionItem
                                                    key=1
                                                    label="Security"
                                                    default_open=second_default_open
                                                    on_open_change=on_item_api_1_change
                                                    on_panel_lifecycle=on_item_api_panel_lifecycle_1
                                                >
                                                    <div>"Security panel content"</div>
                                                </AccordionItem>
                                            }
                                                .into_any()
                                        } else {
                                            view! {
                                                <AccordionItem
                                                    key=1
                                                    label="Security"
                                                    default_open=second_default_open
                                                    on_open_change=on_item_api_1_change
                                                >
                                                    <div>"Security panel content"</div>
                                                </AccordionItem>
                                            }
                                                .into_any()
                                        }}
                                        {if attach_panel_lifecycle {
                                            view! {
                                                <AccordionItem
                                                    key=2
                                                    label="Notifications"
                                                    default_open=third_default_open
                                                    on_open_change=on_item_api_2_change
                                                    on_panel_lifecycle=on_item_api_panel_lifecycle_2
                                                >
                                                    <div>"Notifications panel content"</div>
                                                </AccordionItem>
                                            }
                                                .into_any()
                                        } else {
                                            view! {
                                                <AccordionItem
                                                    key=2
                                                    label="Notifications"
                                                    default_open=third_default_open
                                                    on_open_change=on_item_api_2_change
                                                >
                                                    <div>"Notifications panel content"</div>
                                                </AccordionItem>
                                            }
                                                .into_any()
                                        }}
                                    </Accordion>
                                }
                            }
                        />
                    </div>
                    <div class="docs-stack docs-stack--tight" data-slot="accordion-item-api-events">
                        <span class="docs-search__label">"on_panel_lifecycle feed"</span>
                        <Show
                            when=move || !item_api_lifecycle_events.get().is_empty()
                            fallback=move || view! {
                                <span class="ui-muted">"no lifecycle events yet"</span>
                            }
                        >
                            <ul class="docs-stack docs-stack--tight">
                                {move || {
                                    item_api_lifecycle_events
                                        .get()
                                        .into_iter()
                                        .map(|event| view! { <li>{event}</li> })
                                        .collect_view()
                                }}
                            </ul>
                        </Show>
                    </div>
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
            "/* components/disclosure/src/styles.rs */\n{}",
            ui::disclosure::styles::CSS
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
                test_source_path="components/disclosure/src/styles.rs".to_string()
                test_config_signal=disclosure_actual_config
                description="Disclosure workbench: 对比展示 + config + code + scoped CSS test."
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight">
                        <ui::Switch checked=workbench_controlled set_checked=set_workbench_controlled>
                            "Controlled mode"
                        </ui::Switch>
                        <ui::Switch checked=workbench_disabled set_checked=set_workbench_disabled>
                            "Disabled"
                        </ui::Switch>
                        <ui::Switch checked=workbench_custom_motion set_checked=set_workbench_custom_motion>
                            "Custom motion"
                        </ui::Switch>
                        <ui::Switch checked=workbench_open set_checked=set_workbench_open>
                            "Open state (for controlled/default_open)"
                        </ui::Switch>
                    </div>
                }
            >
                {move || {
                    let controlled = workbench_controlled.get();
                    let disabled = workbench_disabled.get();
                    let custom_motion = workbench_custom_motion.get();
                    let motion = if custom_motion {
                        ui::DisclosureMotion {
                            open_rotation_deg: 135.0,
                            panel_offset_y_px: 10.0,
                            ..ui::DisclosureMotion::default()
                        }
                    } else {
                        ui::DisclosureMotion::default()
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
        let selected_index = tabs_workbench_selected.get();
        let keyboard_activation = if tabs_workbench_manual_mode.get() {
            "TabsKeyboardActivation::Manual"
        } else {
            "TabsKeyboardActivation::Automatic"
        };
        let disabled_indices = if tabs_workbench_disable_settings.get() {
            "vec![2]"
        } else {
            "Vec::<usize>::new()"
        };
        let persist_selected_index = bool_word(tabs_workbench_persist_state.get());

        format!(
            "let saved = load_tabs_workbench_selected();\n\
let (selected, set_selected) = signal(saved.unwrap_or({selected_index}_usize));\n\
let on_change = Callback::new(move |next: usize| set_selected.set(next));\n\
// Workbench keeps interaction context and can optionally persist selected index.\n\
<Tabs\n\
  labels=vec![\"Overview\", \"Details\", \"Settings\"]\n\
  id_base=\"tabs-workbench\".to_string()\n\
  selected_index=selected\n\
  on_selection_change=on_change\n\
  keyboard_activation={keyboard_activation}\n\
  disabled_indices={disabled_indices}\n\
>\n\
  <div>\"Overview panel\"</div>\n\
  <div>\"Details panel\"</div>\n\
  <div>\"Settings panel\"</div>\n\
</Tabs>\n\
// persist_selected_index={persist_selected_index}"
        )
    });
    let workbench_actual_config = Signal::derive(move || {
        let keyboard_activation = if tabs_workbench_manual_mode.get() {
            "manual"
        } else {
            "automatic"
        };
        let disabled_indices = if tabs_workbench_disable_settings.get() {
            vec![2_usize]
        } else {
            Vec::new()
        };
        format!(
            "TabsWorkbenchConfig {{\n  id_base: \"docs-tabs-workbench\",\n  selected_index: {},\n  keyboard_activation: \"{keyboard_activation}\",\n  disabled_indices: {:?},\n  persist_selected_index: {},\n}}",
            tabs_workbench_selected.get(),
            disabled_indices,
            tabs_workbench_persist_state.get(),
        )
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
                test_config_signal=workbench_actual_config
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
    let hello_items: Arc<[String]> = vec!["Overview".to_string(), "Billing".to_string()].into();
    let showcase_items: Arc<[String]> = vec![
        "Overview".to_string(),
        "Billing".to_string(),
        "Integrations".to_string(),
        "Audit Logs".to_string(),
    ]
    .into();
    let showcase_items_for_showcase = showcase_items.clone();
    let showcase_items_for_matrix = showcase_items.clone();
    let showcase_items_for_matrix_after = showcase_items.clone();
    let showcase_items_for_stream_snapshot = showcase_items.clone();
    let showcase_items_for_stream_streaming = showcase_items.clone();
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
    let (workbench_rtl, set_workbench_rtl) = signal(false);
    let (workbench_custom_motion, set_workbench_custom_motion) = signal(false);
    let (workbench_action_count, set_workbench_action_count) = signal(0_u32);
    let (workbench_last_action, set_workbench_last_action) = signal(None::<usize>);
    let on_workbench_action = Callback::new(move |index: usize| {
        set_workbench_last_action.set(Some(index));
        set_workbench_action_count.update(|count| *count += 1);
    });
    let list_code_imports =
        "use leptos::prelude::*;\nuse std::sync::Arc;\nuse ui::{AiOutputStatus, AiRenderMode, AiSpace, List};".to_string();
    let list_snapshot_mode = Signal::derive(|| AiRenderMode::Snapshot);
    let list_streaming_mode = Signal::derive(|| AiRenderMode::Streaming);
    let list_draft_output = Signal::derive(|| AiOutputStatus::Draft);
    let list_verified_output = Signal::derive(|| AiOutputStatus::Verified);
    let (state_matrix_controlled_selected, set_state_matrix_controlled_selected) =
        signal(Some(1_usize));
    let (snapshot_selected, set_snapshot_selected) = signal(Some(0_usize));
    let (streaming_selected, set_streaming_selected) = signal(Some(2_usize));

    let hello_code = Signal::derive(move || {
        r#"let items: Arc<[String]> = vec!["Overview".to_string(), "Billing".to_string()].into();
<List id_base="list-hello".to_string() items=items aria_label="Settings navigation".to_string() />"#
            .to_string()
    });

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
  selected_index=selected_a.into()
  on_selected_index_change=Callback::new(move |next| set_selected_a.set(next))
  aria_label="Default list".to_string()
  disabled_indices=vec![2]
/>
<List
  id_base="list-unsynced".to_string()
  items=items
  selected_index=selected_b.into()
  on_selected_index_change=Callback::new(move |next| set_selected_b.set(next))
  aria_label="Unsynced list".to_string()
  is_active_index_synced_to_selected=false
/>
<List
  id_base="list-disabled".to_string()
  items=vec!["Overview".to_string(), "Billing".to_string(), "Integrations".to_string()].into()
  selected_index=selected_c.into()
  on_selected_index_change=Callback::new(move |next| set_selected_c.set(next))
  aria_label="Disabled list".to_string()
  is_disabled=true
/>
<List
  id_base="list-empty".to_string()
  items=Vec::<String>::new().into()
  selected_index=selected_empty.into()
  on_selected_index_change=Callback::new(move |next| set_selected_empty.set(next))
  aria_label="Empty list".to_string()
/>"#
        .to_string()
    });

    let state_matrix_code = Signal::derive(move || {
        r#"let items: Arc<[String]> = vec![
  "Overview".to_string(),
  "Billing".to_string(),
  "Integrations".to_string(),
  "Audit Logs".to_string(),
].into();

let (controlled_selected, set_controlled_selected) = signal(Some(1_usize));

<List
  id_base="list-matrix-uncontrolled".to_string()
  items=items.clone()
  aria_label="Matrix uncontrolled list".to_string()
/>
<List
  id_base="list-matrix-controlled".to_string()
  items=items.clone()
  selected_index=controlled_selected.into()
  on_selected_index_change=Callback::new(move |next| set_controlled_selected.set(next))
  aria_label="Matrix controlled list".to_string()
/>
<List
  id_base="list-matrix-disabled".to_string()
  items=items
  selected_index=controlled_selected.into()
  on_selected_index_change=Callback::new(move |next| set_controlled_selected.set(next))
  aria_label="Matrix disabled list".to_string()
  is_disabled=true
/>"#
        .to_string()
    });

    let output_mode_code = Signal::derive(move || {
        r#"let items: Arc<[String]> = vec![
  "Overview".to_string(),
  "Billing".to_string(),
  "Integrations".to_string(),
  "Audit Logs".to_string(),
].into();

// List is Streaming Optional; fallback remains snapshot.
<div data-ui-streaming="optional" data-ui-fallback="snapshot">
  <AiSpace mode=Signal::derive(|| AiRenderMode::Snapshot) output_status=Signal::derive(|| AiOutputStatus::Verified)>
    <List id_base="docs-list-snapshot".to_string() items=items.clone() selected_index=snapshot_selected.into() on_selected_index_change=on_snapshot_change aria_label="Snapshot list".to_string() />
  </AiSpace>
  <AiSpace mode=Signal::derive(|| AiRenderMode::Streaming) output_status=Signal::derive(|| AiOutputStatus::Draft)>
    <List id_base="docs-list-streaming".to_string() items=items selected_index=streaming_selected.into() on_selected_index_change=on_streaming_change aria_label="Streaming list".to_string() />
  </AiSpace>
</div>"#
            .to_string()
    });

    let workbench_code = Signal::derive(move || {
        let sync_active = workbench_sync_active.get();
        let root_disabled = workbench_root_disabled.get();
        let disable_last = workbench_disable_last.get();
        let custom_class = workbench_custom_class.get();
        let rtl = workbench_rtl.get();
        let custom_motion = workbench_custom_motion.get();

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
            "  selected_index=selected.into()".to_string(),
            "  default_selected_index=Some(1)".to_string(),
            "  on_selected_index_change=Callback::new(move |next| set_selected.set(next))"
                .to_string(),
            "  id=\"docs-list-workbench-root\".into()".to_string(),
            "  aria_label=\"List workbench\".into()".to_string(),
            "  aria_labelledby=\"docs-list-workbench-heading\".into()".to_string(),
            "  on_action=Callback::new(move |_index| {})".to_string(),
            "  default_active_index=1".to_string(),
        ];

        if !sync_active {
            lines.push("  is_active_index_synced_to_selected=false".to_string());
        }
        if root_disabled {
            lines.push("  is_disabled=true".to_string());
        }
        if disable_last {
            lines.push("  disabled_indices=vec![4]".to_string());
        }
        if custom_class {
            lines.push("  class_name=\"docs-list-workbench--custom\".into()".to_string());
        }
        if rtl {
            lines.push("  lang=\"ar\".into()".to_string());
            lines.push("  dir=A11yDirection::Rtl".to_string());
        } else {
            lines.push("  lang=\"en-US\".into()".to_string());
            lines.push("  dir=A11yDirection::Ltr".to_string());
        }
        if custom_motion {
            lines.push(
                "  motion=ui::list::ListMotion { spring: ui::list::ListMotion::default().spring, highlight_scale: 1.03 }"
                    .to_string(),
            );
        } else {
            lines.push("  motion=ui::list::ListMotion::default()".to_string());
        }

        lines.push("/>".to_string());
        lines.join("\n")
    });

    let workbench_test_css = Signal::derive(move || {
        format!(
            "/* crates/ui/src/list/styles.rs */\n{}\n\n/* ListItem contract */\n{}\n\n/* ListSection contract */\n{}",
            ui::list::styles::CSS,
            ui::list::styles::ITEM_CSS,
            ui::list::styles::SECTION_CSS,
        )
    });

    let workbench_items_for_config = workbench_items.clone();
    let workbench_actual_config = Signal::derive(move || {
        let selected = workbench_selected.get();
        let sync_active = workbench_sync_active.get();
        let root_disabled = workbench_root_disabled.get();
        let disable_last = workbench_disable_last.get();
        let custom_class = workbench_custom_class.get();
        let rtl = workbench_rtl.get();
        let custom_motion = workbench_custom_motion.get();

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
            "ListWorkbenchConfig {{\n  id_base: Some(\"docs-list-workbench\"),\n  items: {:?},\n  selected_index: {selected:?},\n  default_selected_index: Some(1),\n  on_selected_index_change: \"bound(set_workbench_selected)\",\n  id: Some(\"docs-list-workbench-root\"),\n  aria_label: Some(\"List workbench\"),\n  aria_labelledby: Some(\"docs-list-workbench-heading\"),\n  lang: {:?},\n  dir: {:?},\n  is_disabled: {root_disabled},\n  disabled_indices: {},\n  on_action: \"count={} last={:?}\",\n  default_active_index: 1,\n  is_active_index_synced_to_selected: {sync_active},\n  motion: {:?},\n  class_name: {:?},\n  custom_class: {custom_class},\n  class: \"{}\",\n}}",
            workbench_items_for_config.clone(),
            if rtl { "ar" } else { "en-US" },
            if rtl { "rtl" } else { "ltr" },
            if disable_last { "vec![4]" } else { "vec![]" },
            workbench_action_count.get(),
            workbench_last_action.get(),
            if custom_motion { "custom" } else { "default" },
            if custom_class {
                Some("docs-list-workbench--custom")
            } else {
                None::<&str>
            },
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
                title="Hello World (Uncontrolled)"
                description="默认路径：不接受控状态轴，仅传 `id_base + items + aria_label` 即可运行。"
                code_signal=hello_code
            >
                <div class="docs-stack" data-slot="list-hello" style="width: min(100%, 320px);">
                    <List
                        id_base="docs-list-hello".to_string()
                        items=hello_items
                        aria_label="Settings navigation".to_string()
                    />
                </div>
            </Playground>

            <Playground
                title="Workbench（展示 + Config + Code + CSS Test）"
                description="按钮式 workbench：单画布调参，支持 settings / code / css-test 面板联动。"
                code_signal=workbench_code
                code_imports=list_code_imports.clone()
                test_css_source=workbench_test_css
                test_source_path="/root/autodl-tmp/zjj/p/rust-ui/crates/ui/src/list/styles.rs".to_string()
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
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_rtl.get()
                                on:change=move |ev| set_workbench_rtl.set(event_target_checked(&ev))
                            />
                            " RTL (lang + dir)"
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_custom_motion.get()
                                on:change=move |ev| set_workbench_custom_motion.set(event_target_checked(&ev))
                            />
                            " Custom motion"
                        </label>
                    </div>
                }
            >
                <div class="docs-stack" data-slot="list-workbench" style="width: min(100%, 420px);">
                    <span id="docs-list-workbench-heading" class="ui-muted">"List workbench heading"</span>
                    <span class="ui-muted">
                        "selected: "
                        {move || workbench_selected.get().map(|value| value.to_string()).unwrap_or_else(|| "None".to_string())}
                        " · disabled indices: "
                        {move || if workbench_disable_last.get() { "[4]" } else { "[]" }}
                        " · on_action: "
                        {move || format!("{} / {:?}", workbench_action_count.get(), workbench_last_action.get())}
                    </span>
                    {move || {
                        let disable_last = workbench_disable_last.get();
                        let root_disabled = workbench_root_disabled.get();
                        let sync_active = workbench_sync_active.get();
                        let custom_class = workbench_custom_class.get();
                        let rtl = workbench_rtl.get();
                        let custom_motion = workbench_custom_motion.get();

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
                                    selected_index=workbench_selected.into()
                                    default_selected_index=1
                                    on_selected_index_change=Callback::new(move |next| set_workbench_selected.set(next))
                                    id="docs-list-workbench-root".to_string()
                                    aria_label="List workbench".to_string()
                                    aria_labelledby="docs-list-workbench-heading".to_string()
                                    lang=if rtl { "ar".to_string() } else { "en-US".to_string() }
                                    dir=if rtl { A11yDirection::Rtl } else { A11yDirection::Ltr }
                                    is_active_index_synced_to_selected=sync_active
                                    is_disabled=root_disabled
                                    disabled_indices=disabled_indices
                                    on_action=on_workbench_action
                                    default_active_index=1
                                    motion=if custom_motion {
                                        ui::list::ListMotion {
                                            spring: ui::list::ListMotion::default().spring,
                                        }
                                    } else {
                                        ui::list::ListMotion::default()
                                    }
                                    class_name=class_name
                                />
                            </div>
                        }
                        .into_any()
                    }}
                </div>
            </Playground>

            <Playground
                title="状态矩阵 State Matrix（受控 / 非受控）"
                description="同一组数据对照 uncontrolled / controlled / disabled 三种语义状态。"
                code_signal=state_matrix_code
                code_imports=list_code_imports.clone()
            >
                <div class="docs-row" data-slot="list-state-matrix">
                    <div class="docs-stack">
                        <span class="ui-muted">"uncontrolled"</span>
                        <List
                            id_base="docs-list-matrix-uncontrolled".to_string()
                            items=showcase_items_for_matrix.clone()
                            aria_label="Matrix uncontrolled list".to_string()
                        />
                    </div>

                    <div class="docs-stack">
                        <span class="ui-muted">"controlled"</span>
                        <List
                            id_base="docs-list-matrix-controlled".to_string()
                            items=showcase_items_for_matrix.clone()
                            selected_index=state_matrix_controlled_selected.into()
                            on_selected_index_change=Callback::new(move |next| set_state_matrix_controlled_selected.set(next))
                            aria_label="Matrix controlled list".to_string()
                        />
                        <span class="ui-muted">
                            "selected: "
                            {move || state_matrix_controlled_selected.get().map(|value| value.to_string()).unwrap_or_else(|| "None".to_string())}
                        </span>
                    </div>

                    <div class="docs-stack">
                        <span class="ui-muted">"disabled"</span>
                        <List
                            id_base="docs-list-matrix-disabled".to_string()
                            items=showcase_items_for_matrix.clone()
                            selected_index=state_matrix_controlled_selected.into()
                            on_selected_index_change=Callback::new(move |next| set_state_matrix_controlled_selected.set(next))
                            aria_label="Matrix disabled list".to_string()
                            is_disabled=true
                        />
                    </div>
                </div>
            </Playground>

            <Playground
                title="展示：多场景"
                description="同一套 List 在默认、unsynced、disabled root、empty 四种状态下的行为对比。"
                code_signal=showcase_code
                code_imports=list_code_imports.clone()
            >
                <div class="docs-row" data-slot="list-showcase">
                    <div class="docs-stack" style="min-width: 220px;">
                        <span class="ui-muted">"default + disabled option"</span>
                        <List
                            id_base="docs-list-default".to_string()
                            items=showcase_items_for_showcase.clone()
                            selected_index=showcase_selected_default.into()
                            on_selected_index_change=Callback::new(move |next| set_showcase_selected_default.set(next))
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
                            items=showcase_items_for_showcase.clone()
                            selected_index=showcase_selected_unsynced.into()
                            on_selected_index_change=Callback::new(move |next| set_showcase_selected_unsynced.set(next))
                            aria_label="Unsynced list".to_string()
                            is_active_index_synced_to_selected=false
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
                            selected_index=showcase_selected_disabled.into()
                            on_selected_index_change=Callback::new(move |next| set_showcase_selected_disabled.set(next))
                            aria_label="Disabled list".to_string()
                            is_disabled=true
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
                            selected_index=showcase_selected_empty.into()
                            on_selected_index_change=Callback::new(move |next| set_showcase_selected_empty.set(next))
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
                title="State Matrix (Controlled / Uncontrolled / Disabled Comparison)"
                code_signal=state_matrix_code
                code_imports=list_code_imports.clone()
            >
                <div class="docs-row" data-slot="list-state-matrix-after-workbench">
                    <div class="docs-stack">
                        <span class="ui-muted">"uncontrolled"</span>
                        <List
                            id_base="docs-list-matrix-after-uncontrolled".to_string()
                            items=showcase_items_for_matrix_after.clone()
                            aria_label="Matrix uncontrolled list".to_string()
                        />
                    </div>
                    <div class="docs-stack">
                        <span class="ui-muted">"unsynced active index"</span>
                        <List
                            id_base="docs-list-matrix-after-unsynced".to_string()
                            items=showcase_items_for_matrix_after.clone()
                            default_selected_index=1
                            aria_label="Matrix unsynced list".to_string()
                            is_active_index_synced_to_selected=false
                        />
                    </div>
                    <div class="docs-stack">
                        <span class="ui-muted">"disabled root"</span>
                        <List
                            id_base="docs-list-matrix-after-disabled".to_string()
                            items=showcase_items_for_matrix_after.clone()
                            aria_label="Matrix disabled list".to_string()
                            is_disabled=true
                        />
                    </div>
                </div>
            </Playground>

            <Playground
                title="Streaming/Snapshot Display"
                description="List 非正文阅读面：Streaming Optional，fallback=snapshot。"
                code_signal=output_mode_code
                code_imports=list_code_imports
            >
                <div class="docs-row" data-slot="list-streaming-snapshot">
                    <div
                        class="docs-stack"
                        data-ui-streaming="optional"
                        data-ui-fallback="snapshot"
                        data-ui-output-state="snapshot"
                    >
                        <AiSpace mode=list_snapshot_mode output_status=list_verified_output>
                            <List
                                id_base="docs-list-snapshot".to_string()
                                items=showcase_items_for_stream_snapshot
                                selected_index=snapshot_selected.into()
                                on_selected_index_change=Callback::new(move |next| set_snapshot_selected.set(next))
                                aria_label="Snapshot list".to_string()
                            />
                        </AiSpace>
                        <span class="ui-muted">"Snapshot baseline: verified + copy-ready."</span>
                    </div>

                    <div
                        class="docs-stack"
                        data-ui-streaming="optional"
                        data-ui-fallback="snapshot"
                        data-ui-output-state="streaming"
                    >
                        <AiSpace mode=list_streaming_mode output_status=list_draft_output>
                            <List
                                id_base="docs-list-streaming".to_string()
                                items=showcase_items_for_stream_streaming
                                selected_index=streaming_selected.into()
                                on_selected_index_change=Callback::new(move |next| set_streaming_selected.set(next))
                                aria_label="Streaming list".to_string()
                            />
                        </AiSpace>
                        <span class="ui-muted">
                            "Streaming preview keeps fallback=snapshot contract explicit."
                        </span>
                    </div>
                </div>
            </Playground>

            <section class="docs-card docs-prose" data-slot="list-source-first">
                <h3>"Source-first / Copy-Paste Ready"</h3>
                <p>
                    "Each playground supports "
                    <code>"Show code"</code>
                    " with one-click copy. Copied snippets are import-ready via "
                    <code>"apps/docs-app/src/playground.rs::compose_copy_ready_code"</code>
                    "."
                </p>
                <Snippet
                    text="use leptos::prelude::*;\nuse std::sync::Arc;\nuse ui::List;\n\nlet items: Arc<[String]> = vec![\"Overview\".to_string(), \"Billing\".to_string()].into();\n<List id_base=\"docs-list\".to_string() items=items aria_label=\"Settings navigation\".to_string() />".to_string()
                    label="Copy starter".to_string()
                    copyable=true
                    class_name="docs-list-source-copy".to_string()
                />
                <ul data-slot="list-source-paths">
                    <li><code>"components/list/src/mod.rs"</code></li>
                    <li><code>"components/list/src/logic.rs"</code></li>
                    <li><code>"components/list/src/view.rs"</code></li>
                    <li><code>"components/list/src/styles.rs"</code></li>
                    <li><code>"components/list/src/motion.rs"</code></li>
                </ul>
                <ul data-slot="list-source-prerequisites">
                    <li><code>"component-list"</code></li>
                    <li><code>"inject-css"</code></li>
                </ul>
            </section>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn menu() -> AnyView {
    let hello_item_specs = vec![
        MenuItemSpec::action("New file"),
        MenuItemSpec::action("Share with team"),
    ];
    let workbench_items: Arc<[String]> = vec![
        "New file".to_string(),
        "Share with team".to_string(),
        "Sort ascending".to_string(),
    ]
    .into();
    let workbench_item_specs = vec![
        MenuItemSpec::action("New file"),
        MenuItemSpec::action("Share with team"),
        MenuItemSpec::action("Sort ascending"),
    ];

    let (showcase_last_action, set_showcase_last_action) = signal(None::<usize>);
    let on_showcase_action =
        Callback::new(move |index: usize| set_showcase_last_action.set(Some(index)));

    let default_index_options = vec!["0".to_string(), "1".to_string(), "2".to_string()];
    let (workbench_default_index, set_workbench_default_index) = signal(Some(0_usize));
    let (workbench_use_labelledby, set_workbench_use_labelledby) = signal(false);
    let (workbench_is_disabled, set_workbench_is_disabled) = signal(false);
    let (workbench_disabled, set_workbench_disabled) = signal(false);
    let (workbench_disable_second, set_workbench_disable_second) = signal(false);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);
    let (workbench_share_checked, set_workbench_share_checked) = signal(true);
    let (workbench_sort_ascending, set_workbench_sort_ascending) = signal(true);
    let (workbench_action_count, set_workbench_action_count) = signal(0_u32);
    let (workbench_last_action, set_workbench_last_action) = signal(None::<usize>);
    let on_workbench_action = Callback::new(move |index: usize| {
        set_workbench_action_count.update(|count| *count += 1);
        set_workbench_last_action.set(Some(index));
        match index {
            1 => set_workbench_share_checked.update(|value| *value = !*value),
            2 => set_workbench_sort_ascending.update(|value| *value = !*value),
            _ => {}
        }
    });

    let hello_code = Signal::derive(move || {
        r#"<Menu
  id_base="menu-hello".to_string()
  item_specs=vec![MenuItemSpec::action("New file"), MenuItemSpec::action("Share with team")]
  on_action=Callback::new(move |_: usize| {})
/>"#
        .to_string()
    });

    let workbench_code = Signal::derive(move || {
        let default_index = workbench_default_index.get().unwrap_or(0).min(2);
        let disabled_indices = if workbench_disable_second.get() {
            "vec![1]".to_string()
        } else {
            "Vec::<usize>::new()".to_string()
        };
        let class_name = if workbench_custom_class.get() {
            "\"docs-menu-workbench\".to_string()"
        } else {
            "String::new()"
        };
        let aria_label = if workbench_use_labelledby.get() {
            "String::new()"
        } else {
            "\"Workbench menu actions\".to_string()"
        };
        let aria_labelledby = if workbench_use_labelledby.get() {
            "\"docs-menu-workbench-heading\".to_string()"
        } else {
            "String::new()"
        };

        format!(
            "<Menu\n  id_base=\"docs-menu-workbench\".to_string()\n  items=vec![\"New file\".to_string(), \"Share with team\".to_string(), \"Sort ascending\".to_string()].into()\n  on_action=on_action\n  item_specs=vec![\n    MenuItemSpec::action(\"New file\"),\n    MenuItemSpec::action(\"Share with team\"),\n    MenuItemSpec::action(\"Sort ascending\"),\n  ]\n  id=\"docs-menu-workbench-root\".to_string()\n  aria_label={aria_label}\n  aria_labelledby={aria_labelledby}\n  is_disabled=Some({})\n  disabled={}\n  disabled_indices={disabled_indices}\n  item_kinds=vec![\n    MenuItemKind::Action,\n    MenuItemKind::Checkbox {{ is_checked: Signal::derive(move || share_checked.get()) }},\n    MenuItemKind::Radio {{ is_checked: Signal::derive(move || sort_ascending.get()) }},\n  ]\n  default_index={default_index}\n  motion=MenuMotion::default()\n  class_name={class_name}\n/>",
            bool_word(workbench_is_disabled.get()),
            bool_word(workbench_disabled.get()),
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        let default_index = workbench_default_index.get().unwrap_or(0).min(2);
        let disabled_indices = if workbench_disable_second.get() {
            vec![1_usize]
        } else {
            Vec::new()
        };
        let class_name = if workbench_custom_class.get() {
            Some("docs-menu-workbench")
        } else {
            None
        };
        let aria_label = if workbench_use_labelledby.get() {
            None
        } else {
            Some("Workbench menu actions")
        };
        let aria_labelledby = if workbench_use_labelledby.get() {
            Some("docs-menu-workbench-heading")
        } else {
            None
        };
        format!(
            "MenuActualConfig {{\n  id_base: \"docs-menu-workbench\",\n  items: [\"New file\", \"Share with team\", \"Sort ascending\"],\n  on_action: \"count={} last={:?}\",\n  item_specs: [\"action(New file)\", \"action(Share with team)\", \"action(Sort ascending)\"],\n  id: Some(\"docs-menu-workbench-root\"),\n  aria_label: {aria_label:?},\n  aria_labelledby: {aria_labelledby:?},\n  is_disabled: Some({}),\n  disabled: {},\n  disabled_indices: {disabled_indices:?},\n  item_kinds: [\"Action\", \"Checkbox\", \"Radio\"],\n  default_index: {default_index},\n  motion: MenuMotion::default(),\n  class_name: {class_name:?},\n}}",
            workbench_action_count.get(),
            workbench_last_action.get(),
            bool_word(workbench_is_disabled.get()),
            bool_word(workbench_disabled.get()),
        )
    });

    let matrix_code = Signal::derive(move || {
        r#"<Menu id_base="menu-default".to_string() items=vec!["New file".to_string(), "Share with team".to_string(), "Sort ascending".to_string()].into() on_action=Callback::new(move |_: usize| {}) item_specs=vec![MenuItemSpec::action("New file"), MenuItemSpec::action("Share with team"), MenuItemSpec::action("Sort ascending")] default_index=1 />
<Menu id_base="menu-labelledby".to_string() items=vec!["New file".to_string(), "Share with team".to_string(), "Sort ascending".to_string()].into() on_action=Callback::new(move |_: usize| {}) aria_labelledby="menu-matrix-heading".to_string() disabled_indices=vec![1] />
<Menu id_base="menu-disabled".to_string() items=vec!["New file".to_string(), "Share with team".to_string(), "Sort ascending".to_string()].into() on_action=Callback::new(move |_: usize| {}) is_disabled=Some(true) disabled=true class_name="docs-menu-workbench".to_string() />"#
            .to_string()
    });

    view! {
        <ComponentPage
            title="Menu"
            slug="menu"
            group="Collections"
            description="ARIA menu with action / checkbox / radio kinds, full API workbench config, and callback feedback."
        >
            <Playground
                title="Hello World (Default Path)"
                code_signal=hello_code
            >
                <div class="docs-stack docs-stack--tight">
                    <span class="ui-muted">"最小默认路径：仅 `id_base + item_specs + on_action`"</span>
                    <Menu
                        id_base="docs-menu-hello".to_string()
                        item_specs=hello_item_specs
                        on_action=on_showcase_action
                    />
                    <span class="ui-muted">
                        "last action: "
                        {move || showcase_last_action.get().map(|value| value.to_string()).unwrap_or_else(|| "None".to_string())}
                    </span>
                </div>
            </Playground>

            <Playground
                title="Workbench (All API + Actual Config)"
                code_signal=workbench_code
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="menu-workbench-controls">
                        <SegmentedControl
                            id_base="docs-menu-workbench-default-index".to_string()
                            options=default_index_options.clone()
                            selected_index=workbench_default_index
                            set_selected_index=set_workbench_default_index
                            size=SegmentedControlSize::Sm
                            aria_label="Menu default index".to_string()
                        />
                        <Switch checked=workbench_use_labelledby set_checked=set_workbench_use_labelledby>
                            "aria_labelledby"
                        </Switch>
                        <Switch checked=workbench_is_disabled set_checked=set_workbench_is_disabled>
                            "is_disabled"
                        </Switch>
                        <Switch checked=workbench_disabled set_checked=set_workbench_disabled>
                            "disabled"
                        </Switch>
                        <Switch checked=workbench_disable_second set_checked=set_workbench_disable_second>
                            "disabled_indices=[1]"
                        </Switch>
                        <Switch checked=workbench_custom_class set_checked=set_workbench_custom_class>
                            "class_name"
                        </Switch>
                    </div>
                }
            >
                <div class="docs-stack docs-stack--tight">
                    <h3 id="docs-menu-workbench-heading">"Workbench Menu"</h3>
                    <Menu
                        id_base="docs-menu-workbench".to_string()
                        items=workbench_items.clone()
                        on_action=on_workbench_action
                        item_specs=workbench_item_specs.clone()
                        id="docs-menu-workbench-root".to_string()
                        aria_label=if workbench_use_labelledby.get() {
                            String::new()
                        } else {
                            "Workbench menu actions".to_string()
                        }
                        aria_labelledby=if workbench_use_labelledby.get() {
                            "docs-menu-workbench-heading".to_string()
                        } else {
                            String::new()
                        }
                        is_disabled=workbench_is_disabled.get()
                        disabled=workbench_disabled.get()
                        disabled_indices=if workbench_disable_second.get() {
                            vec![1]
                        } else {
                            Vec::new()
                        }
                        item_kinds=vec![
                            MenuItemKind::Action,
                            MenuItemKind::Checkbox {
                                is_checked: Signal::derive(move || workbench_share_checked.get()),
                            },
                            MenuItemKind::Radio {
                                is_checked: Signal::derive(move || workbench_sort_ascending.get()),
                            },
                        ]
                        default_index=workbench_default_index.get().unwrap_or(0).min(2)
                        motion=MenuMotion::default()
                        class_name=if workbench_custom_class.get() {
                            "docs-menu-workbench".to_string()
                        } else {
                            String::new()
                        }
                    />
                    <span class="ui-muted">
                        "actions="
                        {move || workbench_action_count.get()}
                        " · last="
                        {move || workbench_last_action.get().map(|value| value.to_string()).unwrap_or_else(|| "None".to_string())}
                        " · share_checked="
                        {move || workbench_share_checked.get()}
                        " · sort_ascending="
                        {move || workbench_sort_ascending.get()}
                    </span>
                </div>
            </Playground>

            <Playground title="State Matrix (Default / LabelledBy / Disabled)" code_signal=matrix_code>
                <div class="docs-row">
                    <div class="docs-stack docs-stack--tight">
                        <Menu
                            id_base="docs-menu-matrix-default".to_string()
                            items=Arc::from(vec![
                                "New file".to_string(),
                                "Share with team".to_string(),
                                "Sort ascending".to_string(),
                            ])
                            on_action=Callback::new(|_: usize| {})
                            item_specs=vec![
                                MenuItemSpec::action("New file"),
                                MenuItemSpec::action("Share with team"),
                                MenuItemSpec::action("Sort ascending"),
                            ]
                            default_index=1
                            motion=MenuMotion::default()
                        />
                        <span class="ui-muted">"default_index=1 + item_specs"</span>
                    </div>
                    <div class="docs-stack docs-stack--tight">
                        <h4 id="docs-menu-matrix-label">"Matrix Label"</h4>
                        <Menu
                            id_base="docs-menu-matrix-labelledby".to_string()
                            items=Arc::from(vec![
                                "New file".to_string(),
                                "Share with team".to_string(),
                                "Sort ascending".to_string(),
                            ])
                            on_action=Callback::new(|_: usize| {})
                            aria_labelledby="docs-menu-matrix-label".to_string()
                            disabled_indices=vec![1]
                            motion=MenuMotion::default()
                        />
                        <span class="ui-muted">"aria_labelledby + disabled_indices"</span>
                    </div>
                    <div class="docs-stack docs-stack--tight">
                        <Menu
                            id_base="docs-menu-matrix-disabled".to_string()
                            items=Arc::from(vec![
                                "New file".to_string(),
                                "Share with team".to_string(),
                                "Sort ascending".to_string(),
                            ])
                            on_action=Callback::new(|_: usize| {})
                            aria_label="Disabled matrix menu".to_string()
                            is_disabled=true
                            disabled=true
                            class_name="docs-menu-workbench".to_string()
                            motion=MenuMotion::default()
                        />
                        <span class="ui-muted">"is_disabled + disabled + class_name"</span>
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
    let (workbench_custom_motion, set_workbench_custom_motion) = signal(false);
    let workbench_motion = Signal::derive(move || {
        let mut motion = ui::menu_trigger::MenuTriggerMotion::default();
        if workbench_custom_motion.get() {
            motion.popover.offset_y_px = 12.0;
        }
        motion
    });
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
            format!("  is_disabled={}", workbench_disabled.get()),
            "  open=Signal::derive(move || open.get())".to_string(),
            "  is_open=Signal::derive(move || open.get())".to_string(),
            "  default_open=false".to_string(),
            "  on_open_change=Callback::new(move |next| set_open.set(next))".to_string(),
            format!("  is_close_on_action={}", close_on_action),
            "  motion=MenuTriggerMotion::default()".to_string(),
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
            "/* crates/ui/src/menu/trigger/styles.rs */\n{}",
            ui::menu_trigger::styles::CSS
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
        let motion = workbench_motion.get();

        format!(
            "MenuTriggerActualConfig {{\n  id_base: \"docs-menu-trigger-workbench\",\n  items: [\"Profile\", \"Settings\", \"Archive\"],\n  on_action: \"set_workbench_last\",\n  is_disabled: {},\n  disabled: {},\n  disabled_indices: {:?},\n  item_kinds: [Action, Action, Action],\n  is_close_on_action: {},\n  close_on_action: {},\n  placement: PopoverPlacement::{:?},\n  is_open: {},\n  open: {},\n  default_open: false,\n  on_open_change: \"set_workbench_open_raw\",\n  motion: {:?},\n  aria_label: {:?},\n  class_name: {:?},\n  custom_aria_label: {},\n  custom_class_name: {},\n  last_action: {},\n}}",
            workbench_disabled.get(),
            workbench_disabled.get(),
            disabled_indices,
            workbench_close_on_action.get(),
            workbench_close_on_action.get(),
            placement,
            workbench_open_raw.get(),
            workbench_open_raw.get(),
            motion,
            if workbench_custom_label.get() {
                Some("Workbench menu trigger")
            } else {
                None
            },
            if workbench_custom_class.get() {
                Some("docs-menu-trigger-workbench")
            } else {
                None
            },
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
                        items=default_items.clone()
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

            <Playground
                title="Interactive Playground (Display / Config / Code / CSS Test)"
                code_signal=workbench_code
                test_css_source=workbench_test_css_source
                test_source_path="crates/ui/src/menu/trigger/styles.rs".to_string()
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
                        <button
                            type="button"
                            on:click=move |_| set_workbench_custom_motion.update(|value| *value = !*value)
                        >
                            "Toggle custom motion"
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
                                        is_disabled=workbench_disabled.get()
                                        close_on_action=workbench_close_on_action.get()
                                        is_close_on_action=workbench_close_on_action.get()
                                        disabled=workbench_disabled.get()
                                        disabled_indices=disabled_indices
                                        is_open=workbench_open
                                        open=workbench_open
                                        default_open=false
                                        on_open_change=on_workbench_open_change
                                        placement=placement
                                        motion=workbench_motion.get()
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

            <Playground title="State Matrix (Default / Controlled / Disabled)" code_signal=controlled_code>
                <div class="docs-row">
                    <div class="docs-stack">
                        <span class="ui-muted">"Default"</span>
                        <MenuTrigger
                            id_base="docs-menu-trigger-matrix-default".to_string()
                            items=vec![
                                "Profile".to_string(),
                                "Settings".to_string(),
                                "Log out".to_string(),
                            ]
                            on_action=on_action
                            item_kinds=vec![
                                MenuItemKind::Action,
                                MenuItemKind::Action,
                                MenuItemKind::Action,
                            ]
                        >
                            "Default"
                        </MenuTrigger>
                    </div>

                    <div class="docs-stack">
                        <span class="ui-muted">"Controlled + keep open"</span>
                        <MenuTrigger
                            id_base="docs-menu-trigger-matrix-controlled".to_string()
                            items=vec![
                                "Rename".to_string(),
                                "Duplicate".to_string(),
                                "Archive".to_string(),
                            ]
                            on_action=on_action
                            is_open=controlled_open
                            on_open_change=on_open_change
                            is_close_on_action=false
                            close_on_action=false
                            motion=ui::menu_trigger::MenuTriggerMotion::default()
                            item_kinds=vec![
                                MenuItemKind::Action,
                                MenuItemKind::Action,
                                MenuItemKind::Action,
                            ]
                        >
                            "Controlled"
                        </MenuTrigger>
                    </div>

                    <div class="docs-stack">
                        <span class="ui-muted">"Disabled trigger"</span>
                        <MenuTrigger
                            id_base="docs-menu-trigger-matrix-disabled".to_string()
                            items=vec!["Copy".to_string(), "Move".to_string()]
                            on_action=on_action
                            is_disabled=true
                            disabled=true
                            default_open=false
                            item_kinds=vec![MenuItemKind::Action, MenuItemKind::Action]
                        >
                            "Disabled"
                        </MenuTrigger>
                    </div>
                </div>
            </Playground>

            <Playground title="Controlled + persistent open" code_signal=controlled_code>
                <div class="docs-stack">
                    <MenuTrigger
                        id_base="docs-menu-trigger-controlled".to_string()
                        items=controlled_items.clone()
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
                        items=disabled_items.clone()
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
    let showcase_items = items.clone();
    let workbench_items = items.clone();
    let matrix_items = items;

    let (selected_index_raw, set_selected_index_raw) = signal(Some(1_usize));
    let selected_index: ReadSignal<Option<usize>> = selected_index_raw;
    let set_selected_index: WriteSignal<Option<usize>> = set_selected_index_raw;

    let (open_raw, set_open_raw) = signal(false);
    let open: Signal<bool> = Signal::derive(move || open_raw.get());
    let (on_open_change_runs, set_on_open_change_runs) = signal(0_u32);
    let on_open_change = Callback::new(move |next: bool| {
        set_open_raw.set(next);
        set_on_open_change_runs.update(|count| *count += 1);
    });

    let (workbench_is_disabled, set_workbench_is_disabled) = signal(false);
    let (workbench_disabled_alias, set_workbench_disabled_alias) = signal(false);
    let (workbench_disable_last, set_workbench_disable_last) = signal(true);
    let (workbench_place_top, set_workbench_place_top) = signal(false);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);
    let (workbench_rtl, set_workbench_rtl) = signal(false);

    let hello_code = Signal::derive(move || {
        r#"<Select
  id_base="docs-select-hello".to_string()
  items=vec!["Apple".to_string(), "Banana".to_string()]
  selected_index=selected
  set_selected_index=set_selected
/>"#
        .to_string()
    });

    let workbench_code = Signal::derive(move || {
        let class_name = if workbench_custom_class.get() {
            "docs-select-custom"
        } else {
            ""
        };
        let dir = if workbench_rtl.get() {
            "ui_headless::A11yDirection::Rtl"
        } else {
            "ui_headless::A11yDirection::Ltr"
        };
        [
            "<Select".to_string(),
            "  id_base=\"docs-select-workbench\".to_string()".to_string(),
            "  items=vec![\"Apple\".to_string(), \"Banana\".to_string(), \"Cherry\".to_string(), \"Durian\".to_string()]".to_string(),
            "  selected_index=selected_index".to_string(),
            "  set_selected_index=set_selected_index".to_string(),
            format!("  is_disabled={}", bool_word(workbench_is_disabled.get())),
            format!("  disabled={}", bool_word(workbench_disabled_alias.get())),
            "  placeholder=\"Select fruit\".to_string()".to_string(),
            format!(
                "  disabled_indices={}",
                if workbench_disable_last.get() {
                    "vec![3]"
                } else {
                    "Vec::<usize>::new()"
                }
            ),
            format!(
                "  placement={}",
                if workbench_place_top.get() {
                    "PopoverPlacement::TopStart"
                } else {
                    "PopoverPlacement::BottomStart"
                }
            ),
            "  open=open".to_string(),
            "  default_open=false".to_string(),
            "  on_open_change=on_open_change".to_string(),
            "  lang=\"en-US\".to_string()".to_string(),
            format!("  dir={dir}"),
            "  motion=ui::select::SelectMotion::default()".to_string(),
            format!("  class_name={}", rust_string_literal(class_name)),
            "/>".to_string(),
        ]
        .join("\n")
    });

    let workbench_actual_config = Signal::derive(move || {
        let class_name = if workbench_custom_class.get() {
            Some("docs-select-custom")
        } else {
            None
        };
        format!(
            "SelectActualConfig {{\n  id_base: \"docs-select-workbench\",\n  items: [\"Apple\", \"Banana\", \"Cherry\", \"Durian\"],\n  selected_index: {:?},\n  set_selected_index: \"WriteSignal<Option<usize>>\",\n  is_disabled: Some({}),\n  disabled: Some({}),\n  placeholder: Some(\"Select fruit\"),\n  disabled_indices: {},\n  placement: {},\n  open: Some({}),\n  default_open: Some(false),\n  on_open_change: \"runs={}\",\n  lang: Some(\"en-US\"),\n  dir: {},\n  motion: SelectMotion::default(),\n  class_name: {class_name:?},\n}}",
            selected_index_raw.get(),
            bool_word(workbench_is_disabled.get()),
            bool_word(workbench_disabled_alias.get()),
            if workbench_disable_last.get() {
                "vec![3]"
            } else {
                "vec![]"
            },
            if workbench_place_top.get() {
                "PopoverPlacement::TopStart"
            } else {
                "PopoverPlacement::BottomStart"
            },
            bool_word(open_raw.get()),
            on_open_change_runs.get(),
            if workbench_rtl.get() {
                "Some(A11yDirection::Rtl)"
            } else {
                "Some(A11yDirection::Ltr)"
            },
        )
    });

    let matrix_code = Signal::derive(move || {
        r#"<Select id_base="select-default".to_string() items=vec!["Apple".to_string(), "Banana".to_string(), "Cherry".to_string(), "Durian".to_string()] selected_index=selected set_selected_index=set_selected />
<Select id_base="select-top".to_string() items=vec!["Apple".to_string(), "Banana".to_string(), "Cherry".to_string(), "Durian".to_string()] selected_index=selected set_selected_index=set_selected placement=PopoverPlacement::TopStart open=Signal::derive(|| false) default_open=false />
<Select id_base="select-disabled".to_string() items=vec!["Apple".to_string(), "Banana".to_string(), "Cherry".to_string(), "Durian".to_string()] selected_index=selected set_selected_index=set_selected is_disabled=true disabled=true />"#.to_string()
    });

    view! {
        <ComponentPage
            title="Select"
            slug="select"
            group="Collections"
            description="Select playground with full API workbench and callback feedback."
        >
            <Playground title="Hello World (Default Select)" code_signal=hello_code>
                <Select
                    id_base="docs-select-hello".to_string()
                    items=showcase_items
                    selected_index=selected_index
                    set_selected_index=set_selected_index
                />
            </Playground>

            <Playground
                title="Workbench (All API + Actual Config)"
                code_signal=workbench_code
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="select-workbench-controls">
                        <Switch checked=workbench_is_disabled set_checked=set_workbench_is_disabled>
                            "is_disabled"
                        </Switch>
                        <Switch checked=workbench_disabled_alias set_checked=set_workbench_disabled_alias>
                            "disabled alias"
                        </Switch>
                        <Switch checked=workbench_disable_last set_checked=set_workbench_disable_last>
                            "Disable last option"
                        </Switch>
                        <Switch checked=workbench_place_top set_checked=set_workbench_place_top>
                            "Top placement"
                        </Switch>
                        <Switch checked=workbench_custom_class set_checked=set_workbench_custom_class>
                            "Custom class_name"
                        </Switch>
                        <Switch checked=workbench_rtl set_checked=set_workbench_rtl>
                            "RTL dir"
                        </Switch>
                        <ui::Button
                            variant=ui::ButtonVariant::Secondary
                            on_press=Callback::new(move |_| set_open_raw.update(|value| *value = !*value))
                        >
                            "Toggle open signal"
                        </ui::Button>
                    </div>
                }
            >
                <div class="docs-stack docs-stack--tight" data-slot="select-workbench-preview">
                    <Select
                        id_base="docs-select-workbench".to_string()
                        items=workbench_items
                        selected_index=selected_index
                        set_selected_index=set_selected_index
                        is_disabled=workbench_is_disabled.get()
                        disabled=workbench_disabled_alias.get()
                        placeholder="Select fruit".to_string()
                        disabled_indices=if workbench_disable_last.get() {
                            vec![3]
                        } else {
                            Vec::new()
                        }
                        placement=if workbench_place_top.get() {
                            PopoverPlacement::TopStart
                        } else {
                            PopoverPlacement::BottomStart
                        }
                        open=open
                        default_open=false
                        on_open_change=on_open_change
                        lang="en-US".to_string()
                        dir=if workbench_rtl.get() {
                            ui_headless::A11yDirection::Rtl
                        } else {
                            ui_headless::A11yDirection::Ltr
                        }
                        motion=ui::select::SelectMotion::default()
                        class_name=if workbench_custom_class.get() {
                            "docs-select-custom".to_string()
                        } else {
                            String::new()
                        }
                    />
                    <span class="ui-muted" data-slot="select-workbench-feedback">
                        "open: " {move || open_raw.get()}
                        " · on_open_change: " {move || on_open_change_runs.get()}
                        " · selected_index: "
                        {move || selected_index_raw.get().map_or_else(|| "None".to_string(), |it| it.to_string())}
                    </span>
                </div>
            </Playground>

            <Playground title="State Matrix (Default / Top / Disabled)" code_signal=matrix_code>
                <div class="docs-row" data-slot="select-state-matrix">
                    <Select
                        id_base="docs-select-matrix-default".to_string()
                        items=matrix_items.clone()
                        selected_index=selected_index
                        set_selected_index=set_selected_index
                    />
                    <Select
                        id_base="docs-select-matrix-top".to_string()
                        items=matrix_items.clone()
                        selected_index=selected_index
                        set_selected_index=set_selected_index
                        placement=PopoverPlacement::TopStart
                        open=open
                        default_open=false
                    />
                    <Select
                        id_base="docs-select-matrix-disabled".to_string()
                        items=matrix_items
                        selected_index=selected_index
                        set_selected_index=set_selected_index
                        is_disabled=true
                        disabled=true
                    />
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
    let showcase_items_for_hello = showcase_items.clone();
    let showcase_items_for_showcase = showcase_items.clone();
    let showcase_items_for_matrix = showcase_items.clone();
    let showcase_items_for_stream_snapshot = showcase_items.clone();
    let showcase_items_for_stream_streaming = showcase_items.clone();
    let disabled_items = vec!["Alpha".to_string(), "Beta".to_string(), "Gamma".to_string()];
    let empty_items: Vec<String> = Vec::new();
    let snapshot_mode = Signal::derive(|| AiRenderMode::Snapshot);
    let streaming_mode = Signal::derive(|| AiRenderMode::Streaming);
    let verified_output = Signal::derive(|| AiOutputStatus::Verified);
    let draft_output = Signal::derive(|| AiOutputStatus::Draft);
    let combo_box_code_imports = "use leptos::prelude::*;\nuse ui::ComboBox;".to_string();

    let (hello_selected, set_hello_selected) = signal(Some(1_usize));
    let (selected, set_selected) = signal(Some(1_usize));
    let (invalid, set_invalid) = signal(false);

    let (controlled_selected, set_controlled_selected) = signal(Some(2_usize));
    let (controlled_open_raw, set_controlled_open_raw) = signal(false);
    let controlled_open: Signal<bool> = Signal::derive(move || controlled_open_raw.get());
    let on_open_change = Callback::new(move |next: bool| set_controlled_open_raw.set(next));

    let (disabled_selected, set_disabled_selected) = signal(Some(0_usize));
    let (empty_selected, set_empty_selected) = signal(None::<usize>);
    let (snapshot_selected, set_snapshot_selected) = signal(Some(1_usize));
    let (streaming_selected, set_streaming_selected) = signal(Some(2_usize));

    let workbench_items = vec![
        "Rust".to_string(),
        "TypeScript".to_string(),
        "Go".to_string(),
        "Python".to_string(),
        "Zig".to_string(),
    ];
    let persisted_combo_box_workbench_selected = load_combo_box_workbench_selected();
    let (workbench_selected, set_workbench_selected) =
        signal(persisted_combo_box_workbench_selected.or(Some(1_usize)));
    let (workbench_invalid, set_workbench_invalid) = signal(false);
    let (workbench_disabled, set_workbench_disabled) = signal(false);
    let (workbench_disable_last, set_workbench_disable_last) = signal(true);
    let (workbench_controlled_open, set_workbench_controlled_open) = signal(false);
    let (workbench_on_open_change_runs, set_workbench_on_open_change_runs) = signal(0_u32);
    let (workbench_use_controlled_open, set_workbench_use_controlled_open) = signal(true);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);
    let (workbench_persist_state, set_workbench_persist_state) =
        signal(persisted_combo_box_workbench_selected.is_some());
    let on_workbench_open_change = Callback::new(move |next: bool| {
        set_workbench_controlled_open.set(next);
        set_workbench_on_open_change_runs.update(|count| *count += 1);
    });

    Effect::new(move |_| {
        let selected = workbench_selected.get();

        if workbench_persist_state.get() {
            if let Some(selected_index) = selected {
                save_combo_box_workbench_selected(selected_index);
            } else {
                clear_combo_box_workbench_selected();
            }
        } else {
            clear_combo_box_workbench_selected();
        }
    });

    let hello_code = Signal::derive(move || {
        r#"let items = vec![
  "Rust".to_string(),
  "TypeScript".to_string(),
  "Go".to_string(),
];
let (selected, set_selected) = signal(Some(1_usize));

<ComboBox
  id_base="docs-combo-box-hello".to_string()
  label="Language".to_string()
  items=items
  selected_index=selected
  set_selected_index=set_selected
/>"#
        .to_string()
    });

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
            "  is_required=Signal::derive(move || true)".to_string(),
            "  aria_describedby=Signal::derive(|| Some(\"combo-box-help\".to_string()))"
                .to_string(),
            "  description=\"Pick one runtime language\".into()".to_string(),
            "  error=\"Language is required\".into()".to_string(),
            "  placeholder=\"Search language\".into()".to_string(),
            "  empty_message=\"No language found\".into()".to_string(),
            "  toggle_button_aria_label=\"Open language options\".into()".to_string(),
            "  default_open=false".to_string(),
            "  lang=\"en-US\".into()".to_string(),
            "  dir=ui_headless::A11yDirection::Ltr".to_string(),
            "  motion=ui::combo_box::ComboBoxMotion::default()".to_string(),
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
            "/* components/combo-box/src/styles.rs */\n{}",
            ui::combo_box::styles::CSS,
        )
    });

    let output_mode_code = Signal::derive(move || {
        r#"// Streaming is optional for ComboBox; fallback is snapshot.
<div data-ui-streaming="optional" data-ui-fallback="snapshot">
  <AiSpace mode=AiRenderMode::Snapshot output_status=AiOutputStatus::Verified>
    <ComboBox id_base="docs-combo-box-snapshot".to_string() ... />
  </AiSpace>
  <AiSpace mode=AiRenderMode::Streaming output_status=AiOutputStatus::Draft>
    <ComboBox id_base="docs-combo-box-streaming".to_string() ... />
  </AiSpace>
</div>"#
            .to_string()
    });

    let matrix_code = Signal::derive(move || {
        r#"<ComboBox id_base="combo-matrix-default".to_string() label="Default".to_string() items=vec!["Rust".to_string(), "TypeScript".to_string(), "Go".to_string(), "Python".to_string(), "Zig".to_string()] selected_index=selected_default set_selected_index=set_selected_default />
<ComboBox id_base="combo-matrix-controlled".to_string() label="Controlled".to_string() items=vec!["Rust".to_string(), "TypeScript".to_string(), "Go".to_string(), "Python".to_string(), "Zig".to_string()] selected_index=selected_controlled set_selected_index=set_selected_controlled is_open=Signal::derive(move || open.get()) default_open=false />
<ComboBox id_base="combo-matrix-disabled".to_string() label="Disabled".to_string() items=vec!["Rust".to_string(), "TypeScript".to_string(), "Go".to_string(), "Python".to_string(), "Zig".to_string()] selected_index=selected_disabled set_selected_index=set_selected_disabled is_disabled=true disabled_indices=vec![4] />"#.to_string()
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
            "ComboBoxWorkbenchConfig {{\n  id_base: \"docs-combo-box-workbench\",\n  label: \"Language\",\n  items: [\"Rust\", \"TypeScript\", \"Go\", \"Python\", \"Zig\"],\n  selected_index: {selected:?},\n  set_selected_index: \"WriteSignal<Option<usize>>\",\n  is_disabled: Some({disabled}),\n  disabled_indices: {},\n  is_required: Some(true),\n  is_invalid: Some({invalid}),\n  aria_describedby: Some(Some(\"combo-box-help\")),\n  description: Some(\"Pick one runtime language\"),\n  error: Some(\"Language is required\"),\n  placeholder: Some(\"Search language\"),\n  empty_message: Some(\"No language found\"),\n  toggle_button_aria_label: Some(\"Open language options\"),\n  is_open: {},\n  default_open: Some(false),\n  on_open_change: \"runs={}\",\n  lang: Some(\"en-US\"),\n  dir: Some(A11yDirection::Ltr),\n  motion: ComboBoxMotion::default(),\n  class_name: {},\n  controlled_open_enabled: {use_controlled_open},\n  controlled_open_state: {open},\n  custom_class: {custom_class},\n  class: \"{}\",\n}}",
            if disable_last { "vec![4]" } else { "vec![]" },
            if use_controlled_open {
                format!("Some({open})")
            } else {
                "None".to_string()
            },
            workbench_on_open_change_runs.get(),
            if custom_class {
                "Some(\"docs-combo-box-workbench--custom\")"
            } else {
                "None"
            },
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
                title="Hello World (Uncontrolled)"
                description="最小路径：默认 API 即可运行，保留输入筛选 + 列表选择语义。"
                code_signal=hello_code
                code_imports=combo_box_code_imports.clone()
            >
                <AiSpace mode=snapshot_mode output_status=verified_output>
                    <div class="docs-stack" style="width: min(100%, 320px);">
                        <ComboBox
                            id_base="docs-combo-box-hello".to_string()
                            label="Language".to_string()
                            items=showcase_items_for_hello.clone()
                            selected_index=hello_selected
                            set_selected_index=set_hello_selected
                        />
                        <span class="ui-muted">
                            "hello selected: "
                            {move || hello_selected.get().map(|value| value.to_string()).unwrap_or_else(|| "None".to_string())}
                        </span>
                    </div>
                </AiSpace>
            </Playground>

            <Playground
                title="Workbench（展示 + Config + Code + CSS Test）"
                description="按钮式 workbench：单画布调参，支持 settings / code / css-test 面板联动，并可选持久化 selected index。"
                code_signal=workbench_code
                code_imports=combo_box_code_imports.clone()
                test_css_source=workbench_test_css
                test_source_path="/root/autodl-tmp/zjj/p/rust-ui/components/combo-box/src/styles.rs".to_string()
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
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_persist_state.get()
                                on:change=move |ev| set_workbench_persist_state.set(event_target_checked(&ev))
                            />
                            " Persist selected index (optional)"
                        </label>
                    </div>
                }
            >
                <div class="docs-stack" data-slot="combo-box-workbench" style="width: min(100%, 420px);">
                    <div class="docs-row">
                        <ui::Button
                            variant=ui::ButtonVariant::Secondary
                            on_press=Callback::new(move |_| {
                                set_workbench_controlled_open.update(|value| *value = !*value)
                            })
                        >
                            "Toggle open"
                        </ui::Button>
                        <span class="ui-muted">
                            "open: "
                            {move || workbench_controlled_open.get()}
                            " · on_open_change: "
                            {move || workbench_on_open_change_runs.get()}
                            " · selected: "
                            {move || workbench_selected.get().map(|value| value.to_string()).unwrap_or_else(|| "None".to_string())}
                            " · persist selected: "
                            {move || if workbench_persist_state.get() { "on" } else { "off" }}
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
                                        is_required=Signal::derive(move || true)
                                        aria_describedby=Signal::derive(|| {
                                            Some("combo-box-help".to_string())
                                        })
                                        description="Pick one runtime language".to_string()
                                        error="Language is required".to_string()
                                        placeholder="Search language".to_string()
                                        empty_message="No language found".to_string()
                                        toggle_button_aria_label="Open language options".to_string()
                                        is_invalid=Signal::derive(move || invalid)
                                        is_disabled=disabled
                                        disabled_indices=disabled_indices
                                        is_open=controlled_open
                                        default_open=false
                                        on_open_change=on_workbench_open_change
                                        lang="en-US".to_string()
                                        dir=ui_headless::A11yDirection::Ltr
                                        motion=ui::combo_box::ComboBoxMotion::default()
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
                                        is_required=Signal::derive(move || true)
                                        aria_describedby=Signal::derive(|| {
                                            Some("combo-box-help".to_string())
                                        })
                                        description="Pick one runtime language".to_string()
                                        error="Language is required".to_string()
                                        placeholder="Search language".to_string()
                                        empty_message="No language found".to_string()
                                        toggle_button_aria_label="Open language options".to_string()
                                        is_invalid=Signal::derive(move || invalid)
                                        is_disabled=disabled
                                        disabled_indices=disabled_indices
                                        default_open=false
                                        lang="en-US".to_string()
                                        dir=ui_headless::A11yDirection::Ltr
                                        motion=ui::combo_box::ComboBoxMotion::default()
                                        class_name=class_name
                                    />
                                </div>
                            }
                            .into_any()
                        }
                    }}
                </div>
            </Playground>

            <section class="docs-card docs-prose" data-slot="combo-box-state-matrix">
                <h3>"State Matrix"</h3>
                <ul data-slot="combo-box-state-rows">
                    <li>
                        <code>"open mode"</code>
                        " = controlled | uncontrolled"
                    </li>
                    <li>
                        <code>"disabled"</code>
                        " = root disabled | enabled with disabled options"
                    </li>
                    <li>
                        <code>"item set"</code>
                        " = has items | empty"
                    </li>
                    <li>
                        <code>"validation"</code>
                        " = valid | invalid"
                    </li>
                    <li>
                        <code>"selection"</code>
                        " = selected | none"
                    </li>
                </ul>
            </section>

            <Playground
                title="State Matrix (Default / Controlled / Disabled)"
                code_signal=matrix_code
                code_imports=combo_box_code_imports.clone()
            >
                <div class="docs-row" data-slot="combo-box-state-matrix-playground">
                    <ComboBox
                        id_base="docs-combo-box-matrix-default".to_string()
                        label="Default".to_string()
                        items=showcase_items_for_matrix.clone()
                        selected_index=selected
                        set_selected_index=set_selected
                        placeholder="Search language".to_string()
                    />
                    <ComboBox
                        id_base="docs-combo-box-matrix-controlled".to_string()
                        label="Controlled".to_string()
                        items=showcase_items_for_matrix.clone()
                        selected_index=controlled_selected
                        set_selected_index=set_controlled_selected
                        is_open=controlled_open
                        default_open=false
                        on_open_change=on_open_change
                        motion=ui::combo_box::ComboBoxMotion::default()
                    />
                    <ComboBox
                        id_base="docs-combo-box-matrix-disabled".to_string()
                        label="Disabled".to_string()
                        items=showcase_items_for_matrix.clone()
                        selected_index=disabled_selected
                        set_selected_index=set_disabled_selected
                        is_disabled=true
                        disabled_indices=vec![4]
                        class_name="docs-combo-box-workbench--custom".to_string()
                    />
                </div>
            </Playground>

            <Playground
                title="Showcase Variants"
                description="同一套 ComboBox 在校验、受控 open、禁用、空数据四种状态下的对比展示。"
                code_signal=showcase_code
                code_imports=combo_box_code_imports.clone()
            >
                <div class="docs-row" data-slot="combo-box-showcase">
                    <div class="docs-stack" style="min-width: 260px; width: min(100%, 320px);">
                        <span class="ui-muted">"validation + disabled option"</span>
                        <ComboBox
                            id_base="docs-combo-box".to_string()
                            label="Language".to_string()
                            items=showcase_items_for_showcase.clone()
                            selected_index=selected
                            set_selected_index=set_selected
                            disabled_indices=vec![4]
                            description="Pick one runtime language".to_string()
                            error="Language is required".to_string()
                            is_invalid=Signal::derive(move || invalid.get())
                        />
                        <div class="docs-row">
                            <ui::Button
                                variant=ui::ButtonVariant::Secondary
                                on_press=Callback::new(move |_| set_invalid.update(|value| *value = !*value))
                            >
                                {move || if invalid.get() { "Clear invalid" } else { "Mark invalid" }}
                            </ui::Button>
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
                            items=showcase_items_for_showcase.clone()
                            selected_index=controlled_selected
                            set_selected_index=set_controlled_selected
                            is_open=controlled_open
                            on_open_change=on_open_change
                            disabled_indices=vec![4]
                            description="Open state is externally controlled".to_string()
                        />
                        <div class="docs-row">
                            <ui::Button
                                variant=ui::ButtonVariant::Secondary
                                on_press=Callback::new(move |_| {
                                    set_controlled_open_raw.update(|value| *value = !*value)
                                })
                            >
                                "Toggle open"
                            </ui::Button>
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
                title="Streaming/Snapshot Display"
                description="ComboBox 不是正文阅读面：Streaming Optional，fallback=snapshot。"
                code_signal=output_mode_code
                code_imports=combo_box_code_imports
            >
                <div class="docs-row" data-slot="combo-box-streaming-snapshot">
                    <div
                        class="docs-stack"
                        style="min-width: 260px; width: min(100%, 320px);"
                        data-ui-streaming="optional"
                        data-ui-fallback="snapshot"
                        data-ui-output-state="snapshot"
                    >
                        <AiSpace mode=snapshot_mode output_status=verified_output>
                            <ComboBox
                                id_base="docs-combo-box-snapshot".to_string()
                                label="Snapshot mode".to_string()
                                items=showcase_items_for_stream_snapshot
                                selected_index=snapshot_selected
                                set_selected_index=set_snapshot_selected
                            />
                        </AiSpace>
                        <div class="ui-muted">"Snapshot baseline: verified + copy-ready."</div>
                    </div>

                    <div
                        class="docs-stack"
                        style="min-width: 260px; width: min(100%, 320px);"
                        data-ui-streaming="optional"
                        data-ui-fallback="snapshot"
                        data-ui-output-state="streaming"
                    >
                        <AiSpace mode=streaming_mode output_status=draft_output>
                            <ComboBox
                                id_base="docs-combo-box-streaming".to_string()
                                label="Streaming preview".to_string()
                                items=showcase_items_for_stream_streaming
                                selected_index=streaming_selected
                                set_selected_index=set_streaming_selected
                            />
                        </AiSpace>
                        <div class="ui-muted">"Streaming preview keeps fallback=snapshot contract explicit."</div>
                    </div>
                </div>
            </Playground>

            <section class="docs-card docs-prose" data-slot="combo-box-source-first">
                <h3>"Source-first / Copy-Paste Ready"</h3>
                <p>
                    "Each playground supports "
                    <code>"Show code"</code>
                    " with one-click copy. Copied snippets are import-ready via "
                    <code>"apps/docs-app/src/playground.rs::compose_copy_ready_code"</code>
                    "."
                </p>
                <Snippet
                    text="use leptos::prelude::*;\nuse ui::ComboBox;\n\nlet (selected, set_selected) = signal(Some(1_usize));\n<ComboBox id_base=\"docs-combo-box\".to_string() label=\"Language\".to_string() items=vec![\"Rust\".to_string(), \"TypeScript\".to_string()] selected_index=selected set_selected_index=set_selected />".to_string()
                    label="Copy starter".to_string()
                    copyable=true
                    class_name="docs-combo-box-source-copy".to_string()
                />
                <ul data-slot="combo-box-source-paths">
                    <li><code>"components/combo-box/src/mod.rs"</code></li>
                    <li><code>"components/combo-box/src/logic.rs"</code></li>
                    <li><code>"components/combo-box/src/view.rs"</code></li>
                    <li><code>"components/combo-box/src/styles.rs"</code></li>
                    <li><code>"components/combo-box/src/motion.rs"</code></li>
                </ul>
                <ul data-slot="combo-box-source-prerequisites">
                    <li><code>"component-combo_box"</code></li>
                    <li><code>"inject-css"</code></li>
                </ul>
            </section>
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
    let items = vec![
        "San Francisco".to_string(),
        "Seattle".to_string(),
        "Shanghai".to_string(),
        "Shenzhen".to_string(),
        "Singapore".to_string(),
    ];
    let items_for_validation = items.clone();
    let items_for_stream_snapshot = items.clone();
    let items_for_stream_streaming = items.clone();
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
    let snapshot_mode = Signal::derive(|| AiRenderMode::Snapshot);
    let streaming_mode = Signal::derive(|| AiRenderMode::Streaming);
    let verified_output = Signal::derive(|| AiOutputStatus::Verified);
    let draft_output = Signal::derive(|| AiOutputStatus::Draft);
    let (snapshot_selected, set_snapshot_selected) = signal(Some(1_usize));
    let (streaming_selected, set_streaming_selected) = signal(Some(2_usize));

    let disabled_items = vec![
        "Berlin".to_string(),
        "Boston".to_string(),
        "Brisbane".to_string(),
    ];
    let (disabled_selected, set_disabled_selected) = signal(Some(0_usize));

    let empty_items: Vec<String> = Vec::new();
    let empty_items_for_state_matrix = empty_items.clone();
    let (empty_selected, set_empty_selected) = signal(None::<usize>);

    let hello_code = Signal::derive(move || {
        r#"<Autocomplete
  id_base="city".to_string()
  label="City".to_string()
  items=vec!["Sydney".to_string(), "Melbourne".to_string()]
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

    let output_mode_code = Signal::derive(move || {
        r#"// Streaming is optional for Autocomplete; fallback is snapshot.
<div data-ui-streaming="optional" data-ui-fallback="snapshot">
  <AiSpace mode=AiRenderMode::Snapshot output_status=AiOutputStatus::Verified>
    <Autocomplete id_base="docs-autocomplete-snapshot".to_string() ... />
  </AiSpace>
  <AiSpace mode=AiRenderMode::Streaming output_status=AiOutputStatus::Draft>
    <Autocomplete id_base="docs-autocomplete-streaming".to_string() ... />
  </AiSpace>
</div>"#
            .to_string()
    });

    let autocomplete_code_imports = "use leptos::prelude::*;\nuse ui::Autocomplete;".to_string();

    let persisted_autocomplete_workbench_selected = load_autocomplete_workbench_selected();
    let (workbench_selected, set_workbench_selected) =
        signal(persisted_autocomplete_workbench_selected.or(Some(2_usize)));
    let (workbench_invalid, set_workbench_invalid) = signal(false);
    let (workbench_disabled, set_workbench_disabled) = signal(false);
    let (workbench_disable_last, set_workbench_disable_last) = signal(true);
    let (workbench_controlled_open, set_workbench_controlled_open) = signal(false);
    let (workbench_use_controlled_open, set_workbench_use_controlled_open) = signal(true);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);
    let (workbench_persist_state, set_workbench_persist_state) =
        signal(persisted_autocomplete_workbench_selected.is_some());
    let workbench_items = vec![
        "San Francisco".to_string(),
        "Seattle".to_string(),
        "Shanghai".to_string(),
        "Shenzhen".to_string(),
        "Singapore".to_string(),
    ];
    let workbench_items_for_state_matrix = workbench_items.clone();

    Effect::new(move |_| {
        let selected = workbench_selected.get();

        if workbench_persist_state.get() {
            if let Some(selected_index) = selected {
                save_autocomplete_workbench_selected(selected_index);
            } else {
                clear_autocomplete_workbench_selected();
            }
        } else {
            clear_autocomplete_workbench_selected();
        }
    });

    let workbench_code = Signal::derive(move || {
        let invalid = workbench_invalid.get();
        let disabled = workbench_disabled.get();
        let disable_last = workbench_disable_last.get();
        let use_controlled_open = workbench_use_controlled_open.get();
        let custom_class = workbench_custom_class.get();

        let mut lines = vec![
            "let (selected, set_selected) = signal(Some(2_usize));".to_string(),
            "let (open, set_open) = signal(false);".to_string(),
            "<Autocomplete".to_string(),
            "  id_base=\"docs-autocomplete-workbench\".into()".to_string(),
            "  label=\"City\".into()".to_string(),
            "  items=vec![".to_string(),
            "    \"San Francisco\".into(),".to_string(),
            "    \"Seattle\".into(),".to_string(),
            "    \"Shanghai\".into(),".to_string(),
            "    \"Shenzhen\".into(),".to_string(),
            "    \"Singapore\".into(),".to_string(),
            "  ]".to_string(),
            "  selected_index=selected".to_string(),
            "  default_selected_index=2".to_string(),
            "  on_selected_index_change=Callback::new(move |next| set_selected.set(next))"
                .to_string(),
            "  set_selected_index=set_selected".to_string(),
            "  is_required=Signal::derive(move || false)".to_string(),
            "  required=Signal::derive(move || false)".to_string(),
            "  aria_describedby=Signal::derive(move || Some(\"docs-autocomplete-hint\".to_string()))"
                .to_string(),
            "  description=\"Search and pick one city\".into()".to_string(),
            "  error=\"City is required\".into()".to_string(),
            "  placeholder=\"Type…\".into()".to_string(),
            "  empty_message=\"No matches\".into()".to_string(),
            "  default_open=false".to_string(),
            "  lang=\"en\".into()".to_string(),
            "  dir=A11yDirection::Ltr".to_string(),
            "  motion=AutocompleteMotion::default()".to_string(),
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
            lines.push("  open=Signal::derive(move || open.get())".to_string());
            lines
                .push("  on_open_change=Callback::new(move |next| set_open.set(next))".to_string());
        }
        if custom_class {
            lines.push("  class_name=\"docs-autocomplete-workbench--custom\".into()".to_string());
        }

        lines.push("/>".to_string());
        lines.join("\n")
    });

    let workbench_test_css = Signal::derive(move || {
        format!(
            "/* components/autocomplete/src/styles.rs */\n{}",
            ui::autocomplete::styles::CSS,
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        let selected = workbench_selected.get();
        let invalid = workbench_invalid.get();
        let disabled = workbench_disabled.get();
        let disable_last = workbench_disable_last.get();
        let _open = workbench_controlled_open.get();
        let use_controlled_open = workbench_use_controlled_open.get();
        let custom_class = workbench_custom_class.get();

        let mut class = vec!["ui-autocomplete".to_string()];
        if custom_class {
            class.push("docs-autocomplete-workbench--custom".to_string());
        }
        if invalid {
            class.push("ui-autocomplete--invalid".to_string());
        }
        if use_controlled_open {
            class.push("ui-autocomplete--controlled".to_string());
        }

        format!(
            "AutocompleteWorkbenchConfig {{\n  id_base: \"docs-autocomplete-workbench\",\n  label: \"City\",\n  items: [\"San Francisco\", \"Seattle\", \"Shanghai\", \"Shenzhen\", \"Singapore\"],\n  selected_index: {selected:?},\n  default_selected_index: Some(2),\n  on_selected_index_change: Some(\"Callback<Option<usize>>\"),\n  set_selected_index: Some(\"WriteSignal<Option<usize>>\"),\n  is_disabled: Some({disabled}),\n  disabled: {disabled},\n  disabled_indices: {},\n  is_required: Some(false),\n  required: Some(false),\n  is_invalid: Some({invalid}),\n  invalid: Some({invalid}),\n  aria_describedby: Some(\"docs-autocomplete-hint\"),\n  description: Some(\"Search and pick one city\"),\n  error: Some(\"City is required\"),\n  placeholder: Some(\"Type…\"),\n  empty_message: Some(\"No matches\"),\n  is_open: {},\n  open: {},\n  default_open: Some(false),\n  on_open_change: {},\n  lang: Some(\"en\"),\n  dir: Some(\"ltr\"),\n  motion: AutocompleteMotion::default(),\n  class_name: {},\n  class: \"{}\",\n}}",
            if disable_last { "vec![4]" } else { "vec![]" },
            if use_controlled_open {
                "Some(true)"
            } else {
                "None"
            },
            if use_controlled_open {
                "Some(true)"
            } else {
                "None"
            },
            if use_controlled_open {
                "Some(\"Callback<bool>\")"
            } else {
                "None"
            },
            if custom_class {
                "Some(\"docs-autocomplete-workbench--custom\")"
            } else {
                "None"
            },
            class.join(" ")
        )
    });

    view! {
        <ComponentPage
            title="Autocomplete"
            slug="autocomplete"
            group="Collections"
            description="Combobox-like autocomplete with baseline-style root attrs, controlled/uncontrolled open state, and baseline-level active highlight motion."
        >
            <Playground
                title="Hello World"
                code_signal=hello_code
                code_imports=autocomplete_code_imports.clone()
            >
                <AiSpace mode=snapshot_mode output_status=verified_output>
                    <div class="docs-stack" data-slot="autocomplete-hello-world">
                        <Autocomplete
                            id_base="docs-autocomplete-hello".to_string()
                            label="City".to_string()
                            items=hello_items
                        />
                    </div>
                </AiSpace>
            </Playground>

            <Playground
                title="Workbench（展示 + Config + Code + CSS Test）"
                description="Autocomplete 单画布调参：支持 settings / code / css-test 联动，并可选持久化 selected index。"
                code_signal=workbench_code
                code_imports=autocomplete_code_imports.clone()
                test_css_source=workbench_test_css
                test_source_path="/root/autodl-tmp/zjj/p/rust-ui/components/autocomplete/src/styles.rs".to_string()
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="autocomplete-workbench-controls">
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
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_persist_state.get()
                                on:change=move |ev| set_workbench_persist_state.set(event_target_checked(&ev))
                            />
                            " Persist selected index (optional)"
                        </label>
                    </div>
                }
            >
                <div class="docs-stack" data-slot="autocomplete-workbench" style="width: min(100%, 420px);">
                    <div class="docs-row">
                        <ui::Button
                            variant=ui::ButtonVariant::Secondary
                            on_press=Callback::new(move |_| {
                                set_workbench_controlled_open.update(|value| *value = !*value)
                            })
                        >
                            "Toggle open"
                        </ui::Button>
                        <span class="ui-muted">
                            "open: "
                            {move || workbench_controlled_open.get()}
                            " · selected: "
                            {move || workbench_selected.get().map(|value| value.to_string()).unwrap_or_else(|| "None".to_string())}
                            " · persist selected: "
                            {move || if workbench_persist_state.get() { "on" } else { "off" }}
                        </span>
                    </div>

                    {move || {
                        let invalid = workbench_invalid.get();
                        let disabled = workbench_disabled.get();
                        let disable_last = workbench_disable_last.get();
                        let use_controlled_open = workbench_use_controlled_open.get();
                        let custom_class = workbench_custom_class.get();
                        let controlled_open =
                            Signal::derive(move || workbench_controlled_open.get());
                        let on_workbench_open_change =
                            Callback::new(move |next: bool| set_workbench_controlled_open.set(next));
                        let class_name = if custom_class {
                            "docs-autocomplete-workbench--custom".to_string()
                        } else {
                            String::new()
                        };
                        let disabled_indices = if disable_last { vec![4] } else { vec![] };

                        if use_controlled_open {
                            view! {
                                <div class="docs-card" data-slot="autocomplete-workbench-canvas">
                                    <Autocomplete
                                        id_base="docs-autocomplete-workbench".to_string()
                                        label="City".to_string()
                                        items=workbench_items.clone()
                                        selected_index=workbench_selected
                                        set_selected_index=set_workbench_selected
                                        is_open=controlled_open
                                        on_open_change=on_workbench_open_change
                                        is_invalid=Signal::derive(move || invalid)
                                        is_disabled=disabled
                                        disabled_indices=disabled_indices.clone()
                                        description="Search and pick one city".to_string()
                                        error="City is required".to_string()
                                        class_name=class_name.clone()
                                    />
                                </div>
                            }
                            .into_any()
                        } else {
                            view! {
                                <div class="docs-card" data-slot="autocomplete-workbench-canvas">
                                    <Autocomplete
                                        id_base="docs-autocomplete-workbench".to_string()
                                        label="City".to_string()
                                        items=workbench_items.clone()
                                        selected_index=workbench_selected
                                        set_selected_index=set_workbench_selected
                                        is_invalid=Signal::derive(move || invalid)
                                        is_disabled=disabled
                                        disabled_indices=disabled_indices
                                        description="Search and pick one city".to_string()
                                        error="City is required".to_string()
                                        class_name=class_name
                                    />
                                </div>
                            }
                            .into_any()
                        }
                    }}
                </div>
            </Playground>



            <Playground
                title="Selection + Validation"
                code_signal=code
                code_imports=autocomplete_code_imports.clone()
            >
                <div class="docs-stack" data-slot="autocomplete-validation-playground">
                    <Autocomplete
                        id_base="docs-autocomplete".to_string()
                        label="City".to_string()
                        items=items_for_validation
                        selected_index=selected
                        set_selected_index=set_selected
                        disabled_indices=vec![3]
                        description="Search and pick one city".to_string()
                        error="City is required".to_string()
                        is_invalid=Signal::derive(move || invalid.get())
                        placeholder="Type…".to_string()
                    />
                    <div class="docs-row">
                        <ui::Button
                            variant=ui::ButtonVariant::Secondary
                            on_press=Callback::new(move |_| set_invalid.update(|value| *value = !*value))
                        >
                            {move || if invalid.get() { "Clear invalid" } else { "Mark invalid" }}
                        </ui::Button>
                        <span class="ui-muted">
                            "selected: "
                            {move || selected.get().map(|value| value.to_string()).unwrap_or_else(|| "None".to_string())}
                        </span>
                    </div>
                </div>
            </Playground>

            <Playground
                title="Controlled Open State"
                code_signal=controlled_code
                code_imports=autocomplete_code_imports.clone()
            >
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

            <Playground
                title="Disabled + Empty"
                code_signal=states_code
                code_imports=autocomplete_code_imports.clone()
            >
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
                            items=empty_items.clone()
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

            <section class="docs-card docs-prose" data-slot="autocomplete-state-matrix">
                <h3>"状态矩阵 State Matrix（受控 / 非受控）"</h3>
                <ul data-slot="autocomplete-state-rows">
                    <li>
                        <code>"open mode"</code>
                        " = controlled | uncontrolled"
                    </li>
                    <li>
                        <code>"disabled"</code>
                        " = root disabled | enabled with disabled options"
                    </li>
                    <li>
                        <code>"validation"</code>
                        " = valid | invalid"
                    </li>
                    <li>
                        <code>"item set"</code>
                        " = has items | empty"
                    </li>
                    <li>
                        <code>"selection"</code>
                        " = selected | none"
                    </li>
                </ul>
            </section>

            <section class="docs-card docs-prose" data-slot="autocomplete-parameter-matrix">
                <h3>"参数矩阵 Parameter Matrix（API / 默认值）"</h3>
                <ul data-slot="autocomplete-parameter-rows">
                    <li>
                        <code>"is_open + on_open_change + default_open"</code>
                        " = open 受控/非受控轴（default_open 默认 false）"
                    </li>
                    <li>
                        <code>"selected_index + on_selected_index_change + default_selected_index"</code>
                        " = selection 受控/非受控轴（default_selected_index 默认 none，越界值自动忽略）"
                    </li>
                    <li>
                        <code>"set_selected_index"</code>
                        " = 迁移期历史别名（桥接到 on_selected_index_change）"
                    </li>
                    <li>
                        <code>"is_disabled / is_required / is_invalid"</code>
                        " = 布尔轴，默认 false（历史别名：disabled / required / invalid）"
                    </li>
                    <li>
                        <code>"label / id_base / placeholder / empty_message"</code>
                        " = 默认值来自 ui-state-primitives（Options / autocomplete / Type… / No matches）"
                    </li>
                </ul>
            </section>



            <Playground
                title="State Matrix (Validation / Controlled / Empty)"
                code_signal=states_code
                code_imports=autocomplete_code_imports.clone()
            >
                <div class="docs-row" data-slot="autocomplete-state-matrix-playground">
                    <Autocomplete
                        id_base="docs-autocomplete-matrix-invalid".to_string()
                        label="Invalid".to_string()
                        items=workbench_items_for_state_matrix.clone()
                        selected_index=workbench_selected
                        set_selected_index=set_workbench_selected
                        is_invalid=Signal::derive(move || workbench_invalid.get())
                        description="Validation state".to_string()
                        error="City is required".to_string()
                        placeholder="Type…".to_string()
                        empty_message="No matches".to_string()
                    />
                    <Autocomplete
                        id_base="docs-autocomplete-matrix-controlled".to_string()
                        label="Controlled open".to_string()
                        items=workbench_items_for_state_matrix.clone()
                        selected_index=workbench_selected
                        set_selected_index=set_workbench_selected
                        is_open=Signal::derive(move || workbench_controlled_open.get())
                        on_open_change=Callback::new(move |next: bool| set_workbench_controlled_open.set(next))
                        default_open=false
                    />
                    <Autocomplete
                        id_base="docs-autocomplete-matrix-empty".to_string()
                        label="Empty".to_string()
                        items=empty_items_for_state_matrix.clone()
                        selected_index=empty_selected
                        set_selected_index=set_empty_selected
                        empty_message="No matches".to_string()
                    />
                </div>
            </Playground>

            <Playground
                title="Streaming/Snapshot Display"
                description="Autocomplete 不是正文阅读面：Streaming Optional，fallback=snapshot。"
                code_signal=output_mode_code
                code_imports=autocomplete_code_imports.clone()
            >
                <div class="docs-row" data-slot="autocomplete-streaming-snapshot">
                    <div
                        class="docs-stack"
                        style="min-width: 260px; width: min(100%, 320px);"
                        data-ui-streaming="optional"
                        data-ui-fallback="snapshot"
                        data-ui-output-state="snapshot"
                    >
                        <AiSpace mode=snapshot_mode output_status=verified_output>
                            <Autocomplete
                                id_base="docs-autocomplete-snapshot".to_string()
                                label="Snapshot mode".to_string()
                                items=items_for_stream_snapshot
                                selected_index=snapshot_selected
                                set_selected_index=set_snapshot_selected
                            />
                        </AiSpace>
                        <div class="ui-muted">"Snapshot baseline: verified + copy-ready."</div>
                    </div>

                    <div
                        class="docs-stack"
                        style="min-width: 260px; width: min(100%, 320px);"
                        data-ui-streaming="optional"
                        data-ui-fallback="snapshot"
                        data-ui-output-state="streaming"
                    >
                        <AiSpace mode=streaming_mode output_status=draft_output>
                            <Autocomplete
                                id_base="docs-autocomplete-streaming".to_string()
                                label="Streaming preview".to_string()
                                items=items_for_stream_streaming
                                selected_index=streaming_selected
                                set_selected_index=set_streaming_selected
                            />
                        </AiSpace>
                        <div class="ui-muted">"Streaming preview keeps fallback=snapshot contract explicit."</div>
                    </div>
                </div>
            </Playground>

            <section class="docs-card docs-prose" data-slot="autocomplete-source-first">
                <h3>"Source-first / Copy-Paste Ready"</h3>
                <p>
                    "Each playground supports "
                    <code>"Show code"</code>
                    " with one-click copy. Copied snippets are import-ready via "
                    <code>"apps/docs-app/src/playground.rs::compose_copy_ready_code"</code>
                    "."
                </p>
                <Snippet
                    text="use leptos::prelude::*;\nuse ui::Autocomplete;\n\nlet (selected, set_selected) = signal(Some(1_usize));\n<Autocomplete id_base=\"docs-autocomplete\".to_string() label=\"City\".to_string() items=vec![\"Tokyo\".to_string(), \"Osaka\".to_string()] selected_index=selected set_selected_index=set_selected />".to_string()
                    label="Copy starter".to_string()
                    copyable=true
                    class_name="docs-autocomplete-source-copy".to_string()
                />
                <ul data-slot="autocomplete-source-paths">
                    <li><code>"components/autocomplete/src/mod.rs"</code></li>
                    <li><code>"components/autocomplete/src/logic.rs"</code></li>
                    <li><code>"components/autocomplete/src/view.rs"</code></li>
                    <li><code>"components/autocomplete/src/styles.rs"</code></li>
                    <li><code>"components/autocomplete/src/motion.rs"</code></li>
                </ul>
                <ul data-slot="autocomplete-source-prerequisites">
                    <li><code>"component-autocomplete"</code></li>
                    <li><code>"inject-css"</code></li>
                </ul>
            </section>
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
    let showcase_items = default_items.clone();
    let default_playground_items = default_items.clone();
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
            "  is_disabled=Some(false)".to_string(),
            "  is_close_on_action=Some(true)".to_string(),
            "  placement=PopoverPlacement::BottomStart".to_string(),
            "  default_open=Some(false)".to_string(),
            "  trigger_variant=ButtonVariant::Secondary".to_string(),
            "  trigger_size=ButtonSize::Sm".to_string(),
        ];

        if disabled {
            lines.push("  is_disabled=Some(true)".to_string());
        }
        if !close_on_action {
            lines.push("  is_close_on_action=Some(false)".to_string());
        }
        if controlled {
            lines.push("  is_open=Signal::derive(move || open.get())".to_string());
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
            "/* crates/ui/src/menu/dropdown_menu/styles.rs */\n{}",
            ui::dropdown_menu::styles::CSS
        )
    });

    let interactive_config = Signal::derive(move || {
        let items: Vec<&str> = match interactive_item_mode.get().unwrap_or(0) {
            1 => vec!["Rename", "Share"],
            2 => Vec::new(),
            _ => vec!["Duplicate", "Move", "Archive"],
        };
        let item_kinds: Vec<&str> = if items.is_empty() {
            Vec::new()
        } else {
            vec!["Action"; items.len()]
        };
        let motion = if interactive_custom_motion.get() {
            "DropdownMenuMotion { popover: PopoverMotion { initial_scale: 0.96, offset_y_px: 14.0, ..PopoverMotion::default() } }"
        } else {
            "DropdownMenuMotion::default()"
        };
        let class_name: Option<&str> = if interactive_custom_class.get() {
            Some("docs-dropdown-custom")
        } else {
            None
        };
        let on_open_change_feedback = if interactive_controlled.get() {
            format!(
                "set_interactive_open_raw(open={})",
                interactive_open_raw.get()
            )
        } else {
            "uncontrolled".to_string()
        };
        format!(
            "DropdownMenuActualConfig {{\n  id_base: \"docs-dropdown-interactive\",\n  items: {:?},\n  is_disabled: {:?},\n  item_kinds: {:?},\n  is_close_on_action: {:?},\n  placement: {:?},\n  is_open: {:?},\n  default_open: {:?},\n  on_action: \"last={:?}\",\n  on_open_change: {:?},\n  trigger_variant: {:?},\n  trigger_size: {:?},\n  motion: {motion},\n  class_name: {:?},\n  disabled_indices: {:?},\n}}",
            items,
            Some(interactive_disabled.get()),
            item_kinds,
            Some(interactive_close_on_action.get()),
            ui_headless::PopoverPlacement::BottomStart,
            if interactive_controlled.get() {
                Some(interactive_open_raw.get())
            } else {
                None
            },
            Some(false),
            interactive_last.get(),
            on_open_change_feedback,
            ui::ButtonVariant::Secondary,
            ui::ButtonSize::Sm,
            class_name,
            if interactive_with_disabled_items.get() {
                vec![1]
            } else {
                Vec::new()
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

    let matrix_code = Signal::derive(move || {
        r#"let (open, set_open) = signal(false);

<DropdownMenu id_base="dd-matrix-default".to_string() items=vec!["Duplicate".to_string(), "Move".to_string(), "Archive".to_string()] on_action=Callback::new(move |_: usize| {}) placement=PopoverPlacement::BottomStart trigger_variant=ButtonVariant::Secondary trigger_size=ButtonSize::Sm>
  "Default"
</DropdownMenu>
<DropdownMenu id_base="dd-matrix-controlled".to_string() items=vec!["Rename".to_string(), "Move".to_string(), "Share".to_string()] on_action=Callback::new(move |_: usize| {}) is_open=Signal::derive(move || open.get()) default_open=false on_open_change=Callback::new(move |next| set_open.set(next)) is_close_on_action=Some(false) placement=PopoverPlacement::TopEnd trigger_variant=ButtonVariant::Secondary trigger_size=ButtonSize::Sm>
  "Controlled"
</DropdownMenu>
<DropdownMenu id_base="dd-matrix-disabled".to_string() items=vec!["Duplicate".to_string(), "Archive".to_string()] on_action=Callback::new(move |_: usize| {}) is_disabled=Some(true) item_kinds=vec![MenuItemKind::Action, MenuItemKind::Action] motion=DropdownMenuMotion::default() class_name="docs-dropdown-custom".to_string()>
  "Disabled"
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
                title="Hello World (Default DropdownMenu)"
                code_signal=code
            >
                <div class="docs-row">
                    <DropdownMenu
                        id_base="docs-dropdown-showcase".to_string()
                        items=showcase_items.clone()
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

            <Playground
                title="Interactive Playground"
                description="Display + Config + Code + CSS Test: tune close strategy, control mode, and state markers."
                code_signal=interactive_code
                test_css_source=interactive_test_css
                test_source_path="crates/ui/src/menu/dropdown_menu/styles.rs".to_string()
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
                            popover: ui::PopoverMotion {
                                initial_scale: 0.96,
                                offset_y_px: 14.0,
                                ..ui::PopoverMotion::default()
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
                        items=default_playground_items
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

            <Playground title="State Matrix (Default / Controlled / Disabled)" code_signal=matrix_code>
                <div class="docs-row">
                    <DropdownMenu
                        id_base="docs-dropdown-matrix-default".to_string()
                        items=vec![
                            "Duplicate".to_string(),
                            "Move".to_string(),
                            "Archive".to_string(),
                        ]
                        on_action=on_action
                        placement=ui_headless::PopoverPlacement::BottomStart
                        trigger_variant=ui::ButtonVariant::Secondary
                        trigger_size=ui::ButtonSize::Sm
                    >
                        "Default"
                    </DropdownMenu>

                    <DropdownMenu
                        id_base="docs-dropdown-matrix-controlled".to_string()
                        items=vec!["Rename".to_string(), "Move".to_string(), "Share".to_string()]
                        on_action=on_action
                        is_open=controlled_open
                        default_open=false
                        on_open_change=on_open_change
                        is_close_on_action=false
                        placement=ui_headless::PopoverPlacement::TopEnd
                        trigger_variant=ui::ButtonVariant::Secondary
                        trigger_size=ui::ButtonSize::Sm
                    >
                        "Controlled"
                    </DropdownMenu>

                    <DropdownMenu
                        id_base="docs-dropdown-matrix-disabled".to_string()
                        items=vec!["Duplicate".to_string(), "Archive".to_string()]
                        on_action=on_action
                        is_disabled=true
                        item_kinds=vec![MenuItemKind::Action, MenuItemKind::Action]
                        motion=DropdownMenuMotion::default()
                        class_name="docs-dropdown-custom".to_string()
                    >
                        "Disabled"
                    </DropdownMenu>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
pub(super) fn pagination() -> AnyView {
    let total_pages_options = ["10".to_string(), "20".to_string()];
    let siblings_options = ["0".to_string(), "1".to_string(), "2".to_string()];
    let boundaries_options = ["1".to_string(), "2".to_string()];

    let (showcase_page, set_showcase_page) = signal(1_usize);
    let (showcase_last_change, set_showcase_last_change) = signal(None::<usize>);
    let on_showcase_change =
        Callback::new(move |next: usize| set_showcase_last_change.set(Some(next)));

    let (workbench_total_pages_index, set_workbench_total_pages_index) = signal(Some(0_usize));
    let (workbench_siblings_index, set_workbench_siblings_index) = signal(Some(1_usize));
    let (workbench_boundaries_index, set_workbench_boundaries_index) = signal(Some(0_usize));
    let (workbench_disabled, set_workbench_disabled) = signal(false);
    let (workbench_enable_on_change, set_workbench_enable_on_change) = signal(true);
    let (workbench_custom_aria, set_workbench_custom_aria) = signal(true);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);
    let (workbench_page, set_workbench_page) = signal(3_usize);
    let (workbench_last_change, set_workbench_last_change) = signal(None::<usize>);

    let (matrix_first_page, set_matrix_first_page) = signal(1_usize);
    let (matrix_middle_page, set_matrix_middle_page) = signal(6_usize);
    let (matrix_disabled_page, set_matrix_disabled_page) = signal(1_usize);

    let workbench_total_pages = Signal::derive(move || {
        if workbench_total_pages_index.get().unwrap_or(0) == 1 {
            20
        } else {
            10
        }
    });
    let workbench_siblings =
        Signal::derive(move || match workbench_siblings_index.get().unwrap_or(1) {
            0 => 0_usize,
            2 => 2_usize,
            _ => 1_usize,
        });
    let workbench_boundaries = Signal::derive(move || {
        if workbench_boundaries_index.get().unwrap_or(0) == 1 {
            2
        } else {
            1
        }
    });
    let workbench_aria_label = Signal::derive(move || {
        if workbench_custom_aria.get() {
            "Workbench pagination".to_string()
        } else {
            String::new()
        }
    });
    let workbench_class_name = Signal::derive(move || {
        if workbench_custom_class.get() {
            "docs-pagination-custom".to_string()
        } else {
            String::new()
        }
    });

    Effect::new(move |_| {
        let total_pages = workbench_total_pages.get().max(1);
        let current = workbench_page.get();
        if current > total_pages {
            set_workbench_page.set(total_pages);
        }
    });

    let on_workbench_change = Callback::new(move |next: usize| {
        if !workbench_enable_on_change.get_untracked() {
            return;
        }
        set_workbench_last_change.set(Some(next));
    });

    let showcase_code = Signal::derive(move || {
        r#"let (page, set_page) = signal(1_usize);
let on_change = Callback::new(move |next: usize| { /* visible feedback */ });
<Pagination
  total_pages=12
  page=page
  set_page=set_page
  siblings=1
  boundaries=1
  on_change=on_change
  aria_label="Pagination nav".to_string()
/>"#
        .to_string()
    });

    let workbench_code = Signal::derive(move || {
        let on_change_expr = if workbench_enable_on_change.get() {
            "Some(on_workbench_change)"
        } else {
            "None"
        };
        format!(
            "<Pagination\n  total_pages={}\n  page=workbench_page\n  set_page=set_workbench_page\n  siblings={}\n  boundaries={}\n  disabled={}\n  on_change={on_change_expr}\n  aria_label={}\n  class_name={}\n/>",
            workbench_total_pages.get(),
            workbench_siblings.get(),
            workbench_boundaries.get(),
            bool_word(workbench_disabled.get()),
            rust_string_literal(&workbench_aria_label.get()),
            rust_string_literal(&workbench_class_name.get()),
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        format!(
            "PaginationActualConfig {{\n  total_pages: {},\n  page: workbench_page,\n  set_page: set_workbench_page,\n  siblings: {},\n  boundaries: {},\n  disabled: {},\n  on_change: {},\n  aria_label: {:?},\n  class_name: {:?},\n}}",
            workbench_total_pages.get(),
            workbench_siblings.get(),
            workbench_boundaries.get(),
            workbench_disabled.get(),
            if workbench_enable_on_change.get() {
                "Some"
            } else {
                "None"
            },
            workbench_aria_label.get(),
            workbench_class_name.get(),
        )
    });

    let matrix_code = Signal::derive(move || {
        r#"<Pagination total_pages=12 page=matrix_first_page set_page=set_matrix_first_page siblings=1 boundaries=1 />
<Pagination
  total_pages=12
  page=matrix_middle_page
  set_page=set_matrix_middle_page
  siblings=2
  boundaries=2
  aria_label="Middle page".to_string()
  class_name="docs-pagination-custom".to_string()
/>
<Pagination
  total_pages=1
  page=matrix_disabled_page
  set_page=set_matrix_disabled_page
  disabled=true
  on_change=on_workbench_change
/>"#
            .to_string()
    });

    view! {
        <ComponentPage
            title="Pagination"
            slug="pagination"
            group="Collections"
            description="Pagination control with real callback feedback and full API workbench coverage."
        >
            <Playground
                title="Hello World (Default API)"
                code_signal=showcase_code
                code_imports="use leptos::prelude::*;\nuse ui::Pagination;".to_string()
                test_source_path="components/pagination/src/view.rs".to_string()
            >
                <div class="docs-stack docs-stack--tight" data-slot="pagination-showcase-playground">
                    <Pagination
                        total_pages=12
                        page=showcase_page
                        set_page=set_showcase_page
                        siblings=1
                        boundaries=1
                        on_change=on_showcase_change
                        aria_label="Pagination nav".to_string()
                    />
                    <span class="ui-muted">
                        "page: " {move || showcase_page.get()}
                        " · last change: "
                        {move || showcase_last_change.get().map(|value| value.to_string()).unwrap_or_else(|| "None".to_string())}
                    </span>
                </div>
            </Playground>

            <Playground
                title="Workbench (Config + Live Actual Config)"
                code_signal=workbench_code
                code_imports="use leptos::prelude::*;\nuse ui::Pagination;".to_string()
                test_source_path="components/pagination/src/view.rs".to_string()
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="pagination-workbench-controls">
                        <div class="docs-search__label">"total_pages"</div>
                        <select
                            class="docs-search__input"
                            prop:value=move || workbench_total_pages_index.get().unwrap_or(0).to_string()
                            on:change=move |event| {
                                if let Ok(value) = event_target_value(&event).parse::<usize>() {
                                    set_workbench_total_pages_index.set(Some(value.min(1)));
                                }
                            }
                        >
                            {total_pages_options
                                .iter()
                                .enumerate()
                                .map(|(index, label)| view! { <option value=index.to_string()>{label.clone()}</option> })
                                .collect_view()}
                        </select>

                        <div class="docs-search__label">"siblings"</div>
                        <select
                            class="docs-search__input"
                            prop:value=move || workbench_siblings_index.get().unwrap_or(1).to_string()
                            on:change=move |event| {
                                if let Ok(value) = event_target_value(&event).parse::<usize>() {
                                    set_workbench_siblings_index.set(Some(value.min(2)));
                                }
                            }
                        >
                            {siblings_options
                                .iter()
                                .enumerate()
                                .map(|(index, label)| view! { <option value=index.to_string()>{label.clone()}</option> })
                                .collect_view()}
                        </select>

                        <div class="docs-search__label">"boundaries"</div>
                        <select
                            class="docs-search__input"
                            prop:value=move || workbench_boundaries_index.get().unwrap_or(0).to_string()
                            on:change=move |event| {
                                if let Ok(value) = event_target_value(&event).parse::<usize>() {
                                    set_workbench_boundaries_index.set(Some(value.min(1)));
                                }
                            }
                        >
                            {boundaries_options
                                .iter()
                                .enumerate()
                                .map(|(index, label)| view! { <option value=index.to_string()>{label.clone()}</option> })
                                .collect_view()}
                        </select>

                        <label class="docs-choice-row">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_disabled.get()
                                on:change=move |event| set_workbench_disabled.set(event_target_checked(&event))
                            />
                            <span>"disabled"</span>
                        </label>
                        <label class="docs-choice-row">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_enable_on_change.get()
                                on:change=move |event| set_workbench_enable_on_change.set(event_target_checked(&event))
                            />
                            <span>"enable on_change callback"</span>
                        </label>
                        <label class="docs-choice-row">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_custom_aria.get()
                                on:change=move |event| set_workbench_custom_aria.set(event_target_checked(&event))
                            />
                            <span>"custom aria_label"</span>
                        </label>
                        <label class="docs-choice-row">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_custom_class.get()
                                on:change=move |event| set_workbench_custom_class.set(event_target_checked(&event))
                            />
                            <span>"custom class_name"</span>
                        </label>
                    </div>
                }
            >
                <div class="docs-stack docs-stack--tight" data-slot="pagination-workbench-playground">
                    <Pagination
                        total_pages=workbench_total_pages.get()
                        page=workbench_page
                        set_page=set_workbench_page
                        siblings=workbench_siblings.get()
                        boundaries=workbench_boundaries.get()
                        disabled=workbench_disabled.get()
                        on_change=on_workbench_change
                        aria_label=workbench_aria_label.get()
                        class_name=workbench_class_name.get()
                    />
                    <span class="ui-muted">
                        "page: " {move || workbench_page.get()}
                        " · last_change: "
                        {move || workbench_last_change.get().map(|value| value.to_string()).unwrap_or_else(|| "None".to_string())}
                    </span>
                </div>
            </Playground>

            <Playground
                title="State Matrix (Window / Disabled / Callback Comparison)"
                code_signal=matrix_code
                code_imports="use leptos::prelude::*;\nuse ui::Pagination;".to_string()
                test_source_path="components/pagination/src/view.rs".to_string()
            >
                <div class="docs-row docs-row--wrap" data-slot="pagination-matrix-playground">
                    <div class="docs-stack docs-stack--tight">
                        <Pagination
                            total_pages=12
                            page=matrix_first_page
                            set_page=set_matrix_first_page
                            siblings=1
                            boundaries=1
                        />
                        <span class="ui-muted">"first window: " {move || matrix_first_page.get()}</span>
                    </div>

                    <div class="docs-stack docs-stack--tight">
                        <Pagination
                            total_pages=12
                            page=matrix_middle_page
                            set_page=set_matrix_middle_page
                            siblings=2
                            boundaries=2
                            aria_label="Middle page".to_string()
                            class_name="docs-pagination-custom".to_string()
                        />
                        <span class="ui-muted">"middle window: " {move || matrix_middle_page.get()}</span>
                    </div>

                    <div class="docs-stack docs-stack--tight">
                        <Pagination
                            total_pages=1
                            page=matrix_disabled_page
                            set_page=set_matrix_disabled_page
                            disabled=true
                            on_change=on_workbench_change
                        />
                        <span class="ui-muted">"disabled window: " {move || matrix_disabled_page.get()}</span>
                    </div>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
pub(super) fn tag_group() -> AnyView {
    fn default_workbench_tags() -> Vec<Tag> {
        vec![
            Tag::new("tag-rust", "Rust"),
            Tag::new("tag-leptos", "Leptos"),
            Tag::disabled("tag-a11y", "Accessibility"),
            Tag::new("tag-design", "Design tokens"),
        ]
    }

    let (showcase_tags, _set_showcase_tags) = signal(vec![
        Tag::new("tag-showcase-rust", "Rust"),
        Tag::new("tag-showcase-leptos", "Leptos"),
        Tag::new("tag-showcase-ui", "UI primitives"),
    ]);

    let (workbench_tags, set_workbench_tags) = signal(default_workbench_tags());
    let (workbench_disabled, set_workbench_disabled) = signal(false);
    let (workbench_surface_variant, set_workbench_surface_variant) = signal(false);
    let (workbench_large_size, set_workbench_large_size) = signal(false);
    let (workbench_custom_id_base, set_workbench_custom_id_base) = signal(true);
    let (workbench_show_description, set_workbench_show_description) = signal(true);
    let (workbench_show_error, set_workbench_show_error) = signal(true);
    let (workbench_force_invalid, set_workbench_force_invalid) = signal(false);
    let (workbench_required, set_workbench_required) = signal(true);
    let (workbench_external_aria_describedby, set_workbench_external_aria_describedby) =
        signal(false);
    let (workbench_custom_aria_label, set_workbench_custom_aria_label) = signal(false);
    let (workbench_custom_class_name, set_workbench_custom_class_name) = signal(false);
    let (workbench_zh_lang, set_workbench_zh_lang) = signal(false);
    let (workbench_rtl_dir, set_workbench_rtl_dir) = signal(false);
    let (workbench_remove_count, set_workbench_remove_count) = signal(0_u32);
    let (workbench_last_removed, set_workbench_last_removed) = signal(None::<String>);
    let (workbench_next_custom_tag, set_workbench_next_custom_tag) = signal(1_u32);

    let on_workbench_remove = Callback::new(move |tag: Tag| {
        let removed_label = tag.label.clone();
        set_workbench_tags.update(|tags| tags.retain(|item| item.id != tag.id));
        set_workbench_remove_count.update(|count| *count += 1);
        set_workbench_last_removed.set(Some(removed_label));
    });

    let on_add_custom_tag = Callback::new(move |_| {
        let index = workbench_next_custom_tag.get();
        set_workbench_tags.update(|tags| {
            tags.push(Tag::new(
                format!("tag-custom-{index}"),
                format!("Custom tag {index}"),
            ));
        });
        set_workbench_next_custom_tag.update(|next| *next += 1);
    });

    let on_reset_workbench_tags = Callback::new(move |_| {
        set_workbench_tags.set(default_workbench_tags());
        set_workbench_remove_count.set(0);
        set_workbench_last_removed.set(None);
    });

    let on_clear_workbench_tags = Callback::new(move |_| {
        set_workbench_tags.set(Vec::new());
    });

    let workbench_invalid =
        Signal::derive(move || workbench_force_invalid.get() || workbench_tags.get().is_empty());
    let workbench_required_signal = Signal::derive(move || workbench_required.get());
    let workbench_aria_describedby_signal = Signal::derive(move || {
        if workbench_external_aria_describedby.get() {
            Some("tag-group-external-help".to_string())
        } else {
            None
        }
    });

    let hello_code = Signal::derive(move || {
        r#"let (tags, _set_tags) = signal(vec![
  Tag::new("tag-showcase-rust", "Rust"),
  Tag::new("tag-showcase-leptos", "Leptos"),
  Tag::new("tag-showcase-ui", "UI primitives"),
]);

<TagGroup tags=tags label=Some("Project labels".to_string()) />"#
            .to_string()
    });

    let workbench_code = Signal::derive(move || {
        let variant = if workbench_surface_variant.get() {
            "TagVariant::Surface"
        } else {
            "TagVariant::Default"
        };
        let size = if workbench_large_size.get() {
            "TagSize::Lg"
        } else {
            "TagSize::Md"
        };
        let id_base = if workbench_custom_id_base.get() {
            "Some(\"docs-tag-group-workbench\".to_string())"
        } else {
            "None"
        };
        let label = "Some(\"Framework tags\".to_string())";
        let description = if workbench_show_description.get() {
            "Some(\"Remove chips and observe feedback\".to_string())"
        } else {
            "None"
        };
        let error = if workbench_show_error.get() {
            "Some(\"At least one tag is required\".to_string())"
        } else {
            "None"
        };
        let aria_describedby = if workbench_external_aria_describedby.get() {
            "Signal::derive(|| Some(\"tag-group-external-help\".to_string()))"
        } else {
            "Signal::derive(|| None::<String>)"
        };
        let aria_label = if workbench_custom_aria_label.get() {
            "Some(\"Selected framework tags\".to_string())"
        } else {
            "None"
        };
        let class_name = if workbench_custom_class_name.get() {
            rust_string_literal("docs-tag-group-workbench")
        } else {
            "None".to_string()
        };
        let lang = if workbench_zh_lang.get() {
            "Some(\"zh-CN\".to_string())"
        } else {
            "Some(\"en-US\".to_string())"
        };
        let dir = if workbench_rtl_dir.get() {
            "Some(ui_headless::A11yDirection::Rtl)"
        } else {
            "Some(ui_headless::A11yDirection::Ltr)"
        };

        let mut lines = vec![
            "let (tags, set_tags) = signal(vec![ ... ]);".to_string(),
            "let on_remove = Callback::new(move |tag: Tag| {".to_string(),
            "  set_tags.update(|items| items.retain(|item| item.id != tag.id));".to_string(),
            "});".to_string(),
            "<TagGroup".to_string(),
            "  tags=tags".to_string(),
            format!("  disabled={}", bool_word(workbench_disabled.get())),
            "  on_remove=on_remove".to_string(),
            format!("  variant={variant}"),
            format!("  size={size}"),
            format!("  id_base={id_base}"),
            format!("  label={label}"),
            format!("  description={description}"),
            format!("  error={error}"),
            format!(
                "  invalid=Signal::derive(|| {})",
                bool_word(workbench_invalid.get())
            ),
            format!(
                "  required=Signal::derive(|| {})",
                bool_word(workbench_required_signal.get())
            ),
            format!("  aria_describedby={aria_describedby}"),
            format!("  aria_label={aria_label}"),
            format!("  class_name={class_name}"),
            format!("  lang={lang}"),
            format!("  dir={dir}"),
        ];
        push_line_when(&mut lines, true, "/>".to_string());
        lines.join("\n")
    });

    let workbench_actual_config = Signal::derive(move || {
        let tags_repr = {
            let tags = workbench_tags.get();
            if tags.is_empty() {
                "[]".to_string()
            } else {
                format!(
                    "[{}]",
                    tags.into_iter()
                        .map(|tag| format!(
                            "{{ id: {:?}, label: {:?}, disabled: {} }}",
                            tag.id,
                            tag.label,
                            bool_word(tag.disabled)
                        ))
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            }
        };

        let id_base = if workbench_custom_id_base.get() {
            Some("docs-tag-group-workbench".to_string())
        } else {
            None
        };
        let label = Some("Framework tags".to_string());
        let description = if workbench_show_description.get() {
            Some("Remove chips and observe feedback".to_string())
        } else {
            None
        };
        let error = if workbench_show_error.get() {
            Some("At least one tag is required".to_string())
        } else {
            None
        };
        let aria_describedby = if workbench_external_aria_describedby.get() {
            Some("tag-group-external-help".to_string())
        } else {
            None
        };
        let aria_label = if workbench_custom_aria_label.get() {
            Some("Selected framework tags".to_string())
        } else {
            None
        };
        let class_name = if workbench_custom_class_name.get() {
            Some("docs-tag-group-workbench".to_string())
        } else {
            None
        };
        let lang = if workbench_zh_lang.get() {
            Some("zh-CN".to_string())
        } else {
            Some("en-US".to_string())
        };
        let dir = if workbench_rtl_dir.get() {
            "Some(A11yDirection::Rtl)"
        } else {
            "Some(A11yDirection::Ltr)"
        };
        let variant = if workbench_surface_variant.get() {
            "TagVariant::Surface"
        } else {
            "TagVariant::Default"
        };
        let size = if workbench_large_size.get() {
            "TagSize::Lg"
        } else {
            "TagSize::Md"
        };
        let last_removed = workbench_last_removed
            .get()
            .unwrap_or_else(|| "None".to_string());

        format!(
            "TagGroupActualConfig {{\n  tags: {tags_repr},\n  disabled: {},\n  on_remove: \"count={}, last={}\",\n  variant: {variant},\n  size: {size},\n  id_base: {id_base:?},\n  label: {label:?},\n  description: {description:?},\n  error: {error:?},\n  invalid: {},\n  required: {},\n  aria_describedby: {aria_describedby:?},\n  aria_label: {aria_label:?},\n  class_name: {class_name:?},\n  lang: {lang:?},\n  dir: {dir},\n}}",
            bool_word(workbench_disabled.get()),
            workbench_remove_count.get(),
            last_removed,
            bool_word(workbench_invalid.get()),
            bool_word(workbench_required_signal.get()),
        )
    });

    let (matrix_default_tags, _set_matrix_default_tags) = signal(vec![
        Tag::new("tag-matrix-rust", "Rust"),
        Tag::new("tag-matrix-wasm", "WASM"),
        Tag::new("tag-matrix-a11y", "A11y"),
    ]);
    let (matrix_surface_tags, _set_matrix_surface_tags) = signal(vec![
        Tag::new("tag-matrix-design", "Design"),
        Tag::new("tag-matrix-theme", "Theme"),
        Tag::disabled("tag-matrix-tokens", "Tokens"),
    ]);
    let (matrix_invalid_tags, _set_matrix_invalid_tags) = signal(Vec::<Tag>::new());

    let matrix_code = Signal::derive(move || {
        r#"<TagGroup
  tags=default_tags
  variant=TagVariant::Default
  size=TagSize::Md
  label=Some("Default".to_string())
/>
<TagGroup
  tags=surface_tags
  disabled=true
  variant=TagVariant::Surface
  size=TagSize::Lg
  label=Some("Disabled Surface".to_string())
/>
<TagGroup
  tags=invalid_tags
  variant=TagVariant::Default
  size=TagSize::Md
  label=Some("Required".to_string())
  error=Some("At least one tag is required".to_string())
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
            description="TagGroup playground follows showcase/workbench/matrix with full API coverage and callback feedback."
        >
            <Playground
                title="Hello World (Default TagGroup)"
                code_signal=hello_code
                test_source_path="components/tag/src/group/view.rs".to_string()
            >
                <TagGroup
                    tags=showcase_tags
                    label="Project labels".to_string()
                />
            </Playground>

            <Playground
                title="Workbench (All API + Actual Config)"
                code_signal=workbench_code
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="tag-group-workbench-controls">
                        <Switch checked=workbench_disabled set_checked=set_workbench_disabled>
                            "disabled"
                        </Switch>
                        <Switch
                            checked=workbench_surface_variant
                            set_checked=set_workbench_surface_variant
                        >
                            "Surface variant"
                        </Switch>
                        <Switch checked=workbench_large_size set_checked=set_workbench_large_size>
                            "Large size"
                        </Switch>
                        <Switch
                            checked=workbench_custom_id_base
                            set_checked=set_workbench_custom_id_base
                        >
                            "Custom id_base"
                        </Switch>
                        <Switch
                            checked=workbench_show_description
                            set_checked=set_workbench_show_description
                        >
                            "description"
                        </Switch>
                        <Switch checked=workbench_show_error set_checked=set_workbench_show_error>
                            "error"
                        </Switch>
                        <Switch checked=workbench_force_invalid set_checked=set_workbench_force_invalid>
                            "force invalid"
                        </Switch>
                        <Switch checked=workbench_required set_checked=set_workbench_required>
                            "required"
                        </Switch>
                        <Switch
                            checked=workbench_external_aria_describedby
                            set_checked=set_workbench_external_aria_describedby
                        >
                            "aria_describedby"
                        </Switch>
                        <Switch
                            checked=workbench_custom_aria_label
                            set_checked=set_workbench_custom_aria_label
                        >
                            "aria_label"
                        </Switch>
                        <Switch
                            checked=workbench_custom_class_name
                            set_checked=set_workbench_custom_class_name
                        >
                            "class_name"
                        </Switch>
                        <Switch checked=workbench_zh_lang set_checked=set_workbench_zh_lang>
                            "lang zh-CN"
                        </Switch>
                        <Switch checked=workbench_rtl_dir set_checked=set_workbench_rtl_dir>
                            "dir RTL"
                        </Switch>
                        <div class="docs-row docs-row--tight">
                            <ui::Button
                                variant=ui::ButtonVariant::Secondary
                                on_press=on_add_custom_tag
                            >
                                "Add tag"
                            </ui::Button>
                            <ui::Button
                                variant=ui::ButtonVariant::Secondary
                                on_press=on_reset_workbench_tags
                            >
                                "Reset tags"
                            </ui::Button>
                            <ui::Button
                                variant=ui::ButtonVariant::Secondary
                                on_press=on_clear_workbench_tags
                            >
                                "Clear tags"
                            </ui::Button>
                        </div>
                    </div>
                }
                test_source_path="components/tag/src/group/view.rs".to_string()
            >
                <div class="docs-stack docs-stack--tight" data-slot="tag-group-workbench-display">
                    <Show when=move || workbench_external_aria_describedby.get()>
                        <p id="tag-group-external-help" class="ui-muted">
                            "external help text wired by aria_describedby"
                        </p>
                    </Show>
                    <TagGroup
                        tags=workbench_tags
                        disabled=workbench_disabled.get()
                        on_remove=on_workbench_remove
                        variant=if workbench_surface_variant.get() {
                            ui::TagVariant::Surface
                        } else {
                            ui::TagVariant::Default
                        }
                        size=if workbench_large_size.get() {
                            ui::TagSize::Lg
                        } else {
                            ui::TagSize::Md
                        }
                        id_base=if workbench_custom_id_base.get() {
                            "docs-tag-group-workbench".to_string()
                        } else {
                            String::new()
                        }
                        label="Framework tags".to_string()
                        description=if workbench_show_description.get() {
                            "Remove chips and observe feedback".to_string()
                        } else {
                            String::new()
                        }
                        error=if workbench_show_error.get() {
                            "At least one tag is required".to_string()
                        } else {
                            String::new()
                        }
                        invalid=workbench_invalid
                        required=workbench_required_signal
                        aria_describedby=workbench_aria_describedby_signal
                        aria_label=if workbench_custom_aria_label.get() {
                            "Selected framework tags".to_string()
                        } else {
                            String::new()
                        }
                        class_name=if workbench_custom_class_name.get() {
                            "docs-tag-group-workbench".to_string()
                        } else {
                            String::new()
                        }
                        lang=if workbench_zh_lang.get() {
                            "zh-CN".to_string()
                        } else {
                            "en-US".to_string()
                        }
                        dir=if workbench_rtl_dir.get() {
                            ui_headless::A11yDirection::Rtl
                        } else {
                            ui_headless::A11yDirection::Ltr
                        }
                    />
                    <span class="ui-muted" data-slot="tag-group-workbench-feedback">
                        "on_remove count: " {move || workbench_remove_count.get()}
                        " · last removed: "
                        {move || {
                            workbench_last_removed
                                .get()
                                .unwrap_or_else(|| "None".to_string())
                        }}
                        " · remaining: " {move || workbench_tags.get().len()}
                    </span>
                </div>
            </Playground>

            <Playground
                title="State Matrix (Default / Surface / Required)"
                code_signal=matrix_code
                test_source_path="components/tag/src/group/view.rs".to_string()
            >
                <div class="docs-row" data-slot="tag-group-state-matrix">
                    <div class="docs-stack">
                        <TagGroup
                            tags=matrix_default_tags
                            variant=ui::TagVariant::Default
                            size=ui::TagSize::Md
                            label="Default".to_string()
                            description="Removable in normal state".to_string()
                            on_remove=Callback::new(move |_| {})
                        />
                    </div>
                    <div class="docs-stack">
                        <TagGroup
                            tags=matrix_surface_tags
                            disabled=true
                            variant=ui::TagVariant::Surface
                            size=ui::TagSize::Lg
                            label="Disabled Surface".to_string()
                            description="Large + disabled visual variant".to_string()
                        />
                    </div>
                    <div class="docs-stack">
                        <TagGroup
                            tags=matrix_invalid_tags
                            variant=ui::TagVariant::Default
                            size=ui::TagSize::Md
                            label="Required".to_string()
                            error="At least one tag is required".to_string()
                            invalid=Signal::derive(|| true)
                            required=Signal::derive(|| true)
                        />
                    </div>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
