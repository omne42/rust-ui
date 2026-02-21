use crate::{HoverCardMotion, logic, motion};
use leptos::{children::ViewFn, ev, html, portal::Portal, prelude::*};
use std::borrow::Cow;
use ui_headless::{
    HoverCardDismissOptions, HoverCardFocusA11yOptions, HoverCardTriggerOptions, PopoverPlacement,
    PopoverPositionOptions, use_hover_card_dismiss, use_hover_card_focus_a11y,
    use_hover_card_trigger, use_popover_position, use_ui_id_provider,
};

struct TriggerViewInput {
    trigger_class: String,
    trigger_state: crate::HoverCardPartState,
    open_mode_attr: &'static str,
    open_value_source_attr: &'static str,
    open_intent_source_attr: &'static str,
    anchor_ref: NodeRef<html::Span>,
    on_trigger_pointer_enter: Callback<()>,
    on_trigger_pointer_leave: Callback<()>,
    on_trigger_focus_in: Callback<ev::FocusEvent>,
    on_trigger_focus_out: Callback<ev::FocusEvent>,
    on_trigger_key_down: Callback<ev::KeyboardEvent>,
    children: Children,
}

fn render_trigger_view(input: TriggerViewInput) -> impl IntoView {
    let TriggerViewInput {
        trigger_class,
        trigger_state,
        open_mode_attr,
        open_value_source_attr,
        open_intent_source_attr,
        anchor_ref,
        on_trigger_pointer_enter,
        on_trigger_pointer_leave,
        on_trigger_focus_in,
        on_trigger_focus_out,
        on_trigger_key_down,
        children,
    } = input;

    view! {
        <span
            class=trigger_class
            data-slot=trigger_state.slot_attr
            data-state=trigger_state.state_attr
            data-disabled=trigger_state.is_disabled.then_some("true")
            data-enabled=(!trigger_state.is_disabled).then_some("true")
            data-class-source=trigger_state.class_source_attr
            data-motion-source=trigger_state.motion_source_attr
            data-delay-source=trigger_state.delay_source_attr
            data-id-source=trigger_state.id_source_attr
            data-open-mode=open_mode_attr
            data-open-value-source=open_value_source_attr
            data-open-intent-source=open_intent_source_attr
            node_ref=anchor_ref
            on:pointerenter=move |_| on_trigger_pointer_enter.run(())
            on:pointerleave=move |_| on_trigger_pointer_leave.run(())
            on:focusin=move |ev| on_trigger_focus_in.run(ev)
            on:focusout=move |ev| on_trigger_focus_out.run(ev)
            on:keydown=move |ev| on_trigger_key_down.run(ev)
        >
            {children()}
        </span>
    }
}

struct PanelViewInput {
    is_present: ReadSignal<bool>,
    panel_class: StoredValue<String>,
    panel_ref: NodeRef<html::Div>,
    id: StoredValue<String>,
    lang: StoredValue<Option<String>>,
    dir: StoredValue<Option<String>>,
    aria_keyshortcuts: &'static str,
    placement: ReadSignal<PopoverPlacement>,
    panel_state: crate::HoverCardPartState,
    open_signal: Signal<bool>,
    open_mode_attr: &'static str,
    open_value_source_attr: &'static str,
    open_intent_source_attr: &'static str,
    top_px: ReadSignal<f64>,
    left_px: ReadSignal<f64>,
    anchor_width_px: ReadSignal<f64>,
    on_panel_pointer_enter: Callback<()>,
    on_panel_pointer_leave: Callback<()>,
    on_panel_focus_in: Callback<()>,
    on_panel_focus_out: Callback<()>,
    on_panel_key_down: Callback<ev::KeyboardEvent>,
    content: StoredValue<ViewFn>,
}

fn render_panel_view(input: PanelViewInput) -> impl IntoView {
    let PanelViewInput {
        is_present,
        panel_class,
        panel_ref,
        id,
        lang,
        dir,
        aria_keyshortcuts,
        placement,
        panel_state,
        open_signal,
        open_mode_attr,
        open_value_source_attr,
        open_intent_source_attr,
        top_px,
        left_px,
        anchor_width_px,
        on_panel_pointer_enter,
        on_panel_pointer_leave,
        on_panel_focus_in,
        on_panel_focus_out,
        on_panel_key_down,
        content,
    } = input;

    let panel_vars =
        move || logic::compose_panel_vars(top_px.get(), left_px.get(), anchor_width_px.get());

    view! {
        <Show when=move || is_present.get()>
            <Portal>
                <div
                    class=move || panel_class.with_value(|class_name| class_name.clone())
                    node_ref=panel_ref
                    id=move || id.with_value(|id| id.clone())
                    lang=move || lang.with_value(|value| value.clone())
                    dir=move || dir.with_value(|value| value.clone())
                    role="tooltip"
                    aria-keyshortcuts=aria_keyshortcuts
                    data-ui-overlay-portal=""
                    data-placement=move || placement.get().as_str()
                    data-slot=panel_state.slot_attr
                    data-state=panel_state.state_attr
                    data-open=move || open_signal.get().then_some("true")
                    data-closed=move || (!open_signal.get()).then_some("true")
                    data-disabled=panel_state.is_disabled.then_some("true")
                    data-enabled=(!panel_state.is_disabled).then_some("true")
                    data-class-source=panel_state.class_source_attr
                    data-motion-source=panel_state.motion_source_attr
                    data-delay-source=panel_state.delay_source_attr
                    data-id-source=panel_state.id_source_attr
                    data-open-mode=open_mode_attr
                    data-open-value-source=open_value_source_attr
                    data-open-intent-source=open_intent_source_attr
                    style=panel_vars
                    on:pointerenter=move |_| on_panel_pointer_enter.run(())
                    on:pointerleave=move |_| on_panel_pointer_leave.run(())
                    on:focusin=move |_| on_panel_focus_in.run(())
                    on:focusout=move |_| on_panel_focus_out.run(())
                    on:keydown=move |ev| on_panel_key_down.run(ev)
                >
                    {move || content.with_value(|content| content.run())}
                </div>
            </Portal>
        </Show>
    }
}

#[component]
pub fn HoverCard(
    #[prop(into)] content: ViewFn,
    children: Children,
    #[prop(optional, into)] is_disabled: Option<bool>,
    #[prop(optional, into)] disabled: Option<bool>,
    #[prop(optional)] placement: PopoverPlacement,
    #[prop(optional)] is_open: Option<Signal<bool>>,
    #[prop(optional)] open: Option<Signal<bool>>,
    #[prop(optional)] default_open: Option<bool>,
    #[prop(optional)] on_open_change: Option<Callback<bool>>,
    #[prop(optional)] open_delay_ms: Option<u64>,
    #[prop(optional)] close_delay_ms: Option<u64>,
    #[prop(optional)] motion: HoverCardMotion,
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional, into)] id: Option<String>,
    #[prop(optional, into)] lang: Option<String>,
    #[prop(optional, into)] dir: Option<String>,
) -> impl IntoView {
    let class_name = logic::normalize_optional_text(class_name);
    let lang = StoredValue::new(logic::normalize_optional_text(lang));
    let dir = StoredValue::new(logic::normalize_optional_text(dir));
    let delay_state = logic::normalize_delay_state(logic::DelayStateInput {
        open_delay_ms,
        close_delay_ms,
    });
    let open_delay_ms = delay_state.open_delay_ms;
    let close_delay_ms = delay_state.close_delay_ms;
    let has_custom_delays = delay_state.has_custom_delays;
    let is_disabled = logic::resolve_is_disabled(is_disabled, disabled);
    let normalized_open_state = logic::normalize_open_state(logic::OpenStateInput {
        is_open,
        open,
        default_open,
        on_open_change,
    });
    let is_controlled = normalized_open_state.is_controlled;
    let open_mode_attr = logic::open_mode_attr(is_controlled);
    let open_value_source_attr = logic::open_value_source_attr(is_controlled);
    let open_intent_source_attr = logic::open_intent_source_attr();
    let open = normalized_open_state.open;
    let default_open = normalized_open_state.default_open;
    let on_open_change = normalized_open_state.on_open_change;

    let generated_id = use_ui_id_provider()
        .map(|id_provider| Cow::Owned(id_provider.next_prefixed_id("ui-hover-card")))
        .unwrap_or(Cow::Borrowed("ui-hover-card"));
    let (id, has_custom_id) = logic::resolve_id(id, generated_id);
    let id = StoredValue::new(id);

    let trigger = use_hover_card_trigger(HoverCardTriggerOptions {
        is_disabled,
        open_delay_ms,
        close_delay_ms,
        open,
        default_open,
        on_open_change,
    });
    let open_signal = trigger.state.is_open;
    let presence = ui_headless::use_presence(open_signal);

    let part_states = logic::normalize_part_states(logic::PartStatesInput {
        class_name,
        is_open: open_signal.get_untracked(),
        is_disabled,
        motion,
        has_custom_delays,
        has_custom_id,
    });
    let root_state = part_states.root_state;
    let root_class = part_states.root_class;
    let trigger_state = part_states.trigger_state;
    let trigger_class = part_states.trigger_class;
    let panel_state = part_states.panel_state;
    let panel_class = StoredValue::new(part_states.panel_class);

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

    let content = StoredValue::new(content);

    let dismiss_a11y = use_hover_card_dismiss(HoverCardDismissOptions {
        is_open: open_signal,
        dismiss: trigger.state.dismiss,
    });
    let focus_a11y = use_hover_card_focus_a11y(HoverCardFocusA11yOptions {
        hover_card_id: id,
        is_open: open_signal,
        on_focus_in: trigger.handlers.on_trigger_focus_in,
        on_focus_out: trigger.handlers.on_trigger_focus_out,
    });

    let on_trigger_pointer_enter = trigger.handlers.on_trigger_pointer_enter;
    let on_trigger_pointer_leave = trigger.handlers.on_trigger_pointer_leave;
    let on_panel_pointer_enter = trigger.handlers.on_panel_pointer_enter;
    let on_panel_pointer_leave = trigger.handlers.on_panel_pointer_leave;
    let on_panel_focus_in = trigger.handlers.on_panel_focus_in;
    let on_panel_focus_out = trigger.handlers.on_panel_focus_out;

    let on_trigger_focus_in = focus_a11y.handlers.on_focus_in;
    let on_trigger_focus_out = focus_a11y.handlers.on_focus_out;

    let on_trigger_key_down = dismiss_a11y.handlers.on_key_down;
    let on_panel_key_down = dismiss_a11y.handlers.on_key_down;
    let agent_contract = Signal::derive(move || {
        logic::resolve_agent_contract(logic::HoverCardAgentContractInput {
            is_open: open_signal.get(),
            is_controlled,
            is_disabled,
        })
    });

    view! {
        <span
            class=root_class
            lang=move || lang.with_value(|value| value.clone())
            dir=move || dir.with_value(|value| value.clone())
            data-slot=root_state.slot_attr
            data-state=move || logic::state_attr_for_open(open_signal.get())
            data-open=move || open_signal.get().then_some("true")
            data-closed=move || (!open_signal.get()).then_some("true")
            data-disabled=root_state.is_disabled.then_some("true")
            data-enabled=(!root_state.is_disabled).then_some("true")
            data-class-source=root_state.class_source_attr
            data-motion-source=root_state.motion_source_attr
            data-delay-source=root_state.delay_source_attr
            data-id-source=root_state.id_source_attr
            data-custom-class=root_state.has_custom_class_name.then_some("true")
            data-custom-motion=root_state.has_custom_motion.then_some("true")
            data-custom-delay=root_state.has_custom_delays.then_some("true")
            data-custom-id=root_state.has_custom_id.then_some("true")
            data-open-mode=open_mode_attr
            data-open-value-source=open_value_source_attr
            data-open-intent-source=open_intent_source_attr
            data-ui-schema=move || agent_contract.get().schema_name
            data-ui-schema-version=move || agent_contract.get().schema_version.as_str()
            data-ui-intent=move || agent_contract.get().intent.as_str()
            data-ui-action=move || agent_contract.get().action.as_str()
            data-ui-state=move || agent_contract.get().state.as_str()
            data-ui-source=move || agent_contract.get().source.as_str()
            data-ui-config-policy=move || agent_contract.get().config_policy.as_str()
            data-ui-output-status=move || agent_contract.get().output_status.as_str()
            data-ui-capability-open=move || agent_contract.get().capabilities.can_open.then_some("true")
            data-ui-capability-close=move || agent_contract.get().capabilities.can_close.then_some("true")
            data-ui-capability-panel=move || agent_contract.get().capabilities.has_panel.then_some("true")
        >
            {render_trigger_view(TriggerViewInput {
                trigger_class,
                trigger_state,
                open_mode_attr,
                open_value_source_attr,
                open_intent_source_attr,
                anchor_ref,
                on_trigger_pointer_enter,
                on_trigger_pointer_leave,
                on_trigger_focus_in,
                on_trigger_focus_out,
                on_trigger_key_down,
                children,
            })}
            {render_panel_view(PanelViewInput {
                is_present: presence.is_present,
                panel_class,
                panel_ref,
                id,
                lang,
                dir,
                aria_keyshortcuts: dismiss_a11y.attrs.aria_keyshortcuts,
                placement: position.placement,
                panel_state,
                open_signal,
                open_mode_attr,
                open_value_source_attr,
                open_intent_source_attr,
                top_px: position.top_px,
                left_px: position.left_px,
                anchor_width_px: position.anchor_width_px,
                on_panel_pointer_enter,
                on_panel_pointer_leave,
                on_panel_focus_in,
                on_panel_focus_out,
                on_panel_key_down,
                content,
            })}
        </span>
    }
}
