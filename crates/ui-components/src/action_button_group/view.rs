use crate::action_button::ActionButtonSize;
use crate::action_button_group::{
    ActionButtonGroupDensity, ActionButtonGroupMotion, ActionButtonGroupOrientation, logic, motion,
};
use leptos::prelude::*;

#[component]
pub fn ActionButtonGroup(
    children: Children,
    #[prop(optional)] size: ActionButtonSize,
    #[prop(optional)] density: ActionButtonGroupDensity,
    #[prop(optional)] orientation: ActionButtonGroupOrientation,
    #[prop(optional)] is_justified: bool,
    #[prop(optional)] is_quiet: bool,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] motion: ActionButtonGroupMotion,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
) -> impl IntoView {
    let class_name = logic::normalize_optional_text(class_name);
    let (aria_label, has_explicit_label) = logic::normalize_aria_label(aria_label);

    let state = logic::resolve_state(
        orientation,
        density,
        is_justified,
        is_quiet,
        disabled,
        has_explicit_label,
        class_name.is_some(),
    );

    provide_context(logic::ActionButtonGroupContextValue {
        size,
        density: state.density,
        orientation: state.orientation,
        is_justified: state.is_justified,
        is_quiet: state.is_quiet,
        is_disabled: state.is_disabled,
    });

    let class = logic::compose_class_name(class_name, state);
    let motion = motion::sanitize_motion(motion);
    let has_custom_motion = motion != ActionButtonGroupMotion::default();

    view! {
        <div
            class=class
            data-slot="action-button-group"
            data-state=if state.is_disabled { "disabled" } else { "ready" }
            data-motion-source=if has_custom_motion { "custom" } else { "default" }
            data-custom-motion=has_custom_motion.then_some("true")
            data-orientation=state.orientation_attr
            data-density=state.density_attr
            data-horizontal=state.is_horizontal.then_some("true")
            data-vertical=state.is_vertical.then_some("true")
            data-regular=state.is_regular.then_some("true")
            data-compact=state.is_compact.then_some("true")
            data-justified=state.is_justified.then_some("true")
            data-not-justified=state.is_not_justified.then_some("true")
            data-quiet=state.is_quiet.then_some("true")
            data-filled=state.is_filled.then_some("true")
            data-disabled=state.is_disabled.then_some("true")
            data-enabled=state.is_enabled.then_some("true")
            data-has-explicit-label=state.has_explicit_label.then_some("true")
            data-has-fallback-label=state.has_fallback_label.then_some("true")
            role="toolbar"
            aria-label=aria_label
            aria-orientation=state.orientation.aria_orientation()
            aria-disabled=state.is_disabled.then_some("true")
        >
            {children()}
        </div>
    }
}
