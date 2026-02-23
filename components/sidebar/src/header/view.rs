use crate::sidebar_header::{
    SidebarHeaderStateInput,
    logic::{self},
};
use leptos::prelude::*;

#[component]
pub fn SidebarHeader(
    children: Children,
    #[prop(optional)] is_disabled: Option<bool>,
    #[prop(optional)] disabled: bool,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
) -> impl IntoView {
    let is_disabled = logic::resolve_disabled(is_disabled, disabled);
    let (aria_label, has_custom_aria_label) = logic::normalize_aria_label(aria_label);
    let class_name = logic::normalize_optional_text(class_name);
    let has_custom_class_name = class_name.is_some();
    let class_name = StoredValue::new(class_name);

    let state = Memo::new(move |_| {
        logic::resolve_state(SidebarHeaderStateInput {
            disabled: is_disabled,
            has_custom_aria_label,
            has_custom_class_name,
        })
    });

    let class = Memo::new(move |_| logic::compose_class_name(class_name.get_value(), state.get()));

    view! {
        <div
            class=move || class.get()
            data-slot="sidebar-header"
            data-state=move || state.get().state_attr
            data-disabled=move || state.get().disabled.then_some("true")
            data-enabled=move || state.get().enabled.then_some("true")
            data-aria-source=move || state.get().aria_source_attr
            data-class-source=move || state.get().class_source_attr
            data-custom-class=move || state.get().has_custom_class_name.then_some("true")
            role="group"
            aria-label=aria_label
        >
            {children()}
        </div>
    }
}
