use crate::{FlipCardFlipMode, FlipCardMotion, logic, motion};
use leptos::{children::ViewFn, ev, html, prelude::*};
use ui_headless::{A11yDirection, use_ui_id_provider};

fn render_front_face(
    front_class: Memo<String>,
    derived_render_state: Memo<logic::FlipCardDerivedRenderState>,
    front: StoredValue<ViewFn>,
) -> impl IntoView {
    view! {
        <div
            class=move || front_class.get()
            data-slot=move || derived_render_state.get().front.slot_attr
            data-state=move || derived_render_state.get().front.state_attr
            data-visible=move || derived_render_state.get().front_markers.visible_attr
            data-hidden=move || derived_render_state.get().front_markers.hidden_attr
        >
            {move || front.with_value(|front| front.run())}
        </div>
    }
}

fn render_back_face(
    back_class: Memo<String>,
    derived_render_state: Memo<logic::FlipCardDerivedRenderState>,
    back: StoredValue<ViewFn>,
) -> impl IntoView {
    view! {
        <div
            class=move || back_class.get()
            data-slot=move || derived_render_state.get().back.slot_attr
            data-state=move || derived_render_state.get().back.state_attr
            data-visible=move || derived_render_state.get().back_markers.visible_attr
            data-hidden=move || derived_render_state.get().back_markers.hidden_attr
        >
            {move || back.with_value(|back| back.run())}
        </div>
    }
}

#[component]
pub fn FlipCard(
    #[prop(into)] front: ViewFn,
    #[prop(into)] back: ViewFn,
    #[prop(optional, into)] is_flipped: Option<Signal<bool>>,
    #[prop(optional)] default_is_flipped: Option<bool>,
    #[prop(optional)] default_flipped: Option<bool>,
    #[prop(optional)] on_is_flipped_change: Option<Callback<bool>>,
    #[prop(optional)] is_disabled: Option<bool>,
    #[prop(optional)] disabled: Option<bool>,
    #[prop(optional)] flip_mode: Option<FlipCardFlipMode>,
    #[prop(optional)] is_flip_on_hover: Option<bool>,
    #[prop(optional)] flip_on_hover: Option<bool>,
    #[prop(optional)] motion: FlipCardMotion,
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional, into)] id: Option<String>,
    #[prop(optional, into)] lang: Option<String>,
    #[prop(optional)] dir: Option<A11yDirection>,
) -> impl IntoView {
    let logic::FlipCardFlippedAxis {
        controlled_is_flipped,
        default_is_flipped,
        on_is_flipped_change,
        flipped_is_controlled,
        flipped_control_mode_attr,
        flipped_prop_source_attr,
        flipped_default_source_attr,
        flipped_change_source_attr,
    } = logic::normalize_flipped_axis(logic::FlipCardFlippedAxisInput {
        is_flipped,
        default_is_flipped,
        default_flipped,
        on_is_flipped_change,
    });

    let logic::FlipCardBehaviorFlags {
        is_disabled,
        flip_mode,
        disabled_source_attr: _,
        flip_mode_source_attr,
    } = logic::normalize_behavior_flags(logic::FlipCardBehaviorFlagsInput {
        is_disabled,
        disabled,
        flip_mode,
        is_flip_on_hover,
        flip_on_hover,
    });

    let motion = crate::motion::sanitize_motion(motion);
    let class_name = logic::normalize_optional_text(class_name);
    let has_custom_class_name = class_name.is_some();
    let has_custom_motion = motion != FlipCardMotion::default();

    let generated_id = use_ui_id_provider()
        .map(|id_provider| id_provider.next_prefixed_id(logic::DEFAULT_ID_PREFIX))
        .unwrap_or_else(|| logic::DEFAULT_ID_PREFIX.to_string());
    let (id, has_custom_id) = logic::resolve_id(id, generated_id);
    let id = StoredValue::new(id);

    let flipped_state = ui_headless::use_controllable_state(
        controlled_is_flipped,
        Some(default_is_flipped),
        on_is_flipped_change,
    );
    let is_flipped = flipped_state.value;
    let request_is_flipped_change = flipped_state.request_change;

    let flip_card_a11y = ui_headless::use_flip_card(ui_headless::FlipCardOptions {
        is_disabled,
        is_flipped,
        request_is_flipped_change,
        flip_on_hover: flip_mode.is_hover(),
        lang,
        dir,
    });
    let flip_card_a11y_attrs = flip_card_a11y.attrs.clone();
    let flip_card_a11y_handlers = flip_card_a11y.handlers.clone();
    let flip_card_a11y_state = flip_card_a11y.state.clone();
    let a11y_role = flip_card_a11y_attrs.role;
    let a11y_tabindex = flip_card_a11y_attrs.tabindex;
    let a11y_aria_pressed = flip_card_a11y_attrs.aria_pressed;
    let a11y_aria_disabled = flip_card_a11y_attrs.aria_disabled;
    let a11y_lang = flip_card_a11y_attrs.lang.clone();
    let a11y_dir = flip_card_a11y_attrs.dir;
    let is_hovered: Signal<bool> = Signal::derive(move || flip_card_a11y_state.is_hovered.get());

    let derived_render_state = Memo::new(move |_| {
        logic::derive_render_state(logic::FlipCardDerivedRenderStateInput {
            is_disabled,
            is_flipped: is_flipped.get(),
            is_hovered: is_hovered.get(),
            flip_mode,
            flip_mode_source_attr,
            has_custom_class_name,
            has_custom_motion,
            has_custom_id,
            flipped_is_controlled,
            flipped_control_mode_attr,
            flipped_prop_source_attr,
            flipped_default_source_attr,
            flipped_change_source_attr,
        })
    });
    let derived_render_state_for_agent = derived_render_state;
    let agent_action = RwSignal::new(logic::FlipCardAgentAction::SnapshotRender);
    let agent_contract = Memo::new(move |_| {
        logic::resolve_agent_contract(logic::FlipCardAgentContractInput {
            render_state: derived_render_state_for_agent.get(),
            action: agent_action.get(),
        })
    });

    let root_class = Memo::new(move |_| {
        logic::compose_class_name(class_name.clone(), derived_render_state.get().root)
    });
    let front_class =
        Memo::new(move |_| logic::compose_class_name(None, derived_render_state.get().front));
    let back_class =
        Memo::new(move |_| logic::compose_class_name(None, derived_render_state.get().back));

    let root_ref: NodeRef<html::Div> = NodeRef::new();
    motion::attach_motion(root_ref, is_flipped, is_hovered, motion);

    let on_click = {
        let handlers = flip_card_a11y_handlers.clone();
        move |_ev: ev::MouseEvent| {
            if is_disabled {
                return;
            }
            agent_action.set(logic::FlipCardAgentAction::Toggle);
            handlers.on_click.run(());
        }
    };

    let on_key_down = {
        let handlers = flip_card_a11y_handlers.clone();
        move |ev: ev::KeyboardEvent| {
            #[cfg(target_arch = "wasm32")]
            let is_composing = ev.is_composing();
            #[cfg(not(target_arch = "wasm32"))]
            let is_composing = false;

            if handlers.on_key_down.run((ev.key(), is_composing)) {
                agent_action.set(logic::FlipCardAgentAction::Toggle);
                ev.prevent_default();
                ev.stop_propagation();
            }
        }
    };

    let on_pointer_enter = {
        let handlers = flip_card_a11y_handlers.clone();
        move |_ev: ev::PointerEvent| {
            agent_action.set(logic::FlipCardAgentAction::HoverEnter);
            handlers.on_pointer_enter.run(());
        }
    };

    let on_pointer_leave = {
        let handlers = flip_card_a11y_handlers.clone();
        move |_ev: ev::PointerEvent| {
            agent_action.set(logic::FlipCardAgentAction::HoverLeave);
            handlers.on_pointer_leave.run(());
        }
    };

    let on_focus_in = {
        let handlers = flip_card_a11y_handlers.clone();
        move |_ev: ev::FocusEvent| {
            agent_action.set(logic::FlipCardAgentAction::Focus);
            handlers.on_focus.run(());
        }
    };

    let on_focus_out = {
        let handlers = flip_card_a11y_handlers;
        move |_ev: ev::FocusEvent| {
            agent_action.set(logic::FlipCardAgentAction::Blur);
            handlers.on_blur.run(());
        }
    };

    let front = StoredValue::new(front);
    let back = StoredValue::new(back);
    let front_face_view = render_front_face(front_class, derived_render_state, front);
    let back_face_view = render_back_face(back_class, derived_render_state, back);

    view! {
        <div
            class=move || root_class.get()
            id=move || id.with_value(|id| id.clone())
            role=a11y_role
            tabindex=a11y_tabindex
            aria-pressed=move || a11y_aria_pressed.get()
            aria-disabled=a11y_aria_disabled
            lang=a11y_lang.clone()
            dir=a11y_dir
            data-slot=move || derived_render_state.get().root.slot_attr
            data-state=move || derived_render_state.get().root.state_attr
            data-visible=move || derived_render_state.get().root.visibility_attr
            data-flipped=move || derived_render_state.get().root_markers.flipped_attr
            data-default=move || derived_render_state.get().root_markers.default_attr
            data-flipped-control-mode=move || derived_render_state.get().root_markers.flipped_control_mode_attr
            data-flipped-prop-source=move || derived_render_state.get().root_markers.flipped_prop_source_attr
            data-flipped-default-source=move || derived_render_state.get().root_markers.flipped_default_source_attr
            data-flipped-change-source=move || derived_render_state.get().root_markers.flipped_change_source_attr
            data-flipped-controlled=move || derived_render_state.get().root_markers.flipped_controlled_attr
            data-flipped-uncontrolled=move || derived_render_state.get().root_markers.flipped_uncontrolled_attr
            data-disabled=move || derived_render_state.get().root_markers.disabled_attr
            data-enabled=move || derived_render_state.get().root_markers.enabled_attr
            data-hovered=move || derived_render_state.get().root_markers.hovered_attr
            data-flip-mode=move || derived_render_state.get().root.flip_mode_attr
            data-class-source=move || derived_render_state.get().root.class_source_attr
            data-motion-source=move || derived_render_state.get().root.motion_source_attr
            data-id-source=move || derived_render_state.get().root.id_source_attr
            data-flip-mode-source=move || derived_render_state.get().root_markers.flip_mode_source_attr
            data-custom-class=move || derived_render_state.get().root_markers.custom_class_attr
            data-custom-motion=move || derived_render_state.get().root_markers.custom_motion_attr
            data-custom-id=move || derived_render_state.get().root_markers.custom_id_attr
            data-ui-schema=move || agent_contract.get().schema_name
            data-ui-schema-version=move || agent_contract.get().schema_version.as_str()
            data-ui-intent=move || agent_contract.get().intent.as_str()
            data-ui-action=move || agent_contract.get().action.as_str()
            data-ui-state=move || agent_contract.get().state.as_str()
            data-ui-source=move || agent_contract.get().source.as_str()
            data-ui-flipped-source=move || agent_contract.get().flipped_source
            data-ui-mode-source=move || agent_contract.get().mode_source
            data-ui-motion-source=move || agent_contract.get().motion_source
            data-ui-class-source=move || agent_contract.get().class_source
            data-ui-id-source=move || agent_contract.get().id_source
            data-ui-config-policy=move || agent_contract.get().config_policy.as_str()
            node_ref=root_ref
            on:click=on_click
            on:keydown=on_key_down
            on:pointerenter=on_pointer_enter
            on:pointerleave=on_pointer_leave
            on:focusin=on_focus_in
            on:focusout=on_focus_out
        >
            <div class="ui-flip-card__inner" data-slot="flip-card-inner">
                {front_face_view}
                {back_face_view}
            </div>
        </div>
    }
}
