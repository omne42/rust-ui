use crate::text_input::text::{
    TextStateInput,
    logic::{self, TextAlign, TextElement, TextTone, TextWeight},
};
use leptos::prelude::*;

#[component]
pub fn Text(
    text: String,
    #[prop(optional)] tone: TextTone,
    #[prop(optional)] align: TextAlign,
    #[prop(optional)] weight: TextWeight,
    #[prop(optional)] is_disabled: Option<bool>,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] is_truncated: Option<bool>,
    #[prop(optional)] truncate: bool,
    #[prop(optional)] element: TextElement,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
) -> impl IntoView {
    let accessibility = logic::normalize_accessibility_state(logic::AccessibilityStateInput {
        is_disabled,
        disabled,
        is_truncated,
        truncate,
    });
    let is_disabled = accessibility.is_disabled;
    let is_truncated = accessibility.is_truncated;

    let text = logic::normalize_content(Some(text));
    let text = StoredValue::new(text);

    let (aria_label, has_custom_aria_label) = logic::normalize_aria_label(aria_label);

    let class_name = logic::normalize_optional_text(class_name);
    let has_custom_class_name = class_name.is_some();
    let class_name = StoredValue::new(class_name);

    let state = Memo::new(move |_| {
        logic::resolve_state(TextStateInput {
            tone,
            align,
            weight,
            disabled: is_disabled,
            truncate: is_truncated,
            has_custom_aria_label,
            has_custom_class_name,
        })
    });

    let class = Memo::new(move |_| logic::compose_class_name(class_name.get_value(), state.get()));

    match element {
        TextElement::Span => view! {
            <span
                class=move || class.get()
                data-slot="text"
                data-tone=move || state.get().tone_attr
                data-align=move || state.get().align_attr
                data-weight=move || state.get().weight_attr
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
        TextElement::Paragraph => view! {
            <p
                class=move || class.get()
                data-slot="text"
                data-tone=move || state.get().tone_attr
                data-align=move || state.get().align_attr
                data-weight=move || state.get().weight_attr
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
        TextElement::Div => view! {
            <div
                class=move || class.get()
                data-slot="text"
                data-tone=move || state.get().tone_attr
                data-align=move || state.get().align_attr
                data-weight=move || state.get().weight_attr
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
