use crate::OnPress;
use crate::command::{Command, CommandGroup, CommandMotion};
use crate::command_dialog::{CommandDialogPartState, CommandDialogSlot, logic, motion};
use crate::modal::Modal;
use crate::overlay::OverlayMotion;
use leptos::prelude::*;
use std::sync::Arc;
use ui_headless::{Presence, UiTraceEventKind, use_presence, use_ui_trace};

struct CommandDialogRenderInput {
    presence: Presence,
    open: Signal<bool>,
    id_base: StoredValue<String>,
    title: StoredValue<String>,
    description_text: StoredValue<String>,
    overlay_motion: OverlayMotion,
    on_close: OnPress,
    modal_class: StoredValue<String>,
    root_class: Signal<String>,
    root_state: Signal<CommandDialogPartState>,
    groups: StoredValue<Arc<[CommandGroup]>>,
    on_action_wrapped: Callback<String>,
    disabled: bool,
    command_motion: CommandMotion,
    placeholder_text: StoredValue<String>,
    empty_label_text: StoredValue<String>,
    aria_label_text: StoredValue<String>,
    command_class: StoredValue<String>,
}

fn render_dialog_view(input: CommandDialogRenderInput) -> impl IntoView {
    let CommandDialogRenderInput {
        presence,
        open,
        id_base,
        title,
        description_text,
        overlay_motion,
        on_close,
        modal_class,
        root_class,
        root_state,
        groups,
        on_action_wrapped,
        disabled,
        command_motion,
        placeholder_text,
        empty_label_text,
        aria_label_text,
        command_class,
    } = input;

    let agent_contract = Signal::derive(move || logic::resolve_agent_contract(root_state.get()));
    view! {
        <Show when=move || presence.is_present.get()>
            <Modal
                is_open=open
                id_base=id_base.get_value()
                title=title.get_value()
                description=description_text.get_value()
                motion=overlay_motion
                on_close=on_close
                on_exit_complete=presence.finish_exit
                class_name=modal_class.get_value()
            >
                <div
                    class=move || root_class.get()
                    data-slot=move || root_state.get().slot_attr
                    data-state=move || root_state.get().state_attr
                    data-ui-schema=move || agent_contract.get().schema_name
                    data-ui-schema-version=move || agent_contract.get().schema_version.as_str()
                    data-ui-intent=move || agent_contract.get().intent.as_str()
                    data-ui-action=move || agent_contract.get().action.as_str()
                    data-ui-state=move || agent_contract.get().state.as_str()
                    data-ui-source=move || agent_contract.get().source.as_str()
                    data-ui-stream-support=move || agent_contract.get().stream_support.as_str()
                    data-ui-action-source=move || agent_contract.get().action_source
                    data-ui-open-change-source=move || agent_contract.get().open_change_source
                    data-ui-config-policy=move || agent_contract.get().config_policy.as_str()
                    data-stream-mode=move || agent_contract.get().stream_mode.as_str()
                    data-stream-fallback=move || agent_contract.get().stream_fallback.as_str()
                    data-output-status=move || agent_contract.get().output_status.as_str()
                    data-open=move || root_state.get().open_attr
                    data-closed=move || (!root_state.get().is_open).then_some("true")
                    data-description=move || root_state.get().description_attr
                    data-close-on-action=move || root_state.get().close_on_action_attr
                    data-disabled=move || root_state.get().disabled.then_some("true")
                    data-enabled=move || root_state.get().enabled.then_some("true")
                    data-open-mode=move || root_state.get().open_mode_attr
                    data-controlled=move || root_state.get().is_controlled.then_some("true")
                    data-uncontrolled=move || root_state.get().is_uncontrolled.then_some("true")
                    data-id-source=move || root_state.get().id_source_attr
                    data-title-source=move || root_state.get().title_source_attr
                    data-description-source=move || root_state.get().description_source_attr
                    data-placeholder-source=move || root_state.get().placeholder_source_attr
                    data-empty-label-source=move || root_state.get().empty_label_source_attr
                    data-aria-label-source=move || root_state.get().aria_label_source_attr
                    data-class-source=move || root_state.get().class_source_attr
                    data-action-source=move || root_state.get().action_source_attr
                    data-open-change-source=move || root_state.get().open_change_source_attr
                    data-default-open-source=move || root_state.get().default_open_source_attr
                    data-close-on-action-source=move || root_state.get().close_on_action_source_attr
                    data-disabled-source=move || root_state.get().disabled_source_attr
                    data-command-motion-source=move || root_state.get().command_motion_source_attr
                    data-overlay-motion-source=move || root_state.get().overlay_motion_source_attr
                    data-custom-id=move || root_state.get().has_custom_id_base.then_some("true")
                    data-custom-title=move || root_state.get().has_custom_title.then_some("true")
                    data-custom-description=move || root_state.get().has_custom_description.then_some("true")
                    data-custom-placeholder=move || root_state.get().has_custom_placeholder.then_some("true")
                    data-custom-empty-label=move || root_state.get().has_custom_empty_label.then_some("true")
                    data-custom-aria-label=move || root_state.get().has_custom_aria_label.then_some("true")
                    data-custom-class=move || root_state.get().has_custom_class_name.then_some("true")
                    data-custom-action=move || root_state.get().has_custom_on_action.then_some("true")
                    data-custom-open-change=move || root_state.get().has_custom_on_open_change.then_some("true")
                    data-custom-default-open=move || root_state.get().has_custom_default_open.then_some("true")
                    data-custom-close-on-action=move || root_state.get().has_custom_close_on_action.then_some("true")
                    data-custom-disabled=move || root_state.get().has_custom_disabled.then_some("true")
                    data-custom-command-motion=move || root_state.get().has_custom_command_motion.then_some("true")
                    data-custom-overlay-motion=move || root_state.get().has_custom_overlay_motion.then_some("true")
                >
                    <Command
                        id_base=format!("{}-command", id_base.get_value())
                        groups=groups.get_value()
                        on_action=on_action_wrapped
                        is_disabled=disabled
                        motion=command_motion
                        placeholder=placeholder_text.get_value()
                        empty_label=empty_label_text.get_value()
                        aria_label=aria_label_text.get_value()
                        class_name=command_class.get_value()
                    />
                </div>
            </Modal>
        </Show>
    }
}

#[component]
pub fn CommandDialog(
    #[prop(into)] groups: Arc<[CommandGroup]>,
    #[prop(optional)] open: Option<Signal<bool>>,
    #[prop(optional)] default_open: Option<bool>,
    #[prop(optional)] on_open_change: Option<Callback<bool>>,
    #[prop(optional)] on_action: Option<Callback<String>>,
    #[prop(optional, default = logic::DEFAULT_CLOSE_ON_ACTION)] close_on_action: bool,
    #[prop(optional, into)] id_base: Option<String>,
    #[prop(optional, into)] title: Option<String>,
    #[prop(optional, into)] description: Option<String>,
    #[prop(optional)] is_disabled: Option<bool>,
    #[prop(optional, default = logic::DEFAULT_DISABLED)] disabled: bool,
    #[prop(optional)] command_motion: CommandMotion,
    #[prop(optional)] overlay_motion: OverlayMotion,
    #[prop(optional, into)] placeholder: Option<String>,
    #[prop(optional, into)] empty_label: Option<String>,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
) -> impl IntoView {
    let open_prop = open;
    let motion = motion::attach_motion(command_motion, overlay_motion);
    let command_motion = motion.command;
    let overlay_motion = motion.overlay;
    let normalized = logic::normalize_props(logic::CommandDialogNormalizationInput {
        open_input: open_prop.map(|value| value.get_untracked()),
        default_open,
        has_open_prop: open_prop.is_some(),
        has_on_action: on_action.is_some(),
        has_on_open_change: on_open_change.is_some(),
        close_on_action,
        id_base,
        title,
        description,
        is_disabled,
        disabled,
        placeholder,
        empty_label,
        aria_label,
        class_name,
        has_custom_command_motion: command_motion != CommandMotion::default(),
        has_custom_overlay_motion: overlay_motion != OverlayMotion::default(),
    });
    let close_on_action = normalized.close_on_action;
    let disabled = normalized.disabled;
    let open_state_options =
        logic::normalize_open_state_options(normalized.open_input, normalized.default_open);
    let open_state = RwSignal::new(logic::use_overlay_trigger_state(open_state_options));
    Effect::new(move |_| {
        open_state.update(|state| state.sync_controlled(open_prop.map(|value| value.get())));
    });
    let open = Signal::derive(move || open_state.with(|state| state.is_open()));
    let trace = use_ui_trace();
    let on_open_change = StoredValue::new(on_open_change);
    let request_open_change: Callback<bool> = Callback::new(move |next: bool| {
        let current = open_state.with_untracked(|state| state.is_open());
        if !logic::should_emit_open_change(current, next) {
            return;
        }

        if let Some(trace) = trace {
            trace.emit(
                "command-dialog",
                UiTraceEventKind::OpenChange { open: next },
            );
        }
        if let Some(callback) = on_open_change.get_value() {
            callback.run(next);
        }

        open_state.update(|state| {
            logic::apply_open_change(state, open_prop.map(|value| value.get_untracked()), next);
        });
    });

    let state_normalized = StoredValue::new(normalized.clone());
    let id_base = StoredValue::new(normalized.id_base);
    let title = StoredValue::new(normalized.title);
    let description_text = StoredValue::new(normalized.description_text);
    let class_name = StoredValue::new(normalized.class_name);
    let groups = StoredValue::new(groups);
    let on_action = StoredValue::new(on_action);
    let placeholder_text = StoredValue::new(normalized.placeholder_text);
    let empty_label_text = StoredValue::new(normalized.empty_label_text);
    let aria_label_text = StoredValue::new(normalized.aria_label_text);

    let root_state = Signal::derive(move || {
        let normalized = state_normalized.get_value();
        logic::resolve_part_state(&normalized, CommandDialogSlot::Root, open.get())
    });

    let root_class = Signal::derive(move || {
        let state = root_state.get();
        logic::compose_class_name(class_name.get_value(), state)
    });

    let modal_state = logic::resolve_part_state(
        &state_normalized.get_value(),
        CommandDialogSlot::Modal,
        false,
    );
    let modal_class = StoredValue::new(logic::compose_class_name(None, modal_state));

    let command_state = logic::resolve_part_state(
        &state_normalized.get_value(),
        CommandDialogSlot::Command,
        false,
    );
    let command_class = StoredValue::new(logic::compose_class_name(None, command_state));

    let on_close: OnPress = Callback::new(move |_| request_open_change.run(false));
    let on_action_wrapped = Callback::new(move |id: String| {
        if let Some(callback) = on_action.get_value() {
            callback.run(id);
        }
        if close_on_action {
            request_open_change.run(false);
        }
    });
    let presence = use_presence(open);
    render_dialog_view(CommandDialogRenderInput {
        presence,
        open,
        id_base,
        title,
        description_text,
        overlay_motion,
        on_close,
        modal_class,
        root_class,
        root_state,
        groups,
        on_action_wrapped,
        disabled,
        command_motion,
        placeholder_text,
        empty_label_text,
        aria_label_text,
        command_class,
    })
}
