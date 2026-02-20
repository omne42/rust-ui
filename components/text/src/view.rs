use crate::{
    TextStateInput,
    logic::{self, TextAlign, TextElement, TextTone, TextWeight},
};
use leptos::prelude::*;

#[component]
pub fn Text(
    #[prop(optional, into)] text: Option<String>,
    #[prop(optional)] tone: TextTone,
    #[prop(optional)] align: TextAlign,
    #[prop(optional)] weight: TextWeight,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] truncate: bool,
    #[prop(optional)] element: TextElement,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional, into)] slot: Option<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let text = text.unwrap_or_default();
    let has_explicit_text = logic::normalize_optional_text(Some(text.clone())).is_some();
    let text = logic::normalize_content(Some(text));
    let text = StoredValue::new(text);

    let (aria_label, has_custom_aria_label) = logic::normalize_aria_label(aria_label);

    let class_name = logic::normalize_optional_text(class_name);
    let has_custom_class_name = class_name.is_some();
    let class_name = StoredValue::new(class_name);

    let slot = logic::normalize_optional_text(slot);
    let slot_kind_attr = logic::resolve_slot_kind_attr(slot.as_deref());
    let has_named_slot = slot.is_some();
    let slot = StoredValue::new(slot);

    let content_source_attr = if children.is_some() {
        "children"
    } else if has_explicit_text {
        "text"
    } else {
        "default"
    };

    let content: AnyView = if let Some(children) = children {
        children().into_any()
    } else {
        text.get_value().into_view().into_any()
    };

    let state = Memo::new(move |_| {
        logic::resolve_state(TextStateInput {
            tone,
            align,
            weight,
            disabled,
            truncate,
            has_custom_aria_label,
            has_custom_class_name,
            slot_kind_attr,
            has_named_slot,
        })
    });

    let class = Memo::new(move |_| logic::compose_class_name(class_name.get_value(), state.get()));

    match element {
        TextElement::Span => view! {
            <span
                class=move || class.get()
                data-slot="text"
                slot=move || slot.get_value()
                data-slot-name=move || slot.get_value()
                data-slot-kind=move || state.get().slot_kind_attr
                data-has-slot=move || state.get().has_named_slot.then_some("true")
                data-tone=move || state.get().tone_attr
                data-align=move || state.get().align_attr
                data-weight=move || state.get().weight_attr
                data-state=move || state.get().data_state_attr
                data-content-source=content_source_attr
                data-disabled=move || state.get().is_disabled.then_some("true")
                data-truncate=move || state.get().is_truncated.then_some("true")
                data-aria-source=move || state.get().aria_source_attr
                data-custom-class=move || state.get().has_custom_class_name.then_some("true")
                data-class-source=move || state.get().class_source_attr
                aria-label=aria_label
            >
                {content}
            </span>
        }
        .into_any(),
        TextElement::Paragraph => view! {
            <p
                class=move || class.get()
                data-slot="text"
                slot=move || slot.get_value()
                data-slot-name=move || slot.get_value()
                data-slot-kind=move || state.get().slot_kind_attr
                data-has-slot=move || state.get().has_named_slot.then_some("true")
                data-tone=move || state.get().tone_attr
                data-align=move || state.get().align_attr
                data-weight=move || state.get().weight_attr
                data-state=move || state.get().data_state_attr
                data-content-source=content_source_attr
                data-disabled=move || state.get().is_disabled.then_some("true")
                data-truncate=move || state.get().is_truncated.then_some("true")
                data-aria-source=move || state.get().aria_source_attr
                data-custom-class=move || state.get().has_custom_class_name.then_some("true")
                data-class-source=move || state.get().class_source_attr
                aria-label=aria_label
            >
                {content}
            </p>
        }
        .into_any(),
        TextElement::Div => view! {
            <div
                class=move || class.get()
                data-slot="text"
                slot=move || slot.get_value()
                data-slot-name=move || slot.get_value()
                data-slot-kind=move || state.get().slot_kind_attr
                data-has-slot=move || state.get().has_named_slot.then_some("true")
                data-tone=move || state.get().tone_attr
                data-align=move || state.get().align_attr
                data-weight=move || state.get().weight_attr
                data-state=move || state.get().data_state_attr
                data-content-source=content_source_attr
                data-disabled=move || state.get().is_disabled.then_some("true")
                data-truncate=move || state.get().is_truncated.then_some("true")
                data-aria-source=move || state.get().aria_source_attr
                data-custom-class=move || state.get().has_custom_class_name.then_some("true")
                data-class-source=move || state.get().class_source_attr
                aria-label=aria_label
            >
                {content}
            </div>
        }
        .into_any(),
    }
}
