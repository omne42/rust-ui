use super::{TopNavItem, TopNavMotion};
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
    let default_selected_id = default_selected_id.unwrap_or_default();
    let on_selected_id_change =
        on_selected_id_change.unwrap_or_else(|| Callback::new(|_: Option<String>| {}));
    let label = label.unwrap_or_default();
    let class_name = class_name.unwrap_or_default();

    let motion_source = if motion == TopNavMotion::default() {
        "default"
    } else {
        "custom"
    };
    let custom_motion = (motion != TopNavMotion::default()).then_some("true");

    if let Some(selected_id) = selected_id {
        view! {
            <div
                class="ui-top-nav"
                data-slot="top-nav"
                data-motion-source=motion_source
                data-custom-motion=custom_motion
            >
                <NavigationMenu
                    id_base=id_base
                    items=items
                    selected_id=selected_id
                    default_selected_id=default_selected_id
                    on_selected_id_change=on_selected_id_change
                    activate_on_focus=activate_on_focus
                    motion=motion
                    aria_label=label
                    class_name=class_name
                />
            </div>
        }
        .into_any()
    } else {
        view! {
            <div
                class="ui-top-nav"
                data-slot="top-nav"
                data-motion-source=motion_source
                data-custom-motion=custom_motion
            >
                <NavigationMenu
                    id_base=id_base
                    items=items
                    default_selected_id=default_selected_id
                    on_selected_id_change=on_selected_id_change
                    activate_on_focus=activate_on_focus
                    motion=motion
                    aria_label=label
                    class_name=class_name
                />
            </div>
        }
        .into_any()
    }
}
