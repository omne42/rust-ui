use crate::autocomplete::{AutocompleteMotion, logic};
use crate::{ActiveHighlightMotion, presence::use_presence};
use leptos::{ev, html, prelude::*};
use std::{collections::HashSet, sync::Arc};
use ui_headless::{
    ComboBoxOptions, FocusRingOptions, TextFieldOptions, use_combo_box, use_focus_ring,
    use_text_field,
};

#[component]
fn AutocompletePanel(
    open: Signal<bool>,
    aria: ui_headless::ComboBoxAria,
    filtered_indices: Memo<Vec<usize>>,
    items: StoredValue<Arc<[String]>>,
    disabled_indices: Arc<HashSet<usize>>,
    selected_index: ReadSignal<Option<usize>>,
    motion: ActiveHighlightMotion,
    popover_motion: crate::popover::PopoverMotion,
    on_exit_complete: Callback<()>,
) -> impl IntoView {
    let panel_ref: NodeRef<html::Div> = NodeRef::new();
    crate::popover::motion::attach_motion(
        panel_ref,
        open,
        ui_headless::PopoverPlacement::BottomStart,
        on_exit_complete,
        popover_motion,
    );

    let options_ref: NodeRef<html::Div> = NodeRef::new();
    let highlight_ref: NodeRef<html::Div> = NodeRef::new();
    crate::active_highlight::attach_active_highlight_motion(
        options_ref,
        highlight_ref,
        aria.active_index,
        aria.option_id,
        motion,
    );

    let on_panel_pointer_down = move |ev: ev::PointerEvent| {
        // Prevent the input from losing focus while interacting with the list.
        ev.prevent_default();
    };

    view! {
        <div
            class="ui-autocomplete__panel"
            node_ref=panel_ref
            data-slot="autocomplete-panel"
            on:pointerdown=on_panel_pointer_down
        >
            <div
                class="ui-autocomplete__listbox"
                id=aria.listbox.id.clone()
                role=aria.listbox.role
                aria-disabled=aria.listbox.aria_disabled
                data-slot="autocomplete-listbox"
            >
                <div class="ui-autocomplete__options" node_ref=options_ref data-slot="autocomplete-options">
                    <div class="ui-active-highlight" node_ref=highlight_ref data-slot="autocomplete-highlight"></div>
                    {move || {
                        let indices = filtered_indices.get();
                        let items = items.get_value();
                        indices
                            .iter()
                            .copied()
                            .enumerate()
                            .map(|(filtered_index, original_index)| {
                                let id = aria.option_id.run(filtered_index);
                                let label = items.get(original_index).cloned().unwrap_or_default();
                                let is_selected = move || selected_index.get() == Some(original_index);
                                let is_disabled = disabled_indices.contains(&original_index);

                                view! {
                                    <div
                                        id=id
                                        role="option"
                                        aria-selected=move || if is_selected() { Some("true") } else { None }
                                        aria-disabled=if is_disabled { Some("true") } else { None }
                                        class="ui-autocomplete__option"
                                        data-selected=move || if is_selected() { Some("true") } else { None }
                                        data-disabled=if is_disabled { Some("true") } else { None }
                                        on:pointermove=move |_| aria.handlers.on_option_pointer_move.run(filtered_index)
                                        on:click=move |_| aria.handlers.on_option_click.run(filtered_index)
                                    >
                                        {label}
                                    </div>
                                }
                            })
                            .collect_view()
                    }}
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn Autocomplete(
    id_base: String,
    label: String,
    items: Vec<String>,
    selected_index: ReadSignal<Option<usize>>,
    set_selected_index: WriteSignal<Option<usize>>,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] disabled_indices: Vec<usize>,
    #[prop(optional, into)] required: Signal<bool>,
    #[prop(optional, into)] invalid: Signal<bool>,
    #[prop(optional, into)] aria_describedby: Signal<Option<String>>,
    #[prop(optional, into)] description: Option<String>,
    #[prop(optional, into)] error: Option<String>,
    #[prop(optional, into)] placeholder: Option<String>,
    #[prop(optional)] motion: AutocompleteMotion,
    #[prop(optional, into)] class_name: Option<String>,
) -> impl IntoView {
    let id_base = StoredValue::new(id_base);
    let items: StoredValue<Arc<[String]>> = StoredValue::new(items.into());

    let disabled_index_set: HashSet<usize> = disabled_indices.into_iter().collect();
    let has_disabled = !disabled_index_set.is_empty();
    let disabled_indices: Arc<HashSet<usize>> = Arc::new(disabled_index_set);

    let placeholder = placeholder.unwrap_or_else(|| "Search…".to_string());
    let placeholder = StoredValue::new(placeholder);

    let (is_open, set_open) = signal(false);
    let presence = use_presence(is_open.into());

    let (has_typed, set_has_typed) = signal(false);
    let (query, set_query) = signal(String::new());

    let selected_label = Memo::new(move |_| {
        let items = items.get_value();
        selected_index
            .get()
            .and_then(|index| items.get(index).cloned())
    });

    // Keep the input value aligned with selection when the popup closes.
    Effect::new(move |_| {
        if is_open.get() {
            return;
        }
        set_has_typed.set(false);
        let value = selected_label.get().unwrap_or_default();
        set_query.set(value);
    });

    let filtered_indices = Memo::new(move |_| {
        let items = items.get_value();
        logic::filter_indices(items.as_ref(), &query.get(), has_typed.get())
    });

    let (filtered_count, set_filtered_count) = signal(0_usize);
    Effect::new(move |_| set_filtered_count.set(filtered_indices.get().len()));

    let selected_filtered_index = Memo::new(move |_| {
        logic::map_selected_to_filtered(selected_index.get(), &filtered_indices.get())
    });

    let is_item_disabled = has_disabled.then_some({
        let disabled_indices = disabled_indices.clone();
        Callback::new(move |filtered_index: usize| {
            let indices = filtered_indices.get_untracked();
            let Some(original) = logic::map_filtered_to_original(filtered_index, &indices) else {
                return false;
            };
            disabled_indices.contains(&original)
        })
    });

    let focus_ring = use_focus_ring(FocusRingOptions {
        is_disabled: disabled,
    });

    let input_id = format!("{}-input", id_base.get_value());

    let text_field = use_text_field(TextFieldOptions {
        id: input_id,
        has_description: description.is_some(),
        has_error: error.is_some(),
        aria_describedby,
        is_invalid: invalid,
        is_required: required,
    });

    let on_action = Callback::new(move |filtered_index: usize| {
        let indices = filtered_indices.get_untracked();
        let Some(original_index) = logic::map_filtered_to_original(filtered_index, &indices) else {
            return;
        };
        set_selected_index.set(Some(original_index));

        let items = items.get_value();
        let label = items.get(original_index).cloned().unwrap_or_default();
        set_query.set(label);
        set_has_typed.set(false);
        set_open.set(false);
    });

    let aria = use_combo_box(ComboBoxOptions {
        is_disabled: disabled,
        id_base: id_base.get_value(),
        is_open,
        set_open,
        item_count: filtered_count,
        selected_index: selected_filtered_index.into(),
        on_action: Some(on_action),
        is_item_disabled,
    });

    let base_class = "ui-autocomplete".to_string();
    let class = class_name
        .filter(|value| !value.trim().is_empty())
        .map(|value| format!("{base_class} {value}"))
        .unwrap_or(base_class);

    let on_key_down = move |ev: ev::KeyboardEvent| {
        if aria.handlers.on_input_key_down.run(ev.key()) {
            ev.prevent_default();
        }
    };

    let on_focus = move |_| {
        focus_ring.handlers.on_focus.run(());
        if !disabled {
            aria.handlers.open.run(());
        }
    };

    let on_blur = move |_| {
        focus_ring.handlers.on_blur.run(());
        aria.handlers.close.run(());
        set_has_typed.set(false);
    };

    let on_input = move |ev| {
        if disabled {
            return;
        }
        set_has_typed.set(true);
        set_query.set(event_target_value(&ev));
        aria.handlers.open.run(());
    };

    view! {
        <div
            class=class
            class:ui-autocomplete--focus-visible=move || focus_ring.is_focus_visible.get()
            class:ui-autocomplete--invalid=move || invalid.get()
            class:ui-autocomplete--disabled=disabled
            data-slot="autocomplete"
        >
            <label
                class="ui-autocomplete__label"
                for=text_field.label.for_attr.clone()
                data-slot="autocomplete-label"
            >
                {label}
            </label>

            <div class="ui-autocomplete__control" data-slot="autocomplete-control">
                <input
                    class="ui-autocomplete__input"
                    data-slot="autocomplete-input"
                    id=aria.input.id.clone()
                    prop:value=move || query.get()
                    placeholder=move || {
                        (selected_label.get().is_none() && !has_typed.get())
                            .then(|| placeholder.get_value())
                    }
                    disabled=disabled
                    required=move || required.get()
                    role=aria.input.role
                    aria-autocomplete=aria.input.aria_autocomplete
                    aria-controls=aria.input.aria_controls.clone()
                    aria-expanded=move || aria.input.aria_expanded.get()
                    aria-activedescendant=move || aria.input.aria_activedescendant.get()
                    aria-describedby=move || text_field.input.aria_describedby.get()
                    aria-invalid=move || text_field.input.aria_invalid.get()
                    aria-required=move || text_field.input.aria_required.get()
                    aria-disabled=aria.input.aria_disabled
                    on:input=on_input
                    on:keydown=on_key_down
                    on:focus=on_focus
                    on:blur=on_blur
                />

                <Show when=move || presence.is_present.get()>
                    <AutocompletePanel
                        open=is_open.into()
                        aria=aria.clone()
                        filtered_indices=filtered_indices
                        items=items
                        disabled_indices=disabled_indices.clone()
                        selected_index=selected_index
                        motion=motion.highlight
                        popover_motion=motion.popover
                        on_exit_complete=presence.finish_exit
                    />
                </Show>
            </div>

            {description.map(|description| {
                let description_id = text_field.description.id.clone();
                view! {
                    <div
                        class="ui-autocomplete__description"
                        id=description_id
                        data-slot="autocomplete-description"
                    >
                        {description}
                    </div>
                }
            })}

            {error.map(|error| {
                let error_id = text_field.error.id.clone();
                let error_id = StoredValue::new(error_id);
                let error = StoredValue::new(error);
                view! {
                    <Show when=move || invalid.get()>
                        <div
                            class="ui-autocomplete__error"
                            id=move || error_id.get_value()
                            data-slot="autocomplete-error"
                        >
                            {move || error.get_value()}
                        </div>
                    </Show>
                }
            })}
        </div>
    }
}
