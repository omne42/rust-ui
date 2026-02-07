use crate::skeleton::{
    SkeletonVariant,
    logic::{self, SkeletonStateInput},
};
use leptos::prelude::*;

#[component]
pub fn Skeleton(
    #[prop(optional)] variant: SkeletonVariant,
    #[prop(optional, default = true)] shimmer: bool,
    #[prop(optional, into)] class_name: Option<String>,
) -> impl IntoView {
    let class_name = logic::normalize_optional_text(class_name);
    let state = logic::resolve_state(SkeletonStateInput {
        variant,
        shimmer,
        has_custom_class_name: class_name.is_some(),
    });
    let class = logic::compose_class_name(class_name, state);

    view! {
        <div
            class=class
            data-slot="skeleton"
            data-variant=state.variant_attr
            data-state=if state.has_shimmer { "shimmer" } else { "still" }
            data-shimmer=state.has_shimmer.then_some("true")
            data-still=state.is_still.then_some("true")
            data-custom-class=state.has_custom_class_name.then_some("true")
            aria-hidden="true"
        ></div>
    }
}
