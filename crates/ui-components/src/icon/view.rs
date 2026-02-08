use crate::icon::{
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
    children: Children,
) -> impl IntoView {
    let class_name = logic::normalize_optional_text(class_name);
    let (aria_label, has_custom_aria_label) = logic::normalize_aria_label(aria_label);

    let state = logic::resolve_state(IconStateInput {
        size,
        tone,
        disabled,
        decorative,
        has_custom_aria_label,
        has_custom_class_name: class_name.is_some(),
    });

    let class = logic::compose_class_name(class_name, state);

    view! {
        <span
            class=class
            role=(!state.is_decorative).then_some("img")
            aria-label=state.has_accessible_name.then_some(aria_label)
            aria-hidden=state.is_decorative.then_some("true")
            data-slot="icon"
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
