use crate::heading::{
    HeadingStateInput,
    logic::{self, HeadingLevel, HeadingTone},
};
use leptos::prelude::*;

#[component]
pub fn Heading(
    #[prop(optional)] level: HeadingLevel,
    #[prop(optional)] tone: HeadingTone,
    #[prop(optional)] truncate: bool,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
    children: Children,
) -> impl IntoView {
    let (aria_label, has_custom_aria_label) = logic::normalize_aria_label(aria_label);

    let class_name = logic::normalize_optional_text(class_name);
    let has_custom_class_name = class_name.is_some();
    let class_name = StoredValue::new(class_name);

    let state = Memo::new(move |_| {
        logic::resolve_state(HeadingStateInput {
            level,
            tone,
            truncate,
            has_custom_aria_label,
            has_custom_class_name,
        })
    });

    let class = Memo::new(move |_| logic::compose_class_name(class_name.get_value(), state.get()));

    match level {
        HeadingLevel::H1 => view! {
            <h1
                class=move || class.get()
                data-slot="heading"
                data-level=move || state.get().level_attr
                data-tone=move || state.get().tone_attr
                data-state=move || state.get().data_state_attr
                data-truncate=move || state.get().is_truncated.then_some("true")
                data-aria-source=move || state.get().aria_source_attr
                data-custom-class=move || state.get().has_custom_class_name.then_some("true")
                data-class-source=move || state.get().class_source_attr
                aria-label=aria_label
            >
                {children()}
            </h1>
        }
        .into_any(),
        HeadingLevel::H2 => view! {
            <h2
                class=move || class.get()
                data-slot="heading"
                data-level=move || state.get().level_attr
                data-tone=move || state.get().tone_attr
                data-state=move || state.get().data_state_attr
                data-truncate=move || state.get().is_truncated.then_some("true")
                data-aria-source=move || state.get().aria_source_attr
                data-custom-class=move || state.get().has_custom_class_name.then_some("true")
                data-class-source=move || state.get().class_source_attr
                aria-label=aria_label
            >
                {children()}
            </h2>
        }
        .into_any(),
        HeadingLevel::H3 => view! {
            <h3
                class=move || class.get()
                data-slot="heading"
                data-level=move || state.get().level_attr
                data-tone=move || state.get().tone_attr
                data-state=move || state.get().data_state_attr
                data-truncate=move || state.get().is_truncated.then_some("true")
                data-aria-source=move || state.get().aria_source_attr
                data-custom-class=move || state.get().has_custom_class_name.then_some("true")
                data-class-source=move || state.get().class_source_attr
                aria-label=aria_label
            >
                {children()}
            </h3>
        }
        .into_any(),
        HeadingLevel::H4 => view! {
            <h4
                class=move || class.get()
                data-slot="heading"
                data-level=move || state.get().level_attr
                data-tone=move || state.get().tone_attr
                data-state=move || state.get().data_state_attr
                data-truncate=move || state.get().is_truncated.then_some("true")
                data-aria-source=move || state.get().aria_source_attr
                data-custom-class=move || state.get().has_custom_class_name.then_some("true")
                data-class-source=move || state.get().class_source_attr
                aria-label=aria_label
            >
                {children()}
            </h4>
        }
        .into_any(),
        HeadingLevel::H5 => view! {
            <h5
                class=move || class.get()
                data-slot="heading"
                data-level=move || state.get().level_attr
                data-tone=move || state.get().tone_attr
                data-state=move || state.get().data_state_attr
                data-truncate=move || state.get().is_truncated.then_some("true")
                data-aria-source=move || state.get().aria_source_attr
                data-custom-class=move || state.get().has_custom_class_name.then_some("true")
                data-class-source=move || state.get().class_source_attr
                aria-label=aria_label
            >
                {children()}
            </h5>
        }
        .into_any(),
        HeadingLevel::H6 => view! {
            <h6
                class=move || class.get()
                data-slot="heading"
                data-level=move || state.get().level_attr
                data-tone=move || state.get().tone_attr
                data-state=move || state.get().data_state_attr
                data-truncate=move || state.get().is_truncated.then_some("true")
                data-aria-source=move || state.get().aria_source_attr
                data-custom-class=move || state.get().has_custom_class_name.then_some("true")
                data-class-source=move || state.get().class_source_attr
                aria-label=aria_label
            >
                {children()}
            </h6>
        }
        .into_any(),
    }
}
