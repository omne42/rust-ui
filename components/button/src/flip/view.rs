use super::{
    FlipButtonMotion, FlipDirection,
    logic::{self, FlipButtonInputNormalizationInput, FlipButtonStateInput},
    motion,
};
use leptos::children::ViewFn;
use leptos::{html, prelude::*};
use ui_headless::{
    A11yDirection, FocusWithinOptions, HoverOptions, locale_attrs, use_focus_within, use_hover,
};

#[component]
pub fn FlipButton(
    #[prop(optional, into)] from: Option<FlipDirection>,
    #[prop(optional, into)] motion: Option<FlipButtonMotion>,
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional, into)] lang: Option<String>,
    #[prop(optional)] dir: Option<A11yDirection>,
    #[prop(optional)] node_ref: NodeRef<html::Div>,
    #[prop(into)] front: ViewFn,
    #[prop(into)] back: ViewFn,
) -> impl IntoView {
    let normalized = logic::normalize_input(FlipButtonInputNormalizationInput {
        from,
        motion,
        class_name,
    });
    let direction = normalized.direction;
    let motion = normalized.motion;
    let has_custom_motion = normalized.has_custom_motion;
    let has_custom_class_name = normalized.has_custom_class_name;
    let class_name = normalized.class_name;

    let hover = use_hover(HoverOptions { is_disabled: false });
    let focus_within = use_focus_within(FocusWithinOptions { is_disabled: false });

    let state = Signal::derive(move || {
        logic::resolve_state(FlipButtonStateInput {
            direction,
            is_hovered: hover.is_hovered.get(),
            is_focus_within: focus_within.is_focus_within.get(),
            has_custom_class_name,
            has_custom_motion,
        })
    });
    let agent_contract = Signal::derive(move || logic::resolve_agent_contract(state.get()));
    let is_active = Signal::derive(move || state.get().is_active);

    motion::attach_motion(node_ref, is_active, direction, motion);

    let class_name_source = class_name.clone();
    let locale = locale_attrs(super::super::logic::normalize_optional_text(lang), dir);

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
            data-class-source=move || state.get().class_source_attr
            data-motion-source=move || state.get().motion_source_attr
            data-ui-agent-schema=move || agent_contract.get().schema_name
            data-ui-agent-schema-version=move || agent_contract.get().schema_version.as_str()
            data-ui-intent=move || agent_contract.get().intent.as_str()
            data-ui-state=move || agent_contract.get().state.as_str()
            data-ui-capability-press=move || {
                agent_contract.get().capabilities.can_press.then_some("true")
            }
            data-ui-capability-focus=move || {
                agent_contract.get().capabilities.can_focus.then_some("true")
            }
            data-ui-capability-hover=move || {
                agent_contract.get().capabilities.can_hover.then_some("true")
            }
            data-ui-capability-popup-trigger=move || {
                agent_contract
                    .get()
                    .capabilities
                    .can_popup_trigger
                    .then_some("true")
            }
            data-custom-class=move || state.get().has_custom_class_name.then_some("true")
            data-custom-motion=move || state.get().has_custom_motion.then_some("true")
            data-active=move || state.get().is_active.then_some("true")
            data-inactive=move || state.get().is_inactive.then_some("true")
            data-hovered=move || state.get().is_hovered.then_some("true")
            data-focus-within=move || state.get().is_focus_within.then_some("true")
            lang=locale.lang.clone()
            dir=locale.dir
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
