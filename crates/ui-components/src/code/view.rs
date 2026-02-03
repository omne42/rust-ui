use crate::code::CodeVariant;
use leptos::prelude::*;

#[component]
pub fn Code(
    #[prop(optional)] variant: CodeVariant,
    #[prop(optional, into)] class_name: Option<String>,
    children: Children,
) -> impl IntoView {
    let base_class = format!("ui-code {}", variant.class_name());
    let class = class_name
        .filter(|value| !value.trim().is_empty())
        .map(|value| format!("{base_class} {value}"))
        .unwrap_or(base_class);

    view! {
        <code class=class data-slot="code">
            {children()}
        </code>
    }
}
