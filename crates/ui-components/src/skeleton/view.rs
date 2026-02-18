use crate::skeleton::{SkeletonVariant, logic};
use leptos::prelude::*;

#[component]
pub fn Skeleton(
    #[prop(optional)] variant: Option<SkeletonVariant>,
    #[prop(optional)] is_shimmer: Option<bool>,
    #[prop(optional, into)] class_name: Option<String>,
) -> impl IntoView {
    let class_name = logic::normalize_optional_text(class_name);
    let state_input = logic::normalize_state_input(logic::SkeletonViewInput {
        variant,
        is_shimmer,
        has_custom_class_name: class_name.is_some(),
    });
    let state = logic::resolve_state(state_input);
    let class = logic::compose_class_name(class_name, state);

    view! {
        <div
            class=class
            data-slot="skeleton"
            data-variant=state.variant_attr
            data-state=state.state_attr
            data-shimmer=state.has_shimmer.then_some("true")
            data-still=state.is_still.then_some("true")
            data-custom-class=state.has_custom_class_name.then_some("true")
            aria-hidden="true"
        ></div>
    }
}
