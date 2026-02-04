use crate::action_button::ActionButtonSize;
use crate::action_button_group::{ActionButtonGroupDensity, ActionButtonGroupOrientation, logic};
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
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
) -> impl IntoView {
    provide_context(logic::ActionButtonGroupContextValue {
        size,
        density,
        orientation,
        is_justified,
        is_quiet,
        is_disabled: disabled,
    });

    let aria_label = aria_label
        .filter(|value| !value.trim().is_empty())
        .unwrap_or_else(|| "Action button group".to_string());

    let base_class = format!(
        "ui-action-button-group {} {} {}",
        density.class_name(),
        orientation.class_name(),
        if is_justified {
            "ui-action-button-group--justified"
        } else {
            ""
        }
    );

    let class = class_name
        .filter(|value| !value.trim().is_empty())
        .map(|value| format!("{base_class} {value}"))
        .unwrap_or(base_class);

    view! {
        <div
            class=class
            data-slot="action-button-group"
            data-disabled=disabled.then_some("true")
            role="toolbar"
            aria-label=aria_label
            aria-orientation=orientation.aria_orientation()
            aria-disabled=disabled.then_some("true")
        >
            {children()}
        </div>
    }
}
