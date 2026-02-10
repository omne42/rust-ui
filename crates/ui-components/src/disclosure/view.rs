use crate::disclosure::{DisclosureIds, DisclosureMotion, logic, motion};
use crate::overlay_open;
use leptos::{html, prelude::*};
use ui_headless::{
    ButtonOptions, FocusRingOptions, HoverOptions, use_button, use_focus_ring, use_hover,
};

#[component]
pub fn Disclosure(
    id_base: String,
    label: String,
    #[prop(optional)] open: Option<Signal<bool>>,
    #[prop(optional)] default_open: Option<bool>,
    #[prop(optional)] on_open_change: Option<Callback<bool>>,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] motion: DisclosureMotion,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
    children: Children,
) -> impl IntoView {
    let ids = DisclosureIds::new(&id_base);
    let trigger_id = ids.trigger_id.clone();
    let panel_id = ids.panel_id.clone();

    let aria_label = aria_label
        .filter(|value| !value.trim().is_empty())
        .unwrap_or_else(|| label.clone());

    let open_state = overlay_open::use_controllable_open_state(open, default_open, on_open_change);
    let open = open_state.open;
    let request_open_change = open_state.request_open_change;

    let state = Signal::derive(move || logic::resolve_state(open.get(), disabled));

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

    let base_class = "ui-disclosure".to_string();
    let class = class_name
        .filter(|value| !value.trim().is_empty())
        .map(|value| format!("{base_class} {value}"))
        .unwrap_or(base_class);

    let motion_source = if motion == DisclosureMotion::default() {
        "default"
    } else {
        "custom"
    };
    let custom_motion = (motion != DisclosureMotion::default()).then_some("true");

    view! {
        <div
            class=class
            data-slot="disclosure"
            data-open=move || state.get().is_open.then_some("true")
            data-closed=move || state.get().is_closed.then_some("true")
            data-disabled=move || state.get().is_disabled.then_some("true")
            data-motion-source=motion_source
            data-custom-motion=custom_motion
        >
            <button
                type="button"
                class="ui-disclosure__trigger"
                class:ui-disclosure__trigger--focus-visible=move || focus_ring.is_focus_visible.get()
                id=trigger_id.clone()
                data-slot="disclosure-trigger"
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
                <span class="ui-disclosure__label" data-slot="disclosure-label">
                    {label.clone()}
                </span>
                <span
                    node_ref=indicator_ref
                    class="ui-disclosure__indicator"
                    aria-hidden="true"
                    data-slot="disclosure-indicator"
                >
                    "›"
                </span>
            </button>

            <div
                id=panel_id
                class="ui-disclosure__panel"
                node_ref=panel_ref
                role="region"
                aria-labelledby=trigger_id
                hidden=move || panel_hidden.get()
                data-open=move || if open.get() { Some("true") } else { None }
                data-closed=move || (!open.get()).then_some("true")
                data-slot="disclosure-panel"
            >
                <div
                    class="ui-disclosure__panel-surface"
                    node_ref=panel_surface_ref
                    data-slot="disclosure-panel-surface"
                >
                    {children()}
                </div>
            </div>
        </div>
    }
}
