use crate::fieldset::{
    FieldsetStateInput,
    logic::{self, FieldsetOrientation, FieldsetTone},
};
use leptos::{children::ViewFn, prelude::*};

#[component]
pub fn Fieldset(
    children: Children,
    #[prop(optional)] orientation: FieldsetOrientation,
    #[prop(optional)] tone: FieldsetTone,
    #[prop(optional)] required: bool,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] invalid: bool,
    #[prop(optional, into)] legend: Option<String>,
    #[prop(optional, into)] description: Option<String>,
    #[prop(optional, into)] error_message: Option<String>,
    #[prop(optional, into)] actions: Option<ViewFn>,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
) -> impl IntoView {
    let (aria_label, has_custom_aria_label) = logic::normalize_aria_label(aria_label);

    let legend = logic::normalize_optional_text(legend);
    let description = logic::normalize_optional_text(description);
    let (error_message, has_custom_error_message) =
        logic::normalize_error_message(error_message, invalid);

    let has_legend = legend.is_some();
    let has_description = description.is_some();
    let has_error_message = error_message.is_some();

    let legend = StoredValue::new(legend);
    let description = StoredValue::new(description);
    let error_message = StoredValue::new(error_message);

    let actions = StoredValue::new(actions);
    let has_actions = actions.get_value().is_some();

    let class_name = logic::normalize_optional_text(class_name);
    let has_custom_class_name = class_name.is_some();
    let class_name = StoredValue::new(class_name);

    let state = Memo::new(move |_| {
        logic::resolve_state(FieldsetStateInput {
            orientation,
            tone,
            required,
            disabled,
            invalid,
            has_legend,
            has_description,
            has_error_message,
            has_actions,
            has_custom_aria_label,
            has_custom_error_message,
            has_custom_class_name,
        })
    });

    let class = Memo::new(move |_| logic::compose_class_name(class_name.get_value(), state.get()));

    view! {
        <fieldset
            class=move || class.get()
            data-slot="fieldset"
            data-orientation=move || state.get().orientation_attr
            data-tone=move || state.get().tone_attr
            data-state=move || state.get().data_state_attr
            data-message-kind=move || state.get().message_kind_attr
            data-required=move || state.get().is_required.then_some("true")
            data-disabled=move || state.get().is_disabled.then_some("true")
            data-invalid=move || state.get().is_invalid.then_some("true")
            data-has-legend=move || state.get().has_legend.then_some("true")
            data-has-description=move || state.get().has_description.then_some("true")
            data-has-error=move || state.get().has_error_message.then_some("true")
            data-has-actions=move || state.get().has_actions.then_some("true")
            data-aria-source=move || state.get().aria_source_attr
            data-error-source=move || state.get().error_source_attr
            data-custom-class=move || state.get().has_custom_class_name.then_some("true")
            data-class-source=move || state.get().class_source_attr
            aria-label=aria_label
            aria-disabled=move || state.get().is_disabled.then_some("true")
            aria-invalid=move || state.get().is_invalid.then_some("true")
        >
            <Show when=move || state.get().has_legend>
                <legend class="ui-fieldset__legend" data-slot="fieldset-legend">
                    {move || legend.get_value().unwrap_or_default()}
                    <Show when=move || state.get().is_required>
                        <span
                            class="ui-fieldset__required-indicator"
                            data-slot="fieldset-required"
                            aria-hidden="true"
                        >
                            "*"
                        </span>
                    </Show>
                </legend>
            </Show>

            <div class="ui-fieldset__group" data-slot="fieldset-field-group">
                {children()}
            </div>

            <Show when=move || state.get().has_actions>
                {move || {
                    actions.get_value().map(|actions| {
                        view! {
                            <div class="ui-fieldset__actions" data-slot="fieldset-actions">
                                {actions.run()}
                            </div>
                        }
                    })
                }}
            </Show>

            <Show when=move || state.get().message_kind_attr == "description">
                <p class="ui-fieldset__description" data-slot="fieldset-description">
                    {move || description.get_value().unwrap_or_default()}
                </p>
            </Show>

            <Show when=move || state.get().message_kind_attr == "error">
                <p class="ui-fieldset__error" data-slot="fieldset-error" role="alert">
                    {move || error_message.get_value().unwrap_or_default()}
                </p>
            </Show>
        </fieldset>
    }
}
