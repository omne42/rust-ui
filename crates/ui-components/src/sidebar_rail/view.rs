use crate::overlay_open;
use crate::sidebar::SidebarSide;
use crate::sidebar_rail::{
    SidebarRailStateInput,
    logic::{self},
};
use leptos::prelude::*;

#[component]
pub fn SidebarRail(
    #[prop(optional)] open: Option<Signal<bool>>,
    #[prop(optional)] default_open: Option<bool>,
    #[prop(optional)] on_open_change: Option<Callback<bool>>,
    #[prop(optional)] side: SidebarSide,
    #[prop(optional)] disabled: bool,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] label: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
) -> impl IntoView {
    let (aria_label, has_custom_aria_label) = logic::normalize_aria_label(aria_label);
    let (label, has_custom_label) = logic::normalize_label(label);

    let class_name = logic::normalize_optional_text(class_name);
    let has_custom_class_name = class_name.is_some();
    let class_name = StoredValue::new(class_name);

    let default_open = logic::normalize_default_open(default_open);
    let is_controlled = open.is_some();
    let open_state =
        overlay_open::use_controllable_open_state(open, Some(default_open), on_open_change);
    let open = open_state.open;
    let request_open_change = open_state.request_open_change;

    let state = Signal::derive(move || {
        logic::resolve_state(SidebarRailStateInput {
            open: open.get(),
            side,
            disabled,
            is_controlled,
            has_custom_aria_label,
            has_custom_label,
            has_custom_class_name,
        })
    });

    let class =
        Signal::derive(move || logic::compose_class_name(class_name.get_value(), state.get()));

    let on_toggle = Callback::new(move |_| {
        if disabled {
            return;
        }

        request_open_change.run(!open.get_untracked());
    });

    view! {
        <button
            class=move || class.get()
            data-slot="sidebar-rail"
            data-state=move || state.get().state_attr
            data-side=move || state.get().side_attr
            data-open=move || state.get().open.then_some("true")
            data-closed=move || state.get().closed.then_some("true")
            data-disabled=move || state.get().disabled.then_some("true")
            data-enabled=move || state.get().enabled.then_some("true")
            data-controlled=move || state.get().is_controlled.then_some("true")
            data-uncontrolled=move || state.get().is_uncontrolled.then_some("true")
            data-control-mode=move || state.get().control_attr
            data-aria-source=move || state.get().aria_source_attr
            data-label-source=move || state.get().label_source_attr
            data-class-source=move || state.get().class_source_attr
            data-custom-class=move || state.get().has_custom_class_name.then_some("true")
            type="button"
            tabindex=if disabled { -1 } else { 0 }
            disabled=disabled
            aria-disabled=disabled.then_some("true")
            aria-expanded=move || if state.get().open { "true" } else { "false" }
            aria-label=aria_label
            on:click=move |_| on_toggle.run(())
        >
            <span class="ui-sr-only">{label}</span>
        </button>
    }
}
