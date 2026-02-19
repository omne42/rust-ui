use crate::command::{CommandGroup, CommandMotion, CommandPartStateInput, CommandSlot, logic};
use leptos::{ev, html, prelude::*};
use std::sync::Arc;
use ui_headless::{ListBoxOptions, use_listbox};
use ui_visual_primitive::active_highlight::{
    ActiveHighlightMotion, attach_active_highlight_motion,
};

#[component]
pub fn Command(
    id_base: String,
    #[prop(into)] groups: Arc<[CommandGroup]>,
    #[prop(optional)] on_action: Option<Callback<String>>,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] motion: ActiveHighlightMotion,
    #[prop(optional, into)] placeholder: Option<String>,
    #[prop(optional, into)] empty_label: Option<String>,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
) -> impl IntoView {
    let id_base = logic::normalize_id_base(id_base);
    let has_custom_id_base = id_base != logic::DEFAULT_ID_BASE;
    let id_base = StoredValue::new(id_base);

    let (placeholder, has_custom_placeholder) = logic::resolve_placeholder(placeholder);
    let placeholder = StoredValue::new(placeholder);

    let (empty_label, has_custom_empty_label) = logic::resolve_empty_label(empty_label);
    let empty_label = StoredValue::new(empty_label);

    let (aria_label, has_custom_aria_label) = logic::resolve_aria_label(aria_label);
    let aria_label = StoredValue::new(aria_label);

    let class_name = logic::normalize_optional_text(class_name);
    let has_custom_class_name = class_name.is_some();
    let class_name = StoredValue::new(class_name);

    let has_custom_disabled = disabled != logic::DEFAULT_DISABLED;
    let has_custom_on_action = on_action.is_some();
    let has_custom_motion = motion != CommandMotion::default();

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
        id_base: format!("{}-command", id_base.get_value()),
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

    let root_state = Memo::new(move |_| {
        filtered.with(|filtered| {
            logic::resolve_state(CommandPartStateInput {
                slot: CommandSlot::Root,
                item_count: filtered.items.len(),
                group_count: filtered.groups.len(),
                is_disabled: disabled,
                has_query: !query.get().trim().is_empty(),
                has_custom_id_base,
                has_custom_placeholder,
                has_custom_empty_label,
                has_custom_aria_label,
                has_custom_class_name,
                has_custom_disabled,
                has_custom_on_action,
                has_custom_motion,
            })
        })
    });
    let root_state_for_class = root_state;

    let root_class = Memo::new(move |_| {
        logic::compose_class_name(class_name.get_value(), root_state_for_class.get())
    });

    let listbox_id = StoredValue::new(format!("{}-listbox", id_base.get_value()));

    let input_wrap_slot = CommandSlot::InputWrap;
    let input_slot = CommandSlot::Input;
    let list_slot = CommandSlot::List;
    let options_slot = CommandSlot::Options;
    let group_slot = CommandSlot::Group;
    let group_heading_slot = CommandSlot::GroupHeading;
    let group_items_slot = CommandSlot::GroupItems;
    let item_slot = CommandSlot::Item;
    let item_label_slot = CommandSlot::ItemLabel;
    let shortcut_slot = CommandSlot::Shortcut;
    let empty_slot = CommandSlot::Empty;
    let highlight_slot = CommandSlot::Highlight;

    view! {
        <section
            class=move || root_class.get()
            data-slot=move || root_state.get().slot_attr
            data-state=move || root_state.get().state_attr
            data-items=move || root_state.get().item_attr
            data-groups=move || root_state.get().group_attr
            data-query=move || root_state.get().query_attr
            data-disabled=move || root_state.get().disabled_attr
            data-empty=move || root_state.get().is_empty.then_some("true")
            data-has-items=move || root_state.get().has_items.then_some("true")
            data-item-count=move || root_state.get().item_count
            data-group-count=move || root_state.get().group_count
            data-has-query=move || root_state.get().has_query.then_some("true")
            data-is-disabled=move || root_state.get().is_disabled.then_some("true")
            data-is-enabled=move || root_state.get().is_enabled.then_some("true")
            data-id-source=move || root_state.get().id_source_attr
            data-placeholder-source=move || root_state.get().placeholder_source_attr
            data-empty-label-source=move || root_state.get().empty_label_source_attr
            data-aria-label-source=move || root_state.get().aria_label_source_attr
            data-class-source=move || root_state.get().class_source_attr
            data-disabled-source=move || root_state.get().disabled_source_attr
            data-action-source=move || root_state.get().action_source_attr
            data-motion-source=move || root_state.get().motion_source_attr
            data-custom-id=move || root_state.get().has_custom_id_base.then_some("true")
            data-custom-placeholder=move || root_state.get().has_custom_placeholder.then_some("true")
            data-custom-empty-label=move || root_state.get().has_custom_empty_label.then_some("true")
            data-custom-aria-label=move || root_state.get().has_custom_aria_label.then_some("true")
            data-custom-class=move || root_state.get().has_custom_class_name.then_some("true")
            data-custom-disabled=move || root_state.get().has_custom_disabled.then_some("true")
            data-custom-action=move || root_state.get().has_custom_on_action.then_some("true")
            data-custom-motion=move || root_state.get().has_custom_motion.then_some("true")
        >
            <div class=input_wrap_slot.base_class() data-slot=input_wrap_slot.as_attr()>
                <input
                    type="text"
                    class=input_slot.base_class()
                    data-slot=input_slot.as_attr()
                    placeholder=placeholder.get_value()
                    value=move || query.get()
                    disabled=disabled
                    role="combobox"
                    aria-autocomplete="list"
                    aria-expanded="true"
                    aria-label=aria_label.get_value()
                    aria-controls=listbox_id.get_value()
                    aria-activedescendant=move || listbox.attrs.aria_activedescendant.get()
                    on:input=move |ev| set_query.set(event_target_value(&ev))
                    on:keydown=on_input_key_down
                />
            </div>

            <div
                class=list_slot.base_class()
                id=listbox_id.get_value()
                role=listbox.attrs.role
                tabindex=listbox.attrs.tabindex
                aria-label=aria_label.get_value()
                aria-disabled=listbox.attrs.aria_disabled
                data-slot=list_slot.as_attr()
                data-empty=move || root_state.get().is_empty.then_some("true")
            >
                {move || {
                    filtered.with(|state| {
                        if state.items.is_empty() {
                            return view! {
                                <div class=empty_slot.base_class() data-slot=empty_slot.as_attr()>
                                    {empty_label.get_value()}
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
                            <div
                                class=options_slot.base_class()
                                node_ref=options_ref
                                data-slot=options_slot.as_attr()
                            >
                                <div
                                    class=highlight_slot.base_class()
                                    node_ref=highlight_ref
                                    data-slot=highlight_slot.as_attr()
                                ></div>
                                {state
                                    .groups
                                    .iter()
                                    .map(|group| {
                                        let heading = group.heading.clone();
                                        let item_indices = group.item_indices.clone();

                                        view! {
                                            <section class=group_slot.base_class() data-slot=group_slot.as_attr()>
                                                <h3
                                                    class=group_heading_slot.base_class()
                                                    data-slot=group_heading_slot.as_attr()
                                                >
                                                    {heading}
                                                </h3>
                                                <div
                                                    class=group_items_slot.base_class()
                                                    data-slot=group_items_slot.as_attr()
                                                >
                                                    {item_indices
                                                        .into_iter()
                                                        .filter_map(|index| {
                                                            let id = option_id.run(index);
                                                            let item = state.items.get(index).cloned()?;
                                                            let has_shortcut = item.shortcut.is_some();
                                                            let shortcut = StoredValue::new(item.shortcut.unwrap_or_default());
                                                            let item_label = StoredValue::new(item.label);
                                                            let item_disabled = item.disabled;

                                                            Some(view! {
                                                                <div
                                                                    id=id
                                                                    role="option"
                                                                    class=item_slot.base_class()
                                                                    data-slot=item_slot.as_attr()
                                                                    data-index=index
                                                                    data-state=move || {
                                                                        if item_disabled {
                                                                            "disabled"
                                                                        } else if selected_index.get() == Some(index) {
                                                                            "selected"
                                                                        } else if active_index.get() == index {
                                                                            "focused"
                                                                        } else {
                                                                            "idle"
                                                                        }
                                                                    }
                                                                    aria-selected=move || {
                                                                        (selected_index.get() == Some(index)).then_some("true")
                                                                    }
                                                                    aria-disabled=item_disabled.then_some("true")
                                                                    data-disabled=item_disabled.then_some("true")
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
                                                                    <span class=item_label_slot.base_class() data-slot=item_label_slot.as_attr()>
                                                                        {item_label.get_value()}
                                                                    </span>
                                                                    <Show when=move || has_shortcut>
                                                                        <kbd class=shortcut_slot.base_class() data-slot=shortcut_slot.as_attr()>
                                                                            {shortcut.get_value()}
                                                                        </kbd>
                                                                    </Show>
                                                                </div>
                                                            }
                                                            .into_any())
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
