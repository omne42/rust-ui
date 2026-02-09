use crate::{Sidebar, SidebarCollapsible, SidebarSide, SidebarVariant};
use leptos::prelude::*;

#[component]
pub fn Sidenav(
    children: Children,
    #[prop(optional)] open: Option<Signal<bool>>,
    #[prop(optional)] default_open: Option<bool>,
    #[prop(optional)] on_open_change: Option<Callback<bool>>,
    #[prop(optional)] side: SidebarSide,
    #[prop(optional)] variant: SidebarVariant,
    #[prop(optional)] collapsible: SidebarCollapsible,
    #[prop(optional)] disabled: bool,
    #[prop(optional, default = true)] show_trigger: bool,
    #[prop(optional, default = true)] enable_shortcut: bool,
    #[prop(optional, into)] shortcut_key: Option<String>,
    #[prop(optional, into)] trigger_label: Option<String>,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
) -> impl IntoView {
    let default_open = default_open.unwrap_or(true);
    let on_open_change = on_open_change.unwrap_or_else(|| Callback::new(|_: bool| {}));
    let shortcut_key = shortcut_key.unwrap_or_default();
    let trigger_label = trigger_label.unwrap_or_default();
    let aria_label = aria_label.unwrap_or_default();
    let class_name = class_name.unwrap_or_default();

    if let Some(open) = open {
        view! {
            <Sidebar
                open=open
                default_open=default_open
                on_open_change=on_open_change
                side=side
                variant=variant
                collapsible=collapsible
                disabled=disabled
                show_trigger=show_trigger
                enable_shortcut=enable_shortcut
                shortcut_key=shortcut_key
                trigger_label=trigger_label
                aria_label=aria_label
                class_name=class_name
            >
                {children()}
            </Sidebar>
        }
        .into_any()
    } else {
        view! {
            <Sidebar
                default_open=default_open
                on_open_change=on_open_change
                side=side
                variant=variant
                collapsible=collapsible
                disabled=disabled
                show_trigger=show_trigger
                enable_shortcut=enable_shortcut
                shortcut_key=shortcut_key
                trigger_label=trigger_label
                aria_label=aria_label
                class_name=class_name
            >
                {children()}
            </Sidebar>
        }
        .into_any()
    }
}
