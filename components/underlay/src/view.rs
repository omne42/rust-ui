use crate::{UnderlayMotion, UnderlaySlot, logic, motion};
use leptos::{html, prelude::*};
use ui_ai_runtime::{AiOutputStatus, AiRenderMode, use_ai_space_state};
use ui_headless::{
    A11yDirection, OnPress, UnderlayOptions, use_controllable_open_state_traced, use_underlay,
};

#[component]
pub fn Underlay(
    id_base: String,
    #[prop(optional)] is_open: Option<Signal<bool>>,
    #[prop(optional)] open: Option<Signal<bool>>,
    #[prop(optional)] default_open: Option<bool>,
    #[prop(optional)] on_open_change: Option<Callback<bool>>,
    #[prop(optional)] on_close: Option<OnPress>,
    #[prop(optional)] is_transparent: Option<bool>,
    #[prop(optional)] transparent: Option<bool>,
    #[prop(optional)] is_disabled: Option<bool>,
    #[prop(optional)] disabled: Option<bool>,
    #[prop(optional, into)] lang: Option<String>,
    #[prop(optional)] dir: Option<A11yDirection>,
    #[prop(optional)] motion: UnderlayMotion,
    #[prop(optional, into)] class_name: Option<String>,
) -> impl IntoView {
    let class_name = logic::normalize_optional_text(class_name);
    let has_custom_class_name = class_name.is_some();
    let class_name = StoredValue::new(class_name);

    let open_state = logic::normalize_open_state(logic::UnderlayOpenStateInput {
        is_open,
        open,
        default_open,
        on_open_change,
    });
    let flags = logic::normalize_flags(logic::UnderlayFlagsInput {
        is_transparent,
        transparent,
        is_disabled,
        disabled,
    });

    let open_state_signal = use_controllable_open_state_traced(
        "underlay",
        open_state.open,
        Some(open_state.default_open),
        open_state.on_open_change,
    );
    let open_signal = open_state_signal.open;
    let request_open_change = open_state_signal.request_open_change;

    let on_close = StoredValue::new(on_close);
    let can_request_close = matches!(open_state.mode, logic::UnderlayOpenMode::Uncontrolled)
        || open_state.has_open_change_handler;
    let has_close_action = can_request_close || on_close.get_value().is_some();

    let close_action = Callback::new(move |_| {
        if can_request_close {
            request_open_change.run(false);
        }

        if let Some(on_close) = on_close.get_value() {
            on_close.run(());
        }
    });
    let close_action = has_close_action.then_some(close_action);

    let open_state = StoredValue::new(open_state);
    let flags = StoredValue::new(flags);
    let state = Memo::new(move |_| {
        logic::resolve_view_state(logic::UnderlayViewStateInput {
            slot: UnderlaySlot::Root,
            open: open_signal.get(),
            transparent: flags.get_value().transparent,
            disabled: flags.get_value().disabled,
            has_on_close: has_close_action,
            has_custom_class_name,
            open_state: open_state.get_value(),
            flags: flags.get_value(),
        })
    });

    let class =
        Memo::new(move |_| logic::compose_class_name(class_name.get_value(), state.get().part));
    let agent_contract = Memo::new(move |_| logic::resolve_agent_contract(state.get()));
    let ai_space_state_for_mode = use_ai_space_state();
    let stream_mode = Memo::new(move |_| {
        ai_space_state_for_mode
            .map(|state| state.get().mode)
            .unwrap_or(AiRenderMode::Snapshot)
    });
    let ai_space_state_for_status = use_ai_space_state();
    let output_status = Memo::new(move |_| {
        ai_space_state_for_status
            .map(|state| state.get().output_status)
            .unwrap_or(AiOutputStatus::Verified)
    });

    let underlay = use_underlay(UnderlayOptions {
        state: Signal::derive(move || state.get().part),
        on_close: close_action,
        lang,
        dir,
    });
    let role = underlay.attrs.role;
    let aria_hidden = underlay.attrs.aria_hidden;
    let tabindex = underlay.attrs.tabindex;
    let lang = underlay.attrs.lang;
    let dir = underlay.attrs.dir;
    let on_click = underlay.handlers.on_click;

    let root_ref: NodeRef<html::Div> = NodeRef::new();
    motion::attach_motion(root_ref, open_signal, motion);

    view! {
        <div
            id=id_base
            node_ref=root_ref
            class=move || class.get()
            role=role
            aria-hidden=aria_hidden
            tabindex=tabindex
            lang=lang.clone()
            dir=dir
            on:click=move |_| on_click.run(())
            data-slot=move || state.get().part.slot_attr
            data-state=move || state.get().part.state_attr
            data-open=move || state.get().part.open_attr
            data-transparent=move || state.get().part.transparent_attr
            data-disabled=move || state.get().part.disabled_attr
            data-interactive=move || state.get().part.interactive_attr
            data-tone=move || state.get().part.tone_attr
            data-close-mode=move || state.get().part.close_mode_attr
            data-transparent-source=move || state.get().part.transparent_source_attr
            data-disabled-source=move || state.get().part.disabled_source_attr
            data-close-source=move || state.get().part.close_source_attr
            data-class-source=move || state.get().part.class_source_attr
            data-open-mode=move || open_state.get_value().open_mode_attr
            data-open-source=move || open_state.get_value().open_source_attr
            data-open-change-source=move || open_state.get_value().open_change_source_attr
            data-open-prop-source=move || open_state.get_value().open_prop_source_attr
            data-has-default-open=move || {
                open_state.get_value().has_default_open.then_some("true")
            }
            data-transparent-prop-source=move || state.get().transparent_prop_source_attr
            data-disabled-prop-source=move || state.get().disabled_prop_source_attr
            data-controlled=move || {
                (state.get().open_mode_attr == "controlled").then_some("true")
            }
            data-uncontrolled=move || {
                (state.get().open_mode_attr == "uncontrolled").then_some("true")
            }
            data-ui-schema=move || agent_contract.get().schema_name
            data-ui-schema-version=move || agent_contract.get().schema_version.as_str()
            data-ui-intent=move || agent_contract.get().intent.as_str()
            data-ui-action=move || agent_contract.get().action.as_str()
            data-ui-state=move || agent_contract.get().state.as_str()
            data-ui-source=move || agent_contract.get().source.as_str()
            data-ui-stream-support=move || agent_contract.get().stream_support.as_str()
            data-ui-stream-fallback=move || agent_contract.get().stream_fallback.as_str()
            data-ui-stream-mode=move || stream_mode.get().as_str()
            data-ui-output-status=move || output_status.get().as_str()
            data-ui-capability-dismiss=move || {
                agent_contract.get().capabilities.can_dismiss.then_some("true")
            }
            data-ui-capability-external-sync=move || {
                agent_contract
                    .get()
                    .capabilities
                    .can_external_sync
                    .then_some("true")
            }
            data-custom-transparent=move || state.get().part.has_custom_transparent.then_some("true")
            data-custom-disabled=move || state.get().part.has_custom_disabled.then_some("true")
            data-custom-close=move || state.get().part.has_custom_close_handler.then_some("true")
            data-custom-class=move || state.get().part.has_custom_class_name.then_some("true")
        ></div>
    }
}
