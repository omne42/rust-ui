use crate::field_label::{
    FieldLabelStateInput,
    logic::{self, FieldLabelTone},
};
use leptos::prelude::*;

#[component]
pub fn FieldLabel(
    #[prop(optional, into)] text: Option<String>,
    #[prop(optional, into)] for_id: Option<String>,
    #[prop(optional)] required: bool,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] tone: FieldLabelTone,
    #[prop(optional, into)] required_indicator: Option<String>,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
) -> impl IntoView {
    let (text, has_custom_text) = logic::normalize_text(text);
    let (required_indicator, has_custom_indicator) =
        logic::normalize_required_indicator(required_indicator);
    let (aria_label, has_custom_aria_label) = logic::normalize_aria_label(aria_label);

    let for_id = logic::normalize_optional_text(for_id);
    let has_for_id = for_id.is_some();

    let class_name = logic::normalize_optional_text(class_name);
    let class_name = StoredValue::new(class_name);

    let state = Signal::derive(move || {
        logic::resolve_state(FieldLabelStateInput {
            tone,
            required,
            disabled,
            has_for_id,
            has_custom_text,
            has_custom_indicator,
            has_custom_aria_label,
            has_custom_class_name: class_name.get_value().is_some(),
        })
    });

    let class =
        Signal::derive(move || logic::compose_class_name(class_name.get_value(), state.get()));

    view! {
        <label
            class=move || class.get()
            for=for_id
            aria-label=aria_label
            aria-disabled=move || state.get().is_disabled.then_some("true")
            data-slot="field-label"
            data-tone=move || state.get().tone_attr
            data-state=move || if state.get().is_required { "required" } else { "optional" }
            data-required=move || state.get().is_required.then_some("true")
            data-disabled=move || state.get().is_disabled.then_some("true")
            data-has-for=move || state.get().has_for_id.then_some("true")
            data-text-source=move || state.get().text_source_attr
            data-indicator-source=move || state.get().indicator_source_attr
            data-aria-source=move || state.get().aria_source_attr
            data-custom-class=move || state.get().has_custom_class_name.then_some("true")
            data-class-source=move || state.get().class_source_attr
        >
            <span class="ui-field-label__text" data-slot="field-label-text">
                {text}
            </span>

            <Show when=move || state.get().is_required>
                <span
                    class="ui-field-label__required"
                    data-slot="field-label-required"
                    aria-hidden="true"
                >
                    {required_indicator.clone()}
                </span>
            </Show>
        </label>
    }
}
