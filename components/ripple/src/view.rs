use crate::{RippleMotion, logic};
use leptos::{html, prelude::*};
use ui_headless::{A11yDirection, locale_attrs};

#[component]
pub fn MotionRipple(
    #[prop(optional)] node_ref: NodeRef<html::Span>,
    #[prop(optional, into)] is_bounded: Option<bool>,
    #[prop(optional)] motion: RippleMotion,
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional, into)] lang: Option<String>,
    #[prop(optional)] dir: Option<A11yDirection>,
) -> impl IntoView {
    let state = logic::resolve_render_state(logic::RippleRenderInput {
        is_bounded,
        motion,
        class_name,
    });
    let locale = locale_attrs(logic::normalize_optional_text(lang), dir);

    let duration_ms_attr = state.motion.duration_ms.to_string();

    view! {
        <span
            node_ref=node_ref
            class=state.class_name
            style=state.style_vars
            data-slot="ripple"
            data-state=state.state.phase_attr
            data-phase-class=state.state.phase_class
            data-boundary=state.state.boundary_attr
            data-bounded=state.state.is_bounded.then_some("true")
            data-unbounded=state.state.is_unbounded.then_some("true")
            data-motion-source=state.state.motion_source_attr
            data-custom-motion=state.state.has_custom_motion.then_some("true")
            data-custom-class=state.state.has_custom_class_name.then_some("true")
            data-class-source=state.state.class_source_attr
            data-duration-ms=duration_ms_attr
            data-ui-schema="ripple.v1"
            lang=locale.lang.clone()
            dir=locale.dir
            aria-hidden="true"
        ></span>
    }
}
