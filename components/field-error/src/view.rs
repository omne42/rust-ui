use crate::logic::{self, FieldErrorTone};
use leptos::prelude::*;
use ui_headless::{
    A11yDirection, CommonStrings, ErrorMessageOptions, use_error_message, use_ui_i18n,
};

const FIELD_ERROR_ICON_GLYPH: &str = "⚠";

fn field_error_icon_view() -> impl IntoView {
    view! {
        <span class="ui-field-error__icon" data-slot="field-error-icon" aria-hidden="true">
            {FIELD_ERROR_ICON_GLYPH}
        </span>
    }
}

fn field_error_text_view(message: StoredValue<Option<String>>) -> impl IntoView {
    view! {
        <p class="ui-field-error__text" data-slot="field-error-text">
            {move || message.get_value().unwrap_or_default()}
        </p>
    }
}

#[component]
pub fn FieldError(
    #[prop(optional)] tone: FieldErrorTone,
    #[prop(optional)] is_visible: Option<bool>,
    #[prop(optional)] visible: bool,
    #[prop(optional)] is_disabled: Option<bool>,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] is_icon_visible: Option<bool>,
    #[prop(optional)] show_icon: bool,
    #[prop(optional, into)] message: Option<String>,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional, into)] lang: Option<String>,
    #[prop(optional)] dir: Option<A11yDirection>,
) -> impl IntoView {
    let i18n = use_ui_i18n();
    let common = i18n.strings::<CommonStrings>();
    let view_model = logic::resolve_view_model(logic::FieldErrorLogicInput {
        tone,
        is_visible,
        visible,
        is_disabled,
        disabled,
        is_icon_visible,
        show_icon,
        message,
        aria_label,
        class_name,
        default_message: Some(common.field_error_default_message.as_ref().to_string()),
        default_aria_label: Some(common.field_error_aria_label.as_ref().to_string()),
    });
    let logic::FieldErrorViewModel {
        state: resolved_state,
        message,
        aria_label,
        class_name,
        has_custom_message,
        has_custom_aria_label,
        has_custom_class_name,
    } = view_model;
    let state = Memo::new(move |_| resolved_state);
    let message = StoredValue::new(message);
    let class_name = StoredValue::new(class_name);

    let class = Memo::new(move |_| logic::compose_class_name(class_name.get_value(), state.get()));
    let semantics = Memo::new(move |_| {
        use_error_message(ErrorMessageOptions {
            state: logic::resolve_headless_state(
                state.get().tone,
                state.get().is_disabled,
                has_custom_message,
                has_custom_aria_label,
                has_custom_class_name,
            ),
            aria_label: aria_label.clone(),
            lang: lang.clone(),
            dir,
        })
    });

    view! {
        <div
            class=move || class.get()
            data-slot="field-error"
            role=move || semantics.get().attrs.role
            aria-live=move || semantics.get().attrs.aria_live
            aria-label=move || semantics.get().attrs.aria_label.clone()
            aria-disabled=move || semantics.get().attrs.aria_disabled
            aria-hidden=move || (!state.get().is_visible).then_some("true")
            lang=move || semantics.get().attrs.lang.clone()
            dir=move || semantics.get().attrs.dir
            data-tone=move || semantics.get().attrs.data_tone
            data-state=move || state.get().data_state.as_attr()
            data-visible=move || state.get().is_visible.then_some("true")
            data-disabled=move || semantics.get().attrs.data_disabled
            data-show-icon=move || state.get().show_icon.then_some("true")
            data-has-message=move || state.get().has_message.then_some("true")
            data-aria-source=move || semantics.get().attrs.data_aria_source
            data-message-source=move || state.get().message_source.as_attr()
            data-custom-class=move || semantics.get().attrs.data_custom_class
            data-class-source=move || semantics.get().attrs.data_class_source
        >
            <Show when=move || state.get().is_visible>
                <Show when=move || state.get().show_icon>
                    {field_error_icon_view()}
                </Show>
                {field_error_text_view(message)}
            </Show>
        </div>
    }
}
