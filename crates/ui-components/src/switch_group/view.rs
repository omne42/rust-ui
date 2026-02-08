use crate::switch_group::{
    SwitchGroupStateInput,
    logic::{self, SwitchGroupOrientation, SwitchGroupTone},
};
use leptos::prelude::*;

#[component]
pub fn SwitchGroup(
    id_base: String,
    #[prop(optional, into)] label: Option<String>,
    #[prop(optional, into)] description: Option<String>,
    #[prop(optional, into)] error_message: Option<String>,
    #[prop(optional)] orientation: SwitchGroupOrientation,
    #[prop(optional)] tone: SwitchGroupTone,
    #[prop(optional)] required: bool,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] invalid: bool,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
    children: Children,
) -> impl IntoView {
    let ids = logic::resolve_ids(id_base);
    let ids = StoredValue::new(ids);

    let (label, has_custom_label) = logic::normalize_label(label);
    let label = StoredValue::new(label);

    let description = logic::normalize_description(description);
    let has_description = description.is_some();
    let description = StoredValue::new(description);

    let (error_message, has_custom_error_message) =
        logic::normalize_error_message(error_message, invalid);
    let has_error_message = error_message.is_some();
    let error_message = StoredValue::new(error_message);

    let (aria_label, has_custom_aria_label) = logic::normalize_aria_label(aria_label);

    let class_name = logic::normalize_optional_text(class_name);
    let has_custom_class_name = class_name.is_some();
    let class_name = StoredValue::new(class_name);

    let state = Memo::new(move |_| {
        logic::resolve_state(SwitchGroupStateInput {
            orientation,
            tone,
            required,
            disabled,
            invalid,
            has_label: true,
            has_description,
            has_error_message,
            has_custom_label,
            has_custom_aria_label,
            has_custom_error_message,
            has_custom_class_name,
        })
    });

    let class = Memo::new(move |_| logic::compose_class_name(class_name.get_value(), state.get()));

    let describedby = Memo::new(move |_| {
        let mut ids_out = Vec::new();
        let group_ids = ids.get_value();
        let state = state.get();

        if state.has_description {
            ids_out.push(group_ids.description_id.clone());
        }

        if state.shows_error {
            ids_out.push(group_ids.error_id.clone());
        }

        if ids_out.is_empty() {
            None
        } else {
            Some(ids_out.join(" "))
        }
    });

    view! {
        <fieldset
            id=move || ids.get_value().root_id.clone()
            class=move || class.get()
            disabled=disabled
            data-slot="switch-group"
            data-orientation=move || state.get().orientation_attr
            data-tone=move || state.get().tone_attr
            data-state=move || state.get().data_state_attr
            data-message-kind=move || state.get().message_kind_attr
            data-required=move || state.get().is_required.then_some("true")
            data-disabled=move || state.get().is_disabled.then_some("true")
            data-invalid=move || state.get().is_invalid.then_some("true")
            data-has-label=move || state.get().has_label.then_some("true")
            data-has-description=move || state.get().has_description.then_some("true")
            data-has-error=move || state.get().has_error_message.then_some("true")
            data-shows-error=move || state.get().shows_error.then_some("true")
            data-has-messages=move || state.get().has_messages.then_some("true")
            data-label-source=move || state.get().label_source_attr
            data-aria-source=move || state.get().aria_source_attr
            data-error-source=move || state.get().error_source_attr
            data-custom-class=move || state.get().has_custom_class_name.then_some("true")
            data-class-source=move || state.get().class_source_attr
            role="group"
            aria-label=aria_label
            aria-labelledby=move || (!has_custom_aria_label).then(|| ids.get_value().label_id.clone())
            aria-describedby=move || describedby.get()
            aria-required=move || state.get().is_required.then_some("true")
            aria-invalid=move || state.get().is_invalid.then_some("true")
        >
            <legend
                id=move || ids.get_value().label_id.clone()
                class="ui-switch-group__label"
                data-slot="switch-group-label"
            >
                {label.get_value()}
            </legend>

            <div class="ui-switch-group__group" data-slot="switch-group-group">
                {children()}
            </div>

            {state.get().has_description.then(|| {
                let text = description.get_value().unwrap_or_default();
                view! {
                    <p
                        id=move || ids.get_value().description_id.clone()
                        class="ui-switch-group__description"
                        data-slot="switch-group-description"
                    >
                        {text}
                    </p>
                }
            })}

            {state.get().shows_error.then(|| {
                let text = error_message.get_value().unwrap_or_default();
                view! {
                    <p
                        id=move || ids.get_value().error_id.clone()
                        class="ui-switch-group__error"
                        data-slot="switch-group-error"
                    >
                        {text}
                    </p>
                }
            })}
        </fieldset>
    }
}
