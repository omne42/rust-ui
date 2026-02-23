use crate::sidebar_menu_action::{
    SidebarMenuActionStateInput,
    logic::{self},
};
use leptos::prelude::*;

#[component]
pub fn SidebarMenuAction(
    #[prop(optional)] is_hover_only: Option<bool>,
    #[prop(optional, default = true)] hover_only: bool,
    #[prop(optional)] is_disabled: Option<bool>,
    #[prop(optional)] disabled: bool,
    #[prop(optional, into)] label: Option<String>,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional)] on_press: Option<Callback<()>>,
) -> impl IntoView {
    let is_hover_only = logic::resolve_hover_only(is_hover_only, hover_only);
    let is_disabled = logic::resolve_disabled(is_disabled, disabled);
    let label = logic::normalize_label(label);
    let (aria_label, has_custom_aria_label) = logic::normalize_aria_label(aria_label);
    let class_name = logic::normalize_optional_text(class_name);
    let has_custom_class_name = class_name.is_some();
    let class_name = StoredValue::new(class_name);
    let on_press = StoredValue::new(on_press);

    let state = Memo::new(move |_| {
        logic::resolve_state(SidebarMenuActionStateInput {
            hover_only: is_hover_only,
            disabled: is_disabled,
            has_custom_aria_label,
            has_custom_class_name,
        })
    });

    let class = Memo::new(move |_| logic::compose_class_name(class_name.get_value(), state.get()));

    let on_click = move |_| {
        if is_disabled {
            return;
        }
        if let Some(on_press) = on_press.get_value() {
            on_press.run(());
        }
    };

    view! {
        <button
            type="button"
            class=move || class.get()
            data-slot="sidebar-menu-action"
            data-state=move || state.get().state_attr
            data-visibility=move || state.get().visibility_attr
            data-hover-only=move || state.get().hover_only.then_some("true")
            data-always-visible=move || state.get().always_visible.then_some("true")
            data-disabled=move || state.get().disabled.then_some("true")
            data-enabled=move || state.get().enabled.then_some("true")
            data-aria-source=move || state.get().aria_source_attr
            data-class-source=move || state.get().class_source_attr
            data-custom-class=move || state.get().has_custom_class_name.then_some("true")
            aria-label=aria_label
            disabled=is_disabled
            on:click=on_click
        >
            {label}
        </button>
    }
}
