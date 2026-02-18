use crate::color_field::{
    ColorFieldStateInput,
    logic::{self},
};
use crate::color_swatch::ColorSwatch;
use leptos::prelude::*;
use ui_headless::{
    A11yDirection, CommonStrings, locale_attrs, use_controllable_state, use_ui_i18n,
};

#[component]
pub fn ColorField(
    id_base: String,
    #[prop(optional, into)] label: Option<String>,
    #[prop(optional, into)] placeholder: Option<String>,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] value: Option<Signal<Option<String>>>,
    #[prop(optional)] default_value: Option<String>,
    #[prop(optional)] on_value_change: Option<Callback<Option<String>>>,
    #[prop(optional, default = true)] show_preview: bool,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional, into)] lang: Option<String>,
    #[prop(optional)] dir: Option<A11yDirection>,
) -> impl IntoView {
    let i18n = use_ui_i18n();
    let common = i18n.strings::<CommonStrings>();
    let locale = locale_attrs(logic::normalize_optional_text(lang), dir);
    let clear_label = StoredValue::new(common.clear_aria_label.to_string());

    let default_value = logic::normalize_color_value(default_value);
    let value_state = use_controllable_state(value, Some(default_value), on_value_change);
    let value = value_state.value;
    let request_value_change = value_state.request_change;

    let (label, has_custom_label) = logic::normalize_label(label);
    let label = StoredValue::new(label);

    let (placeholder, has_custom_placeholder) = logic::normalize_placeholder(placeholder);
    let placeholder = StoredValue::new(placeholder);

    let (aria_label, has_custom_aria_label) =
        logic::normalize_aria_label(aria_label, &label.get_value());

    let class_name = logic::normalize_optional_text(class_name);
    let has_custom_class_name = class_name.is_some();
    let class_name = StoredValue::new(class_name);

    let preview_color = Memo::new(move |_| logic::sanitize_preview_color(value.get()));

    let state = Memo::new(move |_| {
        let raw_value = value.get();
        let has_value = raw_value.is_some();
        let has_valid_value = preview_color.get().is_some();

        logic::resolve_state(ColorFieldStateInput {
            disabled,
            has_value,
            has_valid_value,
            has_preview: show_preview && has_valid_value,
            has_custom_label,
            has_custom_placeholder,
            has_custom_aria_label,
            has_custom_class_name,
        })
    });

    let class = Memo::new(move |_| logic::compose_class_name(class_name.get_value(), state.get()));

    let label_id = format!("{id_base}-label");
    let input_id = format!("{id_base}-input");

    let on_input = move |ev| {
        if disabled {
            return;
        }

        let raw_value = event_target_value(&ev);
        let next = logic::normalize_color_value(Some(raw_value));
        request_value_change.run(next);
    };

    let on_clear = move |_| {
        if disabled {
            return;
        }

        request_value_change.run(None);
    };

    view! {
        <div
            id=id_base
            class=move || class.get()
            data-slot="color-field"
            data-state=move || state.get().data_state_attr
            data-disabled=move || state.get().is_disabled.then_some("true")
            data-has-value=move || state.get().has_value.then_some("true")
            data-valid=move || state.get().has_valid_value.then_some("true")
            data-invalid=move || {
                (state.get().has_value && !state.get().has_valid_value).then_some("true")
            }
            data-has-preview=move || state.get().has_preview.then_some("true")
            data-label-source=move || state.get().label_source_attr
            data-placeholder-source=move || state.get().placeholder_source_attr
            data-aria-source=move || state.get().aria_source_attr
            data-custom-class=move || state.get().has_custom_class_name.then_some("true")
            data-class-source=move || state.get().class_source_attr
            role="group"
            aria-label=aria_label
            aria-labelledby=label_id.clone()
            lang=locale.lang.clone()
            dir=locale.dir
        >
            <label id=label_id.clone() class="ui-color-field__label" data-slot="color-field-label" for=input_id.clone()>
                {label.get_value()}
            </label>

            <div class="ui-color-field__control" data-slot="color-field-control">
                <Show when=move || show_preview>
                    {move || {
                        let swatch = if let Some(color) = preview_color.get() {
                            view! { <ColorSwatch color=color decorative=true /> }.into_any()
                        } else {
                            view! { <ColorSwatch decorative=true /> }.into_any()
                        };

                        view! {
                            <span class="ui-color-field__preview" data-slot="color-field-preview" aria-hidden="true">
                                {swatch}
                            </span>
                        }
                    }}
                </Show>

                <input
                    id=input_id
                    class="ui-color-field__input"
                    data-slot="color-field-input"
                    type="text"
                    spellcheck="false"
                    disabled=disabled
                    aria-invalid=move || {
                        (state.get().has_value && !state.get().has_valid_value).then_some("true")
                    }
                    placeholder=placeholder.get_value()
                    prop:value=move || value.get().unwrap_or_default()
                    on:input=on_input
                />

                <Show when=move || state.get().has_value>
                    <button
                        type="button"
                        class="ui-color-field__clear"
                        data-slot="color-field-clear"
                        disabled=disabled
                        aria-label=clear_label.get_value()
                        on:click=on_clear
                    >
                        {clear_label.get_value()}
                    </button>
                </Show>
            </div>
        </div>
    }
}
