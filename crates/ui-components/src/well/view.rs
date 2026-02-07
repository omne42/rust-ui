use crate::well::{
    WellDensity, WellStateInput,
    logic::{self, WellTone},
};
use leptos::prelude::*;

#[component]
pub fn Well(
    #[prop(optional)] tone: WellTone,
    #[prop(optional)] density: WellDensity,
    #[prop(optional)] inset: bool,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
    children: Children,
) -> impl IntoView {
    let class_name = logic::normalize_optional_text(class_name);
    let class_name = StoredValue::new(class_name);
    let (aria_label, has_custom_label) = logic::normalize_aria_label(aria_label);

    let state = Signal::derive(move || {
        logic::resolve_state(WellStateInput {
            tone,
            density,
            inset,
            has_custom_label,
            has_custom_class_name: class_name.get_value().is_some(),
        })
    });

    let class =
        Signal::derive(move || logic::compose_class_name(class_name.get_value(), state.get()));

    view! {
        <section
            class=move || class.get()
            data-slot="well"
            data-tone=move || state.get().tone_attr
            data-density=move || state.get().density_attr
            data-state=move || if state.get().is_inset { "inset" } else { "default" }
            data-inset=move || state.get().is_inset.then_some("true")
            data-label-source=move || state.get().label_source_attr
            data-custom-class=move || state.get().has_custom_class_name.then_some("true")
            data-class-source=move || state.get().class_source_attr
            role="region"
            aria-label=aria_label
        >
            {children()}
        </section>
    }
}
