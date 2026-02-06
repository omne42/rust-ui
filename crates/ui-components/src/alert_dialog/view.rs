use crate::alert_dialog::{
    AlertDialogAutoFocusButton, AlertDialogMotion, AlertDialogVariant, logic,
};
use crate::overlay::Overlay;
use crate::{Button, ButtonVariant, OnPress};
use leptos::{html, prelude::*};

#[cfg(target_arch = "wasm32")]
fn focus_button_soon(node_ref: NodeRef<html::Button>) {
    use leptos::wasm_bindgen::{JsCast, closure::Closure};

    fn try_focus(node_ref: &NodeRef<html::Button>) -> bool {
        let Some(el) = node_ref.get_untracked() else {
            return false;
        };
        let _ = el.focus();
        true
    }

    if try_focus(&node_ref) {
        return;
    }

    // Retry once on the next tick (mount/focus trap may run first).
    let Some(window) = web_sys::window() else {
        return;
    };

    let callback = Closure::once_into_js(move || {
        _ = try_focus(&node_ref);
    });

    let Some(callback) = callback.dyn_ref::<js_sys::Function>() else {
        return;
    };

    let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(callback, 0);
}

#[cfg(not(target_arch = "wasm32"))]
fn focus_button_soon(_node_ref: NodeRef<html::Button>) {}

#[component]
pub fn AlertDialog(
    open: Signal<bool>,
    id_base: String,
    title: String,
    on_close: OnPress,
    confirm_label: String,
    on_confirm: OnPress,
    #[prop(optional, into)] description: Option<String>,
    #[prop(optional, into)] cancel_label: Option<String>,
    #[prop(optional, into)] secondary_label: Option<String>,
    #[prop(optional)] on_secondary: Option<OnPress>,
    #[prop(optional)] on_cancel: Option<OnPress>,
    #[prop(optional)] confirm_disabled: bool,
    #[prop(optional)] secondary_disabled: bool,
    #[prop(optional)] auto_focus_button: AlertDialogAutoFocusButton,
    #[prop(optional)] variant: AlertDialogVariant,
    #[prop(optional)] motion: AlertDialogMotion,
    #[prop(optional)] on_exit_complete: Option<Callback<()>>,
) -> impl IntoView {
    let cancel_label = cancel_label.unwrap_or_else(|| "Cancel".to_string());
    let secondary_label = secondary_label.filter(|value| !value.trim().is_empty());

    let view_state = logic::resolve_view_state(
        description.as_deref(),
        &cancel_label,
        secondary_label.as_deref(),
    );

    let title = StoredValue::new(title);
    let description = StoredValue::new(description);
    let cancel_label = StoredValue::new(cancel_label);
    let secondary_label = StoredValue::new(secondary_label);
    let confirm_label = StoredValue::new(confirm_label);
    let class = StoredValue::new(format!("ui-alert-dialog {}", variant.class_name()));

    let on_cancel = StoredValue::new(on_cancel);
    let on_secondary = StoredValue::new(on_secondary);

    let title_id = format!("{id_base}-title");
    let description_id = format!("{id_base}-description");
    let title_id_attr: Signal<String> = title_id.clone().into();
    let description_id_attr: Signal<String> = description_id.clone().into();

    let on_exit_complete = on_exit_complete.unwrap_or_else(|| Callback::new(|_| {}));

    let on_close_for_cancel = on_close;
    let on_close_for_secondary = on_close;
    let on_close_for_confirm = on_close;
    let on_cancel_press: OnPress = Callback::new(move |_| {
        on_close_for_cancel.run(());
        if let Some(callback) = on_cancel.get_value() {
            callback.run(());
        }
    });

    let on_secondary_press: OnPress = Callback::new(move |_| {
        on_close_for_secondary.run(());
        if let Some(callback) = on_secondary.get_value() {
            callback.run(());
        }
    });

    let on_confirm_press: OnPress = Callback::new(move |_| {
        on_close_for_confirm.run(());
        on_confirm.run(());
    });

    let confirm_variant = match variant {
        AlertDialogVariant::Destructive => ButtonVariant::Destructive,
        AlertDialogVariant::Default
        | AlertDialogVariant::Confirmation
        | AlertDialogVariant::Warning
        | AlertDialogVariant::Error => ButtonVariant::Default,
    };

    let show_type_icon = matches!(
        variant,
        AlertDialogVariant::Warning | AlertDialogVariant::Error
    );

    let cancel_ref: NodeRef<html::Button> = NodeRef::new();
    let secondary_ref: NodeRef<html::Button> = NodeRef::new();
    let confirm_ref: NodeRef<html::Button> = NodeRef::new();

    let focus_state = StoredValue::new(false);
    Effect::new(move |_| {
        if !open.get() {
            focus_state.set_value(false);
            return;
        }
        if focus_state.get_value() {
            return;
        }
        focus_state.set_value(true);

        let target = match auto_focus_button {
            AlertDialogAutoFocusButton::Cancel if view_state.show_cancel => Some(cancel_ref),
            AlertDialogAutoFocusButton::Secondary if view_state.show_secondary => {
                Some(secondary_ref)
            }
            AlertDialogAutoFocusButton::Confirm => Some(confirm_ref),
            AlertDialogAutoFocusButton::None => None,
            _ => None,
        };

        if let Some(target) = target {
            focus_button_soon(target);
        }
    });

    let content = move || {
        view! {
            <div class=move || class.get_value() data-slot="alert-dialog">
                <div class="ui-alert-dialog__header" data-slot="alert-dialog-header">
                    <Show when=move || show_type_icon>
                        <span class="ui-alert-dialog__type-icon" data-slot="alert-dialog-type-icon" aria-hidden="true">
                            {match variant {
                                AlertDialogVariant::Warning => view! {
                                    <svg viewBox="0 0 20 20" fill="none">
                                        <path
                                            d="M10 2.8l8.2 14.4c.6 1-.1 2.3-1.3 2.3H3.1c-1.2 0-1.9-1.3-1.3-2.3L10 2.8z"
                                            stroke="currentColor"
                                            stroke_width="1.5"
                                            stroke_linejoin="round"
                                        />
                                        <path
                                            d="M10 7.2v5.8"
                                            stroke="currentColor"
                                            stroke_width="1.5"
                                            stroke_linecap="round"
                                        />
                                        <path
                                            d="M10 15.8h.01"
                                            stroke="currentColor"
                                            stroke_width="2.5"
                                            stroke_linecap="round"
                                        />
                                    </svg>
                                }
                                .into_any(),
                                AlertDialogVariant::Error => view! {
                                    <svg viewBox="0 0 20 20" fill="none">
                                        <path
                                            d="M10 18a8 8 0 1 0 0-16 8 8 0 0 0 0 16z"
                                            stroke="currentColor"
                                            stroke_width="1.5"
                                        />
                                        <path
                                            d="M10 6.2v5.2"
                                            stroke="currentColor"
                                            stroke_width="1.5"
                                            stroke_linecap="round"
                                        />
                                        <path
                                            d="M10 14.2h.01"
                                            stroke="currentColor"
                                            stroke_width="2.5"
                                            stroke_linecap="round"
                                        />
                                    </svg>
                                }
                                .into_any(),
                                _ => ().into_any(),
                            }}
                        </span>
                    </Show>

                    <div class="ui-alert-dialog__header-text" data-slot="alert-dialog-header-text">
                        <h2 class="ui-alert-dialog__title" id=move || title_id_attr.get() data-slot="alert-dialog-title">
                            {move || title.get_value()}
                        </h2>
                        <Show when=move || view_state.show_description>
                            <p
                                class="ui-alert-dialog__description"
                                id=move || description_id_attr.get()
                                data-slot="alert-dialog-description"
                            >
                                {move || description.get_value().unwrap_or_default()}
                            </p>
                        </Show>
                    </div>
                </div>

                <div class="ui-alert-dialog__footer" data-slot="alert-dialog-footer">
                    <Show when=move || view_state.show_cancel>
                        <Button
                            variant=ButtonVariant::Secondary
                            disabled=false
                            node_ref=cancel_ref
                            on_press=on_cancel_press
                        >
                            {move || cancel_label.get_value()}
                        </Button>
                    </Show>
                    <Show when=move || view_state.show_secondary>
                        <Button
                            variant=ButtonVariant::Secondary
                            disabled=secondary_disabled
                            node_ref=secondary_ref
                            on_press=on_secondary_press
                        >
                            {move || secondary_label.get_value().unwrap_or_default()}
                        </Button>
                    </Show>
                    <Button
                        variant=confirm_variant
                        disabled=confirm_disabled
                        node_ref=confirm_ref
                        on_press=on_confirm_press
                    >
                        {move || confirm_label.get_value()}
                    </Button>
                </div>
            </div>
        }
    };

    if view_state.show_description {
        view! {
            <Overlay
                open=open
                on_close=on_close
                role="alertdialog"
                aria_labelledby=title_id.clone()
                aria_describedby=description_id.clone()
                motion=motion.overlay
                on_exit_complete=on_exit_complete
            >
                {content()}
            </Overlay>
        }
    } else {
        view! {
            <Overlay
                open=open
                on_close=on_close
                role="alertdialog"
                aria_labelledby=title_id.clone()
                motion=motion.overlay
                on_exit_complete=on_exit_complete
            >
                {content()}
            </Overlay>
        }
    }
}
