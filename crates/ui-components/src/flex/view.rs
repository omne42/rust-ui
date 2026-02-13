use crate::flex::{
    FlexMotion, FlexStateInput,
    logic::{self, FlexAlign, FlexDirection, FlexGap, FlexJustify, FlexWrap},
    motion,
};
use leptos::{html, prelude::*};

#[component]
pub fn Flex(
    #[prop(optional)] direction: FlexDirection,
    #[prop(optional)] wrap: FlexWrap,
    #[prop(optional)] justify: FlexJustify,
    #[prop(optional)] align: FlexAlign,
    #[prop(optional)] gap: FlexGap,
    #[prop(optional)] inline: bool,
    #[prop(optional)] motion: FlexMotion,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
    children: Children,
) -> impl IntoView {
    let (aria_label, has_custom_aria_label) = logic::normalize_aria_label(aria_label);
    let motion = motion::sanitize_motion(motion);
    let has_custom_motion = motion != FlexMotion::default();
    let root_ref: NodeRef<html::Div> = NodeRef::new();
    motion::attach_motion(root_ref, motion);

    let class_name = logic::normalize_optional_text(class_name);
    let has_custom_class_name = class_name.is_some();
    let class_name = StoredValue::new(class_name);

    let state = Memo::new(move |_| {
        logic::resolve_state(FlexStateInput {
            direction,
            wrap,
            justify,
            align,
            gap,
            inline,
            has_custom_aria_label,
            has_custom_class_name,
        })
    });

    let class = Memo::new(move |_| logic::compose_class_name(class_name.get_value(), state.get()));

    view! {
        <div
            node_ref=root_ref
            class=move || class.get()
            data-slot="flex"
            data-direction=move || state.get().direction_attr
            data-wrap=move || state.get().wrap_attr
            data-justify=move || state.get().justify_attr
            data-align=move || state.get().align_attr
            data-gap=move || state.get().gap_attr
            data-inline=move || state.get().is_inline.then_some("true")
            data-state=move || state.get().data_state_attr
            data-aria-source=move || state.get().aria_source_attr
            data-custom-class=move || state.get().has_custom_class_name.then_some("true")
            data-class-source=move || state.get().class_source_attr
            data-motion-source=if has_custom_motion { "custom" } else { "default" }
            data-custom-motion=has_custom_motion.then_some("true")
            aria-label=aria_label
        >
            {children()}
        </div>
    }
}
