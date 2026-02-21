use super::{
    KbdSize,
    logic::{self, KbdLogicInput},
};
use leptos::prelude::*;

fn render_keys_slot(keys: Option<String>) -> impl IntoView {
    keys.map(|keys| view! { <span class="ui-kbd__keys" data-slot="kbd-keys">{keys}</span> })
}

#[component]
pub fn Kbd(
    #[prop(optional)] size: Option<KbdSize>,
    #[prop(optional, into)] keys: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
    children: Children,
) -> impl IntoView {
    let view_model = logic::resolve_view_model(KbdLogicInput {
        size,
        keys,
        class_name,
    });

    view! {
        <kbd
            class=view_model.class
            data-slot="kbd"
            data-size=view_model.state.size_attr
            data-state=view_model.state.state_attr
            data-keys=view_model.state.has_keys.then_some("true")
            data-custom-class=view_model.state.has_custom_class_name.then_some("true")
        >
            {render_keys_slot(view_model.keys)}
            <span class="ui-kbd__label" data-slot="kbd-label">
                {children()}
            </span>
        </kbd>
    }
}
