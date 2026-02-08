use crate::view::{
    ViewStateInput,
    logic::{self, ViewBackground, ViewBorder, ViewElement, ViewPadding, ViewRadius, ViewShadow},
};
use leptos::prelude::*;

#[component]
pub fn View(
    #[prop(optional)] background: ViewBackground,
    #[prop(optional)] border: ViewBorder,
    #[prop(optional)] padding: ViewPadding,
    #[prop(optional)] radius: ViewRadius,
    #[prop(optional)] shadow: ViewShadow,
    #[prop(optional)] element: ViewElement,
    #[prop(optional)] fluid: bool,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
    children: Children,
) -> impl IntoView {
    let (aria_label, has_custom_aria_label) = logic::normalize_aria_label(aria_label);

    let class_name = logic::normalize_optional_text(class_name);
    let has_custom_class_name = class_name.is_some();
    let class_name = StoredValue::new(class_name);

    let state = Memo::new(move |_| {
        logic::resolve_state(ViewStateInput {
            background,
            border,
            padding,
            radius,
            shadow,
            element,
            fluid,
            has_custom_aria_label,
            has_custom_class_name,
        })
    });

    let class = Memo::new(move |_| logic::compose_class_name(class_name.get_value(), state.get()));

    match element {
        ViewElement::Div => view! {
            <div
                class=move || class.get()
                data-slot="view"
                data-element=move || state.get().element_attr
                data-background=move || state.get().background_attr
                data-border=move || state.get().border_attr
                data-padding=move || state.get().padding_attr
                data-radius=move || state.get().radius_attr
                data-shadow=move || state.get().shadow_attr
                data-state=move || state.get().data_state_attr
                data-fluid=move || state.get().is_fluid.then_some("true")
                data-aria-source=move || state.get().aria_source_attr
                data-custom-class=move || state.get().has_custom_class_name.then_some("true")
                data-class-source=move || state.get().class_source_attr
                aria-label=aria_label
            >
                {children()}
            </div>
        }
        .into_any(),
        ViewElement::Span => view! {
            <span
                class=move || class.get()
                data-slot="view"
                data-element=move || state.get().element_attr
                data-background=move || state.get().background_attr
                data-border=move || state.get().border_attr
                data-padding=move || state.get().padding_attr
                data-radius=move || state.get().radius_attr
                data-shadow=move || state.get().shadow_attr
                data-state=move || state.get().data_state_attr
                data-fluid=move || state.get().is_fluid.then_some("true")
                data-aria-source=move || state.get().aria_source_attr
                data-custom-class=move || state.get().has_custom_class_name.then_some("true")
                data-class-source=move || state.get().class_source_attr
                aria-label=aria_label
            >
                {children()}
            </span>
        }
        .into_any(),
        ViewElement::Section => view! {
            <section
                class=move || class.get()
                data-slot="view"
                data-element=move || state.get().element_attr
                data-background=move || state.get().background_attr
                data-border=move || state.get().border_attr
                data-padding=move || state.get().padding_attr
                data-radius=move || state.get().radius_attr
                data-shadow=move || state.get().shadow_attr
                data-state=move || state.get().data_state_attr
                data-fluid=move || state.get().is_fluid.then_some("true")
                data-aria-source=move || state.get().aria_source_attr
                data-custom-class=move || state.get().has_custom_class_name.then_some("true")
                data-class-source=move || state.get().class_source_attr
                aria-label=aria_label
            >
                {children()}
            </section>
        }
        .into_any(),
    }
}
