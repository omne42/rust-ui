use crate::overlay_open;
use crate::sidebar_group::logic::{self, SidebarGroupStateInput};
use leptos::{ev, prelude::*};

#[component]
pub fn SidebarGroup(
    children: Children,
    #[prop(optional)] open: Option<Signal<bool>>,
    #[prop(optional)] default_open: Option<bool>,
    #[prop(optional)] on_open_change: Option<Callback<bool>>,
    #[prop(optional)] on_action: Option<Callback<()>>,
    #[prop(optional)] collapsible: bool,
    #[prop(optional)] disabled: bool,
    #[prop(optional, default = true)] show_label: bool,
    #[prop(optional, default = true)] show_action: bool,
    #[prop(optional, into)] label: Option<String>,
    #[prop(optional, into)] action_label: Option<String>,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
) -> impl IntoView {
    let class_name = logic::normalize_optional_text(class_name);
    let label = logic::normalize_label(label);
    let action_label = logic::normalize_action_label(action_label);
    let aria_label = logic::normalize_aria_label(aria_label);

    let default_open = logic::normalize_default_open(default_open);
    let is_controlled = open.is_some();
    let open_state =
        overlay_open::use_controllable_open_state(open, Some(default_open), on_open_change);
    let open = open_state.open;
    let request_open_change = open_state.request_open_change;

    let class_name = StoredValue::new(class_name);
    let label = StoredValue::new(label);
    let action_label = StoredValue::new(action_label);
    let aria_label = StoredValue::new(aria_label);
    let on_action = StoredValue::new(on_action);

    let state = Signal::derive(move || {
        logic::resolve_state(SidebarGroupStateInput {
            open: if collapsible { open.get() } else { true },
            collapsible,
            disabled,
            show_label,
            show_action,
            has_label: true,
            has_action: on_action.get_value().is_some(),
            is_controlled,
            has_custom_class_name: class_name.get_value().is_some(),
        })
    });

    let class =
        Signal::derive(move || logic::compose_class_name(class_name.get_value(), state.get()));

    let toggle_open = Callback::new(move |_| {
        if disabled || !collapsible {
            return;
        }
        request_open_change.run(!open.get_untracked());
    });

    let run_action = Callback::new(move |_| {
        if disabled {
            return;
        }
        if let Some(on_action) = on_action.get_value() {
            on_action.run(());
        }
    });

    let on_label_key_down = move |event: ev::KeyboardEvent| {
        if disabled || !collapsible {
            return;
        }

        if event.key() == "Enter" || event.key() == " " {
            request_open_change.run(!open.get_untracked());
            event.prevent_default();
        }
    };

    view! {
        <section
            class=move || class.get()
            data-slot="sidebar-group"
            data-state=move || state.get().state_attr
            data-open=move || state.get().open.then_some("true")
            data-closed=move || state.get().closed.then_some("true")
            data-collapsible=move || state.get().collapse_attr
            data-disabled=move || state.get().disabled.then_some("true")
            data-enabled=move || state.get().enabled.then_some("true")
            data-show-label=move || state.get().show_label.then_some("true")
            data-show-action=move || state.get().show_action.then_some("true")
            data-controlled=move || state.get().is_controlled.then_some("true")
            data-uncontrolled=move || state.get().is_uncontrolled.then_some("true")
            data-control-mode=move || state.get().control_attr
            data-class-source=move || state.get().class_source_attr
            data-custom-class=move || state.get().has_custom_class_name.then_some("true")
            role="group"
            aria-label=aria_label.get_value()
        >
            <div class="ui-sidebar-group__header" data-slot="sidebar-group-header">
                <Show when=move || state.get().show_label>
                    <button
                        class="ui-sidebar-group__label"
                        data-slot="sidebar-group-label"
                        data-collapsible=move || state.get().collapsible.then_some("true")
                        type="button"
                        disabled=disabled
                        aria-disabled=disabled.then_some("true")
                        aria-expanded=move || if state.get().open { "true" } else { "false" }
                        on:click=move |_| toggle_open.run(())
                        on:keydown=on_label_key_down
                    >
                        <span>{label.get_value()}</span>
                    </button>
                </Show>

                <Show when=move || state.get().show_action>
                    <button
                        class="ui-sidebar-group__action"
                        data-slot="sidebar-group-action"
                        type="button"
                        disabled=disabled
                        aria-disabled=disabled.then_some("true")
                        aria-label=action_label.get_value()
                        on:click=move |_| run_action.run(())
                    >
                        {action_label.get_value()}
                    </button>
                </Show>

                <Show when=move || state.get().collapsible>
                    <button
                        class="ui-sidebar-group__toggle"
                        data-slot="sidebar-group-toggle"
                        data-open=move || state.get().open.then_some("true")
                        type="button"
                        disabled=disabled
                        aria-disabled=disabled.then_some("true")
                        aria-expanded=move || if state.get().open { "true" } else { "false" }
                        aria-label="Toggle group"
                        on:click=move |_| toggle_open.run(())
                    >
                        "▾"
                    </button>
                </Show>
            </div>

            <div
                class="ui-sidebar-group__content"
                data-slot="sidebar-group-content"
                hidden=move || !state.get().open
                data-open=move || state.get().open.then_some("true")
                data-closed=move || state.get().closed.then_some("true")
            >
                {children()}
            </div>
        </section>
    }
}
