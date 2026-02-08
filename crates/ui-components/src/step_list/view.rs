use crate::{
    overlay_open,
    step_list::{
        StepListItem, StepListItemStateInput, StepListOrientation, StepListSize,
        StepListStateInput,
        logic::{self},
    },
};
use leptos::prelude::*;

#[component]
pub fn StepList(
    steps: ReadSignal<Vec<StepListItem>>,
    #[prop(optional)] orientation: StepListOrientation,
    #[prop(optional)] size: StepListSize,
    #[prop(optional)] emphasized: bool,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] selected_index: Option<Signal<Option<usize>>>,
    #[prop(optional)] default_selected_index: Option<usize>,
    #[prop(optional)] completed_indices: Vec<usize>,
    #[prop(optional)] on_selected_change: Option<Callback<Option<usize>>>,
    #[prop(optional, into)] id_base: Option<String>,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
) -> impl IntoView {
    let id_base =
        logic::normalize_optional_text(id_base).unwrap_or_else(|| "ui-step-list".to_string());
    let root_id = id_base.clone();
    let id_base_for_items = id_base.clone();

    let items = Memo::new(move |_| logic::normalize_items(steps.get()));

    let default_selected_index =
        logic::sanitize_index(default_selected_index, items.get_untracked().len());

    let selected_state = overlay_open::use_controllable_state(
        selected_index,
        Some(default_selected_index),
        on_selected_change,
    );
    let selected_state_request_change = selected_state.request_change;

    let completed_indices = StoredValue::new(completed_indices);

    let (aria_label, has_custom_aria_label) = logic::normalize_aria_label(aria_label);

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
            emphasized,
            disabled,
            item_count: items.len(),
            selected_index,
            completed_count,
            disabled_count,
            has_custom_aria_label,
            has_custom_class_name,
        })
    });

    let class = Memo::new(move |_| logic::compose_class_name(class_name.get_value(), state.get()));

    view! {
        <ol
            id=root_id
            class=move || class.get()
            role="list"
            aria-label=aria_label
            data-slot="step-list"
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
                let item_count = normalized_items.len();
                let selected_index =
                    logic::resolve_selected_index(&normalized_items, selected_state.value.get());
                let first_enabled_index = logic::first_enabled_index(&normalized_items);
                let completed_indices =
                    logic::normalize_completed_indices(item_count, completed_indices.get_value());

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
                            disabled: disabled || item.disabled,
                        });

                        let marker_text = item_state.marker_number.to_string();
                        let item_id = item.id;
                        let item_label = item.label;
                        let item_label_for_aria = item_label.clone();
                        let item_description = item.description;
                        let step_button_id = format!("{}-{}", id_base_for_items, item_id);

                        view! {
                            <li
                                class=item_state.status_class
                                data-slot="step-list-item"
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
                                    class="ui-step-list__button"
                                    aria-current=item_state.is_current.then_some("step")
                                    aria-label=item_label_for_aria
                                    aria-disabled=item_state.is_disabled.then_some("true")
                                    tabindex=if item_state.is_disabled {
                                        -1
                                    } else if item_state.is_current
                                        || (selected_index.is_none() && first_enabled_index == Some(index))
                                    {
                                        0
                                    } else {
                                        -1
                                    }
                                    disabled=item_state.is_disabled
                                    data-slot="step-list-button"
                                    data-status=item_state.status_attr
                                    on:click=move |_| {
                                        if item_state.is_selectable {
                                            selected_state_request_change.run(Some(index));
                                        }
                                    }
                                    on:keydown=move |ev| {
                                        if item_state.is_disabled {
                                            return;
                                        }

                                        let items = items.get_untracked();
                                        let next = match ev.key().as_str() {
                                            "ArrowRight" if orientation == StepListOrientation::Horizontal => {
                                                logic::next_enabled_index(&items, index, 1)
                                            }
                                            "ArrowLeft" if orientation == StepListOrientation::Horizontal => {
                                                logic::next_enabled_index(&items, index, -1)
                                            }
                                            "ArrowDown" if orientation == StepListOrientation::Vertical => {
                                                logic::next_enabled_index(&items, index, 1)
                                            }
                                            "ArrowUp" if orientation == StepListOrientation::Vertical => {
                                                logic::next_enabled_index(&items, index, -1)
                                            }
                                            "Home" => logic::first_enabled_index(&items),
                                            "End" => logic::last_enabled_index(&items),
                                            _ => None,
                                        };

                                        if let Some(next) = next {
                                            selected_state_request_change.run(Some(next));
                                            ev.prevent_default();
                                        }
                                    }
                                >
                                    <span
                                        class="ui-step-list__marker"
                                        data-slot="step-list-marker"
                                        aria-hidden="true"
                                    >
                                        {marker_text}
                                    </span>
                                    <span class="ui-step-list__content" data-slot="step-list-content">
                                        <span class="ui-step-list__label" data-slot="step-list-label">
                                            {item_label}
                                        </span>
                                        {item_description.map(|description| {
                                            view! {
                                                <span
                                                    class="ui-step-list__description"
                                                    data-slot="step-list-description"
                                                >
                                                    {description}
                                                </span>
                                            }
                                        })}
                                    </span>
                                </button>
                                <span
                                    class="ui-step-list__connector"
                                    data-slot="step-list-connector"
                                    data-last=(index + 1 >= item_count).then_some("true")
                                    aria-hidden="true"
                                ></span>
                            </li>
                        }
                    })
                    .collect_view()
            }}
        </ol>
    }
}
