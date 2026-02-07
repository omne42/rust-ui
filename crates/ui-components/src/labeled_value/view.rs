use crate::labeled_value::{
    LabeledValueOrientation, LabeledValueStateInput,
    logic::{self, LabeledValueTone},
};
use leptos::prelude::*;

#[component]
pub fn LabeledValue(
    #[prop(optional, into)] label: Option<String>,
    #[prop(optional, into)] value: Option<String>,
    #[prop(optional, into)] description: Option<String>,
    #[prop(optional)] orientation: LabeledValueOrientation,
    #[prop(optional)] tone: LabeledValueTone,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
) -> impl IntoView {
    let (label, has_custom_label) = logic::normalize_label_text(label);
    let (value, has_custom_value) = logic::normalize_value_text(value);
    let description = logic::normalize_optional_text(description);
    let has_description = description.is_some();
    let (aria_label, has_custom_aria_label) = logic::normalize_aria_label(aria_label);
    let class_name = logic::normalize_optional_text(class_name);
    let class_name = StoredValue::new(class_name);

    let state = Signal::derive(move || {
        logic::resolve_state(LabeledValueStateInput {
            orientation,
            tone,
            has_custom_label,
            has_custom_value,
            has_description,
            has_custom_aria_label,
            has_custom_class_name: class_name.get_value().is_some(),
        })
    });

    let class =
        Signal::derive(move || logic::compose_class_name(class_name.get_value(), state.get()));

    view! {
        <div
            class=move || class.get()
            data-slot="labeled-value"
            data-orientation=move || state.get().orientation_attr
            data-tone=move || state.get().tone_attr
            data-state=move || if state.get().has_description { "with-description" } else { "default" }
            data-has-description=move || state.get().has_description.then_some("true")
            data-label-source=move || state.get().label_source_attr
            data-value-source=move || state.get().value_source_attr
            data-aria-source=move || state.get().aria_source_attr
            data-custom-class=move || state.get().has_custom_class_name.then_some("true")
            data-class-source=move || state.get().class_source_attr
            role="group"
            aria-label=aria_label
        >
            <span class="ui-labeled-value__label" data-slot="labeled-value-label">
                {label}
            </span>
            <span class="ui-labeled-value__value" data-slot="labeled-value-value">
                {value}
            </span>
            {description.map(|description| {
                view! {
                    <span class="ui-labeled-value__description" data-slot="labeled-value-description">
                        {description}
                    </span>
                }
            })}
        </div>
    }
}
