use crate::TextField;
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
    let description = description.unwrap_or_default();
    let error = error.unwrap_or_default();
    let placeholder = placeholder.unwrap_or_default();
    let class_name = class_name.unwrap_or_default();
    let input_type = input_type.unwrap_or("text");

    view! {
        <TextField
            id=id
            label=label
            value=value
            set_value=set_value
            disabled=disabled
            read_only=read_only
            required=required
            invalid=invalid
            description=description
            error=error
            placeholder=placeholder
            input_type=input_type
            class_name=class_name
            node_ref=node_ref
        />
    }
}
