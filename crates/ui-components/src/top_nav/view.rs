use super::{
    TopNavItem, TopNavMotion, TopNavStateInput,
    logic::{self},
};
use crate::NavigationMenu;
use leptos::prelude::*;

#[component]
pub fn TopNav(
    id_base: String,
    items: Vec<TopNavItem>,
    #[prop(optional)] selected_id: Option<Signal<Option<String>>>,
    #[prop(optional, into)] default_selected_id: Option<String>,
    #[prop(optional)] on_selected_id_change: Option<Callback<Option<String>>>,
    #[prop(default = true)] activate_on_focus: bool,
    #[prop(optional)] motion: TopNavMotion,
    #[prop(optional, into)] label: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
) -> impl IntoView {
    let (label, has_custom_label) = logic::normalize_label(label);
    let class_name = logic::normalize_optional_text(class_name);
    let has_custom_class_name = class_name.is_some();
    let navigation_class_name = class_name.clone().unwrap_or_default();

    let (default_selected_id, has_default_selected_id) =
        logic::normalize_default_selected_id(default_selected_id);

    let state = logic::resolve_state(TopNavStateInput {
        is_controlled: selected_id.is_some(),
        has_default_selected_id,
        activate_on_focus,
        has_custom_label,
        has_custom_class_name,
        has_custom_motion: motion != TopNavMotion::default(),
    });

    let class_name = logic::compose_class_name(class_name, state);

    let navigation_menu: AnyView = match (selected_id, on_selected_id_change) {
        (Some(selected_id), Some(on_selected_id_change)) => view! {
            <NavigationMenu
                id_base=id_base
                items=items
                selected_id=selected_id
                default_selected_id=default_selected_id
                on_selected_id_change=on_selected_id_change
                activate_on_focus=activate_on_focus
                motion=motion
                aria_label=label
                class_name=navigation_class_name
            />
        }
        .into_any(),
        (Some(selected_id), None) => view! {
            <NavigationMenu
                id_base=id_base
                items=items
                selected_id=selected_id
                default_selected_id=default_selected_id
                activate_on_focus=activate_on_focus
                motion=motion
                aria_label=label
                class_name=navigation_class_name
            />
        }
        .into_any(),
        (None, Some(on_selected_id_change)) => view! {
            <NavigationMenu
                id_base=id_base
                items=items
                default_selected_id=default_selected_id
                on_selected_id_change=on_selected_id_change
                activate_on_focus=activate_on_focus
                motion=motion
                aria_label=label
                class_name=navigation_class_name
            />
        }
        .into_any(),
        (None, None) => view! {
            <NavigationMenu
                id_base=id_base
                items=items
                default_selected_id=default_selected_id
                activate_on_focus=activate_on_focus
                motion=motion
                aria_label=label
                class_name=navigation_class_name
            />
        }
        .into_any(),
    };

    view! {
        <div
            class=class_name
            data-slot="top-nav"
            data-state=state.state_attr
            data-selection-mode=state.selection_mode_attr
            data-default-selection=state.default_selection_attr
            data-has-default-selection=state.has_default_selected_id.then_some("true")
            data-focus-activation=state.focus_activation_attr
            data-label-source=state.label_source_attr
            data-class-source=state.class_source_attr
            data-motion-source=state.motion_source_attr
            data-custom-label=state.has_custom_label.then_some("true")
            data-custom-motion=state.has_custom_motion.then_some("true")
            data-custom-class=state.has_custom_class_name.then_some("true")
        >
            {navigation_menu}
        </div>
    }
}
