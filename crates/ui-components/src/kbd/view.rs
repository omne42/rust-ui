use crate::kbd::KbdSize;
use leptos::prelude::*;

#[component]
pub fn Kbd(
    #[prop(optional)] size: KbdSize,
    #[prop(optional, into)] keys: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
    children: Children,
) -> impl IntoView {
    let keys = keys.filter(|value| !value.trim().is_empty());

    let base_class = format!("ui-kbd {}", size.class_name());
    let class = class_name
        .filter(|value| !value.trim().is_empty())
        .map(|value| format!("{base_class} {value}"))
        .unwrap_or(base_class);

    view! {
        <kbd class=class data-slot="kbd">
            {keys.map(|keys| view! { <span class="ui-kbd__keys" data-slot="kbd-keys">{keys}</span> })}
            {children()}
        </kbd>
    }
}
