use super::logic::{self, SidebarCollapsible, SidebarSide, SidebarStateInput, SidebarVariant};
use leptos::{ev, prelude::*};
use ui_headless as overlay_open;

#[component]
pub fn Sidebar(
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
    let class_name = logic::normalize_optional_text(class_name);
    let trigger_label = logic::normalize_optional_text(trigger_label)
        .unwrap_or_else(|| "Toggle sidebar".to_string());
    let aria_label = logic::normalize_aria_label(aria_label);
    let default_open = logic::normalize_default_open(default_open);
    let shortcut_key = logic::normalize_shortcut_key(shortcut_key, enable_shortcut);

    let is_controlled = open.is_some();
    let open_state = overlay_open::use_controllable_open_state_traced(
        "sidebar",
        open,
        Some(default_open),
        on_open_change,
    );
    let open = open_state.open;
    let request_open_change = open_state.request_open_change;

    let class_name = StoredValue::new(class_name);
    let trigger_label = StoredValue::new(trigger_label);
    let aria_label = StoredValue::new(aria_label);
    let shortcut_key = StoredValue::new(shortcut_key);

    let state = Signal::derive(move || {
        logic::resolve_state(SidebarStateInput {
            side,
            variant,
            collapsible,
            open: open.get(),
            disabled,
            is_controlled,
            show_trigger,
            has_shortcut_key: shortcut_key.get_value().is_some(),
            has_custom_class_name: class_name.get_value().is_some(),
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

    let on_key_down = move |event: ev::KeyboardEvent| {
        if logic::should_toggle_for_shortcut(
            &event.key(),
            event.ctrl_key(),
            event.meta_key(),
            shortcut_key.get_value().as_deref(),
            disabled,
        ) {
            request_open_change.run(!open.get_untracked());
            event.prevent_default();
        }
    };

    view! {
        <aside
            class=move || class.get()
            data-slot="sidebar"
            data-side=move || state.get().side_attr
            data-variant=move || state.get().variant_attr
            data-collapsible=move || state.get().collapsible_attr
            data-state=move || state.get().state_attr
            data-open=move || state.get().open.then_some("true")
            data-closed=move || state.get().closed.then_some("true")
            data-disabled=move || state.get().disabled.then_some("true")
            data-enabled=move || state.get().enabled.then_some("true")
            data-controlled=move || state.get().is_controlled.then_some("true")
            data-uncontrolled=move || state.get().is_uncontrolled.then_some("true")
            data-controls=move || state.get().control_attr
            data-class-source=move || state.get().class_source_attr
            data-custom-class=move || state.get().has_custom_class_name.then_some("true")
            data-shortcut=move || shortcut_key.get_value()
            role="complementary"
            aria-label=aria_label.get_value()
            aria-keyshortcuts=move || logic::shortcut_hint(shortcut_key.get_value())
            on:keydown=on_key_down
        >
            <Show when=move || state.get().show_trigger>
                <button
                    class="ui-sidebar__trigger"
                    data-slot="sidebar-trigger"
                    type="button"
                    disabled=disabled
                    aria-disabled=disabled.then_some("true")
                    aria-expanded=move || if state.get().open { "true" } else { "false" }
                    aria-label=trigger_label.get_value()
                    on:click=move |_| on_toggle.run(())
                >
                    {trigger_label.get_value()}
                </button>
            </Show>

            <div class="ui-sidebar__panel" data-slot="sidebar-panel">
                <div class="ui-sidebar__content" data-slot="sidebar-content">
                    {children()}
                </div>
            </div>

            <Show when=move || state.get().show_rail>
                <button
                    class="ui-sidebar__rail"
                    data-slot="sidebar-rail"
                    type="button"
                    tabindex=if disabled { -1 } else { 0 }
                    disabled=disabled
                    aria-disabled=disabled.then_some("true")
                    aria-label="Toggle sidebar"
                    on:click=move |_| on_toggle.run(())
                >
                    <span class="ui-sr-only">"toggle sidebar"</span>
                </button>
            </Show>
        </aside>
    }
}
