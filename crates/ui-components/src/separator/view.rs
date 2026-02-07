use crate::separator::{
    SeparatorElementType, SeparatorMotion, SeparatorOrientation,
    logic::{self, SeparatorStateInput},
    motion as separator_motion,
};
use leptos::{html, prelude::*};

#[component]
pub fn Separator(
    #[prop(optional)] orientation: SeparatorOrientation,
    #[prop(optional)] decorative: bool,
    #[prop(optional)] element_type: SeparatorElementType,
    #[prop(optional)] motion: SeparatorMotion,
    #[prop(optional, into)] class_name: Option<String>,
) -> impl IntoView {
    let class_name = logic::normalize_optional_text(class_name);
    let state = logic::resolve_state(SeparatorStateInput {
        orientation,
        element_type,
        decorative,
        has_custom_class_name: class_name.is_some(),
    });
    let class = logic::compose_class_name(class_name, state);

    let role = state.is_semantic.then_some("separator");
    let aria_orientation = state
        .is_semantic
        .then_some(state.aria_orientation)
        .flatten();
    let aria_hidden = state.is_decorative.then_some("true");

    if matches!(state.element_type, SeparatorElementType::Hr) {
        let node_ref: NodeRef<html::Hr> = NodeRef::new();
        separator_motion::attach_motion(node_ref, state.orientation, motion);

        view! {
            <hr
                node_ref=node_ref
                class=class
                data-slot="separator"
                data-state=state.state_attr
                data-orientation=state.orientation_attr
                data-element=state.element_attr
                data-decorative=state.is_decorative.then_some("true")
                data-semantic=state.is_semantic.then_some("true")
                data-custom-class=state.has_custom_class_name.then_some("true")
                role=role
                aria-orientation=aria_orientation
                aria-hidden=aria_hidden
            />
        }
        .into_any()
    } else {
        let node_ref: NodeRef<html::Div> = NodeRef::new();
        separator_motion::attach_motion(node_ref, state.orientation, motion);

        view! {
            <div
                node_ref=node_ref
                class=class
                data-slot="separator"
                data-state=state.state_attr
                data-orientation=state.orientation_attr
                data-element=state.element_attr
                data-decorative=state.is_decorative.then_some("true")
                data-semantic=state.is_semantic.then_some("true")
                data-custom-class=state.has_custom_class_name.then_some("true")
                role=role
                aria-orientation=aria_orientation
                aria-hidden=aria_hidden
            ></div>
        }
        .into_any()
    }
}
