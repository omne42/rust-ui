use crate::overlays::{
    OverlaysRootStateInput,
    logic::{self, compose_root_class_name, resolve_root_state},
};
use leptos::prelude::*;
use ui_headless::{A11yDirection, labeled_group_attrs};

#[component]
pub fn OverlaysRoot(
    #[prop(optional, into)] id_base: Option<String>,
    #[prop(optional)] is_open: bool,
    #[prop(optional)] is_modal: bool,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] lang: Option<String>,
    #[prop(optional)] dir: Option<A11yDirection>,
    #[prop(optional, into)] class_name: Option<String>,
    children: Children,
) -> impl IntoView {
    let (id_base, has_custom_id_base) = logic::normalize_id_base(id_base);
    let (aria_label, has_custom_aria_label) = logic::normalize_aria_label(aria_label);
    let group_a11y = labeled_group_attrs(aria_label, logic::normalize_optional_text(lang), dir);

    let class_name = logic::normalize_optional_text(class_name);
    let has_custom_class_name = class_name.is_some();
    let class_name = StoredValue::new(class_name);

    let state = Memo::new(move |_| {
        resolve_root_state(OverlaysRootStateInput {
            open: is_open,
            modal: is_modal,
            has_custom_id_base,
            has_custom_class_name,
        })
    });

    let class = Memo::new(move |_| compose_root_class_name(class_name.get_value(), state.get()));

    view! {
        <div
            id=id_base
            class=move || class.get()
            role=group_a11y.role
            aria-label=group_a11y.aria_label.clone()
            lang=group_a11y.lang.clone()
            dir=group_a11y.dir
            data-slot="overlays"
            data-state=move || state.get().data_state_attr
            data-layer=move || state.get().layer_kind_attr
            data-open=move || state.get().is_open.then_some("true")
            data-closed=move || state.get().is_closed.then_some("true")
            data-aria-label-source=if has_custom_aria_label { "custom" } else { "default" }
            data-id-source=move || state.get().id_source_attr
            data-class-source=move || state.get().class_source_attr
            data-custom-aria-label=has_custom_aria_label.then_some("true")
            data-custom-id=move || state.get().has_custom_id_base.then_some("true")
            data-custom-class=move || state.get().has_custom_class_name.then_some("true")
        >
            {children()}
        </div>
    }
}
