use crate::sidebar_content::{
    SidebarContentStateInput,
    logic::{self},
};
use leptos::prelude::*;

#[component]
pub fn SidebarContent(
    children: Children,
    #[prop(optional)] disabled: bool,
    #[prop(optional, default = true)] padded: bool,
    #[prop(optional, default = true)] scrollable: bool,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
) -> impl IntoView {
    let (aria_label, has_custom_aria_label) = logic::normalize_aria_label(aria_label);
    let class_name = logic::normalize_optional_text(class_name);
    let has_custom_class_name = class_name.is_some();
    let class_name = StoredValue::new(class_name);

    let state = Memo::new(move |_| {
        logic::resolve_state(SidebarContentStateInput {
            disabled,
            padded,
            scrollable,
            has_custom_aria_label,
            has_custom_class_name,
        })
    });

    let class = Memo::new(move |_| logic::compose_class_name(class_name.get_value(), state.get()));

    view! {
        <div
            class=move || class.get()
            data-slot="sidebar-content"
            data-state=move || state.get().state_attr
            data-padding=move || state.get().padding_attr
            data-scroll=move || state.get().scroll_attr
            data-padded=move || state.get().padded.then_some("true")
            data-compact=move || state.get().compact.then_some("true")
            data-scrollable=move || state.get().scrollable.then_some("true")
            data-static=move || state.get().static_layout.then_some("true")
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
