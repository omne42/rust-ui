use crate::sonner::Sonner;
use crate::toast::ToastStore;
use crate::toast::{ToastMotion, ToastStoreOptions};
use crate::toaster::{
    ToasterPartStateInput, ToasterPosition, ToasterSlot, ToasterStoreSource, logic,
};
use leptos::prelude::*;
use ui_headless::{A11yDirection, region_attrs};

#[component]
pub fn Toaster(
    #[prop(optional)] position: ToasterPosition,
    #[prop(optional, default = logic::DEFAULT_PORTAL)] portal: bool,
    #[prop(optional, default = logic::DEFAULT_MAX_TOASTS)] max_toasts: usize,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional, into)] lang: Option<String>,
    #[prop(optional)] dir: Option<A11yDirection>,
    #[prop(optional)] motion: ToastMotion,
    #[prop(optional)] store: Option<ToastStore>,
) -> impl IntoView {
    let motion = crate::toaster::motion::sanitize_motion(motion);
    let normalized = logic::normalize_props(logic::ToasterNormalizeInput {
        position,
        portal,
        max_toasts,
        aria_label,
        class_name,
        motion,
    });
    let class_name = StoredValue::new(normalized.class_name);
    let region_a11y = region_attrs(normalized.aria_label, lang, dir);

    let (store, store_source) = if let Some(provided_store) = store {
        (provided_store, ToasterStoreSource::Provided)
    } else if let Some(context_store) = crate::toast::use_toast_store() {
        (context_store, ToasterStoreSource::Context)
    } else {
        (
            crate::toast::provide_toast_store(ToastStoreOptions {
                max_toasts: normalized.max_toasts,
            }),
            ToasterStoreSource::Local,
        )
    };

    let root_state = logic::resolve_state(ToasterPartStateInput {
        slot: ToasterSlot::Root,
        position: normalized.position,
        portal: normalized.portal,
        max_toasts: normalized.max_toasts,
        has_custom_position: normalized.has_custom_position,
        has_custom_portal: normalized.has_custom_portal,
        has_custom_max_toasts: normalized.has_custom_max_toasts,
        has_custom_aria_label: normalized.has_custom_aria_label,
        has_custom_class_name: normalized.has_custom_class_name,
        has_custom_motion: normalized.has_custom_motion,
        store_source,
    });

    let sonner_state = logic::resolve_state(ToasterPartStateInput {
        slot: ToasterSlot::Sonner,
        position: normalized.position,
        portal: normalized.portal,
        max_toasts: normalized.max_toasts,
        has_custom_position: normalized.has_custom_position,
        has_custom_portal: normalized.has_custom_portal,
        has_custom_max_toasts: normalized.has_custom_max_toasts,
        has_custom_aria_label: normalized.has_custom_aria_label,
        has_custom_class_name: false,
        has_custom_motion: normalized.has_custom_motion,
        store_source,
    });

    let root_class_name = logic::compose_class_name(class_name.get_value(), root_state);
    let sonner_class_name = logic::compose_class_name(None, sonner_state);
    let sonner_position = logic::map_to_sonner_position(root_state.position);
    let agent_contract = logic::agent_contract();

    view! {
        <section
            class=root_class_name
            data-ui-schema=agent_contract.schema_attr
            data-ui-intent=agent_contract.intent_attr
            data-ui-action-model=agent_contract.action_model_attr
            data-ui-stream-support=agent_contract.stream_support_attr
            data-ui-stream-fallback=agent_contract.stream_fallback_attr
            data-ui-output-status=agent_contract.output_status_attr
            data-ui-state-axis=agent_contract.state_axis_attr
            data-ui-source-axis=agent_contract.source_axis_attr
            data-slot=root_state.slot_attr
            data-state=root_state.state_attr
            data-queue=root_state.queue_attr
            data-position=root_state.position_attr
            data-portal=root_state.portal_attr
            data-max-toasts=root_state.max_toasts
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
            data-sonner-slot=sonner_state.slot_attr
            data-sonner-state=sonner_state.state_attr
            data-sonner-position=sonner_state.position_attr
            data-sonner-portal=sonner_state.portal_attr
            data-sonner-queue=sonner_state.queue_attr
            role=region_a11y.role
            aria-label=region_a11y.aria_label
            lang=region_a11y.lang
            dir=region_a11y.dir
        >
            <Sonner
                store=store
                position=sonner_position
                portal=sonner_state.portal
                max_toasts=sonner_state.max_toasts
                motion=motion
                class_name=sonner_class_name
            />
        </section>
    }
}
