use super::{
    SidebarMotion,
    logic::{self, SidebarCollapsible, SidebarSide, SidebarStateInput, SidebarVariant},
    motion,
};
use leptos::{ev, prelude::*};
use ui_headless::{
    self as headless, A11yDirection, SidebarKeyDownInput, SidebarRootOptions,
    SidebarToggleButtonA11yOptions,
};

#[component]
pub fn Sidebar(
    children: Children,
    #[prop(optional)] open: Option<Signal<bool>>,
    #[prop(optional)] default_open: Option<bool>,
    #[prop(optional)] on_open_change: Option<Callback<bool>>,
    #[prop(optional)] side: SidebarSide,
    #[prop(optional)] variant: SidebarVariant,
    #[prop(optional)] collapsible: SidebarCollapsible,
    #[prop(optional)] is_disabled: Option<bool>,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] is_trigger_visible: Option<bool>,
    #[prop(optional, default = true)] show_trigger: bool,
    #[prop(optional)] is_shortcut_enabled: Option<bool>,
    #[prop(optional, default = true)] enable_shortcut: bool,
    #[prop(optional, into)] shortcut_key: Option<String>,
    #[prop(optional, into)] trigger_label: Option<String>,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional)] motion: SidebarMotion,
    #[prop(optional, into)] lang: Option<String>,
    #[prop(optional)] dir: Option<A11yDirection>,
) -> impl IntoView {
    let is_disabled = logic::resolve_disabled(is_disabled, disabled);
    let is_trigger_visible = logic::resolve_trigger_visibility(is_trigger_visible, show_trigger);
    let is_shortcut_enabled = logic::resolve_shortcut_enabled(is_shortcut_enabled, enable_shortcut);
    let class_name = logic::normalize_optional_text(class_name);
    let trigger_label = logic::normalize_trigger_label(trigger_label);
    let rail_label = trigger_label.clone();
    let aria_label = logic::normalize_aria_label(aria_label);
    let default_open = logic::normalize_default_open(default_open);
    let shortcut_key = logic::normalize_shortcut_key(shortcut_key, is_shortcut_enabled);
    let motion = motion::sanitize_motion(motion);
    let motion_source_attr = motion::source_attr(motion);
    let motion_style = StoredValue::new(motion::attach_motion(motion));

    let is_controlled = open.is_some();
    let open_state = headless::use_controllable_open_state_traced(
        "sidebar",
        open,
        Some(default_open),
        on_open_change,
    );
    let open = open_state.open;
    let request_open_change = open_state.request_open_change;

    let on_toggle = Callback::new(move |_| {
        if is_disabled {
            return;
        }

        request_open_change.run(!open.get_untracked());
    });
    let on_shortcut_toggle = on_toggle;

    let root_contract = headless::use_sidebar_root(SidebarRootOptions {
        is_disabled,
        shortcut_key: shortcut_key.clone(),
        aria_label: aria_label.clone(),
        lang: lang.clone(),
        dir,
        on_shortcut_toggle: Some(on_shortcut_toggle),
    });
    let trigger_a11y = headless::sidebar_toggle_button_a11y_attrs(
        open,
        SidebarToggleButtonA11yOptions {
            is_disabled,
            aria_label: trigger_label.clone(),
            lang: lang.clone(),
            dir,
        },
    );
    let rail_a11y = headless::sidebar_toggle_button_a11y_attrs(
        open,
        SidebarToggleButtonA11yOptions {
            is_disabled,
            aria_label: rail_label.clone(),
            lang,
            dir,
        },
    );
    let root_role = root_contract.attrs.role;
    let root_aria_label = root_contract.attrs.aria_label;
    let root_aria_keyshortcuts = root_contract.attrs.aria_keyshortcuts;
    let root_lang = root_contract.attrs.lang;
    let root_dir = root_contract.attrs.dir;
    let root_shortcut_source = root_contract.state.shortcut_source_attr;
    let root_on_key_down = root_contract.handlers.on_key_down;
    let trigger_aria_disabled = trigger_a11y.aria_disabled;
    let trigger_aria_expanded = trigger_a11y.aria_expanded;
    let trigger_aria_label = StoredValue::new(trigger_a11y.aria_label);
    let trigger_lang = StoredValue::new(trigger_a11y.lang);
    let trigger_dir = StoredValue::new(trigger_a11y.dir);
    let rail_aria_disabled = rail_a11y.aria_disabled;
    let rail_aria_expanded = rail_a11y.aria_expanded;
    let rail_aria_label = StoredValue::new(rail_a11y.aria_label);
    let rail_lang = StoredValue::new(rail_a11y.lang);
    let rail_dir = StoredValue::new(rail_a11y.dir);
    let on_toggle_for_trigger = on_toggle;
    let on_toggle_for_rail = on_toggle;

    let class_name = StoredValue::new(class_name);
    let trigger_label = StoredValue::new(trigger_label);
    let rail_label = StoredValue::new(rail_label);
    let shortcut_key = StoredValue::new(shortcut_key);

    let state = Signal::derive(move || {
        logic::resolve_state(SidebarStateInput {
            side,
            variant,
            collapsible,
            open: open.get(),
            disabled: is_disabled,
            is_controlled,
            show_trigger: is_trigger_visible,
            has_shortcut_key: shortcut_key.get_value().is_some(),
            has_custom_class_name: class_name.get_value().is_some(),
        })
    });

    let class =
        Signal::derive(move || logic::compose_class_name(class_name.get_value(), state.get()));

    let on_key_down = move |event: ev::KeyboardEvent| {
        if root_on_key_down.run(SidebarKeyDownInput {
            key: event.key(),
            ctrl_key: event.ctrl_key(),
            meta_key: event.meta_key(),
        }) {
            event.prevent_default();
        }
    };

    view! {
        <aside
            class=move || class.get()
            style=move || motion_style.get_value()
            data-slot="sidebar"
            data-motion-source=motion_source_attr
            data-custom-motion=(motion_source_attr == "custom").then_some("true")
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
            data-has-shortcut=move || shortcut_key.get_value().as_ref().map(|_| "true")
            data-shortcut-source=root_shortcut_source
            role=root_role
            aria-label=root_aria_label
            aria-keyshortcuts=root_aria_keyshortcuts
            lang=root_lang
            dir=root_dir
            on:keydown=on_key_down
        >
            <Show when=move || state.get().show_trigger>
                <button
                    class="ui-sidebar__trigger"
                    data-slot="sidebar-trigger"
                    type="button"
                    disabled=is_disabled
                    aria-disabled=trigger_aria_disabled
                    aria-expanded=move || trigger_aria_expanded.get()
                    aria-label=move || trigger_aria_label.get_value()
                    lang=move || trigger_lang.get_value()
                    dir=move || trigger_dir.get_value()
                    on:click=move |_| on_toggle_for_trigger.run(())
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
                    tabindex=if is_disabled { -1 } else { 0 }
                    disabled=is_disabled
                    aria-disabled=rail_aria_disabled
                    aria-expanded=move || rail_aria_expanded.get()
                    aria-label=move || rail_aria_label.get_value()
                    lang=move || rail_lang.get_value()
                    dir=move || rail_dir.get_value()
                    on:click=move |_| on_toggle_for_rail.run(())
                >
                    <span class="ui-sr-only">{move || rail_label.get_value()}</span>
                </button>
            </Show>
        </aside>
    }
}
