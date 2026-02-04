use crate::select::logic;
use crate::{Button, ListBox, OnPress, Popover, presence::use_presence};
use leptos::{ev, html, prelude::*};
use std::{collections::HashSet, sync::Arc, time::Duration};
use ui_headless::PopoverPlacement;

#[component]
pub fn Select(
    id_base: String,
    items: Vec<String>,
    selected_index: ReadSignal<Option<usize>>,
    set_selected_index: WriteSignal<Option<usize>>,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] placeholder: Option<String>,
    #[prop(optional)] disabled_indices: Vec<usize>,
    #[prop(optional)] placement: PopoverPlacement,
) -> impl IntoView {
    let items: StoredValue<Arc<[String]>> = StoredValue::new(items.into());
    let disabled_set: HashSet<usize> = disabled_indices.iter().copied().collect();
    let disabled_set: StoredValue<Arc<HashSet<usize>>> = StoredValue::new(Arc::new(disabled_set));
    let disabled_indices: StoredValue<Vec<usize>> = StoredValue::new(disabled_indices);

    let (open_focus, set_open_focus) = signal(logic::SelectOpenFocusStrategy::Selected);
    let (typeahead, set_typeahead) = signal(String::new());
    let (last_typed_at, set_last_typed_at) = signal(None::<std::time::Instant>);
    let typeahead_timeout = Duration::from_millis(500);

    let (is_open, set_open) = signal(false);
    let presence = use_presence(is_open.into());

    let anchor_ref: NodeRef<html::Button> = NodeRef::new();

    let on_trigger_press: OnPress = Callback::new(move |_| {
        if disabled {
            return;
        }
        if items.get_value().is_empty() {
            return;
        }
        set_open_focus.set(logic::SelectOpenFocusStrategy::Selected);
        set_open.update(|open| *open = !*open);
    });
    let on_close: OnPress = Callback::new(move |_| set_open.set(false));

    let placeholder = placeholder.unwrap_or_else(|| "Select…".to_string());
    let trigger_label = Memo::new({
        let placeholder = placeholder.clone();
        move |_| {
            let items = items.get_value();
            selected_index
                .get()
                .and_then(|i| items.get(i).cloned())
                .unwrap_or_else(|| placeholder.clone())
        }
    });

    let listbox_id = format!("{id_base}-listbox");
    let id_base = StoredValue::new(id_base);
    let listbox_id = StoredValue::new(listbox_id);

    let on_action = Callback::new(move |_| set_open.set(false));

    let on_key_down = move |ev: ev::KeyboardEvent| {
        if disabled {
            return;
        }
        let items = items.get_value();
        if items.is_empty() {
            return;
        }
        let key = ev.key();
        let is_open = is_open.get_untracked();

        match key.as_str() {
            "ArrowDown" => {
                if is_open {
                    return;
                }
                set_open_focus.set(logic::SelectOpenFocusStrategy::First);
                set_open.set(true);
                ev.prevent_default();
            }
            "ArrowUp" => {
                if is_open {
                    return;
                }
                set_open_focus.set(logic::SelectOpenFocusStrategy::Last);
                set_open.set(true);
                ev.prevent_default();
            }
            "ArrowLeft" | "ArrowRight" => {
                if is_open {
                    return;
                }
                let direction = if key == "ArrowLeft" {
                    logic::SelectHorizontalNav::Previous
                } else {
                    logic::SelectHorizontalNav::Next
                };

                let disabled = disabled_set.get_value();
                let target = logic::resolve_horizontal_nav_target(
                    selected_index.get_untracked(),
                    direction,
                    items.len(),
                    disabled.as_ref(),
                );
                if let Some(target) = target {
                    set_selected_index.set(Some(target));
                }
                ev.prevent_default();
            }
            "Enter" => {
                set_open_focus.set(logic::SelectOpenFocusStrategy::First);
            }
            _ => {
                if is_open {
                    return;
                }
                let Some(ch) = logic::typeahead_char(&key) else {
                    return;
                };

                let now = std::time::Instant::now();
                let mut query = typeahead.get_untracked();
                if last_typed_at
                    .get_untracked()
                    .map(|t| now.duration_since(t) > typeahead_timeout)
                    .unwrap_or(true)
                {
                    query.clear();
                }
                query.push(ch);

                let disabled = disabled_set.get_value();
                let count = items.len();
                if count == 0 {
                    return;
                }

                let start = selected_index
                    .get_untracked()
                    .map(|idx| (idx + 1) % count)
                    .unwrap_or(0);

                let mut matched =
                    logic::find_typeahead_match(&query, start, items.as_ref(), disabled.as_ref());
                if matched.is_none() && query.len() > 1 {
                    let single = ch.to_string();
                    matched = logic::find_typeahead_match(
                        &single,
                        start,
                        items.as_ref(),
                        disabled.as_ref(),
                    );
                    if matched.is_some() {
                        query = single;
                    }
                }

                set_typeahead.set(query);
                set_last_typed_at.set(Some(now));
                if let Some(next) = matched {
                    set_selected_index.set(Some(next));
                }
            }
        }
    };

    let on_key_up = move |ev: ev::KeyboardEvent| {
        if disabled {
            return;
        }
        let items = items.get_value();
        if items.is_empty() {
            return;
        }

        let key = ev.key();
        if matches!(key.as_str(), " " | "Space" | "Spacebar") {
            set_open_focus.set(logic::SelectOpenFocusStrategy::First);
        }
    };

    view! {
        <div class="ui-select" on:keydown=on_key_down on:keyup=on_key_up>
            <Button
                disabled=disabled
                node_ref=anchor_ref
                on_press=on_trigger_press
                aria_haspopup="listbox"
                aria_expanded=is_open.into()
                aria_controls=listbox_id.get_value()
            >
                {move || trigger_label.get()}
            </Button>

            <Show when=move || presence.is_present.get()>
                <Popover
                    open=is_open.into()
                    anchor_ref=anchor_ref
                    on_close=on_close
                    placement=placement
                    on_exit_complete=presence.finish_exit
                >
                    <div class="ui-select__panel">
                        {move || {
                            let focus = open_focus.get_untracked();
                            let default_index = match focus {
                                logic::SelectOpenFocusStrategy::Last => {
                                    items.get_value().len().saturating_sub(1)
                                }
                                logic::SelectOpenFocusStrategy::Selected
                                | logic::SelectOpenFocusStrategy::First => 0,
                            };
                            let sync_active_index_to_selected =
                                matches!(focus, logic::SelectOpenFocusStrategy::Selected);

                            view! {
                        <ListBox
                            id_base=id_base.get_value()
                            id=listbox_id.get_value()
                            class_name="ui-select__listbox"
                            items=items.get_value()
                            selected_index=selected_index
                            set_selected_index=set_selected_index
                            disabled=disabled
                            disabled_indices=disabled_indices.get_value()
                            on_action=on_action
                            default_index=default_index
                            sync_active_index_to_selected=sync_active_index_to_selected
                        />
                            }
                        }}
                    </div>
                </Popover>
            </Show>
        </div>
    }
}
