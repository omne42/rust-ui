use crate::ripple::{
    logic::{self, RippleStateInput},
    motion::{self, RippleMotion},
};
use leptos::{html, prelude::*};

#[component]
pub fn MotionRipple(
    #[prop(optional)] node_ref: NodeRef<html::Span>,
    #[prop(default = true)] bounded: bool,
    #[prop(optional)] motion: RippleMotion,
    #[prop(optional, into)] class_name: Option<String>,
) -> impl IntoView {
    let class_name = logic::normalize_optional_text(class_name);
    let motion = motion::sanitize_motion(motion);

    let state = logic::resolve_state(RippleStateInput {
        phase: logic::resolve_phase(motion.enabled),
        boundary: logic::resolve_boundary(bounded),
        has_custom_motion: motion != RippleMotion::default(),
        has_custom_class_name: class_name.is_some(),
    });

    let class = logic::compose_class_name(class_name, state);
    let duration_ms_attr = motion.duration_ms.to_string();

    view! {
        <span
            node_ref=node_ref
            class=class
            data-slot="ripple"
            data-state=state.phase_attr
            data-phase-class=state.phase_class
            data-boundary=state.boundary_attr
            data-bounded=state.is_bounded.then_some("true")
            data-unbounded=state.is_unbounded.then_some("true")
            data-motion-source=state.motion_source_attr
            data-custom-motion=state.has_custom_motion.then_some("true")
            data-custom-class=state.has_custom_class_name.then_some("true")
            data-class-source=state.class_source_attr
            data-duration-ms=duration_ms_attr
            aria-hidden="true"
        ></span>
    }
}
