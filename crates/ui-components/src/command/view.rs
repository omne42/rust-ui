use crate::active_highlight::attach_active_highlight_motion;
use crate::command::{
    CommandGroup, CommandMotion,
    logic::{self, CommandStateInput},
};
use leptos::{ev, html, prelude::*};
use std::sync::Arc;
use ui_headless::{ListBoxOptions, use_listbox};

#[component]
pub fn Command(
    id_base: String,
    #[prop(into)] groups: Arc<[CommandGroup]>,
    #[prop(optional)] on_action: Option<Callback<String>>,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] motion: CommandMotion,
    #[prop(optional, into)] placeholder: Option<String>,
    #[prop(optional, into)] empty_label: Option<String>,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
) -> impl IntoView {
    let placeholder = logic::normalize_placeholder(placeholder);
    let empty_label = logic::normalize_empty_label(empty_label);
    let aria_label = logic::normalize_aria_label(aria_label);

    let class_name = logic::normalize_optional_text(class_name);
    let has_custom_class_name = class_name.is_some();
    let class_name = StoredValue::new(class_name);

    let groups = StoredValue::new(groups);
    let (query, set_query) = signal(String::new());

    let filtered = Memo::new(move |_| {
        let groups = groups.get_value();
        logic::filter_groups(&groups, &query.get())
    });

    let (item_count, set_item_count) = signal(filtered.get_untracked().items.len());
    Effect::new(move |_| {
        set_item_count.set(filtered.with(|state| state.items.len()));
    });

    let (selected_index, set_selected_index) = signal(None::<usize>);
    Effect::new(move |_| {
        let count = item_count.get();
        match selected_index.get() {
            Some(index) if index < count => {}
            _ if count == 0 => set_selected_index.set(None),
            _ => set_selected_index.set(Some(0)),
        }
    });

    let is_item_disabled = Callback::new(move |index: usize| {
        filtered.with(|state| {
            state
                .items
                .get(index)
                .map(|item| item.disabled)
                .unwrap_or(true)
        })
    });

    let item_text = Callback::new(move |index: usize| {
        filtered.with(|state| {
            state
                .items
                .get(index)
                .map(|item| item.label.clone())
                .unwrap_or_default()
        })
    });

    let on_action_by_index = on_action.map(|on_action| {
        Callback::new(move |index: usize| {
            if let Some(id) =
                filtered.with(|state| state.items.get(index).map(|item| item.id.clone()))
            {
                on_action.run(id);
            }
        })
    });

    let listbox = use_listbox(ListBoxOptions {
        is_disabled: disabled,
        should_loop: true,
        id_base: format!("{id_base}-command"),
        default_index: 0,
        sync_active_index_to_selected: true,
        item_count,
        selected_index,
        set_selected_index,
        on_action: on_action_by_index,
        is_item_disabled: Some(is_item_disabled),
        item_text: Some(item_text),
    });

    let options_ref: NodeRef<html::Div> = NodeRef::new();
    let highlight_ref: NodeRef<html::Div> = NodeRef::new();
    attach_active_highlight_motion(
        options_ref,
        highlight_ref,
        listbox.active_index,
        listbox.option_id,
        motion,
    );

    let on_input_key_down = {
        let on_key_down = listbox.handlers.on_key_down;
        move |ev: ev::KeyboardEvent| {
            let key = ev.key();

            if key == "Escape" && !query.get_untracked().is_empty() {
                set_query.set(String::new());
                ev.prevent_default();
                return;
            }

            let should_delegate = matches!(
                key.as_str(),
                "ArrowDown" | "ArrowUp" | "Home" | "End" | "Enter"
            );

            if should_delegate && on_key_down.run(key) {
                ev.prevent_default();
            }
        }
    };

    let state = Signal::derive(move || {
        filtered.with(|filtered| {
            logic::resolve_state(CommandStateInput {
                item_count: filtered.items.len(),
                group_count: filtered.groups.len(),
                is_disabled: disabled,
                has_query: !query.get().trim().is_empty(),
                has_custom_class_name,
            })
        })
    });

    let class =
        Signal::derive(move || logic::compose_class_name(class_name.get_value(), state.get()));

    let listbox_id = format!("{id_base}-listbox");

    view! {
        <section
            class=move || class.get()
            data-slot="command"
            data-state=move || state.get().data_state_attr
            data-disabled=move || state.get().is_disabled.then_some("true")
            data-empty=move || state.get().is_empty.then_some("true")
            data-has-results=move || state.get().has_items.then_some("true")
            data-has-query=move || state.get().has_query.then_some("true")
            data-item-count=move || state.get().item_count.to_string()
            data-group-count=move || state.get().group_count.to_string()
        >
            <div class="ui-command__input-wrap" data-slot="command-input-wrap">
                <input
                    type="text"
                    class="ui-command__input"
                    data-slot="command-input"
                    placeholder=placeholder
                    value=move || query.get()
                    disabled=disabled
                    role="combobox"
                    aria-autocomplete="list"
                    aria-expanded="true"
                    aria-label=aria_label.clone()
                    aria-controls=listbox_id.clone()
                    aria-activedescendant=move || listbox.attrs.aria_activedescendant.get()
                    on:input=move |ev| set_query.set(event_target_value(&ev))
                    on:keydown=on_input_key_down
                />
            </div>

            <div
                class="ui-command__list"
                id=listbox_id.clone()
                role=listbox.attrs.role
                tabindex=listbox.attrs.tabindex
                aria-label=aria_label
                aria-disabled=listbox.attrs.aria_disabled
                data-slot="command-list"
                data-empty=move || state.get().is_empty.then_some("true")
            >
                {move || {
                    filtered.with(|state| {
                        if state.items.is_empty() {
                            return view! {
                                <div class="ui-command__empty" data-slot="command-empty">
                                    {empty_label.clone()}
                                </div>
                            }
                            .into_any();
                        }

                        let option_id = listbox.option_id;
                        let active_index = listbox.active_index;
                        let selected_index = listbox.selected_index;
                        let on_option_pointer_move = listbox.handlers.on_option_pointer_move;
                        let on_option_click = listbox.handlers.on_option_click;

                        view! {
                            <div class="ui-command__options" node_ref=options_ref data-slot="command-options">
                                <div class="ui-active-highlight" node_ref=highlight_ref data-slot="command-highlight"></div>
                                {state
                                    .groups
                                    .iter()
                                    .map(|group| {
                                        let heading = group.heading.clone();
                                        let item_indices = group.item_indices.clone();

                                        view! {
                                            <section class="ui-command__group" data-slot="command-group">
                                                <h3 class="ui-command__group-heading" data-slot="command-group-heading">
                                                    {heading}
                                                </h3>
                                                <div class="ui-command__group-items" data-slot="command-group-items">
                                                    {item_indices
                                                        .into_iter()
                                                        .map(|index| {
                                                            let id = option_id.run(index);
                                                            let item = state
                                                                .items
                                                                .get(index)
                                                                .cloned()
                                                                .expect("filtered command item index should always exist");
                                                            let has_shortcut = item.shortcut.is_some();
                                                            let shortcut = StoredValue::new(item.shortcut.unwrap_or_default());

                                                            view! {
                                                                <div
                                                                    id=id
                                                                    role="option"
                                                                    class="ui-command__option"
                                                                    data-slot="command-item"
                                                                    aria-selected=move || {
                                                                        (selected_index.get() == Some(index)).then_some("true")
                                                                    }
                                                                    aria-disabled=item.disabled.then_some("true")
                                                                    data-disabled=item.disabled.then_some("true")
                                                                    data-focused=move || {
                                                                        (active_index.get() == index).then_some("true")
                                                                    }
                                                                    data-selected=move || {
                                                                        (selected_index.get() == Some(index)).then_some("true")
                                                                    }
                                                                    on:pointermove=move |_| on_option_pointer_move.run(index)
                                                                    on:click=move |_| {
                                                                        on_option_pointer_move.run(index);
                                                                        on_option_click.run(index);
                                                                    }
                                                                >
                                                                    <span class="ui-command__item-label" data-slot="command-item-label">
                                                                        {item.label}
                                                                    </span>
                                                                    <Show when=move || has_shortcut>
                                                                        <kbd class="ui-command__shortcut" data-slot="command-shortcut">
                                                                            {shortcut.get_value()}
                                                                        </kbd>
                                                                    </Show>
                                                                </div>
                                                            }
                                                            .into_any()
                                                        })
                                                        .collect_view()}
                                                </div>
                                            </section>
                                        }
                                    })
                                    .collect_view()}
                            </div>
                        }
                        .into_any()
                    })
                }}
            </div>
        </section>
    }
}
