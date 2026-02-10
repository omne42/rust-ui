use crate::sidenav::{SidenavStateInput, logic};
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
    let has_custom_open_handler = on_open_change.is_some();
    let on_open_change = on_open_change.unwrap_or_else(|| Callback::new(|_: bool| {}));

    let class_name = logic::normalize_optional_text(class_name);
    let class_name_for_inner = class_name.clone().unwrap_or_default();

    let (aria_label, has_custom_aria_label) = logic::normalize_aria_label(aria_label);
    let (trigger_label, has_custom_trigger_label) = logic::normalize_trigger_label(trigger_label);
    let (shortcut_key, has_custom_shortcut_key) =
        logic::normalize_shortcut_key(shortcut_key, enable_shortcut);

    let state = logic::resolve_state(SidenavStateInput {
        disabled,
        show_trigger,
        enable_shortcut,
        is_controlled: open.is_some(),
        initial_open: default_open,
        has_custom_shortcut_key,
        has_custom_trigger_label,
        has_custom_aria_label,
        has_custom_class_name: class_name.is_some(),
        has_custom_open_handler,
    });

    let class = logic::compose_class_name(class_name, state);

    if let Some(open) = open {
        view! {
            <div
                class=class
                data-slot="sidenav"
                data-state=state.state_attr
                data-open-mode=state.open_mode_attr
                data-initial-open=state.initial_open_attr
                data-trigger-mode=state.trigger_mode_attr
                data-shortcut-mode=state.shortcut_mode_attr
                data-label-source=state.label_source_attr
                data-trigger-source=state.trigger_source_attr
                data-shortcut-source=state.shortcut_source_attr
                data-class-source=state.class_source_attr
                data-handler-source=state.handler_source_attr
                data-disabled=state.is_disabled.then_some("true")
                data-controlled=state.is_controlled.then_some("true")
                data-uncontrolled=(!state.is_controlled).then_some("true")
                data-custom-class=state.has_custom_class_name.then_some("true")
            >
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
                    class_name=class_name_for_inner
                >
                    {children()}
                </Sidebar>
            </div>
        }
        .into_any()
    } else {
        view! {
            <div
                class=class
                data-slot="sidenav"
                data-state=state.state_attr
                data-open-mode=state.open_mode_attr
                data-initial-open=state.initial_open_attr
                data-trigger-mode=state.trigger_mode_attr
                data-shortcut-mode=state.shortcut_mode_attr
                data-label-source=state.label_source_attr
                data-trigger-source=state.trigger_source_attr
                data-shortcut-source=state.shortcut_source_attr
                data-class-source=state.class_source_attr
                data-handler-source=state.handler_source_attr
                data-disabled=state.is_disabled.then_some("true")
                data-controlled=state.is_controlled.then_some("true")
                data-uncontrolled=(!state.is_controlled).then_some("true")
                data-custom-class=state.has_custom_class_name.then_some("true")
            >
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
                    class_name=class_name_for_inner
                >
                    {children()}
                </Sidebar>
            </div>
        }
        .into_any()
    }
}
