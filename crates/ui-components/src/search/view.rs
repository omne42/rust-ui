use crate::{SearchField, SearchFieldMotion};
use leptos::{html, prelude::*};

#[component]
pub fn Search(
    id: String,
    label: String,
    value: ReadSignal<String>,
    set_value: WriteSignal<String>,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] read_only: bool,
    #[prop(optional, into)] required: Signal<bool>,
    #[prop(optional, into)] invalid: Signal<bool>,
    #[prop(optional, into)] aria_describedby: Signal<Option<String>>,
    #[prop(optional, into)] description: Option<String>,
    #[prop(optional, into)] error: Option<String>,
    #[prop(optional, into)] placeholder: Option<String>,
    #[prop(optional)] on_submit: Option<Callback<String>>,
    #[prop(optional)] on_clear: Option<Callback<()>>,
    #[prop(optional)] motion: SearchFieldMotion,
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional)] node_ref: NodeRef<html::Input>,
) -> impl IntoView {
    let description = description.unwrap_or_default();
    let error = error.unwrap_or_default();
    let placeholder = placeholder.unwrap_or_default();
    let class_name = class_name.unwrap_or_default();
    let on_submit = on_submit.unwrap_or_else(|| Callback::new(|_: String| {}));
    let on_clear = on_clear.unwrap_or_else(|| Callback::new(|()| {}));

    view! {
        <SearchField
            id=id
            label=label
            value=value
            set_value=set_value
            disabled=disabled
            read_only=read_only
            required=required
            invalid=invalid
            aria_describedby=aria_describedby
            description=description
            error=error
            placeholder=placeholder
            on_submit=on_submit
            on_clear=on_clear
            motion=motion
            class_name=class_name
            node_ref=node_ref
        />
    }
}
