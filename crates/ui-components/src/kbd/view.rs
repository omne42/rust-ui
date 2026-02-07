use crate::kbd::{
    KbdSize,
    logic::{self, KbdStateInput},
};
use leptos::prelude::*;

#[component]
pub fn Kbd(
    #[prop(optional)] size: KbdSize,
    #[prop(optional, into)] keys: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
    children: Children,
) -> impl IntoView {
    let keys = logic::normalize_optional_text(keys);
    let class_name = logic::normalize_optional_text(class_name);
    let state = logic::resolve_state(KbdStateInput {
        size,
        has_keys: keys.is_some(),
        has_custom_class_name: class_name.is_some(),
    });
    let class = logic::compose_class_name(class_name, state);

    view! {
        <kbd
            class=class
            data-slot="kbd"
            data-size=state.size_attr
            data-state=state.state_attr
            data-keys=state.has_keys.then_some("true")
            data-custom-class=state.has_custom_class_name.then_some("true")
        >
            {keys.map(|keys| view! { <span class="ui-kbd__keys" data-slot="kbd-keys">{keys}</span> })}
            <span class="ui-kbd__label" data-slot="kbd-label">
                {children()}
            </span>
        </kbd>
    }
}
