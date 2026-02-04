use crate::hover_card::{HoverCardMotion, logic, motion};
use leptos::{children::ViewFn, ev, html, portal::Portal, prelude::*};
use ui_headless::{
    FocusWithinOptions, HoverOptions, PopoverPlacement, PopoverPositionOptions, use_focus_within,
    use_hover, use_popover_position,
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

#[cfg(target_arch = "wasm32")]
fn attach_position_vars(
    panel_ref: NodeRef<html::Div>,
    position: ui_headless::PopoverPositionState,
) {
    use leptos::prelude::*;
    use leptos::wasm_bindgen::JsCast;

    Effect::new(move |_| {
        let _ = position.top_px.get();
        let _ = position.left_px.get();
        let _ = position.anchor_width_px.get();

        let Some(panel) = panel_ref.get_untracked() else {
            return;
        };
        let element: leptos::web_sys::HtmlElement = panel.unchecked_into();
        let style = element.style();

        let _ = style.set_property(
            "--ui-hover-card-top",
            &format!("{}px", position.top_px.get_untracked()),
        );
        let _ = style.set_property(
            "--ui-hover-card-left",
            &format!("{}px", position.left_px.get_untracked()),
        );
        let _ = style.set_property(
            "--ui-hover-card-anchor-width",
            &format!("{}px", position.anchor_width_px.get_untracked()),
        );
    });
}

#[cfg(not(target_arch = "wasm32"))]
fn attach_position_vars(
    _panel_ref: NodeRef<html::Div>,
    _position: ui_headless::PopoverPositionState,
) {
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
    let trigger_hover = use_hover(HoverOptions {
        is_disabled: disabled,
    });
    let panel_hover = use_hover(HoverOptions {
        is_disabled: disabled,
    });
    let trigger_focus = use_focus_within(FocusWithinOptions {
        is_disabled: disabled,
    });
    let panel_focus = use_focus_within(FocusWithinOptions {
        is_disabled: disabled,
    });

    let id = id.unwrap_or_else(|| format!("ui-hover-card-{}", next_id()));
    let id = StoredValue::new(id);

    let (open, set_open) = signal(false);
    let open_signal: Signal<bool> = open.into();
    let presence = crate::presence::use_presence(open_signal);

    let timers = logic::HoverCardTimers::new();
    on_cleanup({
        let timers = timers.clone();
        move || {
            #[cfg(target_arch = "wasm32")]
            timers.clear();
            #[cfg(not(target_arch = "wasm32"))]
            let _ = &timers;
        }
    });

    let wants_open = Signal::derive(move || {
        trigger_hover.is_hovered.get()
            || panel_hover.is_hovered.get()
            || trigger_focus.is_focus_within.get()
            || panel_focus.is_focus_within.get()
    });

    Effect::new(move |_| {
        if disabled {
            set_open.set(false);
            return;
        }
        let intent = if wants_open.get() {
            logic::HoverCardIntent::Open
        } else {
            logic::HoverCardIntent::Close
        };
        logic::drive_open_state(intent, open_delay_ms, close_delay_ms, set_open, &timers);
    });

    let anchor_ref: NodeRef<html::Button> = NodeRef::new();
    let panel_ref: NodeRef<html::Div> = NodeRef::new();

    let position = use_popover_position(PopoverPositionOptions {
        anchor_ref,
        panel_ref,
        placement,
        ..Default::default()
    });
    attach_position_vars(panel_ref, position.clone());

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
            set_open.set(false);
        }
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
                on:pointerenter=move |_| trigger_hover.handlers.on_pointer_enter.run(())
                on:pointerleave=move |_| trigger_hover.handlers.on_pointer_leave.run(())
                on:focusin=move |_| trigger_focus.handlers.on_focus_in.run(())
                on:focusout=move |_| trigger_focus.handlers.on_focus_out.run(())
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
                        data-placement=move || position.placement.get().as_str()
                        data-slot="hover-card-panel"
                        on:pointerenter=move |_| panel_hover.handlers.on_pointer_enter.run(())
                        on:pointerleave=move |_| panel_hover.handlers.on_pointer_leave.run(())
                        on:focusin=move |_| panel_focus.handlers.on_focus_in.run(())
                        on:focusout=move |_| panel_focus.handlers.on_focus_out.run(())
                        on:keydown=on_key_down
                    >
                        {move || content.with_value(|content| content.run())}
                    </div>
                </Portal>
            </Show>
        </span>
    }
}
