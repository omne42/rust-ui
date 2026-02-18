use crate::spacer::{
    SpacerAxis, SpacerSize,
    logic::{self, SpacerStateInput},
};
use leptos::{html, prelude::*};
use ui_headless::{A11yDirection, SpacerOptions, use_spacer};

#[component]
pub fn Spacer(
    #[prop(optional)] axis: SpacerAxis,
    #[prop(optional)] size: SpacerSize,
    #[prop(optional, into)] lang: Option<String>,
    #[prop(optional)] dir: Option<A11yDirection>,
    #[prop(optional)] motion: super::motion::SpacerMotion,
    #[prop(optional, into)] class_name: Option<String>,
) -> impl IntoView {
    let class_name = logic::normalize_optional_text(class_name);
    let state = logic::resolve_state(SpacerStateInput {
        axis,
        size,
        has_custom_class_name: class_name.is_some(),
    });
    let class = logic::compose_class_name(class_name, state);
    let spacer = use_spacer(SpacerOptions { state, lang, dir });
    let attrs = spacer.attrs;
    let motion = super::motion::sanitize_motion(motion);
    let motion_source = super::motion::source_attr(motion);
    let custom_motion = (motion_source == "custom").then_some("true");
    let node_ref: NodeRef<html::Div> = NodeRef::new();
    super::motion::attach_motion(node_ref, motion);

    view! {
        <div
            class=class
            node_ref=node_ref
            role=attrs.role
            aria-hidden=attrs.aria_hidden
            lang=attrs.lang.clone()
            dir=attrs.dir
            data-slot=attrs.data_slot
            data-axis=attrs.data_axis
            data-size=attrs.data_size
            data-state=attrs.data_state
            data-vertical=attrs.data_vertical
            data-horizontal=attrs.data_horizontal
            data-custom-class=attrs.data_custom_class
            data-motion-source=motion_source
            data-custom-motion=custom_motion
        ></div>
    }
}
