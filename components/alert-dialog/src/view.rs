use crate::OnPress;
use crate::alert_dialog::{
    AlertDialogAutoFocusButton, AlertDialogMotion, AlertDialogPartStateInput, AlertDialogSlot,
    AlertDialogVariant, logic,
};
use crate::button::{Button, ButtonVariant};
use crate::overlay::Overlay;
use leptos::{html, prelude::*};
use std::sync::Arc;
use ui_headless::{A11yDirection, locale_attrs};

#[cfg(target_arch = "wasm32")]
fn focus_button_soon(node_ref: NodeRef<html::Button>) {
    use leptos::wasm_bindgen::{JsCast, closure::Closure};

    fn try_focus(node_ref: &NodeRef<html::Button>) -> bool {
        let Some(el) = node_ref.get_untracked() else {
            return false;
        };
        drop(el.focus());
        true
    }

    if try_focus(&node_ref) {
        return;
    }

    let Some(window) = web_sys::window() else {
        return;
    };

    let callback = Closure::once_into_js(move || {
        _ = try_focus(&node_ref);
    });

    let Some(callback) = callback.dyn_ref::<js_sys::Function>() else {
        return;
    };

    drop(window.set_timeout_with_callback_and_timeout_and_arguments_0(callback, 0));
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
    #[prop(optional)] is_confirm_disabled: Option<bool>,
    #[prop(optional)] confirm_disabled: Option<bool>,
    #[prop(optional)] is_secondary_disabled: Option<bool>,
    #[prop(optional)] secondary_disabled: Option<bool>,
    #[prop(optional, default = logic::DEFAULT_AUTO_FOCUS_BUTTON)]
    auto_focus_button: AlertDialogAutoFocusButton,
    #[prop(optional)] variant: AlertDialogVariant,
    #[prop(optional)] motion: AlertDialogMotion,
    #[prop(optional)] on_exit_complete: Option<Callback<()>>,
    #[prop(optional, into)] lang: Option<String>,
    #[prop(optional)] dir: Option<A11yDirection>,
) -> impl IntoView {
    let id_base = logic::normalize_id_base(id_base);
    let has_custom_id_base = id_base != logic::DEFAULT_ID_BASE;

    let title = logic::normalize_required_text(title, logic::DEFAULT_TITLE);
    let has_custom_title = title != logic::DEFAULT_TITLE;
    let title = StoredValue::new(title);

    let description = logic::normalize_optional_text(description);
    let show_description = description.is_some();
    let has_custom_description = show_description;
    let description = StoredValue::new(description);

    let confirm_label = logic::normalize_required_text(confirm_label, logic::DEFAULT_CONFIRM_LABEL);
    let has_custom_confirm_label = confirm_label != logic::DEFAULT_CONFIRM_LABEL;
    let confirm_label = StoredValue::new(confirm_label);

    let cancel_label = logic::normalize_cancel_label(cancel_label);
    let has_custom_cancel_label = cancel_label != logic::DEFAULT_CANCEL_LABEL;
    let show_cancel = !cancel_label.trim().is_empty();
    let cancel_label = StoredValue::new(cancel_label);

    let secondary_label = logic::normalize_secondary_label(secondary_label);
    let has_custom_secondary_label = secondary_label.is_some();
    let show_secondary = secondary_label.is_some();
    let secondary_label = StoredValue::new(secondary_label);

    let has_custom_on_secondary = on_secondary.is_some();
    let has_custom_on_cancel = on_cancel.is_some();
    let on_secondary = StoredValue::new(on_secondary);
    let on_cancel = StoredValue::new(on_cancel);

    let confirm_disabled = logic::resolve_disabled_flag(
        is_confirm_disabled,
        confirm_disabled,
        logic::DEFAULT_CONFIRM_DISABLED,
    );
    let secondary_disabled = logic::resolve_disabled_flag(
        is_secondary_disabled,
        secondary_disabled,
        logic::DEFAULT_SECONDARY_DISABLED,
    );

    let has_custom_auto_focus_button = auto_focus_button != logic::DEFAULT_AUTO_FOCUS_BUTTON;
    let motion = crate::alert_dialog::motion::sanitize_motion(motion);
    let has_custom_motion = motion != AlertDialogMotion::default();
    let has_on_exit_complete = on_exit_complete.is_some();
    let locale = locale_attrs(lang, dir);
    let locale_lang = StoredValue::new(locale.lang);
    let locale_dir = locale.dir;

    let on_close = StoredValue::new(on_close);
    let on_confirm = StoredValue::new(on_confirm);
    let on_exit_complete =
        StoredValue::new(on_exit_complete.unwrap_or_else(|| Callback::new(|_| {})));

    let title_id = format!("{id_base}-title");
    let description_id = format!("{id_base}-description");
    let title_id_attr: Signal<String> = title_id.clone().into();
    let description_id_attr: Signal<String> = description_id.clone().into();

    let root_state = Memo::new(move |_| {
        logic::resolve_state(AlertDialogPartStateInput {
            slot: AlertDialogSlot::Root,
            is_open: open.get(),
            variant,
            auto_focus_button,
            show_description,
            show_cancel,
            show_secondary,
            confirm_disabled,
            secondary_disabled,
            has_custom_id_base,
            has_custom_title,
            has_custom_description,
            has_custom_confirm_label,
            has_custom_cancel_label,
            has_custom_secondary_label,
            has_custom_on_cancel,
            has_custom_on_secondary,
            has_custom_auto_focus_button,
            has_custom_motion,
            has_on_exit_complete,
        })
    });
    let root_class = Memo::new(move |_| logic::compose_class_name(None, root_state.get()));

    let make_state = |slot| {
        logic::resolve_state(AlertDialogPartStateInput {
            slot,
            is_open: open.get_untracked(),
            variant,
            auto_focus_button,
            show_description,
            show_cancel,
            show_secondary,
            confirm_disabled,
            secondary_disabled,
            has_custom_id_base,
            has_custom_title,
            has_custom_description,
            has_custom_confirm_label,
            has_custom_cancel_label,
            has_custom_secondary_label,
            has_custom_on_cancel,
            has_custom_on_secondary,
            has_custom_auto_focus_button,
            has_custom_motion,
            has_on_exit_complete,
        })
    };

    let header_state = make_state(AlertDialogSlot::Header);
    let header_class = StoredValue::new(logic::compose_class_name(None, header_state));

    let header_text_state = make_state(AlertDialogSlot::HeaderText);
    let header_text_class = StoredValue::new(logic::compose_class_name(None, header_text_state));

    let type_icon_state = make_state(AlertDialogSlot::TypeIcon);
    let type_icon_class = StoredValue::new(logic::compose_class_name(None, type_icon_state));

    let title_state = make_state(AlertDialogSlot::Title);
    let title_class = StoredValue::new(logic::compose_class_name(None, title_state));

    let description_state = make_state(AlertDialogSlot::Description);
    let description_class = StoredValue::new(logic::compose_class_name(None, description_state));

    let footer_state = make_state(AlertDialogSlot::Footer);
    let footer_class = StoredValue::new(logic::compose_class_name(None, footer_state));

    let cancel_state = make_state(AlertDialogSlot::CancelAction);
    let cancel_class = StoredValue::new(logic::compose_class_name(None, cancel_state));

    let secondary_state = make_state(AlertDialogSlot::SecondaryAction);
    let secondary_class = StoredValue::new(logic::compose_class_name(None, secondary_state));

    let confirm_state = make_state(AlertDialogSlot::ConfirmAction);
    let confirm_class = StoredValue::new(logic::compose_class_name(None, confirm_state));

    let on_cancel_press: OnPress = Callback::new(move |_| {
        on_close.get_value().run(());
        if let Some(callback) = on_cancel.get_value() {
            callback.run(());
        }
    });

    let on_secondary_press: OnPress = Callback::new(move |_| {
        on_close.get_value().run(());
        if let Some(callback) = on_secondary.get_value() {
            callback.run(());
        }
    });

    let on_confirm_press: OnPress = Callback::new(move |_| {
        on_close.get_value().run(());
        on_confirm.get_value().run(());
    });

    let confirm_variant = match variant {
        AlertDialogVariant::Destructive => ButtonVariant::Destructive,
        AlertDialogVariant::Default
        | AlertDialogVariant::Confirmation
        | AlertDialogVariant::Warning
        | AlertDialogVariant::Error => ButtonVariant::Default,
    };

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
            AlertDialogAutoFocusButton::Cancel if show_cancel => Some(cancel_ref),
            AlertDialogAutoFocusButton::Secondary if show_secondary => Some(secondary_ref),
            AlertDialogAutoFocusButton::Confirm => Some(confirm_ref),
            AlertDialogAutoFocusButton::None => None,
            _ => None,
        };

        if let Some(target) = target {
            focus_button_soon(target);
        }
    });

    let content = Arc::new(move || {
        view! {
            <div
                class=move || root_class.get()
                lang=move || locale_lang.with_value(|value| value.clone())
                dir=locale_dir
                data-slot=move || root_state.get().slot_attr
                data-state=move || root_state.get().state_attr
                data-open=move || open.get().then_some("true")
                data-closed=move || (!open.get()).then_some("true")
                data-variant=move || root_state.get().variant_attr
                data-description=move || root_state.get().description_attr
                data-cancel=move || root_state.get().cancel_attr
                data-secondary=move || root_state.get().secondary_attr
                data-confirm-disabled=move || root_state.get().confirm_disabled_attr
                data-secondary-disabled=move || root_state.get().secondary_disabled_attr
                data-auto-focus=move || root_state.get().auto_focus_attr
                data-with-description=move || root_state.get().show_description.then_some("true")
                data-show-cancel=move || root_state.get().show_cancel.then_some("true")
                data-show-secondary=move || root_state.get().show_secondary.then_some("true")
                data-with-type-icon=move || root_state.get().show_type_icon.then_some("true")
                data-custom-variant=move || root_state.get().has_custom_variant.then_some("true")
                data-custom-id=move || root_state.get().has_custom_id_base.then_some("true")
                data-custom-title=move || root_state.get().has_custom_title.then_some("true")
                data-custom-description=move || root_state.get().has_custom_description.then_some("true")
                data-custom-confirm=move || (root_state.get().confirm_source_attr == "custom").then_some("true")
                data-custom-cancel=move || (root_state.get().cancel_source_attr == "custom").then_some("true")
                data-custom-secondary=move || (root_state.get().secondary_source_attr == "custom").then_some("true")
                data-custom-auto-focus=move || {
                    (root_state.get().auto_focus_source_attr == "custom").then_some("true")
                }
                data-custom-motion=move || root_state.get().has_custom_motion.then_some("true")
                data-custom-exit=move || root_state.get().has_on_exit_complete.then_some("true")
                data-variant-source=move || root_state.get().variant_source_attr
                data-id-source=move || root_state.get().id_source_attr
                data-title-source=move || root_state.get().title_source_attr
                data-description-source=move || root_state.get().description_source_attr
                data-cancel-source=move || root_state.get().cancel_source_attr
                data-secondary-source=move || root_state.get().secondary_source_attr
                data-confirm-source=move || root_state.get().confirm_source_attr
                data-auto-focus-source=move || root_state.get().auto_focus_source_attr
                data-motion-source=move || root_state.get().motion_source_attr
                data-exit-source=move || root_state.get().exit_source_attr
            >
                <div
                    class=move || header_class.with_value(|class_name| class_name.clone())
                    data-slot=header_state.slot_attr
                    data-state=header_state.state_attr
                >
                    <Show when=move || root_state.get().show_type_icon>
                        <span
                            class=move || type_icon_class.with_value(|class_name| class_name.clone())
                            data-slot=type_icon_state.slot_attr
                            data-state=type_icon_state.state_attr
                            data-variant=type_icon_state.variant_attr
                            aria-hidden="true"
                        >
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

                    <div
                        class=move || header_text_class.with_value(|class_name| class_name.clone())
                        data-slot=header_text_state.slot_attr
                        data-state=header_text_state.state_attr
                    >
                        <h2
                            class=move || title_class.with_value(|class_name| class_name.clone())
                            id=move || title_id_attr.get()
                            data-slot=title_state.slot_attr
                            data-state=title_state.state_attr
                            data-title-source=title_state.title_source_attr
                        >
                            {move || title.get_value()}
                        </h2>
                        <Show when=move || root_state.get().show_description>
                            <p
                                class=move || {
                                    description_class.with_value(|class_name| class_name.clone())
                                }
                                id=move || description_id_attr.get()
                                data-slot=description_state.slot_attr
                                data-state=description_state.state_attr
                                data-description-source=description_state.description_source_attr
                            >
                                {move || description.get_value().unwrap_or_default()}
                            </p>
                        </Show>
                    </div>
                </div>

                <div
                    class=move || footer_class.with_value(|class_name| class_name.clone())
                    data-slot=footer_state.slot_attr
                    data-state=footer_state.state_attr
                >
                    <Show when=move || root_state.get().show_cancel>
                        <span
                            class=move || cancel_class.with_value(|class_name| class_name.clone())
                            data-slot=cancel_state.slot_attr
                            data-state=cancel_state.state_attr
                            data-cancel-source=cancel_state.cancel_source_attr
                        >
                            <Button
                                variant=ButtonVariant::Secondary
                                is_disabled=false
                                node_ref=cancel_ref
                                on_press=on_cancel_press
                            >
                                {move || cancel_label.get_value()}
                            </Button>
                        </span>
                    </Show>
                    <Show when=move || root_state.get().show_secondary>
                        <span
                            class=move || secondary_class.with_value(|class_name| class_name.clone())
                            data-slot=secondary_state.slot_attr
                            data-state=secondary_state.state_attr
                            data-secondary-source=secondary_state.secondary_source_attr
                        >
                            <Button
                                variant=ButtonVariant::Secondary
                                is_disabled=secondary_disabled
                                node_ref=secondary_ref
                                on_press=on_secondary_press
                            >
                                {move || secondary_label.get_value().unwrap_or_default()}
                            </Button>
                        </span>
                    </Show>
                    <span
                        class=move || confirm_class.with_value(|class_name| class_name.clone())
                        data-slot=confirm_state.slot_attr
                        data-state=confirm_state.state_attr
                        data-confirm-source=confirm_state.confirm_source_attr
                    >
                        <Button
                            variant=confirm_variant
                            is_disabled=confirm_disabled
                            node_ref=confirm_ref
                            on_press=on_confirm_press
                        >
                            {move || confirm_label.get_value()}
                        </Button>
                    </span>
                </div>
            </div>
        }
    });

    if show_description {
        view! {
            <Overlay
                open=open
                on_close=on_close.get_value()
                role="alertdialog"
                aria_labelledby=title_id.clone()
                aria_describedby=description_id.clone()
                motion=motion.overlay
                on_exit_complete=on_exit_complete.get_value()
            >
                {{
                    let content = content.clone();
                    move || content()
                }}
            </Overlay>
        }
        .into_any()
    } else {
        view! {
            <Overlay
                open=open
                on_close=on_close.get_value()
                role="alertdialog"
                aria_labelledby=title_id.clone()
                motion=motion.overlay
                on_exit_complete=on_exit_complete.get_value()
            >
                {{
                    let content = content.clone();
                    move || content()
                }}
            </Overlay>
        }
        .into_any()
    }
}
