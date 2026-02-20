use crate::{
    FieldErrorStateInput,
    logic::{self, FieldErrorTone},
};
use leptos::prelude::*;

#[component]
pub fn FieldError(
    #[prop(optional)] tone: FieldErrorTone,
    #[prop(optional)] visible: bool,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] show_icon: bool,
    #[prop(optional, into)] message: Option<String>,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
) -> impl IntoView {
    let (aria_label, has_custom_aria_label) = logic::normalize_aria_label(aria_label);
    let (message, has_custom_message) = logic::normalize_message(message, visible);

    let has_message = message.is_some();
    let message = StoredValue::new(message);

    let class_name = logic::normalize_optional_text(class_name);
    let has_custom_class_name = class_name.is_some();
    let class_name = StoredValue::new(class_name);

    let state = Memo::new(move |_| {
        logic::resolve_state(FieldErrorStateInput {
            tone,
            visible,
            disabled,
            show_icon,
            has_message,
            has_custom_aria_label,
            has_custom_message,
            has_custom_class_name,
        })
    });

    let class = Memo::new(move |_| logic::compose_class_name(class_name.get_value(), state.get()));

    view! {
        <div
            class=move || class.get()
            data-slot="field-error"
            data-tone=move || state.get().tone_attr
            data-state=move || state.get().data_state_attr
            data-visible=move || state.get().is_visible.then_some("true")
            data-disabled=move || state.get().is_disabled.then_some("true")
            data-show-icon=move || state.get().show_icon.then_some("true")
            data-has-message=move || state.get().has_message.then_some("true")
            data-aria-source=move || state.get().aria_source_attr
            data-message-source=move || state.get().message_source_attr
            data-custom-class=move || state.get().has_custom_class_name.then_some("true")
            data-class-source=move || state.get().class_source_attr
            aria-label=aria_label
            aria-disabled=move || state.get().is_disabled.then_some("true")
            aria-hidden=move || (!state.get().is_visible).then_some("true")
        >
            <Show when=move || state.get().is_visible>
                <Show when=move || state.get().show_icon>
                    <span class="ui-field-error__icon" data-slot="field-error-icon" aria-hidden="true">
                        "⚠"
                    </span>
                </Show>
                <p class="ui-field-error__text" data-slot="field-error-text" role="alert">
                    {move || message.get_value().unwrap_or_default()}
                </p>
            </Show>
        </div>
    }
}
