use crate::button_group::{ButtonGroupMotion, ButtonGroupOrientation, logic, motion};
use leptos::{html, prelude::*};

#[component]
pub fn ButtonGroup(
    #[prop(optional)] orientation: ButtonGroupOrientation,
    #[prop(optional)] attached: bool,
    #[prop(optional)] motion: ButtonGroupMotion,
    #[prop(optional)] node_ref: NodeRef<html::Div>,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
    children: Children,
) -> impl IntoView {
    let (aria_label, has_explicit_label) = logic::normalize_aria_label(aria_label);

    let state = Memo::new(move |_| logic::resolve_state(orientation, attached, has_explicit_label));

    let base_class = format!("ui-button-group {}", orientation.class_name());
    let base_class = if attached {
        format!("{base_class} ui-button-group--attached")
    } else {
        base_class
    };

    let class = class_name
        .filter(|value| !value.trim().is_empty())
        .map(|value| format!("{base_class} {value}"))
        .unwrap_or(base_class);

    motion::attach_motion(node_ref, motion);

    let motion_source = if motion == ButtonGroupMotion::default() {
        "default"
    } else {
        "custom"
    };
    let custom_motion = (motion != ButtonGroupMotion::default()).then_some("true");

    view! {
        <div
            node_ref=node_ref
            class=class
            data-slot="button-group"
            data-orientation=orientation.data_orientation()
            data-horizontal=move || state.get().is_horizontal.then_some("true")
            data-vertical=move || state.get().is_vertical.then_some("true")
            data-attached=move || state.get().is_attached.then_some("true")
            data-detached=move || state.get().is_detached.then_some("true")
            data-has-explicit-label=move || state.get().has_explicit_label.then_some("true")
            data-has-fallback-label=move || state.get().has_fallback_label.then_some("true")
            data-motion-source=motion_source
            data-custom-motion=custom_motion
            role="group"
            aria-label=aria_label
        >
            {children()}
        </div>
    }
}
