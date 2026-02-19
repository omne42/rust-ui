use super::{
    DisclosureGroupStateInput,
    logic::{self, DisclosureGroupSelectionMode},
};
use crate::accordion::{Accordion, AccordionItem, AccordionMotion, AccordionSelectionMode};
use leptos::{children::ChildrenFragment as Children, prelude::*};
use std::collections::BTreeSet;
use ui_headless as overlay_open;

#[component]
pub fn DisclosureGroup(
    labels: Vec<String>,
    id_base: String,
    #[prop(optional)] expanded_indices: Option<Signal<BTreeSet<usize>>>,
    #[prop(optional)] default_expanded_indices: Option<BTreeSet<usize>>,
    #[prop(optional)] on_expanded_change: Option<Callback<BTreeSet<usize>>>,
    #[prop(optional)] selection_mode: DisclosureGroupSelectionMode,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] disabled_indices: Vec<usize>,
    #[prop(optional)] motion: AccordionMotion,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
    children: Children,
) -> impl IntoView {
    let panels = children().nodes;
    let item_count = labels.len().min(panels.iter().len());
    let labels = labels.into_iter().take(item_count);
    let panels = panels.into_iter().take(item_count);
    let has_controlled_expanded_indices = expanded_indices.is_some();
    let has_default_expanded_indices = default_expanded_indices.is_some();
    let expanded_axis_state = logic::resolve_expanded_axis_state(
        has_controlled_expanded_indices,
        has_default_expanded_indices,
    );

    let default_expanded_indices = logic::normalize_expanded_indices(
        selection_mode,
        &default_expanded_indices.unwrap_or_default(),
        item_count,
    );

    let expanded_state = overlay_open::use_controllable_state(
        expanded_indices,
        Some(default_expanded_indices),
        on_expanded_change,
    );

    let expanded_indices = Memo::new({
        let expanded_indices = expanded_state.value;
        move |_| {
            logic::normalize_expanded_indices(selection_mode, &expanded_indices.get(), item_count)
        }
    });

    let request_expanded_change = {
        let request_expanded_change = expanded_state.request_change;
        Callback::new(move |next: BTreeSet<usize>| {
            let next = logic::normalize_expanded_indices(selection_mode, &next, item_count);
            request_expanded_change.run(next);
        })
    };

    let expanded_signal: Signal<BTreeSet<usize>> = Signal::derive(move || expanded_indices.get());

    let (aria_label, has_custom_aria_label) = logic::normalize_aria_label(aria_label);

    let class_name = logic::normalize_optional_text(class_name);
    let has_custom_class_name = class_name.is_some();
    let class_name = StoredValue::new(class_name);

    let disabled_indices = std::sync::Arc::new(disabled_indices);
    let has_per_item_disabled = !disabled_indices.is_empty();

    let state = Memo::new(move |_| {
        logic::resolve_state(DisclosureGroupStateInput {
            selection_mode,
            item_count,
            expanded_count: expanded_indices.get().len(),
            disabled,
            has_disabled_items: disabled || has_per_item_disabled,
            has_custom_aria_label,
            has_custom_class_name,
        })
    });

    let class = Memo::new(move |_| logic::compose_class_name(class_name.get_value(), state.get()));
    let motion_source = if motion == AccordionMotion::default() {
        "default"
    } else {
        "custom"
    };
    let custom_motion = (motion != AccordionMotion::default()).then_some("true");

    let accordion_selection_mode = match selection_mode {
        DisclosureGroupSelectionMode::Single => AccordionSelectionMode::Single,
        DisclosureGroupSelectionMode::Multiple => AccordionSelectionMode::Multiple,
    };
    let accordion_items = labels
        .zip(panels)
        .enumerate()
        .map({
            let disabled_indices = disabled_indices.clone();
            move |(index, (label, panel))| {
                let is_item_disabled = disabled_indices.contains(&index);
                let item_open = Signal::derive(move || expanded_signal.get().contains(&index));
                let on_item_open_change = Callback::new(move |is_open: bool| {
                    let mut next = expanded_signal.get_untracked();
                    if is_open {
                        next.insert(index);
                    } else {
                        next.remove(&index);
                    }
                    request_expanded_change.run(next);
                });
                view! {
                    <AccordionItem
                        key=index
                        label=label
                        open=item_open
                        on_open_change=on_item_open_change
                        is_disabled=is_item_disabled
                    >
                        {panel}
                    </AccordionItem>
                }
            }
        })
        .collect_view();

    view! {
        <div
            class=move || class.get()
            data-slot="disclosure-group"
            data-selection-mode=move || state.get().selection_mode_attr
            data-state=move || state.get().data_state_attr
            data-empty=move || state.get().is_empty.then_some("true")
            data-has-items=move || state.get().has_items.then_some("true")
            data-expanded-count=move || state.get().expanded_count.to_string()
            data-all-collapsed=move || (!state.get().has_expanded_items).then_some("true")
            data-multiple-expanded=move || state.get().has_multiple_expanded.then_some("true")
            data-disabled=move || state.get().is_disabled.then_some("true")
            data-has-disabled-items=move || state.get().has_disabled_items.then_some("true")
            data-aria-source=move || state.get().aria_source_attr
            data-expanded-control-mode=expanded_axis_state.control_mode_attr
            data-expanded-controlled=expanded_axis_state.is_controlled.then_some("true")
            data-expanded-uncontrolled=(!expanded_axis_state.is_controlled).then_some("true")
            data-default-expanded-source=expanded_axis_state.default_expanded_source_attr
            data-custom-class=move || state.get().has_custom_class_name.then_some("true")
            data-class-source=move || state.get().class_source_attr
            data-motion-source=motion_source
            data-custom-motion=custom_motion
            role="group"
            aria-label=aria_label
        >
            <div class="ui-disclosure-group__list" data-slot="disclosure-group-list">
                <Accordion
                    id_base=id_base
                    selection_mode=accordion_selection_mode
                    is_disabled=disabled
                    motion=motion
                    class_name="ui-disclosure-group__accordion".to_string()
                >
                    {accordion_items}
                </Accordion>
            </div>
        </div>
    }
}
