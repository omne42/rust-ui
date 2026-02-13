use crate::divider::{
    DividerMotion, DividerOrientation,
    logic::{self, DividerStateInput},
    motion,
};
use leptos::{html, prelude::*};

#[component]
pub fn Divider(
    #[prop(optional)] orientation: DividerOrientation,
    #[prop(optional)] motion: DividerMotion,
    #[prop(optional, into)] class_name: Option<String>,
) -> impl IntoView {
    let class_name = logic::normalize_optional_text(class_name);
    let state = logic::resolve_state(DividerStateInput {
        orientation,
        has_custom_class_name: class_name.is_some(),
    });
    let class = logic::compose_class_name(class_name, state);
    let motion = motion::sanitize_motion(motion);
    let has_custom_motion = motion != DividerMotion::default();
    let root_ref: NodeRef<html::Div> = NodeRef::new();
    motion::attach_motion(root_ref, orientation, motion);

    view! {
        <div
            node_ref=root_ref
            class=class
            data-slot="divider"
            data-orientation=state.orientation_attr
            data-state=state.orientation_attr
            data-horizontal=state.is_horizontal.then_some("true")
            data-vertical=state.is_vertical.then_some("true")
            data-custom-class=state.has_custom_class_name.then_some("true")
            data-motion-source=if has_custom_motion { "custom" } else { "default" }
            data-custom-motion=has_custom_motion.then_some("true")
            role="separator"
            aria-orientation=state.aria_orientation
        ></div>
    }
}
