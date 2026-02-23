use super::logic::{self, SidebarGroupStateInput};
use leptos::{ev, prelude::*};
use ui_headless::{self as overlay_open, A11yDirection};

#[component]
pub fn SidebarGroup(
    children: Children,
    #[prop(optional)] open: Option<Signal<bool>>,
    #[prop(optional)] default_open: Option<bool>,
    #[prop(optional)] on_open_change: Option<Callback<bool>>,
    #[prop(optional)] on_action: Option<Callback<()>>,
    #[prop(optional)] is_collapsible: Option<bool>,
    #[prop(optional)] collapsible: bool,
    #[prop(optional)] is_disabled: Option<bool>,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] is_label_visible: Option<bool>,
    #[prop(optional, default = true)] show_label: bool,
    #[prop(optional)] is_action_visible: Option<bool>,
    #[prop(optional, default = true)] show_action: bool,
    #[prop(optional, into)] label: Option<String>,
    #[prop(optional, into)] action_label: Option<String>,
    #[prop(optional, into)] toggle_label: Option<String>,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional, into)] lang: Option<String>,
    #[prop(optional)] dir: Option<A11yDirection>,
) -> impl IntoView {
    let is_collapsible = logic::resolve_collapsible(is_collapsible, collapsible);
    let is_disabled = logic::resolve_disabled(is_disabled, disabled);
    let is_label_visible = logic::resolve_label_visibility(is_label_visible, show_label);
    let is_action_visible = logic::resolve_action_visibility(is_action_visible, show_action);
    let class_name = logic::normalize_optional_text(class_name);
    let label = logic::normalize_label(label);
    let action_label = logic::normalize_action_label(action_label);
    let toggle_label = logic::normalize_toggle_label(toggle_label);
    let aria_label = logic::normalize_aria_label(aria_label);
    let group_a11y = overlay_open::labeled_group_attrs(aria_label.clone(), lang, dir);
    let group_role = group_a11y.role;
    let group_aria_label = group_a11y.aria_label;
    let group_lang = group_a11y.lang;
    let group_dir = group_a11y.dir;

    let default_open = logic::normalize_default_open(default_open);
    let is_controlled = open.is_some();
    let open_state = overlay_open::use_controllable_open_state_traced(
        "sidebar-group",
        open,
        Some(default_open),
        on_open_change,
    );
    let open = open_state.open;
    let request_open_change = open_state.request_open_change;

    let class_name = StoredValue::new(class_name);
    let label = StoredValue::new(label);
    let action_label = StoredValue::new(action_label);
    let toggle_label = StoredValue::new(toggle_label);
    let aria_label = StoredValue::new(group_aria_label);
    let group_lang = StoredValue::new(group_lang);
    let group_dir = StoredValue::new(group_dir);
    let on_action = StoredValue::new(on_action);

    let state = Signal::derive(move || {
        logic::resolve_state(SidebarGroupStateInput {
            open: logic::resolve_effective_open(open.get(), is_collapsible),
            collapsible: is_collapsible,
            disabled: is_disabled,
            show_label: is_label_visible,
            show_action: is_action_visible,
            has_label: true,
            has_action: on_action.get_value().is_some(),
            is_controlled,
            has_custom_class_name: class_name.get_value().is_some(),
        })
    });

    let class =
        Signal::derive(move || logic::compose_class_name(class_name.get_value(), state.get()));

    let toggle_open = Callback::new(move |_| {
        if !logic::can_toggle_open(is_disabled, is_collapsible) {
            return;
        }
        request_open_change.run(logic::next_toggled_open(open.get_untracked()));
    });

    let run_action = Callback::new(move |_| {
        if is_disabled {
            return;
        }
        if let Some(on_action) = on_action.get_value() {
            on_action.run(());
        }
    });

    let on_label_key_down = move |event: ev::KeyboardEvent| {
        if !logic::can_toggle_open(is_disabled, is_collapsible) {
            return;
        }

        if event.key() == "Enter" || event.key() == " " {
            request_open_change.run(logic::next_toggled_open(open.get_untracked()));
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
            role=group_role
            aria-label=aria_label.get_value()
            lang=move || group_lang.get_value()
            dir=move || group_dir.get_value()
        >
            <div class="ui-sidebar-group__header" data-slot="sidebar-group-header">
                <Show when=move || state.get().show_label>
                    <button
                        class="ui-sidebar-group__label"
                        data-slot="sidebar-group-label"
                        data-collapsible=move || state.get().collapsible.then_some("true")
                        type="button"
                        disabled=is_disabled
                        aria-disabled=is_disabled.then_some("true")
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
                        disabled=is_disabled
                        aria-disabled=is_disabled.then_some("true")
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
                        disabled=is_disabled
                        aria-disabled=is_disabled.then_some("true")
                        aria-expanded=move || if state.get().open { "true" } else { "false" }
                        aria-label=toggle_label.get_value()
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
