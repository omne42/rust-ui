use crate::{ComboBox, ComboBoxMotion};
use leptos::prelude::*;

#[component]
pub fn Combobox(
    id_base: String,
    label: String,
    items: Vec<String>,
    selected_index: ReadSignal<Option<usize>>,
    set_selected_index: WriteSignal<Option<usize>>,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] disabled_indices: Vec<usize>,
    #[prop(optional, into)] required: Signal<bool>,
    #[prop(optional, into)] invalid: Signal<bool>,
    #[prop(optional, into)] description: Option<String>,
    #[prop(optional, into)] error: Option<String>,
    #[prop(optional, into)] placeholder: Option<String>,
    #[prop(optional)] motion: ComboBoxMotion,
    #[prop(optional, into)] class_name: Option<String>,
) -> impl IntoView {
    let description = description.unwrap_or_default();
    let error = error.unwrap_or_default();
    let placeholder = placeholder.unwrap_or_default();
    let class_name = class_name.unwrap_or_default();

    view! {
        <ComboBox
            id_base=id_base
            label=label
            items=items
            selected_index=selected_index
            set_selected_index=set_selected_index
            disabled=disabled
            disabled_indices=disabled_indices
            required=required
            invalid=invalid
            description=description
            error=error
            placeholder=placeholder
            motion=motion
            class_name=class_name
        />
    }
}
