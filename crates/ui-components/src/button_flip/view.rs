use crate::button_flip::{
    FlipButtonMotion, FlipDirection,
    logic::{self, FlipButtonStateInput},
    motion,
};
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
    let class_name = logic::normalize_optional_text(class_name);
    let has_custom_class_name = class_name.is_some();

    let hover = use_hover(HoverOptions { is_disabled: false });
    let focus_within = use_focus_within(FocusWithinOptions { is_disabled: false });

    let (is_active, set_active) = signal(false);
    Effect::new(move |_| {
        set_active.set(hover.is_hovered.get() || focus_within.is_focus_within.get());
    });

    motion::attach_motion(node_ref, is_active, from, motion);

    let state = Memo::new(move |_| {
        logic::resolve_state(FlipButtonStateInput {
            direction: from,
            is_hovered: hover.is_hovered.get(),
            is_focus_within: focus_within.is_focus_within.get(),
            is_active: is_active.get(),
            has_custom_class_name,
        })
    });

    let class_name_source = class_name.clone();

    let front = StoredValue::new(front);
    let back = StoredValue::new(back);

    view! {
        <div
            node_ref=node_ref
            class=move || logic::compose_class_name(class_name_source.clone(), state.get())
            data-slot="flip-button"
            data-from=move || state.get().direction_attr
            data-state=move || state.get().state_attr
            data-hover=move || state.get().hover_attr
            data-focus-within-state=move || state.get().focus_within_attr
            data-custom-class=move || state.get().has_custom_class_name.then_some("true")
            data-active=move || state.get().is_active.then_some("true")
            data-inactive=move || state.get().is_inactive.then_some("true")
            data-hovered=move || state.get().is_hovered.then_some("true")
            data-focus-within=move || state.get().is_focus_within.then_some("true")
            data-motion-source=if motion == FlipButtonMotion::default() {
                "default"
            } else {
                "custom"
            }
            data-custom-motion=(motion != FlipButtonMotion::default()).then_some("true")
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
