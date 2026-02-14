use crate::action_group::{
    ActionGroupItem, ActionGroupSelectionMode, ActionGroupStateInput,
    logic::{self, ActionGroupTone},
};
use leptos::prelude::*;
use std::collections::BTreeSet;
use ui_headless as overlay_open;

#[component]
pub fn ActionGroup(
    id_base: String,
    items: Vec<ActionGroupItem>,
    #[prop(optional)] tone: ActionGroupTone,
    #[prop(optional)] selection_mode: ActionGroupSelectionMode,
    #[prop(optional)] disabled: bool,
    #[prop(optional, into)] selected_ids: Option<Signal<BTreeSet<String>>>,
    #[prop(optional)] default_selected_ids: Option<BTreeSet<String>>,
    #[prop(optional)] on_selected_ids_change: Option<Callback<BTreeSet<String>>>,
    #[prop(optional)] on_selected_change: Option<Callback<BTreeSet<String>>>,
    #[prop(optional)] on_action: Option<Callback<String>>,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
) -> impl IntoView {
    let items = logic::normalize_items(items);
    let item_ids = logic::collect_item_ids(&items);

    let default_selected_ids = logic::sanitize_selected_ids(
        default_selected_ids.unwrap_or_default(),
        &item_ids,
        selection_mode,
    );

    let on_selected_change = on_selected_ids_change.or(on_selected_change);
    let selected_state = overlay_open::use_controllable_state(
        selected_ids,
        Some(default_selected_ids),
        on_selected_change,
    );
    let selected_ids = selected_state.value;
    let request_selected_change = selected_state.request_change;

    let (aria_label, has_custom_aria_label) = logic::normalize_aria_label(aria_label);

    let class_name = logic::normalize_optional_text(class_name);
    let has_custom_class_name = class_name.is_some();
    let class_name = StoredValue::new(class_name);

    let items = StoredValue::new(items);
    let item_ids = StoredValue::new(item_ids);
    let on_action = StoredValue::new(on_action);

    let resolved_selected_ids = Memo::new(move |_| {
        logic::sanitize_selected_ids(selected_ids.get(), &item_ids.get_value(), selection_mode)
    });

    let state = Memo::new(move |_| {
        logic::resolve_state(ActionGroupStateInput {
            tone,
            selection_mode,
            disabled,
            item_count: items.get_value().len(),
            selected_count: resolved_selected_ids.get().len(),
            has_custom_aria_label,
            has_custom_class_name,
        })
    });

    let class = Memo::new(move |_| logic::compose_class_name(class_name.get_value(), state.get()));

    view! {
        <div
            id=id_base
            class=move || class.get()
            data-slot="action-group"
            data-tone=move || state.get().tone_attr
            data-selection-mode=move || state.get().selection_mode_attr
            data-state=move || state.get().data_state_attr
            data-disabled=move || state.get().is_disabled.then_some("true")
            data-has-selection=move || state.get().has_selection.then_some("true")
            data-item-count=move || state.get().item_count.to_string()
            data-selected-count=move || state.get().selected_count.to_string()
            data-aria-source=move || state.get().aria_source_attr
            data-custom-class=move || state.get().has_custom_class_name.then_some("true")
            data-class-source=move || state.get().class_source_attr
            role="toolbar"
            aria-label=aria_label
        >
            <ul class="ui-action-group__list" data-slot="action-group-list">
                {move || {
                    let resolved_selected_ids = resolved_selected_ids.get();
                    items
                        .get_value()
                        .into_iter()
                        .enumerate()
                        .map(|(index, item)| {
                            let item_id_for_action = item.id.clone();
                            let item_id_for_selection = item.id.clone();
                            let is_item_disabled = disabled || item.disabled;
                            let is_selected = resolved_selected_ids.contains(&item.id);

                            let on_click = move |_| {
                                if is_item_disabled {
                                    return;
                                }

                                if let Some(on_action) = on_action.get_value() {
                                    on_action.run(item_id_for_action.clone());
                                }

                                let selected_ids = logic::sanitize_selected_ids(
                                    selected_ids.get_untracked(),
                                    &item_ids.get_value(),
                                    selection_mode,
                                );
                                let next = logic::toggle_selected_id(
                                    selected_ids,
                                    &item_id_for_selection,
                                    &item_ids.get_value(),
                                    selection_mode,
                                );
                                request_selected_change.run(next);
                            };

                            let item_class = format!(
                                "ui-action-group__item{}{}",
                                if is_selected {
                                    " ui-action-group__item--selected"
                                } else {
                                    ""
                                },
                                if is_item_disabled {
                                    " ui-action-group__item--disabled"
                                } else {
                                    ""
                                }
                            );

                            view! {
                                <li class="ui-action-group__node" data-slot="action-group-node" data-index=index>
                                    <button
                                        type="button"
                                        class=item_class
                                        data-slot="action-group-item"
                                        data-id=item.id.clone()
                                        data-selected=is_selected.then_some("true")
                                        data-disabled=is_item_disabled.then_some("true")
                                        disabled=is_item_disabled
                                        aria-pressed=if is_selected { Some("true") } else { Some("false") }
                                        on:click=on_click
                                    >
                                        {item.label}
                                    </button>
                                </li>
                            }
                        })
                        .collect_view()
                }}
            </ul>
        </div>
    }
}
