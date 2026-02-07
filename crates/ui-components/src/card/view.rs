use crate::card::{
    CardVariant,
    logic::{self, CardStateInput},
};
use leptos::prelude::*;

#[component]
pub fn Card(
    #[prop(optional)] variant: CardVariant,
    #[prop(optional, default = true)] padded: bool,
    #[prop(optional, into)] class_name: Option<String>,
    children: Children,
) -> impl IntoView {
    let class_name = logic::normalize_optional_text(class_name);
    let state = logic::resolve_state(CardStateInput {
        variant,
        padded,
        has_custom_class_name: class_name.is_some(),
    });
    let class = logic::compose_class_name(class_name, state);

    view! {
        <section
            class=class
            data-slot="card"
            data-variant=state.variant_attr
            data-state=if state.is_padded { "padded" } else { "flush" }
            data-padded=state.is_padded.then_some("true")
            data-flush=state.is_flush.then_some("true")
            data-custom-class=state.has_custom_class_name.then_some("true")
        >
            {children()}
        </section>
    }
}
