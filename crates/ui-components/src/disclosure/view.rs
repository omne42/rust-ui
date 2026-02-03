use crate::disclosure::{DisclosureIds, DisclosureMotion, motion};
use leptos::{html, prelude::*};
use ui_headless::{
    ButtonOptions, FocusRingOptions, HoverOptions, use_button, use_focus_ring, use_hover,
};

#[component]
pub fn Disclosure(
    id_base: String,
    label: String,
    open: ReadSignal<bool>,
    set_open: WriteSignal<bool>,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] motion: DisclosureMotion,
    #[prop(optional)] on_change: Option<Callback<bool>>,
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

    let on_press = Callback::new(move |_| {
        let next = !open.get_untracked();
        set_open.set(next);
        if let Some(on_change) = on_change {
            on_change.run(next);
        }
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

    let base_class = "ui-disclosure".to_string();
    let class = class_name
        .filter(|value| !value.trim().is_empty())
        .map(|value| format!("{base_class} {value}"))
        .unwrap_or(base_class);

    view! {
        <div class=class data-slot="disclosure">
            <button
                type="button"
                class="ui-disclosure__trigger"
                class:ui-disclosure__trigger--focus-visible=move || focus_ring.is_focus_visible.get()
                id=trigger_id
                aria-label=aria_label
                aria-expanded=move || if open.get() { "true" } else { "false" }
                aria-controls=panel_id.clone()
                disabled=disabled
                data-open=move || if open.get() { Some("true") } else { None }
                data-hovered=move || if hover.is_hovered.get() { Some("true") } else { None }
                data-pressed=move || if aria.is_pressed.get() { Some("true") } else { None }
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
                role="region"
                aria-labelledby=id_base.clone() + "-trigger"
                hidden=move || !open.get()
                data-slot="disclosure-panel"
            >
                {children()}
            </div>
        </div>
    }
}
