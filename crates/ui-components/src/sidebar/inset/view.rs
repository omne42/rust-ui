use crate::sidebar::SidebarSide;
use crate::sidebar_inset::{
    SidebarInsetStateInput,
    logic::{self},
};
use leptos::prelude::*;

#[component]
pub fn SidebarInset(
    children: Children,
    #[prop(optional)] side: SidebarSide,
    #[prop(optional, default = true)] padded: bool,
    #[prop(optional, default = true)] recessed: bool,
    #[prop(optional)] disabled: bool,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
) -> impl IntoView {
    let (aria_label, has_custom_aria_label) = logic::normalize_aria_label(aria_label);
    let class_name = logic::normalize_optional_text(class_name);
    let has_custom_class_name = class_name.is_some();
    let class_name = StoredValue::new(class_name);

    let state = Memo::new(move |_| {
        logic::resolve_state(SidebarInsetStateInput {
            side,
            padded,
            recessed,
            disabled,
            has_custom_aria_label,
            has_custom_class_name,
        })
    });

    let class = Memo::new(move |_| logic::compose_class_name(class_name.get_value(), state.get()));

    view! {
        <section
            class=move || class.get()
            data-slot="sidebar-inset"
            data-side=move || state.get().side_attr
            data-state=move || state.get().state_attr
            data-padding=move || state.get().padding_attr
            data-surface=move || state.get().surface_attr
            data-padded=move || state.get().padded.then_some("true")
            data-compact=move || state.get().compact.then_some("true")
            data-recessed=move || state.get().recessed.then_some("true")
            data-plain=move || state.get().plain.then_some("true")
            data-disabled=move || state.get().disabled.then_some("true")
            data-enabled=move || state.get().enabled.then_some("true")
            data-aria-source=move || state.get().aria_source_attr
            data-class-source=move || state.get().class_source_attr
            data-custom-class=move || state.get().has_custom_class_name.then_some("true")
            role="region"
            aria-label=aria_label
        >
            {children()}
        </section>
    }
}
