use crate::field_form::field_label::logic::{self, FieldLabelStateInput, FieldLabelTone};
use leptos::prelude::*;
use ui_headless::{A11yDirection, FieldLabelOptions, use_field_label};

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
    #[prop(optional, into)] lang: Option<String>,
    #[prop(optional)] dir: Option<A11yDirection>,
) -> impl IntoView {
    let (text, has_custom_text) = logic::normalize_text(text);
    let (required_indicator, has_custom_indicator) =
        logic::normalize_required_indicator(required_indicator);
    let (aria_label, has_custom_aria_label) = logic::normalize_aria_label(aria_label);

    let for_id = logic::normalize_optional_text(for_id);
    let has_for_id = for_id.is_some();

    let class_name = logic::normalize_optional_text(class_name);
    let has_custom_class_name = class_name.is_some();
    let class_name = StoredValue::new(class_name);

    let state = Memo::new(move |_| {
        logic::resolve_state(FieldLabelStateInput {
            tone,
            required,
            disabled,
            has_for_id,
            has_custom_text,
            has_custom_indicator,
            has_custom_aria_label,
            has_custom_class_name,
        })
    });
    let class = Memo::new(move |_| logic::compose_class_name(class_name.get_value(), state.get()));
    let semantics = Memo::new(move |_| {
        use_field_label(FieldLabelOptions {
            state: state.get(),
            aria_label: aria_label.clone(),
            lang: lang.clone(),
            dir,
        })
    });

    view! {
        <label
            class=move || class.get()
            for=for_id
            aria-label=move || semantics.get().attrs.aria_label
            aria-disabled=move || semantics.get().attrs.aria_disabled
            lang=move || semantics.get().attrs.lang
            dir=move || semantics.get().attrs.dir
            data-slot="field-label"
            data-tone=move || semantics.get().attrs.data_tone
            data-state=move || semantics.get().attrs.data_state
            data-required=move || semantics.get().attrs.data_required
            data-disabled=move || semantics.get().attrs.data_disabled
            data-has-for=move || semantics.get().attrs.data_has_for
            data-text-source=move || semantics.get().attrs.data_text_source
            data-indicator-source=move || semantics.get().attrs.data_indicator_source
            data-aria-source=move || semantics.get().attrs.data_aria_source
            data-custom-class=move || semantics.get().attrs.data_custom_class
            data-class-source=move || semantics.get().attrs.data_class_source
        >
            <span class="ui-field-label__text" data-slot="field-label-text">
                {text}
            </span>

            <Show when=move || semantics.get().state.is_required>
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
