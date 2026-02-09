use crate::command::{Command, CommandGroup, CommandMotion};
use crate::command_dialog::logic::{self, CommandDialogStateInput};
use crate::overlay_open;
use crate::presence::use_presence;
use crate::{Modal, OnPress, OverlayMotion};
use leptos::prelude::*;
use std::sync::Arc;

#[component]
pub fn CommandDialog(
    #[prop(into)] groups: Arc<[CommandGroup]>,
    #[prop(optional)] open: Option<Signal<bool>>,
    #[prop(optional)] default_open: Option<bool>,
    #[prop(optional)] on_open_change: Option<Callback<bool>>,
    #[prop(optional)] on_action: Option<Callback<String>>,
    #[prop(optional, default = true)] close_on_action: bool,
    #[prop(optional, into)] id_base: Option<String>,
    #[prop(optional, into)] title: Option<String>,
    #[prop(optional, into)] description: Option<String>,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] command_motion: CommandMotion,
    #[prop(optional)] overlay_motion: OverlayMotion,
    #[prop(optional, into)] placeholder: Option<String>,
    #[prop(optional, into)] empty_label: Option<String>,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
) -> impl IntoView {
    let id_base = logic::normalize_id_base(id_base);
    let title = logic::normalize_title(title);
    let description = logic::normalize_optional_text(description);
    let class_name = logic::normalize_optional_text(class_name);

    let is_controlled = open.is_some();
    let open_state = overlay_open::use_controllable_open_state(open, default_open, on_open_change);
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

    let state = Signal::derive(move || {
        logic::resolve_state(CommandDialogStateInput {
            is_open: open.get(),
            has_description: description.get_value().is_some(),
            close_on_action,
            disabled,
            is_controlled,
            has_custom_class_name: class_name.get_value().is_some(),
        })
    });

    let class = Signal::derive(move || {
        let state = state.get();
        logic::compose_class_name(class_name.get_value(), state)
    });

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
                class_name="ui-command-dialog__modal".to_string()
            >
                <div
                    class=move || class.get()
                    data-slot="command-dialog"
                    data-state=move || state.get().state_attr
                    data-open=move || open.get().then_some("true")
                    data-closed=move || (!open.get()).then_some("true")
                    data-description=move || state.get().description_attr
                    data-close-on-action=move || state.get().close_on_action_attr
                    data-disabled=move || state.get().disabled.then_some("true")
                    data-enabled=move || state.get().enabled.then_some("true")
                    data-controlled=move || state.get().is_controlled.then_some("true")
                    data-uncontrolled=move || state.get().is_uncontrolled.then_some("true")
                    data-class-source=move || state.get().class_source_attr
                    data-custom-class=move || state.get().has_custom_class_name.then_some("true")
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
                        class_name="ui-command-dialog__command".to_string()
                    />
                </div>
            </Modal>
        </Show>
    }
}
