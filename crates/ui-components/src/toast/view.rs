use crate::OnPress;
use crate::toast::{ToastMotion, ToastStore, ToastStoreOptions, logic, motion};
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
    let id = logic::normalize_optional_text(id);
    let id = StoredValue::new(id);

    let title = logic::normalize_title(title);

    let description = logic::normalize_description(description);
    let has_description = description.is_some();
    let description = StoredValue::new(description);

    let class_name = logic::normalize_optional_text(class_name);
    let has_custom_class_name = class_name.is_some();
    let class_name = StoredValue::new(class_name);

    let on_close = StoredValue::new(on_close.unwrap_or_else(|| Callback::new(|_| {})));
    let on_exit_complete = on_exit_complete.unwrap_or_else(|| Callback::new(|_| {}));

    let class = Signal::derive(move || {
        logic::compose_toast_class_name(
            class_name.get_value(),
            variant,
            open.get(),
            has_description,
            has_custom_class_name,
        )
    });

    let state_attr = Signal::derive(move || if open.get() { "open" } else { "closing" });

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
            data-slot="toast"
            data-id=move || id.get_value()
            data-state=move || state_attr.get()
            data-variant=variant.class_name()
            data-description=if has_description { "present" } else { "absent" }
            data-open=move || open.get().then_some("true")
            data-custom-class=has_custom_class_name.then_some("true")
            node_ref=root_ref
            role="status"
            aria-live=variant.aria_live()
            aria-atomic="true"
            on:keydown=on_key_down
        >
            <div class="ui-toast__content" data-slot="toast-content">
                <div class="ui-toast__title" data-slot="toast-title">{title}</div>
                {move || {
                    description.get_value().map(|description| {
                        view! {
                            <div class="ui-toast__description" data-slot="toast-description">
                                {description}
                            </div>
                        }
                    })
                }}
            </div>
            <button
                type="button"
                class="ui-toast__close"
                data-slot="toast-close"
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
    #[prop(optional, default = true)] portal: bool,
    #[prop(optional, default = 3)] max_toasts: usize,
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional)] store: Option<ToastStore>,
) -> impl IntoView {
    let viewport_class_name = logic::compose_toast_viewport_class_name(class_name);
    let viewport_class_name = StoredValue::new(viewport_class_name);

    let store = store
        .or_else(crate::toast::use_toast_store)
        .unwrap_or_else(|| crate::toast::provide_toast_store(ToastStoreOptions { max_toasts }));
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

    if portal {
        view! {
            <Portal>
                <div
                    class=move || viewport_class_name.get_value()
                    data-ui-overlay-portal=""
                    data-slot="toast-viewport"
                >
                    <For each=move || items.get() key=|toast| toast.id.clone() children=render_item />
                </div>
            </Portal>
        }
        .into_any()
    } else {
        view! {
            <div class=move || viewport_class_name.get_value() data-slot="toast-viewport">
                <For each=move || items.get() key=|toast| toast.id.clone() children=render_item />
            </div>
        }
        .into_any()
    }
}
