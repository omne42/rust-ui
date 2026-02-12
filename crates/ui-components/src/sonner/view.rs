use crate::sonner::{SonnerPartStateInput, SonnerPosition, SonnerSlot, SonnerStoreSource, logic};
use crate::toast::{ToastMotion, ToastStore, ToastStoreOptions, ToastViewport};
use leptos::prelude::*;

#[component]
pub fn Sonner(
    #[prop(optional)] position: SonnerPosition,
    #[prop(optional, default = logic::DEFAULT_PORTAL)] portal: bool,
    #[prop(optional, default = logic::DEFAULT_MAX_TOASTS)] max_toasts: usize,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional)] motion: ToastMotion,
    #[prop(optional)] store: Option<ToastStore>,
) -> impl IntoView {
    let class_name = logic::normalize_optional_text(class_name);
    let has_custom_class_name = class_name.is_some();
    let class_name = StoredValue::new(class_name);

    let (aria_label, has_custom_aria_label) = logic::normalize_aria_label(aria_label);
    let aria_label = StoredValue::new(aria_label);

    let has_custom_position = position != SonnerPosition::default();
    let has_custom_portal = portal != logic::DEFAULT_PORTAL;
    let has_custom_max_toasts = max_toasts != logic::DEFAULT_MAX_TOASTS;
    let has_custom_motion = motion != ToastMotion::default();

    let normalized_max_toasts = logic::normalize_max_toasts(max_toasts);
    let (store, store_source) = if let Some(provided_store) = store {
        (provided_store, SonnerStoreSource::Provided)
    } else if let Some(context_store) = crate::toast::use_toast_store() {
        (context_store, SonnerStoreSource::Context)
    } else {
        (
            crate::toast::provide_toast_store(ToastStoreOptions {
                max_toasts: normalized_max_toasts,
            }),
            SonnerStoreSource::Local,
        )
    };

    let root_state = logic::resolve_state(SonnerPartStateInput {
        slot: SonnerSlot::Root,
        position,
        portal,
        max_toasts,
        has_custom_position,
        has_custom_portal,
        has_custom_max_toasts,
        has_custom_aria_label,
        has_custom_class_name,
        has_custom_motion,
        store_source,
    });

    let viewport_state = logic::resolve_state(SonnerPartStateInput {
        slot: SonnerSlot::Viewport,
        position,
        portal,
        max_toasts,
        has_custom_position,
        has_custom_portal,
        has_custom_max_toasts,
        has_custom_aria_label,
        has_custom_class_name: false,
        has_custom_motion,
        store_source,
    });

    let root_class_name = logic::compose_class_name(class_name.get_value(), root_state);
    let viewport_class_name = logic::compose_class_name(None, viewport_state);

    view! {
        <section
            class=root_class_name
            data-slot=root_state.slot_attr
            data-state=root_state.state_attr
            data-queue=root_state.queue_attr
            data-position=root_state.position_attr
            data-portal=root_state.portal_attr
            data-max-toasts=root_state.max_toasts.to_string()
            data-position-source=root_state.position_source_attr
            data-portal-source=root_state.portal_source_attr
            data-max-toasts-source=root_state.max_toasts_source_attr
            data-aria-source=root_state.aria_source_attr
            data-class-source=root_state.class_source_attr
            data-motion-source=root_state.motion_source_attr
            data-store-source=root_state.store_source_attr
            data-custom-position=root_state.has_custom_position.then_some("true")
            data-custom-portal=root_state.has_custom_portal.then_some("true")
            data-custom-max-toasts=root_state.has_custom_max_toasts.then_some("true")
            data-custom-motion=root_state.has_custom_motion.then_some("true")
            data-custom-class=root_state.has_custom_class_name.then_some("true")
            data-custom-aria=root_state.has_custom_aria_label.then_some("true")
            data-viewport-slot=viewport_state.slot_attr
            data-viewport-state=viewport_state.state_attr
            data-viewport-position=viewport_state.position_attr
            data-viewport-portal=viewport_state.portal_attr
            data-viewport-queue=viewport_state.queue_attr
            role="region"
            aria-label=aria_label.get_value()
        >
            <ToastViewport
                store=store
                motion=motion
                portal=viewport_state.portal
                max_toasts=viewport_state.max_toasts
                class_name=viewport_class_name
            />
        </section>
    }
}
