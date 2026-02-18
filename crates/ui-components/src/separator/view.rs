use crate::separator::{
    SeparatorElementType, SeparatorMotion, SeparatorOrientation,
    logic::{self, SeparatorNormalizeInput},
    motion as separator_motion,
};
use leptos::{html, prelude::*};
use ui_headless::{A11yDirection, SeparatorOptions, use_separator};

#[component]
pub fn Separator(
    #[prop(optional)] orientation: Option<SeparatorOrientation>,
    #[prop(optional)] is_decorative: Option<bool>,
    #[prop(optional)] element_type: Option<SeparatorElementType>,
    #[prop(optional, into)] lang: Option<String>,
    #[prop(optional)] dir: Option<A11yDirection>,
    #[prop(optional)] motion: SeparatorMotion,
    #[prop(optional, into)] class_name: Option<String>,
) -> impl IntoView {
    let motion = crate::separator::motion::sanitize_motion(motion);
    let normalized = logic::normalize_props(SeparatorNormalizeInput {
        orientation,
        is_decorative,
        element_type,
        class_name,
    });
    let state = logic::resolve_state(normalized.state_input);
    let class = logic::compose_class_name(normalized.class_name, state);
    let separator_a11y = use_separator(SeparatorOptions { state, lang, dir });

    let role = separator_a11y.attrs.role;
    let aria_orientation = separator_a11y.attrs.aria_orientation;
    let aria_hidden = separator_a11y.attrs.aria_hidden;
    let lang = separator_a11y.attrs.lang;
    let dir = separator_a11y.attrs.dir;
    let motion_source = if motion == SeparatorMotion::default() {
        "default"
    } else {
        "custom"
    };
    let custom_motion = (motion != SeparatorMotion::default()).then_some("true");

    if matches!(state.element_type, SeparatorElementType::Hr) {
        let node_ref: NodeRef<html::Hr> = NodeRef::new();
        separator_motion::attach_motion(node_ref, state.orientation, motion);

        view! {
            <hr
                node_ref=node_ref
                class=class
                data-slot="separator"
                data-state=state.state_attr
                data-state-source=state.state_source_attr
                data-ui-schema=state.ui_schema_attr
                data-ui-intent=state.intent_attr
                data-ui-action=state.action_attr
                data-output-mode=state.output_mode_attr
                data-streaming-fallback=state.streaming_fallback_attr
                data-output-status=state.output_status_attr
                data-orientation=state.orientation_attr
                data-element=state.element_attr
                data-decorative=state.is_decorative.then_some("true")
                data-semantic=state.is_semantic.then_some("true")
                data-custom-class=state.has_custom_class_name.then_some("true")
                data-motion-source=motion_source
                data-custom-motion=custom_motion
                role=role
                aria-orientation=aria_orientation
                aria-hidden=aria_hidden
                lang=lang
                dir=dir
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
                data-state-source=state.state_source_attr
                data-ui-schema=state.ui_schema_attr
                data-ui-intent=state.intent_attr
                data-ui-action=state.action_attr
                data-output-mode=state.output_mode_attr
                data-streaming-fallback=state.streaming_fallback_attr
                data-output-status=state.output_status_attr
                data-orientation=state.orientation_attr
                data-element=state.element_attr
                data-decorative=state.is_decorative.then_some("true")
                data-semantic=state.is_semantic.then_some("true")
                data-custom-class=state.has_custom_class_name.then_some("true")
                data-motion-source=motion_source
                data-custom-motion=custom_motion
                role=role
                aria-orientation=aria_orientation
                aria-hidden=aria_hidden
                lang=lang
                dir=dir
            ></div>
        }
        .into_any()
    }
}
