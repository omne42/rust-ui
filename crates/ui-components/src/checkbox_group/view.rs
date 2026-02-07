use crate::checkbox_group::logic::{
    CheckboxGroupOptions, normalize_label, normalize_optional_text, resolve_ids, resolve_state,
    use_checkbox_group,
};
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
    let ids = resolve_ids(&id);
    let legend_id = StoredValue::new(ids.legend_id);

    let label = StoredValue::new(normalize_label(label));
    let description = normalize_optional_text(description);
    let error = normalize_optional_text(error);

    let has_description = description.is_some();
    let has_error = error.is_some();

    let aria = use_checkbox_group(CheckboxGroupOptions {
        id: id.clone(),
        has_description,
        has_error,
        aria_describedby,
        is_invalid: invalid,
        is_required: required,
    });

    let state = Memo::new(move |_| {
        resolve_state(
            disabled,
            invalid.get(),
            required.get(),
            has_description,
            has_error,
        )
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
            class:ui-checkbox-group--invalid=move || state.get().is_invalid
            class:ui-checkbox-group--required=move || state.get().is_required
            disabled=disabled
            aria-labelledby=legend_id.get_value()
            aria-describedby=move || aria.fieldset.aria_describedby.get()
            aria-invalid=move || aria.fieldset.aria_invalid.get()
            aria-required=move || aria.fieldset.aria_required.get()
            data-slot="checkbox-group"
            data-disabled=move || state.get().is_disabled.then_some("true")
            data-enabled=move || state.get().is_enabled.then_some("true")
            data-invalid=move || state.get().is_invalid.then_some("true")
            data-valid=move || state.get().is_valid.then_some("true")
            data-required=move || state.get().is_required.then_some("true")
            data-optional=move || state.get().is_optional.then_some("true")
            data-has-description=move || state.get().has_description.then_some("true")
            data-has-error=move || state.get().has_error.then_some("true")
            data-shows-error=move || state.get().shows_error.then_some("true")
            data-has-messages=move || state.get().has_messages.then_some("true")
        >
            <legend
                class="ui-checkbox-group__label"
                id=legend_id.get_value()
                data-slot="checkbox-group-label"
            >
                {label.get_value()}
            </legend>

            <div class="ui-checkbox-group__list" data-slot="checkbox-group-list">
                {children()}
            </div>

            {description.map(|description| {
                let description_id = StoredValue::new(aria.description.id.clone());
                let description = StoredValue::new(description);
                view! {
                    <div
                        class="ui-checkbox-group__description"
                        id=description_id.get_value()
                        data-slot="checkbox-group-description"
                    >
                        {description.get_value()}
                    </div>
                }
            })}

            {error.map(|error| {
                let error_id = StoredValue::new(aria.error.id.clone());
                let error = StoredValue::new(error);
                view! {
                    <Show when=move || state.get().shows_error>
                        <div
                            class="ui-checkbox-group__error"
                            id=error_id.get_value()
                            data-slot="checkbox-group-error"
                        >
                            {error.get_value()}
                        </div>
                    </Show>
                }
            })}
        </fieldset>
    }
}
