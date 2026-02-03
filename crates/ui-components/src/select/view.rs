use crate::{Button, ListBox, OnPress, Popover, presence::use_presence};
use leptos::{html, prelude::*};
use std::sync::Arc;
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
    let disabled_indices: StoredValue<Vec<usize>> = StoredValue::new(disabled_indices);
    let (is_open, set_open) = signal(false);
    let presence = use_presence(is_open.into());

    let anchor_ref: NodeRef<html::Button> = NodeRef::new();

    let on_trigger_press: OnPress = Callback::new(move |_| set_open.update(|open| *open = !*open));
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

    view! {
        <div class="ui-select">
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
                        />
                    </div>
                </Popover>
            </Show>
        </div>
    }
}
