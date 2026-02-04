use crate::button_flip::{FlipButtonMotion, FlipDirection, motion};
use leptos::children::ViewFn;
use leptos::{html, prelude::*};
use ui_headless::{FocusWithinOptions, HoverOptions, use_focus_within, use_hover};

#[component]
pub fn FlipButton(
    #[prop(optional)] from: FlipDirection,
    #[prop(optional)] motion: FlipButtonMotion,
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional)] node_ref: NodeRef<html::Div>,
    #[prop(into)] front: ViewFn,
    #[prop(into)] back: ViewFn,
) -> impl IntoView {
    let hover = use_hover(HoverOptions { is_disabled: false });
    let focus_within = use_focus_within(FocusWithinOptions { is_disabled: false });

    let (is_active, set_active) = signal(false);
    Effect::new(move |_| {
        set_active.set(hover.is_hovered.get() || focus_within.is_focus_within.get());
    });

    motion::attach_motion(node_ref, is_active, from, motion);

    let base_class = "ui-flip-button".to_string();
    let class = class_name
        .filter(|value| !value.trim().is_empty())
        .map(|value| format!("{base_class} {value}"))
        .unwrap_or(base_class);

    let front = StoredValue::new(front);
    let back = StoredValue::new(back);

    view! {
        <div
            node_ref=node_ref
            class=class
            data-slot="flip-button"
            data-from=from.as_attr()
            data-active=move || if is_active.get() { Some("true") } else { None }
            on:pointerenter=move |_| hover.handlers.on_pointer_enter.run(())
            on:pointerleave=move |_| hover.handlers.on_pointer_leave.run(())
            on:focusin=move |_| focus_within.handlers.on_focus_in.run(())
            on:focusout=move |_| focus_within.handlers.on_focus_out.run(())
        >
            <div class="ui-flip-button__face ui-flip-button__front" data-slot="flip-button-front">
                {front.get_value().run()}
            </div>
            <div class="ui-flip-button__face ui-flip-button__back" data-slot="flip-button-back">
                {back.get_value().run()}
            </div>
        </div>
    }
}
