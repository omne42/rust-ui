use crate::Select;
use leptos::prelude::*;
use ui_headless::PopoverPlacement;

#[component]
pub fn Picker(
    id_base: String,
    items: Vec<String>,
    selected_index: ReadSignal<Option<usize>>,
    set_selected_index: WriteSignal<Option<usize>>,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] placeholder: Option<String>,
    #[prop(optional)] disabled_indices: Vec<usize>,
    #[prop(optional)] placement: PopoverPlacement,
    #[prop(optional)] open: Option<Signal<bool>>,
    #[prop(optional)] default_open: Option<bool>,
    #[prop(optional)] on_open_change: Option<Callback<bool>>,
) -> impl IntoView {
    let placeholder = placeholder.unwrap_or_default();
    let default_open = default_open.unwrap_or(false);
    let on_open_change = on_open_change.unwrap_or_else(|| Callback::new(|_: bool| {}));

    if let Some(open) = open {
        view! {
            <Select
                id_base=id_base
                items=items
                selected_index=selected_index
                set_selected_index=set_selected_index
                disabled=disabled
                placeholder=placeholder
                disabled_indices=disabled_indices
                placement=placement
                open=open
                default_open=default_open
                on_open_change=on_open_change
            />
        }
        .into_any()
    } else {
        view! {
            <Select
                id_base=id_base
                items=items
                selected_index=selected_index
                set_selected_index=set_selected_index
                disabled=disabled
                placeholder=placeholder
                disabled_indices=disabled_indices
                placement=placement
                default_open=default_open
                on_open_change=on_open_change
            />
        }
        .into_any()
    }
}
