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

    let anchor_ref: NodeRef<html::Button> = NodeRef::new();
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
        if ev.key() == "Escape" {
            trigger.state.dismiss.run(());
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
        <span class=class data-slot="hover-card">
            <button
                type="button"
                class="ui-hover-card__trigger"
                data-slot="hover-card-trigger"
                node_ref=anchor_ref
                disabled=disabled
                aria-describedby=move || presence.is_present.get().then(|| id.with_value(|id| id.clone()))
                on:pointerenter=move |_| trigger.handlers.on_trigger_pointer_enter.run(())
                on:pointerleave=move |_| trigger.handlers.on_trigger_pointer_leave.run(())
                on:focusin=move |_| trigger.handlers.on_trigger_focus_in.run(())
                on:focusout=move |_| trigger.handlers.on_trigger_focus_out.run(())
                on:keydown=on_key_down
            >
                {children()}
            </button>

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
