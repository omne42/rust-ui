use crate::content::{
    ContentStateInput,
    logic::{self, ContentTone},
};
use leptos::prelude::*;

#[component]
pub fn Content(
    #[prop(optional)] tone: ContentTone,
    #[prop(optional)] padded: bool,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
    children: Children,
) -> impl IntoView {
    let (aria_label, has_custom_aria_label) = logic::normalize_aria_label(aria_label);

    let class_name = logic::normalize_optional_text(class_name);
    let has_custom_class_name = class_name.is_some();
    let class_name = StoredValue::new(class_name);

    let state = Memo::new(move |_| {
        logic::resolve_state(ContentStateInput {
            tone,
            padded,
            has_custom_aria_label,
            has_custom_class_name,
        })
    });

    let class = Memo::new(move |_| logic::compose_class_name(class_name.get_value(), state.get()));

    view! {
        <section
            class=move || class.get()
            data-slot="content"
            data-tone=move || state.get().tone_attr
            data-state=move || state.get().data_state_attr
            data-padded=move || state.get().is_padded.then_some("true")
            data-aria-source=move || state.get().aria_source_attr
            data-custom-class=move || state.get().has_custom_class_name.then_some("true")
            data-class-source=move || state.get().class_source_attr
            aria-label=aria_label
        >
            {children()}
        </section>
    }
}
