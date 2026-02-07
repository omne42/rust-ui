use crate::code::{
    CodeVariant,
    logic::{self, CodeStateInput},
};
use leptos::prelude::*;

#[component]
pub fn Code(
    #[prop(optional)] variant: CodeVariant,
    #[prop(optional, into)] class_name: Option<String>,
    children: Children,
) -> impl IntoView {
    let class_name = logic::normalize_optional_text(class_name);
    let state = logic::resolve_state(CodeStateInput {
        variant,
        has_custom_class_name: class_name.is_some(),
    });
    let class = logic::compose_class_name(class_name, state);

    view! {
        <code
            class=class
            data-slot="code"
            data-variant=state.variant_attr
            data-state=state.state_attr
            data-inline=state.is_inline.then_some("true")
            data-block=state.is_block.then_some("true")
            data-custom-class=state.has_custom_class_name.then_some("true")
        >
            {children()}
        </code>
    }
}
