use crate::separator::{SeparatorElementType, SeparatorMotion, SeparatorOrientation, logic};
use leptos::prelude::*;

#[component]
pub fn Separator(
    #[prop(optional)] orientation: SeparatorOrientation,
    #[prop(optional)] decorative: bool,
    #[prop(optional)] element_type: SeparatorElementType,
    #[prop(optional)] motion: SeparatorMotion,
    #[prop(optional, into)] class_name: Option<String>,
) -> impl IntoView {
    let _ = motion;

    let state = logic::resolve_state(orientation, element_type, decorative);

    let base_class = format!("ui-separator {}", state.orientation.class_name());
    let class = class_name
        .filter(|value| !value.trim().is_empty())
        .map(|value| format!("{base_class} {value}"))
        .unwrap_or(base_class);

    let role = (!state.is_decorative).then_some("separator");
    let aria_orientation = (!state.is_decorative).then_some(state.orientation.aria_orientation());
    let aria_hidden = state.is_decorative.then_some("true");

    if matches!(state.element_type, SeparatorElementType::Hr) {
        view! {
            <hr
                class=class
                data-slot="separator"
                data-orientation=state.orientation.aria_orientation().unwrap_or("horizontal")
                data-element=state.element_type.as_attr()
                data-decorative=state.is_decorative.then_some("true")
                role=role
                aria-orientation=aria_orientation.flatten()
                aria-hidden=aria_hidden
            />
        }
        .into_any()
    } else {
        view! {
            <div
                class=class
                data-slot="separator"
                data-orientation=state.orientation.aria_orientation().unwrap_or("horizontal")
                data-element=state.element_type.as_attr()
                data-decorative=state.is_decorative.then_some("true")
                role=role
                aria-orientation=aria_orientation.flatten()
                aria-hidden=aria_hidden
            ></div>
        }
        .into_any()
    }
}
