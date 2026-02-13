use crate::list::{
    ListMotion,
    logic::{self, ListStateInput},
};
use leptos::prelude::*;
use std::sync::Arc;

#[component]
pub fn List(
    id_base: String,
    #[prop(into)] items: Arc<[String]>,
    selected_index: ReadSignal<Option<usize>>,
    set_selected_index: WriteSignal<Option<usize>>,
    #[prop(optional, into)] id: Option<String>,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] aria_labelledby: Option<String>,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] disabled_indices: Vec<usize>,
    #[prop(optional)] on_action: Option<Callback<usize>>,
    #[prop(optional, default = 0)] default_index: usize,
    #[prop(optional, default = true)] sync_active_index_to_selected: bool,
    #[prop(optional)] motion: ListMotion,
    #[prop(optional, into)] class_name: Option<String>,
) -> impl IntoView {
    let id = id.unwrap_or_default();
    let on_action = on_action.unwrap_or_else(|| Callback::new(|_| {}));

    let aria_label = logic::normalize_optional_text(aria_label);
    let aria_labelledby = logic::normalize_optional_text(aria_labelledby);
    let class_name = logic::normalize_optional_text(class_name);

    let state = logic::resolve_state(ListStateInput {
        item_count: items.len(),
        disabled,
        has_disabled_items: !disabled_indices.is_empty(),
        has_custom_aria_label: aria_label.is_some(),
        has_custom_aria_labelledby: aria_labelledby.is_some(),
        has_custom_class_name: class_name.is_some(),
    });
    let class_name = logic::compose_class_name(class_name, state);
    let motion = crate::list::motion::sanitize_motion(motion);
    let aria_label = aria_label.unwrap_or_default();
    let aria_labelledby = aria_labelledby.unwrap_or_default();

    view! {
        <crate::listbox::ListBox
            id_base=id_base
            items=items
            selected_index=selected_index
            set_selected_index=set_selected_index
            id=id
            aria_label=aria_label
            aria_labelledby=aria_labelledby
            disabled=disabled
            disabled_indices=disabled_indices
            on_action=on_action
            default_index=default_index
            sync_active_index_to_selected=sync_active_index_to_selected
            motion=motion.active_highlight
            class_name=class_name
        />
    }
}
