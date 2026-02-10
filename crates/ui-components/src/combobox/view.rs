use crate::combobox::{
    ComboboxStateInput,
    logic::{self},
};
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
    let (label, has_custom_label) = logic::resolve_label(label);

    let description = logic::normalize_optional_text(description);
    let has_custom_description = description.is_some();
    let description = description.unwrap_or_default();

    let error = logic::normalize_optional_text(error);
    let has_custom_error = error.is_some();
    let error = error.unwrap_or_default();

    let placeholder = logic::normalize_optional_text(placeholder);
    let has_custom_placeholder = placeholder.is_some();
    let placeholder = placeholder.unwrap_or_default();

    let class_name = logic::normalize_optional_text(class_name);
    let has_custom_class_name = class_name.is_some();
    let class_name_for_inner = class_name.clone().unwrap_or_default();

    let item_count = items.len();
    let disabled_option_count = disabled_indices
        .iter()
        .filter(|index| **index < item_count)
        .count();

    let state = Signal::derive(move || {
        logic::resolve_state(ComboboxStateInput {
            item_count,
            disabled_option_count,
            selected_index: selected_index.get(),
            required: required.get(),
            invalid: invalid.get(),
            disabled,
            has_custom_label,
            has_custom_description,
            has_custom_error,
            has_custom_placeholder,
            has_custom_class_name,
            has_custom_motion: motion != ComboBoxMotion::default(),
        })
    });

    let class = Signal::derive(move || logic::compose_class_name(class_name.clone(), state.get()));

    view! {
        <div
            class=move || class.get()
            data-slot="combobox"
            data-state=move || state.get().state_attr
            data-selection=move || state.get().selection_attr
            data-options=move || state.get().options_attr
            data-requirement=move || state.get().requirement_attr
            data-label-source=move || state.get().label_source_attr
            data-description-source=move || state.get().description_source_attr
            data-error-source=move || state.get().error_source_attr
            data-placeholder-source=move || state.get().placeholder_source_attr
            data-class-source=move || state.get().class_source_attr
            data-motion-source=move || state.get().motion_source_attr
            data-custom-motion=move || state.get().has_custom_motion.then_some("true")
            data-custom-class=move || state.get().has_custom_class_name.then_some("true")
            data-has-disabled-options=move || state.get().has_disabled_options.then_some("true")
        >
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
                class_name=class_name_for_inner
            />
        </div>
    }
}
