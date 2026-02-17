use super::{
    SkeletonGroupDensity, SkeletonGroupLayout, SkeletonGroupStateInput, SkeletonGroupVariant, logic,
};
use leptos::prelude::*;

#[component]
pub fn SkeletonGroup(
    #[prop(optional, default = true)] is_loading: bool,
    #[prop(optional)] is_skeleton_only: bool,
    #[prop(optional)] variant: SkeletonGroupVariant,
    #[prop(optional)] layout: SkeletonGroupLayout,
    #[prop(optional)] density: SkeletonGroupDensity,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
    children: Children,
) -> impl IntoView {
    let (aria_label, has_custom_aria_label) = logic::normalize_aria_label(aria_label);
    let class_name = logic::normalize_optional_text(class_name);

    let state = logic::resolve_state(SkeletonGroupStateInput {
        is_loading,
        is_skeleton_only,
        variant,
        layout,
        density,
        has_custom_aria_label,
        has_custom_class_name: class_name.is_some(),
    });

    let class = logic::compose_class_name(class_name, state);

    view! {
        <div
            class=class
            data-slot="skeleton-group"
            data-state=state.state_attr
            data-visibility=state.visibility_attr
            data-loading-mode=state.loading_mode_attr
            data-variant=state.variant_attr
            data-layout=state.layout_attr
            data-density=state.density_attr
            data-loading=state.is_loading.then_some("true")
            data-loaded=state.is_loaded.then_some("true")
            data-skeleton-only=state.is_skeleton_only.then_some("true")
            data-label-source=state.label_source_attr
            data-custom-class=state.has_custom_class_name.then_some("true")
            data-class-source=state.class_source_attr
            role="group"
            hidden=state.should_hide_root
            aria-label=aria_label
            aria-busy=state.is_loading.then_some("true")
        >
            {children()}
        </div>
    }
    .into_any()
}
