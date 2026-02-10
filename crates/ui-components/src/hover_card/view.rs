use crate::hover_card::{HoverCardMotion, motion};
use leptos::{children::ViewFn, ev, html, portal::Portal, prelude::*};
use ui_headless::{
    HoverCardTriggerOptions, PopoverPlacement, PopoverPositionOptions, use_hover_card_trigger,
    use_popover_position,
};

fn next_id() -> u64 {
    use std::cell::Cell;
    thread_local! {
        static NEXT: Cell<u64> = const { Cell::new(1) };
    }
    NEXT.with(|cell| {
        let id = cell.get();
        cell.set(id + 1);
        id
    })
}

#[component]
pub fn HoverCard(
    #[prop(into)] content: ViewFn,
    children: Children,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] placement: PopoverPlacement,
    #[prop(optional, default = 140)] open_delay_ms: u64,
    #[prop(optional, default = 180)] close_delay_ms: u64,
    #[prop(optional)] motion: HoverCardMotion,
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional, into)] id: Option<String>,
) -> impl IntoView {
    let id = id.unwrap_or_else(|| format!("ui-hover-card-{}", next_id()));
    let id = StoredValue::new(id);

    let trigger = use_hover_card_trigger(HoverCardTriggerOptions {
        is_disabled: disabled,
        open_delay_ms,
        close_delay_ms,
    });
    let open_signal: Signal<bool> = trigger.state.is_open.into();
    let presence = crate::presence::use_presence(open_signal);

    let anchor_ref: NodeRef<html::Span> = NodeRef::new();
    let panel_ref: NodeRef<html::Div> = NodeRef::new();

    let position = use_popover_position(PopoverPositionOptions {
        anchor_ref,
        panel_ref,
        placement,
        ..Default::default()
    });

    motion::attach_motion(
        panel_ref,
        open_signal,
        position.placement.into(),
        presence.finish_exit,
        motion,
    );

    let base_class = "ui-hover-card".to_string();
    let class = class_name
        .filter(|value| !value.trim().is_empty())
        .map(|value| format!("{base_class} {value}"))
        .unwrap_or(base_class);

    let content = StoredValue::new(content);

    let on_key_down = move |ev: ev::KeyboardEvent| {
        if ev.key() != "Escape" {
            return;
        }

        if !open_signal.get_untracked() {
            return;
        }

        #[cfg(target_arch = "wasm32")]
        let is_composing = ev.is_composing();
        #[cfg(not(target_arch = "wasm32"))]
        let is_composing = false;

        if is_composing {
            return;
        }

        ev.stop_propagation();
        ev.prevent_default();
        trigger.state.dismiss.run(());
    };

    #[cfg(target_arch = "wasm32")]
    let focus_target = StoredValue::new_local(None::<leptos::web_sys::Element>);

    #[cfg(target_arch = "wasm32")]
    on_cleanup(move || {
        if let Some(target) = focus_target.get_value() {
            let _ = target.remove_attribute("aria-describedby");
        }
    });

    #[cfg(target_arch = "wasm32")]
    Effect::new(move |_| {
        let is_open = open_signal.get();
        let Some(target) = focus_target.get_value() else {
            return;
        };

        let id = id.with_value(|id| id.clone());
        if is_open {
            let _ = target.set_attribute("aria-describedby", &id);
        } else {
            let _ = target.remove_attribute("aria-describedby");
        }
    });

    let on_focus_in = move |_ev: ev::FocusEvent| {
        trigger.handlers.on_trigger_focus_in.run(());

        #[cfg(target_arch = "wasm32")]
        {
            use leptos::wasm_bindgen::JsCast;

            if let Some(target) = focus_target.get_value() {
                let _ = target.remove_attribute("aria-describedby");
            }

            let Some(target) = _ev.target() else {
                focus_target.set_value(None);
                return;
            };

            let Ok(target) = target.dyn_into::<leptos::web_sys::Element>() else {
                focus_target.set_value(None);
                return;
            };

            if open_signal.get_untracked() {
                let id = id.with_value(|id| id.clone());
                let _ = target.set_attribute("aria-describedby", &id);
            }

            focus_target.set_value(Some(target));
        }
    };

    let on_focus_out = move |_ev: ev::FocusEvent| {
        trigger.handlers.on_trigger_focus_out.run(());

        #[cfg(target_arch = "wasm32")]
        {
            if let Some(target) = focus_target.get_value() {
                let _ = target.remove_attribute("aria-describedby");
            }
            focus_target.set_value(None);
        }
    };

    let panel_vars = move || {
        format!(
            "--ui-hover-card-top: {}px; --ui-hover-card-left: {}px; --ui-hover-card-anchor-width: {}px;",
            position.top_px.get(),
            position.left_px.get(),
            position.anchor_width_px.get()
        )
    };

    view! {
        <span
                class=class
                data-slot="hover-card"
                data-state=move || if open_signal.get() { "open" } else { "closed" }
                data-open=move || open_signal.get().then_some("true")
                data-closed=move || (!open_signal.get()).then_some("true")
                data-disabled=disabled.then_some("true")
                data-enabled=(!disabled).then_some("true")
                data-motion-source=if motion == HoverCardMotion::default() {
                    "default"
                } else {
                    "custom"
                }
                data-custom-motion=(motion != HoverCardMotion::default()).then_some("true")
            >
            <span
                class="ui-hover-card__trigger"
                data-slot="hover-card-trigger"
                node_ref=anchor_ref
                on:pointerenter=move |_| trigger.handlers.on_trigger_pointer_enter.run(())
                on:pointerleave=move |_| trigger.handlers.on_trigger_pointer_leave.run(())
                on:focusin=on_focus_in
                on:focusout=on_focus_out
                on:keydown=on_key_down
            >
                {children()}
            </span>

            <Show when=move || presence.is_present.get()>
                <Portal>
                    <div
                        class="ui-hover-card__panel"
                        node_ref=panel_ref
                        id=move || id.with_value(|id| id.clone())
                        role="tooltip"
                        data-ui-overlay-portal=""
                        data-placement=move || position.placement.get().as_str()
                        data-slot="hover-card-panel"
                        style=panel_vars
                        on:pointerenter=move |_| trigger.handlers.on_panel_pointer_enter.run(())
                        on:pointerleave=move |_| trigger.handlers.on_panel_pointer_leave.run(())
                        on:focusin=move |_| trigger.handlers.on_panel_focus_in.run(())
                        on:focusout=move |_| trigger.handlers.on_panel_focus_out.run(())
                        on:keydown=on_key_down
                    >
                        {move || content.with_value(|content| content.run())}
                    </div>
                </Portal>
            </Show>
        </span>
    }
}
