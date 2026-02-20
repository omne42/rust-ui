use crate::{
    IconSize, IconStateInput, IconTone,
    logic::{self},
};
use leptos::prelude::*;

#[component]
pub fn Icon(
    #[prop(optional)] size: IconSize,
    #[prop(optional)] tone: IconTone,
    #[prop(optional)] disabled: bool,
    #[prop(default = true)] decorative: bool,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional, into)] slot: Option<String>,
    children: Children,
) -> impl IntoView {
    let class_name = logic::normalize_optional_text(class_name);
    let (aria_label, has_custom_aria_label) = logic::normalize_aria_label(aria_label);
    let slot = logic::normalize_optional_text(slot);
    let slot_kind_attr = logic::resolve_slot_kind_attr(slot.as_deref());
    let has_named_slot = slot.is_some();

    let state = logic::resolve_state(IconStateInput {
        size,
        tone,
        disabled,
        decorative,
        has_custom_aria_label,
        has_custom_class_name: class_name.is_some(),
        slot_kind_attr,
        has_named_slot,
    });

    let class = logic::compose_class_name(class_name, state);

    view! {
        <span
            class=class
            role=(!state.is_decorative).then_some("img")
            aria-label=state.has_accessible_name.then_some(aria_label)
            aria-hidden=state.is_decorative.then_some("true")
            data-slot="icon"
            slot=slot.clone()
            data-slot-name=slot.clone()
            data-slot-kind=state.slot_kind_attr
            data-has-slot=state.has_named_slot.then_some("true")
            data-size=state.size_attr
            data-tone=state.tone_attr
            data-state=state.data_state_attr
            data-disabled=state.is_disabled.then_some("true")
            data-decorative=state.is_decorative.then_some("true")
            data-has-label=state.has_accessible_name.then_some("true")
            data-aria-source=state.aria_source_attr
            data-custom-class=state.has_custom_class_name.then_some("true")
            data-class-source=state.class_source_attr
        >
            <span class="ui-icon__glyph" data-slot="icon-glyph" aria-hidden="true">
                {children()}
            </span>
        </span>
    }
}
