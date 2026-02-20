use crate::{
    DescriptionStateInput,
    logic::{self, DescriptionElement, DescriptionTone},
};
use leptos::prelude::*;

#[component]
pub fn Description(
    text: String,
    #[prop(optional)] tone: DescriptionTone,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] truncate: bool,
    #[prop(optional)] element: DescriptionElement,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
) -> impl IntoView {
    let text = logic::normalize_content(Some(text));
    let text = StoredValue::new(text);

    let (aria_label, has_custom_aria_label) = logic::normalize_aria_label(aria_label);

    let class_name = logic::normalize_optional_text(class_name);
    let has_custom_class_name = class_name.is_some();
    let class_name = StoredValue::new(class_name);

    let state = Memo::new(move |_| {
        logic::resolve_state(DescriptionStateInput {
            tone,
            disabled,
            truncate,
            has_custom_aria_label,
            has_custom_class_name,
        })
    });

    let class = Memo::new(move |_| logic::compose_class_name(class_name.get_value(), state.get()));

    match element {
        DescriptionElement::Span => view! {
            <span
                class=move || class.get()
                data-slot="description"
                slot="description"
                data-tone=move || state.get().tone_attr
                data-state=move || state.get().data_state_attr
                data-disabled=move || state.get().is_disabled.then_some("true")
                data-truncate=move || state.get().is_truncated.then_some("true")
                data-aria-source=move || state.get().aria_source_attr
                data-custom-class=move || state.get().has_custom_class_name.then_some("true")
                data-class-source=move || state.get().class_source_attr
                aria-label=aria_label
            >
                {text.get_value()}
            </span>
        }
        .into_any(),
        DescriptionElement::Paragraph => view! {
            <p
                class=move || class.get()
                data-slot="description"
                slot="description"
                data-tone=move || state.get().tone_attr
                data-state=move || state.get().data_state_attr
                data-disabled=move || state.get().is_disabled.then_some("true")
                data-truncate=move || state.get().is_truncated.then_some("true")
                data-aria-source=move || state.get().aria_source_attr
                data-custom-class=move || state.get().has_custom_class_name.then_some("true")
                data-class-source=move || state.get().class_source_attr
                aria-label=aria_label
            >
                {text.get_value()}
            </p>
        }
        .into_any(),
        DescriptionElement::Div => view! {
            <div
                class=move || class.get()
                data-slot="description"
                slot="description"
                data-tone=move || state.get().tone_attr
                data-state=move || state.get().data_state_attr
                data-disabled=move || state.get().is_disabled.then_some("true")
                data-truncate=move || state.get().is_truncated.then_some("true")
                data-aria-source=move || state.get().aria_source_attr
                data-custom-class=move || state.get().has_custom_class_name.then_some("true")
                data-class-source=move || state.get().class_source_attr
                aria-label=aria_label
            >
                {text.get_value()}
            </div>
        }
        .into_any(),
    }
}
