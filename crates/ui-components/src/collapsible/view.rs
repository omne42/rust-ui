use crate::collapsible::{
    CollapsibleMotion, CollapsibleStateInput,
    logic::{self},
};
use crate::disclosure::{DisclosureIds, motion};
use crate::overlay_open;
use leptos::{html, prelude::*};
use ui_headless::{
    ButtonOptions, FocusRingOptions, HoverOptions, use_button, use_focus_ring, use_hover,
};

#[component]
pub fn Collapsible(
    id_base: String,
    title: String,
    #[prop(optional)] open: Option<Signal<bool>>,
    #[prop(optional)] default_open: Option<bool>,
    #[prop(optional)] on_open_change: Option<Callback<bool>>,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] motion: CollapsibleMotion,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
    children: Children,
) -> impl IntoView {
    let id_base = logic::normalize_id_base(id_base);
    let title = logic::resolve_title(title);
    let (aria_label, has_custom_aria_label) = logic::resolve_aria_label(&title, aria_label);
    let normalized_class_name = logic::normalize_optional_text(class_name);
    let has_custom_class_name = normalized_class_name.is_some();
    let normalized_class_name = StoredValue::new(normalized_class_name);

    let is_controlled = open.is_some();
    let has_custom_motion = motion != CollapsibleMotion::default();

    let ids = DisclosureIds::new(&id_base);
    let trigger_id = ids.trigger_id.clone();
    let panel_id = ids.panel_id.clone();

    let open_state = overlay_open::use_controllable_open_state(open, default_open, on_open_change);
    let open = open_state.open;
    let request_open_change = open_state.request_open_change;

    let on_press = Callback::new(move |_| {
        let next = !open.get_untracked();
        request_open_change.run(next);
    });

    let aria = use_button(ButtonOptions {
        is_disabled: disabled,
        on_press: Some(on_press),
        ..Default::default()
    });

    let focus_ring = use_focus_ring(FocusRingOptions {
        is_disabled: disabled,
    });
    let hover = use_hover(HoverOptions {
        is_disabled: disabled,
    });

    let indicator_ref: NodeRef<html::Span> = NodeRef::new();
    motion::attach_indicator_motion(indicator_ref, open, motion);

    let panel_ref: NodeRef<html::Div> = NodeRef::new();
    let panel_surface_ref: NodeRef<html::Div> = NodeRef::new();
    let panel_hidden = RwSignal::new(!open.get_untracked());
    motion::attach_panel_motion(panel_ref, panel_surface_ref, open, panel_hidden, motion);

    let title = StoredValue::new(title);

    let state = Memo::new(move |_| {
        logic::resolve_state(CollapsibleStateInput {
            is_open: open.get(),
            is_disabled: disabled,
            is_controlled,
            has_custom_aria_label,
            has_custom_class_name,
            has_custom_motion,
        })
    });

    let class = Memo::new(move |_| {
        logic::compose_class_name(normalized_class_name.get_value(), state.get())
    });

    view! {
        <div
            class=move || class.get()
            data-slot="collapsible"
            data-state=move || state.get().state_attr
            data-open=move || state.get().is_open.then_some("true")
            data-closed=move || state.get().is_closed.then_some("true")
            data-disabled=move || state.get().is_disabled.then_some("true")
            data-open-mode=move || state.get().open_mode_attr
            data-label-source=move || state.get().label_source_attr
            data-class-source=move || state.get().class_source_attr
            data-motion-source=move || state.get().motion_source_attr
            data-custom-motion=move || state.get().has_custom_motion.then_some("true")
            data-custom-class=move || state.get().has_custom_class_name.then_some("true")
        >
            <button
                type="button"
                class="ui-disclosure__trigger ui-collapsible__trigger"
                class:ui-disclosure__trigger--focus-visible=move || focus_ring.is_focus_visible.get()
                id=trigger_id.clone()
                data-slot="collapsible-trigger"
                data-state=move || state.get().state_attr
                data-open-mode=move || state.get().open_mode_attr
                data-label-source=move || state.get().label_source_attr
                data-motion-source=move || state.get().motion_source_attr
                data-custom-motion=move || state.get().has_custom_motion.then_some("true")
                aria-label=aria_label
                aria-expanded=move || if open.get() { "true" } else { "false" }
                aria-controls=panel_id.clone()
                disabled=disabled
                data-open=move || if open.get() { Some("true") } else { None }
                data-closed=move || (!open.get()).then_some("true")
                data-hovered=move || if hover.is_hovered.get() { Some("true") } else { None }
                data-pressed=move || if aria.is_pressed.get() { Some("true") } else { None }
                data-disabled=disabled.then_some("true")
                role=aria.attrs.role
                tabindex=aria.attrs.tabindex
                aria-disabled=aria.attrs.aria_disabled
                on:pointerdown=move |_| aria.handlers.press.on_pointer_down.run(())
                on:pointerup=move |_| aria.handlers.press.on_pointer_up.run(())
                on:pointercancel=move |_| aria.handlers.press.on_pointer_cancel.run(())
                on:pointerenter=move |_| hover.handlers.on_pointer_enter.run(())
                on:pointerleave=move |_| hover.handlers.on_pointer_leave.run(())
                on:click=move |_| aria.handlers.press.on_click.run(())
                on:keydown=move |ev| {
                    let key = ev.key();
                    if aria.handlers.press.on_key_down.run(key) {
                        ev.prevent_default();
                    }
                }
                on:keyup=move |ev| {
                    let key = ev.key();
                    if aria.handlers.press.on_key_up.run(key) {
                        ev.prevent_default();
                    }
                }
                on:focus=move |_| focus_ring.handlers.on_focus.run(())
                on:blur=move |_| {
                    aria.handlers.press.on_blur.run(());
                    focus_ring.handlers.on_blur.run(());
                }
            >
                <span class="ui-disclosure__label ui-collapsible__label" data-slot="collapsible-label">
                    {title.get_value()}
                </span>
                <span
                    node_ref=indicator_ref
                    class="ui-disclosure__indicator ui-collapsible__indicator"
                    aria-hidden="true"
                    data-slot="collapsible-indicator"
                >
                    "›"
                </span>
            </button>

            <div
                id=panel_id
                class="ui-disclosure__panel ui-collapsible__panel"
                node_ref=panel_ref
                role="region"
                aria-labelledby=trigger_id
                hidden=move || panel_hidden.get()
                data-open=move || if open.get() { Some("true") } else { None }
                data-closed=move || (!open.get()).then_some("true")
                data-state=move || state.get().state_attr
                data-open-mode=move || state.get().open_mode_attr
                data-motion-source=move || state.get().motion_source_attr
                data-custom-motion=move || state.get().has_custom_motion.then_some("true")
                data-slot="collapsible-panel"
            >
                <div
                    class="ui-disclosure__panel-surface ui-collapsible__panel-surface"
                    node_ref=panel_surface_ref
                    data-slot="collapsible-panel-surface"
                >
                    {children()}
                </div>
            </div>
        </div>
    }
}
