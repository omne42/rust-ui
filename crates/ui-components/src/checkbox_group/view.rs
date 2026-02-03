use crate::checkbox_group::logic::{CheckboxGroupOptions, use_checkbox_group};
use leptos::prelude::*;

#[component]
pub fn CheckboxGroup(
    id: String,
    label: String,
    #[prop(optional, into)] description: Option<String>,
    #[prop(optional, into)] error: Option<String>,
    #[prop(optional, into)] invalid: Signal<bool>,
    #[prop(optional, into)] required: Signal<bool>,
    #[prop(optional)] disabled: bool,
    #[prop(optional, into)] aria_describedby: Signal<Option<String>>,
    #[prop(optional, into)] class_name: Option<String>,
    children: Children,
) -> impl IntoView {
    let aria = use_checkbox_group(CheckboxGroupOptions {
        id: id.clone(),
        has_description: description.is_some(),
        has_error: error.is_some(),
        aria_describedby,
        is_invalid: invalid,
        is_required: required,
    });

    let base_class = "ui-checkbox-group".to_string();
    let class = class_name
        .filter(|value| !value.trim().is_empty())
        .map(|value| format!("{base_class} {value}"))
        .unwrap_or(base_class);

    view! {
        <fieldset
            id=id
            class=class
            class:ui-checkbox-group--invalid=move || invalid.get()
            class:ui-checkbox-group--required=move || required.get()
            disabled=disabled
            aria-describedby=move || aria.fieldset.aria_describedby.get()
            aria-invalid=move || aria.fieldset.aria_invalid.get()
            aria-required=move || aria.fieldset.aria_required.get()
            data-slot="checkbox-group"
        >
            <legend
                class="ui-checkbox-group__label"
                data-slot="checkbox-group-label"
            >
                {label}
            </legend>

            <div class="ui-checkbox-group__list" data-slot="checkbox-group-list">
                {children()}
            </div>

            {description.map(|description| {
                let description_id = aria.description.id.clone();
                view! {
                    <div
                        class="ui-checkbox-group__description"
                        id=description_id
                        data-slot="checkbox-group-description"
                    >
                        {description}
                    </div>
                }
            })}

            {error.map(|error| {
                let error_id = aria.error.id.clone();
                let error_id = StoredValue::new(error_id);
                let error = StoredValue::new(error);
                view! {
                    <Show when=move || invalid.get()>
                        <div
                            class="ui-checkbox-group__error"
                            id=move || error_id.get_value()
                            data-slot="checkbox-group-error"
                        >
                            {move || error.get_value()}
                        </div>
                    </Show>
                }
            })}
        </fieldset>
    }
}
