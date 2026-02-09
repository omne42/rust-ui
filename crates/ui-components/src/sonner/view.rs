use crate::sonner::logic;
use crate::toast::{ToastMotion, ToastStore, ToastStoreOptions, ToastViewport};
use leptos::prelude::*;

#[component]
pub fn Sonner(
    #[prop(optional)] position: crate::sonner::SonnerPosition,
    #[prop(optional, default = true)] portal: bool,
    #[prop(optional, default = 3)] max_toasts: usize,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional)] motion: ToastMotion,
    #[prop(optional)] store: Option<ToastStore>,
) -> impl IntoView {
    let class_name = logic::normalize_optional_text(class_name);
    let (aria_label, has_custom_aria_label) = logic::normalize_aria_label(aria_label);

    let state = logic::resolve_state(logic::SonnerStateInput {
        position,
        portal,
        max_toasts,
        has_custom_aria_label,
        has_custom_class_name: class_name.is_some(),
    });

    let root_class_name = logic::compose_root_class_name(class_name, &state);
    let viewport_class_name = logic::compose_viewport_class_name(state.position);
    let store = store
        .or_else(crate::toast::use_toast_store)
        .unwrap_or_else(|| {
            crate::toast::provide_toast_store(ToastStoreOptions {
                max_toasts: state.max_toasts,
            })
        });

    view! {
        <section
            class=root_class_name
            data-slot="sonner"
            data-position=state.position_attr
            data-portal=state.portal_attr
            data-max-toasts=state.max_toasts.to_string()
            data-aria-source=state.aria_source_attr
            data-class-source=state.class_source_attr
            role="region"
            aria-label=aria_label
        >
            <ToastViewport
                store=store
                motion=motion
                portal=state.portal
                max_toasts=state.max_toasts
                class_name=viewport_class_name
            />
        </section>
    }
}
