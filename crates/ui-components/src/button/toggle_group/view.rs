use super::super::toggle_button::{ToggleButton, ToggleButtonSize, ToggleButtonVariant};
use super::{
    ToggleGroupItem, ToggleGroupStateInput,
    logic::{self, ToggleGroupOrientation, ToggleGroupSelectionMode},
};
use leptos::prelude::*;
use std::collections::BTreeSet;
use ui_headless as overlay_open;

#[component]
pub fn ToggleGroup(
    id_base: String,
    items: Vec<ToggleGroupItem>,
    #[prop(optional)] selection_mode: ToggleGroupSelectionMode,
    #[prop(optional)] selected_ids: Option<Signal<BTreeSet<String>>>,
    #[prop(optional)] default_selected_ids: Option<BTreeSet<String>>,
    #[prop(optional)] on_selected_ids_change: Option<Callback<BTreeSet<String>>>,
    #[prop(optional)] on_action: Option<Callback<String>>,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] attached: bool,
    #[prop(optional)] orientation: ToggleGroupOrientation,
    #[prop(optional)] variant: ToggleButtonVariant,
    #[prop(optional)] size: ToggleButtonSize,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
) -> impl IntoView {
    let items = logic::normalize_items(items);
    let item_ids = logic::collect_item_ids(&items);

    let default_selected_ids = logic::sanitize_selected_ids(
        default_selected_ids.unwrap_or_default(),
        &item_ids,
        &items,
        selection_mode,
    );

    let selected_state = overlay_open::use_controllable_state(
        selected_ids,
        Some(default_selected_ids),
        on_selected_ids_change,
    );
    let selected_ids = selected_state.value;
    let request_selected_ids_change = selected_state.request_change;

    let (aria_label, has_custom_aria_label) = logic::normalize_aria_label(aria_label);

    let class_name = logic::normalize_optional_text(class_name);
    let has_custom_class_name = class_name.is_some();
    let class_name = StoredValue::new(class_name);

    let items = StoredValue::new(items);
    let item_ids = StoredValue::new(item_ids);
    let on_action = StoredValue::new(on_action);

    let resolved_selected_ids = Signal::derive(move || {
        logic::sanitize_selected_ids(
            selected_ids.get(),
            &item_ids.get_value(),
            &items.get_value(),
            selection_mode,
        )
    });

    let state = Signal::derive(move || {
        let items_value = items.get_value();
        logic::resolve_state(ToggleGroupStateInput {
            orientation,
            selection_mode,
            disabled,
            attached,
            item_count: items_value.len(),
            selected_count: resolved_selected_ids.get().len(),
            disabled_item_count: items_value.iter().filter(|item| item.disabled).count(),
            has_custom_aria_label,
            has_custom_class_name,
        })
    });

    let class =
        Signal::derive(move || logic::compose_class_name(class_name.get_value(), state.get()));

    view! {
        <div
            id=id_base
            class=move || class.get()
            data-slot="toggle-group"
            data-orientation=move || state.get().orientation_attr
            data-selection-mode=move || state.get().selection_mode_attr
            data-state=move || state.get().data_state_attr
            data-disabled=move || state.get().is_disabled.then_some("true")
            data-attached=move || state.get().is_attached.then_some("true")
            data-empty=move || state.get().is_empty.then_some("true")
            data-has-selection=move || state.get().has_selection.then_some("true")
            data-item-count=move || state.get().item_count.to_string()
            data-selected-count=move || state.get().selected_count.to_string()
            data-has-disabled-items=move || state.get().has_disabled_items.then_some("true")
            data-disabled-item-count=move || state.get().disabled_item_count.to_string()
            data-aria-source=move || state.get().aria_source_attr
            data-class-source=move || state.get().class_source_attr
            data-custom-class=move || state.get().has_custom_class_name.then_some("true")
            role="group"
            aria-label=aria_label
        >
            <div class="ui-toggle-group__items" data-slot="toggle-group-items">
                <For
                    each=move || items.get_value()
                    key=|item| item.id.clone()
                    children=move |item| {
                        let item_id = item.id.clone();
                        let item_label = item.label.clone();
                        let item_is_disabled = disabled || item.disabled;

                        let (item_selected, set_item_selected) = signal(
                            resolved_selected_ids.get_untracked().contains(&item_id),
                        );

                        {
                            let item_id = item_id.clone();
                            Effect::new(move |_| {
                                let next = resolved_selected_ids.get().contains(&item_id);
                                set_item_selected.set(next);
                            });
                        }

                        let on_item_change = {
                            let item_id = item_id.clone();
                            Callback::new(move |next_selected: bool| {
                                if item_is_disabled {
                                    return;
                                }

                                if let Some(on_action) = on_action.get_value() {
                                    on_action.run(item_id.clone());
                                }

                                let next_ids = logic::toggle_selected_id(
                                    resolved_selected_ids.get_untracked(),
                                    &item_id,
                                    &item_ids.get_value(),
                                    &items.get_value(),
                                    selection_mode,
                                    next_selected,
                                );
                                request_selected_ids_change.run(next_ids);
                            })
                        };

                        view! {
                            <ToggleButton
                                selected=item_selected
                                set_selected=set_item_selected
                                disabled=item_is_disabled
                                variant=variant
                                size=size
                                on_change=on_item_change
                                class_name="ui-toggle-group__item".to_string()
                                aria_label=item_label.clone()
                            >
                                {item_label}
                            </ToggleButton>
                        }
                    }
                />
            </div>
        </div>
    }
}
