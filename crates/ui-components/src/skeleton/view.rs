use crate::skeleton::SkeletonVariant;
use leptos::prelude::*;

#[component]
pub fn Skeleton(
    #[prop(optional)] variant: SkeletonVariant,
    #[prop(optional, default = true)] shimmer: bool,
    #[prop(optional, into)] class_name: Option<String>,
) -> impl IntoView {
    let base_class = format!("ui-skeleton {}", variant.class_name());
    let class = class_name
        .filter(|value| !value.trim().is_empty())
        .map(|value| format!("{base_class} {value}"))
        .unwrap_or(base_class);

    view! {
        <div
            class=class
            class:ui-skeleton--shimmer=shimmer
            data-slot="skeleton"
            aria-hidden="true"
        ></div>
    }
}
