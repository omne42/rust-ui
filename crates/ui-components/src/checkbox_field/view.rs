use crate::checkbox_field::{
    CheckboxFieldIndicatorPlacement, CheckboxFieldStateInput, CheckboxFieldTone, logic,
};
use crate::{Checkbox, CheckboxVariant};
use leptos::prelude::*;

#[component]
pub fn CheckboxField(
    checked: ReadSignal<bool>,
    set_checked: WriteSignal<bool>,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] invalid: bool,
    #[prop(optional, into)] id_base: Option<String>,
    #[prop(optional, into)] label: Option<String>,
    #[prop(optional, into)] description: Option<String>,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional)] tone: CheckboxFieldTone,
    #[prop(optional)] indicator_placement: CheckboxFieldIndicatorPlacement,
    #[prop(optional, into)] class_name: Option<String>,
) -> impl IntoView {
    let id_base = StoredValue::new(logic::normalize_id_base(id_base));

    let (label, has_custom_label) = logic::normalize_label(label);
    let label = StoredValue::new(label);

    let description = logic::normalize_optional_text(description);
    let has_description = description.is_some();
    let description = StoredValue::new(description);

    let (checkbox_aria_label, has_custom_aria_label) =
        logic::normalize_aria_label(aria_label, &label.get_value());
    let checkbox_aria_label = StoredValue::new(checkbox_aria_label);

    let class_name = logic::normalize_optional_text(class_name);
    let class_name = StoredValue::new(class_name);

    let state = Memo::new(move |_| {
        logic::resolve_state(CheckboxFieldStateInput {
            checked: checked.get(),
            disabled,
            invalid,
            tone,
            indicator_placement,
            has_description,
            has_custom_label,
            has_custom_aria_label,
            has_custom_class_name: class_name.get_value().is_some(),
        })
    });

    let class =
        Signal::derive(move || logic::compose_class_name(class_name.get_value(), state.get()));

    let description_id = Memo::new(move |_| format!("{}-description", id_base.get_value()));

    let checkbox_class = if matches!(indicator_placement, CheckboxFieldIndicatorPlacement::End) {
        "ui-checkbox-field__checkbox ui-checkbox-field__checkbox--indicator-end".to_string()
    } else {
        "ui-checkbox-field__checkbox".to_string()
    };

    let checkbox_variant = if invalid {
        CheckboxVariant::Accent
    } else {
        CheckboxVariant::Default
    };

    view! {
        <div
            id=move || id_base.get_value()
            class=move || class.get()
            data-slot="checkbox-field"
            data-state=move || state.get().state_attr
            data-tone=move || state.get().tone_attr
            data-indicator-placement=move || state.get().indicator_placement_attr
            data-checked=move || state.get().is_checked.then_some("true")
            data-unchecked=move || state.get().is_unchecked.then_some("true")
            data-disabled=move || state.get().is_disabled.then_some("true")
            data-invalid=move || state.get().is_invalid.then_some("true")
            data-description=move || state.get().description_attr
            data-label-source=move || state.get().label_source_attr
            data-aria-source=move || state.get().aria_source_attr
            data-custom-class=move || state.get().has_custom_class_name.then_some("true")
            data-class-source=move || state.get().class_source_attr
            role="group"
            aria-label=move || checkbox_aria_label.get_value()
            aria-describedby=move || state.get().has_description.then(|| description_id.get())
            aria-disabled=move || state.get().is_disabled.then_some("true")
            aria-invalid=move || state.get().is_invalid.then_some("true")
        >
            <Checkbox
                checked=checked
                set_checked=set_checked
                disabled=disabled
                variant=checkbox_variant
                class_name=checkbox_class
                aria_label=checkbox_aria_label.get_value()
            >
                <span class="ui-checkbox-field__label" data-slot="checkbox-field-label">
                    {move || label.get_value()}
                </span>
            </Checkbox>

            <Show when=move || state.get().has_description>
                <p
                    id=move || description_id.get()
                    class="ui-checkbox-field__description"
                    data-slot="checkbox-field-description"
                >
                    {move || description.get_value().unwrap_or_default()}
                </p>
            </Show>
        </div>
    }
}
