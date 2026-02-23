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
    Switch, Tabs, TabsKeyboardActivation, Tag, TagGroup, TagGroupItem,
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

#[path = "collections/accordion.rs"]
mod accordion;
#[path = "collections/autocomplete.rs"]
mod autocomplete;
#[path = "collections/combo_box.rs"]
mod combo_box;
#[path = "collections/disclosure.rs"]
mod disclosure;
#[path = "collections/dropdown_menu.rs"]
mod dropdown_menu;
#[path = "collections/list.rs"]
mod list;
#[path = "collections/menu.rs"]
mod menu;
#[path = "collections/menu_trigger.rs"]
mod menu_trigger;
#[path = "collections/pagination.rs"]
mod pagination;
#[path = "collections/select.rs"]
mod select;
#[path = "collections/tabs.rs"]
mod tabs;
#[path = "collections/tag_group.rs"]
mod tag_group;

pub(super) use accordion::accordion;
pub(super) use autocomplete::autocomplete;
pub(super) use combo_box::combo_box;
pub(super) use disclosure::disclosure;
pub(super) use dropdown_menu::dropdown_menu;
pub(super) use list::list;
pub(super) use menu::menu;
pub(super) use menu_trigger::menu_trigger;
pub(super) use pagination::pagination;
pub(super) use select::select;
pub(super) use tabs::tabs;
pub(super) use tag_group::tag_group;
