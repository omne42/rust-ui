use crate::picker::{PickerStateInput, logic};
use crate::{Select, SelectMotion};
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
    #[prop(optional)] motion: SelectMotion,
    #[prop(optional, into)] class_name: Option<String>,
) -> impl IntoView {
    let placeholder = logic::normalize_optional_text(placeholder);
    let has_custom_placeholder = placeholder.is_some();
    let placeholder_for_inner = placeholder.clone().unwrap_or_default();

    let class_name = logic::normalize_optional_text(class_name);
    let has_custom_class_name = class_name.is_some();
    let class_name_for_wrapper = class_name.clone();

    let has_custom_open_handler = on_open_change.is_some();
    let on_open_change_for_inner = on_open_change.unwrap_or_else(|| Callback::new(|_: bool| {}));

    let is_controlled = open.is_some();

    let default_open = default_open.unwrap_or(false);

    let item_count = items.len();
    let has_items = item_count > 0;
    let disabled_option_count = disabled_indices.len();
    let has_disabled_indices = disabled_option_count > 0;

    let has_custom_placement = placement != PopoverPlacement::default();
    let has_custom_motion = motion != SelectMotion::default();

    let state = Signal::derive(move || {
        logic::resolve_state(PickerStateInput {
            disabled,
            has_items,
            has_selection: selected_index.get().is_some(),
            has_disabled_indices,
            is_controlled,
            default_open,
            has_custom_placeholder,
            has_custom_open_handler,
            has_custom_class_name,
            has_custom_placement,
            has_custom_motion,
        })
    });

    let class = Signal::derive(move || {
        logic::compose_class_name(class_name_for_wrapper.clone(), state.get())
    });

    if let Some(open) = open {
        view! {
            <div
                class=move || class.get()
                data-slot="picker"
                data-state=move || state.get().state_attr
                data-selection=move || state.get().selection_attr
                data-disabled-options=move || state.get().disabled_options_attr
                data-open-mode=move || state.get().open_mode_attr
                data-initial-open=move || state.get().initial_open_attr
                data-placeholder-source=move || state.get().placeholder_source_attr
                data-handler-source=move || state.get().handler_source_attr
                data-class-source=move || state.get().class_source_attr
                data-placement-source=move || state.get().placement_source_attr
                data-motion-source=move || state.get().motion_source_attr
                data-item-count=item_count.to_string()
                data-disabled-option-count=disabled_option_count.to_string()
                data-disabled=move || state.get().is_disabled.then_some("true")
                data-controlled=move || state.get().is_controlled.then_some("true")
                data-uncontrolled=move || (!state.get().is_controlled).then_some("true")
                data-custom-class=move || state.get().has_custom_class_name.then_some("true")
                data-custom-motion=move || state.get().has_custom_motion.then_some("true")
            >
                <Select
                    id_base=id_base
                    items=items
                    selected_index=selected_index
                    set_selected_index=set_selected_index
                    disabled=disabled
                    placeholder=placeholder_for_inner.clone()
                    disabled_indices=disabled_indices
                    placement=placement
                    open=open
                    default_open=default_open
                    on_open_change=on_open_change_for_inner
                    motion=motion
                />
            </div>
        }
        .into_any()
    } else {
        view! {
            <div
                class=move || class.get()
                data-slot="picker"
                data-state=move || state.get().state_attr
                data-selection=move || state.get().selection_attr
                data-disabled-options=move || state.get().disabled_options_attr
                data-open-mode=move || state.get().open_mode_attr
                data-initial-open=move || state.get().initial_open_attr
                data-placeholder-source=move || state.get().placeholder_source_attr
                data-handler-source=move || state.get().handler_source_attr
                data-class-source=move || state.get().class_source_attr
                data-placement-source=move || state.get().placement_source_attr
                data-motion-source=move || state.get().motion_source_attr
                data-item-count=item_count.to_string()
                data-disabled-option-count=disabled_option_count.to_string()
                data-disabled=move || state.get().is_disabled.then_some("true")
                data-controlled=move || state.get().is_controlled.then_some("true")
                data-uncontrolled=move || (!state.get().is_controlled).then_some("true")
                data-custom-class=move || state.get().has_custom_class_name.then_some("true")
                data-custom-motion=move || state.get().has_custom_motion.then_some("true")
            >
                <Select
                    id_base=id_base
                    items=items
                    selected_index=selected_index
                    set_selected_index=set_selected_index
                    disabled=disabled
                    placeholder=placeholder_for_inner
                    disabled_indices=disabled_indices
                    placement=placement
                    default_open=default_open
                    on_open_change=on_open_change_for_inner
                    motion=motion
                />
            </div>
        }
        .into_any()
    }
}
