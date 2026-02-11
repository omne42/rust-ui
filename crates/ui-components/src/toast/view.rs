use crate::OnPress;
use crate::toast::{
    ToastMotion, ToastPartStateInput, ToastSlot, ToastStore, ToastStoreOptions, ToastStoreSource,
    ToastViewportSlot, ToastViewportStateInput, logic, motion,
};
use leptos::{ev, html, portal::Portal, prelude::*};

#[component]
pub fn Toast(
    title: String,
    open: Signal<bool>,
    #[prop(optional, into)] id: Option<String>,
    #[prop(optional, into)] description: Option<String>,
    #[prop(optional)] variant: crate::toast::ToastVariant,
    #[prop(optional)] motion: ToastMotion,
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional)] on_close: Option<OnPress>,
    #[prop(optional)] on_exit_complete: Option<Callback<()>>,
) -> impl IntoView {
    let motion = crate::toast::motion::sanitize_motion(motion);
    let id = logic::normalize_optional_text(id);
    let has_custom_id = id.is_some();
    let id = StoredValue::new(id);

    let title = logic::normalize_title(title);

    let description = logic::normalize_description(description);
    let has_description = description.is_some();
    let has_custom_description = has_description;
    let description = StoredValue::new(description);

    let class_name = logic::normalize_optional_text(class_name);
    let has_custom_class_name = class_name.is_some();
    let class_name = StoredValue::new(class_name);

    let has_custom_on_close = on_close.is_some();
    let on_close = StoredValue::new(on_close.unwrap_or_else(|| Callback::new(|_| {})));

    let has_custom_on_exit_complete = on_exit_complete.is_some();
    let on_exit_complete = on_exit_complete.unwrap_or_else(|| Callback::new(|_| {}));

    let has_custom_motion = motion != ToastMotion::default();

    let state = Memo::new(move |_| {
        logic::resolve_state(ToastPartStateInput {
            slot: ToastSlot::Root,
            variant,
            is_open: open.get(),
            has_description,
            has_custom_id,
            has_custom_description,
            has_custom_class_name,
            has_custom_motion,
            has_custom_on_close,
            has_custom_on_exit_complete,
        })
    });

    let class = Memo::new(move |_| logic::compose_class_name(class_name.get_value(), state.get()));

    let root_ref: NodeRef<html::Div> = NodeRef::new();
    motion::attach_motion(root_ref, open, on_exit_complete, motion);

    let on_key_down = move |ev: ev::KeyboardEvent| {
        if ev.key() == "Escape" {
            on_close.get_value().run(());
        }
    };

    view! {
        <div
            class=move || class.get()
            data-slot=move || state.get().slot_attr
            data-id=move || id.get_value()
            data-state=move || state.get().state_attr
            data-variant=move || state.get().variant_attr
            data-description=move || state.get().description_attr
            data-open=move || state.get().open_attr
            data-close-mode=move || state.get().close_mode_attr
            data-id-source=move || state.get().id_source_attr
            data-description-source=move || state.get().description_source_attr
            data-class-source=move || state.get().class_source_attr
            data-motion-source=move || state.get().motion_source_attr
            data-close-source=move || state.get().close_source_attr
            data-exit-source=move || state.get().exit_source_attr
            data-custom-id=move || state.get().has_custom_id.then_some("true")
            data-custom-description=move || state.get().has_custom_description.then_some("true")
            data-custom-class=move || state.get().has_custom_class_name.then_some("true")
            data-custom-motion=move || state.get().has_custom_motion.then_some("true")
            data-custom-close=move || state.get().has_custom_on_close.then_some("true")
            data-custom-exit=move || state.get().has_custom_on_exit_complete.then_some("true")
            node_ref=root_ref
            role="status"
            aria-live=variant.aria_live()
            aria-atomic="true"
            on:keydown=on_key_down
        >
            <div class="ui-toast__content" data-slot=ToastSlot::Content.as_attr()>
                <div class="ui-toast__title" data-slot=ToastSlot::Title.as_attr()>{title}</div>
                {move || {
                    description.get_value().map(|description| {
                        view! {
                            <div class="ui-toast__description" data-slot=ToastSlot::Description.as_attr()>
                                {description}
                            </div>
                        }
                    })
                }}
            </div>
            <button
                type="button"
                class="ui-toast__close"
                data-slot=ToastSlot::Close.as_attr()
                aria-label="Dismiss toast"
                on:click=move |_| on_close.get_value().run(())
            >
                "×"
            </button>
        </div>
    }
}

#[component]
pub fn ToastViewport(
    #[prop(optional)] motion: ToastMotion,
    #[prop(optional, default = logic::DEFAULT_VIEWPORT_PORTAL)] portal: bool,
    #[prop(optional, default = logic::DEFAULT_VIEWPORT_MAX_TOASTS)] max_toasts: usize,
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional)] store: Option<ToastStore>,
) -> impl IntoView {
    let motion = crate::toast::motion::sanitize_motion(motion);
    let class_name = logic::normalize_optional_text(class_name);
    let has_custom_class_name = class_name.is_some();

    let has_custom_portal = portal != logic::DEFAULT_VIEWPORT_PORTAL;
    let has_custom_max_toasts = max_toasts != logic::DEFAULT_VIEWPORT_MAX_TOASTS;
    let has_custom_motion = motion != ToastMotion::default();

    let normalized_max_toasts = logic::normalize_viewport_max_toasts(max_toasts);
    let (store, store_source) = if let Some(provided_store) = store {
        (provided_store, ToastStoreSource::Provided)
    } else if let Some(context_store) = crate::toast::use_toast_store() {
        (context_store, ToastStoreSource::Context)
    } else {
        (
            crate::toast::provide_toast_store(ToastStoreOptions {
                max_toasts: normalized_max_toasts,
            }),
            ToastStoreSource::Local,
        )
    };

    let viewport_state = logic::resolve_viewport_state(ToastViewportStateInput {
        slot: ToastViewportSlot::Root,
        portal,
        max_toasts,
        has_custom_portal,
        has_custom_max_toasts,
        has_custom_class_name,
        has_custom_motion,
        store_source,
    });

    let viewport_class_name = logic::compose_viewport_class_name(class_name, viewport_state);
    let viewport_class_name = StoredValue::new(viewport_class_name);
    let viewport_state = StoredValue::new(viewport_state);

    let store = StoredValue::new(store);
    let items = Signal::derive(move || store.get_value().toasts().get());

    let render_item = move |toast: crate::toast::ToastInstance| {
        let id = toast.id.clone();
        let title = toast.title.clone();
        let description = toast.description.clone();
        let variant = toast.variant;
        let open: Signal<bool> = toast.open.into();

        let on_exit_complete = {
            let store = store.get_value();
            let id = id.clone();
            Callback::new(move |_| store.remove(&id))
        };

        let on_close = Callback::new({
            let toast = toast.clone();
            move |_| toast.set_open.set(false)
        });

        view! {
            <Toast
                id=id
                title=title
                description=description.unwrap_or_default()
                variant=variant
                open=open
                motion=motion
                on_close=on_close
                on_exit_complete=on_exit_complete
            />
        }
    };

    if viewport_state.get_value().portal {
        view! {
            <Portal>
                <div
                    class=move || viewport_class_name.get_value()
                    data-ui-overlay-portal=""
                    data-slot=move || viewport_state.get_value().slot_attr
                    data-state=move || viewport_state.get_value().state_attr
                    data-queue=move || viewport_state.get_value().queue_attr
                    data-portal=move || viewport_state.get_value().portal_attr
                    data-max-toasts=move || viewport_state.get_value().max_toasts.to_string()
                    data-portal-source=move || viewport_state.get_value().portal_source_attr
                    data-max-toasts-source=move || viewport_state.get_value().max_toasts_source_attr
                    data-class-source=move || viewport_state.get_value().class_source_attr
                    data-motion-source=move || viewport_state.get_value().motion_source_attr
                    data-store-source=move || viewport_state.get_value().store_source_attr
                    data-custom-portal=move || viewport_state.get_value().has_custom_portal.then_some("true")
                    data-custom-max-toasts=move || viewport_state.get_value().has_custom_max_toasts.then_some("true")
                    data-custom-class=move || viewport_state.get_value().has_custom_class_name.then_some("true")
                    data-custom-motion=move || viewport_state.get_value().has_custom_motion.then_some("true")
                >
                    <For each=move || items.get() key=|toast| toast.id.clone() children=render_item />
                </div>
            </Portal>
        }
        .into_any()
    } else {
        view! {
            <div
                class=move || viewport_class_name.get_value()
                data-slot=move || viewport_state.get_value().slot_attr
                data-state=move || viewport_state.get_value().state_attr
                data-queue=move || viewport_state.get_value().queue_attr
                data-portal=move || viewport_state.get_value().portal_attr
                data-max-toasts=move || viewport_state.get_value().max_toasts.to_string()
                data-portal-source=move || viewport_state.get_value().portal_source_attr
                data-max-toasts-source=move || viewport_state.get_value().max_toasts_source_attr
                data-class-source=move || viewport_state.get_value().class_source_attr
                data-motion-source=move || viewport_state.get_value().motion_source_attr
                data-store-source=move || viewport_state.get_value().store_source_attr
                data-custom-portal=move || viewport_state.get_value().has_custom_portal.then_some("true")
                data-custom-max-toasts=move || viewport_state.get_value().has_custom_max_toasts.then_some("true")
                data-custom-class=move || viewport_state.get_value().has_custom_class_name.then_some("true")
                data-custom-motion=move || viewport_state.get_value().has_custom_motion.then_some("true")
            >
                <For each=move || items.get() key=|toast| toast.id.clone() children=render_item />
            </div>
        }
        .into_any()
    }
}
