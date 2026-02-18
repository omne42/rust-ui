use crate::step_list::{
    StepListItem, StepListItemStateInput, StepListOrientation, StepListSize, StepListStateInput,
    logic::{self},
};
use leptos::prelude::*;
use ui_headless as headless;
use ui_headless::{
    A11yDirection, resolve_step_list_next_index, step_list_item_contract, step_list_root_a11y_attrs,
};

const TRACE_COMPONENT: &str = "step_list";
const SLOT_ROOT: &str = "step-list";
const SLOT_ITEM: &str = "step-list-item";
const SLOT_BUTTON: &str = "step-list-button";
const SLOT_MARKER: &str = "step-list-marker";
const SLOT_CONTENT: &str = "step-list-content";
const SLOT_LABEL: &str = "step-list-label";
const SLOT_DESCRIPTION: &str = "step-list-description";
const SLOT_CONNECTOR: &str = "step-list-connector";
const CLASS_BUTTON: &str = "ui-step-list__button";
const CLASS_MARKER: &str = "ui-step-list__marker";
const CLASS_CONTENT: &str = "ui-step-list__content";
const CLASS_LABEL: &str = "ui-step-list__label";
const CLASS_DESCRIPTION: &str = "ui-step-list__description";
const CLASS_CONNECTOR: &str = "ui-step-list__connector";

#[derive(Clone)]
struct StepListInteractionModel {
    items: Memo<Vec<StepListItem>>,
    orientation: StepListOrientation,
    selected_state_value: Signal<Option<usize>>,
    selected_state_request_change: Callback<Option<usize>>,
    trace: Option<ui_headless::UiTrace>,
}

fn format_trace_index(value: Option<usize>) -> String {
    value.map_or_else(|| "none".to_string(), |index| index.to_string())
}

fn emit_selection_trace(
    trace: Option<ui_headless::UiTrace>,
    source: &'static str,
    index: usize,
    prev: Option<usize>,
    next: Option<usize>,
) {
    if let Some(trace) = trace {
        trace.emit(
            TRACE_COMPONENT,
            ui_headless::UiTraceEventKind::Note {
                message: format!(
                    "intent=selection;source={source};index={index};prev={};next={}",
                    format_trace_index(prev),
                    format_trace_index(next),
                ),
            },
        );
    }
}

fn render_item_description(description: Option<String>) -> AnyView {
    match description {
        Some(description) => view! {
            <span class=CLASS_DESCRIPTION data-slot=SLOT_DESCRIPTION>
                {description}
            </span>
        }
        .into_any(),
        None => ().into_any(),
    }
}

fn render_step_list_item(
    index: usize,
    item_count: usize,
    item: StepListItem,
    item_state: crate::step_list::StepListItemState,
    item_contract: ui_headless::StepListItemContract,
    id_base_for_items: &str,
    interaction: StepListInteractionModel,
) -> AnyView {
    let marker_text = item_state.marker_number.to_string();
    let item_id = item.id;
    let item_label = item.label;
    let item_label_for_aria = item_label.clone();
    let item_description = item.description;
    let step_button_id = format!("{id_base_for_items}-{item_id}");
    let click_interaction = interaction.clone();
    let keydown_interaction = interaction;

    view! {
        <li
            class=item_state.status_class
            data-slot=SLOT_ITEM
            data-index=index
            data-id=item_id
            data-status=item_state.status_attr
            data-current=item_state.is_current.then_some("true")
            data-completed=item_state.is_completed.then_some("true")
            data-disabled=item_state.is_disabled.then_some("true")
            data-pending=item_state.is_pending.then_some("true")
            data-selectable=item_state.is_selectable.then_some("true")
        >
            <button
                type="button"
                id=step_button_id
                class=CLASS_BUTTON
                aria-current=item_contract.attrs.aria_current
                aria-label=item_label_for_aria
                aria-disabled=item_contract.attrs.aria_disabled
                tabindex=item_contract.attrs.tabindex
                disabled=item_contract.state.is_disabled
                data-slot=SLOT_BUTTON
                data-status=item_state.status_attr
                on:click=move |_| {
                    if item_contract.state.is_selectable {
                        let next = Some(index);
                        emit_selection_trace(
                            click_interaction.trace,
                            "pointer",
                            index,
                            click_interaction.selected_state_value.get_untracked(),
                            next,
                        );
                        click_interaction.selected_state_request_change.run(next);
                    }
                }
                on:keydown=move |ev| {
                    if item_contract.state.is_disabled {
                        return;
                    }

                    let items = keydown_interaction.items.get_untracked();
                    let key = ev.key();
                    if let Some(next) = resolve_step_list_next_index(
                        &items,
                        keydown_interaction.orientation,
                        index,
                        key.as_str(),
                    ) {
                        let next_selection = Some(next);
                        emit_selection_trace(
                            keydown_interaction.trace,
                            "keyboard",
                            index,
                            keydown_interaction.selected_state_value.get_untracked(),
                            next_selection,
                        );
                        keydown_interaction.selected_state_request_change.run(Some(next));
                        ev.prevent_default();
                    }
                }
            >
                <span class=CLASS_MARKER data-slot=SLOT_MARKER aria-hidden="true">
                    {marker_text}
                </span>
                <span class=CLASS_CONTENT data-slot=SLOT_CONTENT>
                    <span class=CLASS_LABEL data-slot=SLOT_LABEL>
                        {item_label}
                    </span>
                    {render_item_description(item_description)}
                </span>
            </button>
            <span
                class=CLASS_CONNECTOR
                data-slot=SLOT_CONNECTOR
                data-last=(index + 1 >= item_count).then_some("true")
                aria-hidden="true"
            ></span>
        </li>
    }
    .into_any()
}

fn render_step_list_items(
    normalized_items: Vec<StepListItem>,
    selected_index: Option<usize>,
    completed_indices: std::collections::BTreeSet<usize>,
    is_disabled: bool,
    id_base_for_items: &str,
    interaction: StepListInteractionModel,
) -> AnyView {
    let item_count = normalized_items.len();
    let first_enabled_index = logic::first_enabled_index(&normalized_items);

    normalized_items
        .into_iter()
        .enumerate()
        .map(|(index, item)| {
            let is_completed = completed_indices.contains(&index)
                || selected_index.is_some_and(|selected| index < selected);

            let item_state = logic::resolve_item_state(StepListItemStateInput {
                index,
                selected_index,
                completed: is_completed,
                disabled: is_disabled || item.disabled,
            });

            let item_contract = step_list_item_contract(ui_headless::StepListItemA11yInput {
                index,
                selected_index,
                first_enabled_index,
                is_current: item_state.is_current,
                is_disabled: item_state.is_disabled,
                is_selectable: item_state.is_selectable,
            });

            render_step_list_item(
                index,
                item_count,
                item,
                item_state,
                item_contract,
                id_base_for_items,
                interaction.clone(),
            )
        })
        .collect_view()
        .into_any()
}

#[component]
pub fn StepList(
    steps: ReadSignal<Vec<StepListItem>>,
    #[prop(optional)] orientation: StepListOrientation,
    #[prop(optional)] size: StepListSize,
    #[prop(optional)] is_emphasized: bool,
    #[prop(optional)] is_disabled: bool,
    #[prop(optional)] selected_index: Option<Signal<Option<usize>>>,
    #[prop(optional)] default_selected_index: Option<usize>,
    #[prop(optional)] completed_indices: Vec<usize>,
    #[prop(optional)] on_selected_index_change: Option<Callback<Option<usize>>>,
    #[prop(optional, into)] id_base: Option<String>,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional, into)] lang: Option<String>,
    #[prop(optional)] dir: Option<A11yDirection>,
) -> impl IntoView {
    let id_base = logic::normalize_id_base(id_base);
    let root_id = id_base.clone();
    let id_base_for_items = id_base.clone();

    let items = Memo::new(move |_| logic::normalize_items(steps.get()));

    let selection_axis = logic::normalize_selection_axis(logic::StepListSelectionAxisInput {
        selected_index,
        default_selected_index,
        on_selected_index_change,
        item_count: items.get_untracked().len(),
    });

    let selected_state = headless::use_controllable_state(
        selection_axis.selected_index,
        Some(selection_axis.default_selected_index),
        selection_axis.on_selected_index_change,
    );
    let selected_state_request_change = selected_state.request_change;
    let selected_state_value = selected_state.value;
    let trace = ui_headless::use_ui_trace();

    let completed_indices = StoredValue::new(completed_indices);

    let (aria_label, has_custom_aria_label) = logic::normalize_aria_label(aria_label);
    let root_a11y = step_list_root_a11y_attrs(aria_label, lang, dir);
    let root_role = root_a11y.role;
    let root_aria_label = root_a11y.aria_label;
    let root_lang = root_a11y.lang;
    let root_dir = root_a11y.dir;

    let class_name = logic::normalize_optional_text(class_name);
    let has_custom_class_name = class_name.is_some();
    let class_name = StoredValue::new(class_name);

    let state = Memo::new(move |_| {
        let items = items.get();
        let selected_index = logic::resolve_selected_index(&items, selected_state.value.get());
        let completed_indices =
            logic::normalize_completed_indices(items.len(), completed_indices.get_value());

        let completed_count = items
            .iter()
            .enumerate()
            .filter(|(index, item)| {
                !item.disabled
                    && (completed_indices.contains(index)
                        || selected_index.is_some_and(|selected| *index < selected))
            })
            .count();

        let disabled_count = items.iter().filter(|item| item.disabled).count();

        logic::resolve_state(StepListStateInput {
            orientation,
            size,
            emphasized: is_emphasized,
            disabled: is_disabled,
            item_count: items.len(),
            selected_index,
            completed_count,
            disabled_count,
            has_custom_aria_label,
            has_custom_class_name,
        })
    });

    let class = Memo::new(move |_| logic::compose_class_name(class_name.get_value(), state.get()));
    let interaction_model = StepListInteractionModel {
        items,
        orientation,
        selected_state_value,
        selected_state_request_change,
        trace,
    };

    view! {
        <ol
            id=root_id
            class=move || class.get()
            role=root_role
            aria-label=root_aria_label
            lang=root_lang.clone()
            dir=root_dir
            data-slot=SLOT_ROOT
            data-orientation=move || state.get().orientation_attr
            data-size=move || state.get().size_attr
            data-state=move || state.get().data_state_attr
            data-disabled=move || state.get().is_disabled.then_some("true")
            data-empty=move || state.get().is_empty.then_some("true")
            data-emphasized=move || state.get().is_emphasized.then_some("true")
            data-count=move || state.get().item_count.to_string()
            data-selected-index=move || state.get().selected_index.map(|index| index.to_string())
            data-completed-count=move || state.get().completed_count.to_string()
            data-disabled-count=move || state.get().disabled_count.to_string()
            data-has-selection=move || state.get().has_selection.then_some("true")
            data-has-completed=move || state.get().has_completed_steps.then_some("true")
            data-emphasis-source=move || state.get().emphasis_attr
            data-aria-source=move || state.get().aria_source_attr
            data-custom-class=move || state.get().has_custom_class_name.then_some("true")
            data-class-source=move || state.get().class_source_attr
        >
            {move || {
                let normalized_items = items.get();
                let selected_index = logic::resolve_selected_index(
                    &normalized_items,
                    selected_state_value.get(),
                );
                let completed_indices =
                    logic::normalize_completed_indices(normalized_items.len(), completed_indices.get_value());

                render_step_list_items(
                    normalized_items,
                    selected_index,
                    completed_indices,
                    is_disabled,
                    &id_base_for_items,
                    interaction_model.clone(),
                )
            }}
        </ol>
    }
}
