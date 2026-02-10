use crate::auto_height::{
    AutoHeightMotion,
    logic::{self, AutoHeightStateInput},
    motion,
};
use leptos::{html, prelude::*};

#[component]
pub fn AutoHeight(
    children: Children,
    #[prop(optional)] motion: AutoHeightMotion,
    #[prop(optional, into)] class_name: Option<String>,
) -> impl IntoView {
    let class_name = logic::normalize_optional_text(class_name);
    let state = logic::resolve_state(AutoHeightStateInput {
        animate_height: motion.animate_height,
        has_custom_class_name: class_name.is_some(),
        has_custom_motion: motion != AutoHeightMotion::default(),
    });

    let container_ref: NodeRef<html::Div> = NodeRef::new();
    let content_ref: NodeRef<html::Div> = NodeRef::new();
    motion::attach_motion(container_ref, content_ref, motion);

    let class = logic::compose_class_name(class_name, state);

    view! {
        <div
            class=class
            node_ref=container_ref
            data-slot="auto-height"
            data-state=if state.animate_height { "animated" } else { "static" }
            data-animated=state.animate_height.then_some("true")
            data-static=state.is_static.then_some("true")
            data-overflow-hidden=state.overflow_hidden.then_some("true")
            data-custom-class=state.has_custom_class_name.then_some("true")
            data-motion-source=if state.has_custom_motion { "custom" } else { "default" }
            data-custom-motion=state.has_custom_motion.then_some("true")
        >
            <div
                class="ui-auto-height__content"
                node_ref=content_ref
                data-slot="auto-height-content"
                data-state=if state.animate_height { "animated" } else { "static" }
            >
                {children()}
            </div>
        </div>
    }
}
