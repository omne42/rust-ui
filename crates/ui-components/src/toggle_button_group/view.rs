use crate::toggle_button_group::{
    ToggleButtonGroupMotion, ToggleButtonGroupOrientation, logic, motion,
};
use leptos::prelude::*;

#[component]
pub fn ToggleButtonGroup(
    #[prop(optional)] orientation: ToggleButtonGroupOrientation,
    #[prop(optional)] attached: bool,
    #[prop(optional)] motion: ToggleButtonGroupMotion,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
    children: Children,
) -> impl IntoView {
    let (aria_label, has_explicit_label) = logic::normalize_aria_label(aria_label);

    let state = Memo::new(move |_| logic::resolve_state(orientation, attached, has_explicit_label));
    let motion = motion::sanitize_motion(motion);
    let has_custom_motion = motion != ToggleButtonGroupMotion::default();

    let base_class = format!("ui-toggle-button-group {}", orientation.class_name());
    let base_class = if attached {
        format!("{base_class} ui-toggle-button-group--attached")
    } else {
        base_class
    };

    let class = class_name
        .filter(|value| !value.trim().is_empty())
        .map(|value| format!("{base_class} {value}"))
        .unwrap_or(base_class);

    view! {
        <div
            class=class
            data-slot="toggle-button-group"
            data-motion-source=if has_custom_motion { "custom" } else { "default" }
            data-custom-motion=has_custom_motion.then_some("true")
            data-orientation=orientation.data_orientation()
            data-horizontal=move || state.get().is_horizontal.then_some("true")
            data-vertical=move || state.get().is_vertical.then_some("true")
            data-attached=move || state.get().is_attached.then_some("true")
            data-detached=move || state.get().is_detached.then_some("true")
            data-has-explicit-label=move || state.get().has_explicit_label.then_some("true")
            data-has-fallback-label=move || state.get().has_fallback_label.then_some("true")
            role="group"
            aria-label=aria_label
        >
            {children()}
        </div>
    }
}
