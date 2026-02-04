use crate::toast::{ToastMotion, ToastStore, ToastStoreOptions, motion};
use leptos::{ev, html, portal::Portal, prelude::*};

#[component]
pub fn ToastViewport(
    #[prop(optional)] motion: ToastMotion,
    #[prop(optional, default = true)] portal: bool,
    #[prop(optional, default = 3)] max_toasts: usize,
) -> impl IntoView {
    let store = crate::toast::use_toast_store()
        .unwrap_or_else(|| crate::toast::provide_toast_store(ToastStoreOptions { max_toasts }));
    let store = StoredValue::new(store);

    let items = Signal::derive(move || store.get_value().toasts().get());

    if portal {
        view! {
            <Portal>
                <div class="ui-toast-viewport" data-slot="toast-viewport">
                    <For
                        each=move || items.get()
                        key=|toast| toast.id.clone()
                        children=move |toast| {
                            view! { <ToastItem toast=toast store=store.get_value() motion=motion /> }
                        }
                    />
                </div>
            </Portal>
        }
        .into_any()
    } else {
        view! {
            <div class="ui-toast-viewport" data-slot="toast-viewport">
                <For
                    each=move || items.get()
                    key=|toast| toast.id.clone()
                    children=move |toast| {
                        view! { <ToastItem toast=toast store=store.get_value() motion=motion /> }
                    }
                />
            </div>
        }
        .into_any()
    }
}

#[component]
fn ToastItem(
    toast: crate::toast::ToastInstance,
    store: ToastStore,
    motion: ToastMotion,
) -> impl IntoView {
    let id = toast.id.clone();
    let title = toast.title.clone();
    let description = toast.description.clone();
    let variant = toast.variant;

    let root_ref: NodeRef<html::Div> = NodeRef::new();
    let open: Signal<bool> = toast.open.into();

    let on_exit_complete = {
        let store = store.clone();
        let id = id.clone();
        Callback::new(move |_| store.remove(&id))
    };
    motion::attach_motion(root_ref, open, on_exit_complete, motion);

    let on_close = Callback::new({
        let toast = toast.clone();
        move |_| toast.set_open.set(false)
    });

    let on_key_down = move |ev: ev::KeyboardEvent| {
        if ev.key() == "Escape" {
            on_close.run(());
        }
    };

    view! {
        <div
            class=format!("ui-toast {}", variant.class_name())
            data-slot="toast"
            node_ref=root_ref
            role="status"
            aria-live=variant.aria_live()
            aria-atomic="true"
            on:keydown=on_key_down
        >
            <div class="ui-toast__content" data-slot="toast-content">
                <div class="ui-toast__title" data-slot="toast-title">{title}</div>
                {description.filter(|v| !v.trim().is_empty()).map(|description| view! {
                    <div class="ui-toast__description" data-slot="toast-description">{description}</div>
                })}
            </div>
            <button
                type="button"
                class="ui-toast__close"
                data-slot="toast-close"
                aria-label="Dismiss toast"
                on:click=move |_| on_close.run(())
            >
                "×"
            </button>
        </div>
    }
}
