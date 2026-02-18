use crate::divider::{
    DividerMotion, DividerOrientation,
    logic::{self, DividerStateInput},
    motion,
};
use leptos::{html, prelude::*};
use ui_headless::{A11yDirection, DividerOptions, use_divider};

#[component]
pub fn Divider(
    #[prop(optional)] orientation: DividerOrientation,
    #[prop(optional)] motion: DividerMotion,
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional, into)] lang: Option<String>,
    #[prop(optional)] dir: Option<A11yDirection>,
) -> impl IntoView {
    let class_name = logic::normalize_optional_text(class_name);
    let state = logic::resolve_state(DividerStateInput {
        orientation,
        has_custom_class_name: class_name.is_some(),
    });
    let class = logic::compose_class_name(class_name, state);
    let divider = use_divider(DividerOptions { state, lang, dir });
    let attrs = divider.attrs;
    let motion = motion::sanitize_motion(motion);
    let has_custom_motion = motion != DividerMotion::default();
    let root_ref: NodeRef<html::Div> = NodeRef::new();
    motion::attach_motion(root_ref, orientation, motion);

    view! {
        <div
            node_ref=root_ref
            class=class
            data-slot="divider"
            data-orientation=attrs.data_orientation
            data-state=attrs.data_state
            data-horizontal=attrs.data_horizontal
            data-vertical=attrs.data_vertical
            data-custom-class=attrs.data_custom_class
            data-motion-source=if has_custom_motion { "custom" } else { "default" }
            data-custom-motion=has_custom_motion.then_some("true")
            role=attrs.role
            aria-orientation=attrs.aria_orientation
            lang=attrs.lang
            dir=attrs.dir
        ></div>
    }
}
