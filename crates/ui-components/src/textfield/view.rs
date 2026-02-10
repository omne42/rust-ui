use crate::TextField;
use crate::textfield::{
    TextfieldStateInput,
    logic::{self},
};
use leptos::{html, prelude::*};

#[component]
pub fn Textfield(
    id: String,
    label: String,
    value: ReadSignal<String>,
    set_value: WriteSignal<String>,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] read_only: bool,
    #[prop(optional, into)] required: Signal<bool>,
    #[prop(optional, into)] invalid: Signal<bool>,
    #[prop(optional, into)] description: Option<String>,
    #[prop(optional, into)] error: Option<String>,
    #[prop(optional, into)] placeholder: Option<String>,
    #[prop(optional)] input_type: Option<&'static str>,
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional)] node_ref: NodeRef<html::Input>,
) -> impl IntoView {
    let (label, has_custom_label) = logic::resolve_label(label);

    let description = logic::normalize_optional_text(description);
    let has_custom_description = description.is_some();
    let description_for_inner = description.clone().unwrap_or_default();

    let error = logic::normalize_optional_text(error);
    let has_custom_error = error.is_some();
    let error_for_inner = error.clone().unwrap_or_default();

    let placeholder = logic::normalize_optional_text(placeholder);
    let has_custom_placeholder = placeholder.is_some();
    let placeholder_for_inner = placeholder.clone().unwrap_or_default();

    let (input_type, has_custom_input_type) = logic::resolve_input_type(input_type);

    let class_name = logic::normalize_optional_text(class_name);
    let has_custom_class_name = class_name.is_some();
    let class_name_for_inner = class_name.clone().unwrap_or_default();

    let state = Signal::derive(move || {
        logic::resolve_state(TextfieldStateInput {
            disabled,
            read_only,
            required: required.get(),
            invalid: invalid.get(),
            has_value: !value.get().is_empty(),
            has_custom_label,
            has_custom_description,
            has_custom_error,
            has_custom_placeholder,
            has_custom_input_type,
            has_custom_class_name,
        })
    });

    let class = Signal::derive(move || logic::compose_class_name(class_name.clone(), state.get()));

    view! {
        <div
            class=move || class.get()
            data-slot="textfield"
            data-state=move || state.get().state_attr
            data-value=move || state.get().value_attr
            data-requirement=move || state.get().requirement_attr
            data-label-source=move || state.get().label_source_attr
            data-description-source=move || state.get().description_source_attr
            data-error-source=move || state.get().error_source_attr
            data-placeholder-source=move || state.get().placeholder_source_attr
            data-type-source=move || state.get().type_source_attr
            data-class-source=move || state.get().class_source_attr
            data-custom-class=move || state.get().has_custom_class_name.then_some("true")
        >
            <TextField
                id=id
                label=label
                value=value
                set_value=set_value
                disabled=disabled
                read_only=read_only
                required=required
                invalid=invalid
                description=description_for_inner
                error=error_for_inner
                placeholder=placeholder_for_inner
                input_type=input_type
                class_name=class_name_for_inner
                node_ref=node_ref
            />
        </div>
    }
}
