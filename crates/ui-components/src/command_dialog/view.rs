use crate::command::{Command, CommandGroup, CommandMotion};
use crate::command_dialog::{CommandDialogPartStateInput, CommandDialogSlot, logic};
use crate::{Modal, OnPress, OverlayMotion};
use leptos::prelude::*;
use std::sync::Arc;
use ui_headless as overlay_open;
use ui_headless::use_presence;

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
    #[prop(optional, default = logic::DEFAULT_DISABLED)] disabled: bool,
    #[prop(optional)] command_motion: CommandMotion,
    #[prop(optional)] overlay_motion: OverlayMotion,
    #[prop(optional, into)] placeholder: Option<String>,
    #[prop(optional, into)] empty_label: Option<String>,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
) -> impl IntoView {
    let id_base = logic::normalize_id_base(id_base);
    let has_custom_id_base = id_base != logic::DEFAULT_ID_BASE;

    let title = logic::normalize_title(title);
    let has_custom_title = title != logic::DEFAULT_TITLE;

    let description = logic::normalize_optional_text(description);
    let has_custom_description = description.is_some();

    let placeholder = logic::normalize_optional_text(placeholder);
    let has_custom_placeholder = placeholder.is_some();

    let empty_label = logic::normalize_optional_text(empty_label);
    let has_custom_empty_label = empty_label.is_some();

    let aria_label = logic::normalize_optional_text(aria_label);
    let has_custom_aria_label = aria_label.is_some();

    let class_name = logic::normalize_optional_text(class_name);
    let has_custom_class_name = class_name.is_some();

    let has_custom_on_action = on_action.is_some();
    let has_custom_on_open_change = on_open_change.is_some();
    let is_controlled = open.is_some();
    let has_custom_default_open = default_open.is_some();
    let has_custom_close_on_action = close_on_action != logic::DEFAULT_CLOSE_ON_ACTION;
    let has_custom_disabled = disabled != logic::DEFAULT_DISABLED;

    let has_custom_command_motion = command_motion != CommandMotion::default();
    let has_custom_overlay_motion = overlay_motion != OverlayMotion::default();

    let open_state = overlay_open::use_controllable_open_state_traced(
        "command-dialog",
        open,
        default_open,
        on_open_change,
    );
    let open = open_state.open;
    let request_open_change = open_state.request_open_change;

    let id_base = StoredValue::new(id_base);
    let title = StoredValue::new(title);
    let description = StoredValue::new(description);
    let class_name = StoredValue::new(class_name);
    let groups = StoredValue::new(groups);
    let on_action = StoredValue::new(on_action);
    let placeholder = StoredValue::new(placeholder);
    let empty_label = StoredValue::new(empty_label);
    let aria_label = StoredValue::new(aria_label);

    let root_state = Signal::derive(move || {
        logic::resolve_state(CommandDialogPartStateInput {
            slot: CommandDialogSlot::Root,
            is_open: open.get(),
            has_description: description.get_value().is_some(),
            close_on_action,
            disabled,
            is_controlled,
            has_custom_id_base,
            has_custom_title,
            has_custom_description,
            has_custom_placeholder,
            has_custom_empty_label,
            has_custom_aria_label,
            has_custom_class_name,
            has_custom_on_action,
            has_custom_on_open_change,
            has_custom_default_open,
            has_custom_close_on_action,
            has_custom_disabled,
            has_custom_command_motion,
            has_custom_overlay_motion,
        })
    });

    let root_class = Signal::derive(move || {
        let state = root_state.get();
        logic::compose_class_name(class_name.get_value(), state)
    });

    let modal_state = logic::resolve_state(CommandDialogPartStateInput {
        slot: CommandDialogSlot::Modal,
        is_open: false,
        has_description: description.get_value().is_some(),
        close_on_action,
        disabled,
        is_controlled,
        has_custom_id_base,
        has_custom_title,
        has_custom_description,
        has_custom_placeholder,
        has_custom_empty_label,
        has_custom_aria_label,
        has_custom_class_name: false,
        has_custom_on_action,
        has_custom_on_open_change,
        has_custom_default_open,
        has_custom_close_on_action,
        has_custom_disabled,
        has_custom_command_motion,
        has_custom_overlay_motion,
    });
    let modal_class = StoredValue::new(logic::compose_class_name(None, modal_state));

    let command_state = logic::resolve_state(CommandDialogPartStateInput {
        slot: CommandDialogSlot::Command,
        is_open: false,
        has_description: description.get_value().is_some(),
        close_on_action,
        disabled,
        is_controlled,
        has_custom_id_base,
        has_custom_title,
        has_custom_description,
        has_custom_placeholder,
        has_custom_empty_label,
        has_custom_aria_label,
        has_custom_class_name: false,
        has_custom_on_action,
        has_custom_on_open_change,
        has_custom_default_open,
        has_custom_close_on_action,
        has_custom_disabled,
        has_custom_command_motion,
        has_custom_overlay_motion,
    });
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

    view! {
        <Show when=move || presence.is_present.get()>
            <Modal
                open=open
                id_base=id_base.get_value()
                title=title.get_value()
                description=description.get_value().unwrap_or_default()
                motion=overlay_motion
                on_close=on_close
                on_exit_complete=presence.finish_exit
                class_name=modal_class.get_value()
            >
                <div
                    class=move || root_class.get()
                    data-slot=move || root_state.get().slot_attr
                    data-state=move || root_state.get().state_attr
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
                    data-custom-description=move || {
                        root_state.get().has_custom_description.then_some("true")
                    }
                    data-custom-placeholder=move || {
                        root_state.get().has_custom_placeholder.then_some("true")
                    }
                    data-custom-empty-label=move || {
                        root_state.get().has_custom_empty_label.then_some("true")
                    }
                    data-custom-aria-label=move || {
                        root_state.get().has_custom_aria_label.then_some("true")
                    }
                    data-custom-class=move || root_state.get().has_custom_class_name.then_some("true")
                    data-custom-action=move || root_state.get().has_custom_on_action.then_some("true")
                    data-custom-open-change=move || {
                        root_state.get().has_custom_on_open_change.then_some("true")
                    }
                    data-custom-default-open=move || {
                        root_state.get().has_custom_default_open.then_some("true")
                    }
                    data-custom-close-on-action=move || {
                        root_state.get().has_custom_close_on_action.then_some("true")
                    }
                    data-custom-disabled=move || root_state.get().has_custom_disabled.then_some("true")
                    data-custom-command-motion=move || {
                        root_state.get().has_custom_command_motion.then_some("true")
                    }
                    data-custom-overlay-motion=move || {
                        root_state.get().has_custom_overlay_motion.then_some("true")
                    }
                >
                    <Command
                        id_base=format!("{}-command", id_base.get_value())
                        groups=groups.get_value()
                        on_action=on_action_wrapped
                        disabled=disabled
                        motion=command_motion
                        placeholder=placeholder.get_value().unwrap_or_default()
                        empty_label=empty_label.get_value().unwrap_or_default()
                        aria_label=aria_label.get_value().unwrap_or_default()
                        class_name=command_class.get_value()
                    />
                </div>
            </Modal>
        </Show>
    }
}
