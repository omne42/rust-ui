use crate::command_dialog::{
    CommandDialogPartState, CommandDialogPartStateInput, CommandDialogSlot,
};

pub const DEFAULT_ID_BASE: &str = "ui-command-dialog";
pub const DEFAULT_TITLE: &str = "Command Menu";
pub const DEFAULT_CLOSE_ON_ACTION: bool = true;
pub const DEFAULT_DISABLED: bool = false;
pub const DEFAULT_DEFAULT_OPEN: bool = false;

pub fn state_attr(is_open: bool) -> &'static str {
    if is_open { "open" } else { "closed" }
}

pub fn description_attr(has_description: bool) -> &'static str {
    if has_description { "present" } else { "absent" }
}

pub fn close_on_action_attr(close_on_action: bool) -> &'static str {
    if close_on_action { "true" } else { "false" }
}

pub fn disabled_attr(disabled: bool) -> &'static str {
    if disabled { "true" } else { "false" }
}

pub fn open_mode_attr(is_controlled: bool) -> &'static str {
    if is_controlled {
        "controlled"
    } else {
        "uncontrolled"
    }
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.into())
    })
}

pub fn normalize_id_base(value: Option<String>) -> String {
    normalize_optional_text(value).unwrap_or_else(|| DEFAULT_ID_BASE.into())
}

pub fn normalize_title(value: Option<String>) -> String {
    normalize_optional_text(value).unwrap_or_else(|| DEFAULT_TITLE.into())
}

fn source_attr(is_custom: bool) -> &'static str {
    if is_custom { "custom" } else { "default" }
}

pub fn resolve_state(input: CommandDialogPartStateInput) -> CommandDialogPartState {
    let enabled = !input.disabled;
    let is_uncontrolled = !input.is_controlled;

    CommandDialogPartState {
        slot: input.slot,
        slot_attr: input.slot.as_attr(),
        base_class: input.slot.base_class(),
        state_attr: state_attr(input.is_open),
        description_attr: description_attr(input.has_description),
        close_on_action_attr: close_on_action_attr(input.close_on_action),
        disabled_attr: disabled_attr(input.disabled),
        open_mode_attr: open_mode_attr(input.is_controlled),
        open_attr: input.is_open.then_some("true"),
        is_open: input.is_open,
        has_description: input.has_description,
        close_on_action: input.close_on_action,
        disabled: input.disabled,
        enabled,
        is_controlled: input.is_controlled,
        is_uncontrolled,
        has_custom_id_base: input.has_custom_id_base,
        has_custom_title: input.has_custom_title,
        has_custom_description: input.has_custom_description,
        has_custom_placeholder: input.has_custom_placeholder,
        has_custom_empty_label: input.has_custom_empty_label,
        has_custom_aria_label: input.has_custom_aria_label,
        has_custom_class_name: input.has_custom_class_name,
        has_custom_on_action: input.has_custom_on_action,
        has_custom_on_open_change: input.has_custom_on_open_change,
        has_custom_default_open: input.has_custom_default_open,
        has_custom_close_on_action: input.has_custom_close_on_action,
        has_custom_disabled: input.has_custom_disabled,
        has_custom_command_motion: input.has_custom_command_motion,
        has_custom_overlay_motion: input.has_custom_overlay_motion,
        id_source_attr: source_attr(input.has_custom_id_base),
        title_source_attr: source_attr(input.has_custom_title),
        description_source_attr: source_attr(input.has_custom_description),
        placeholder_source_attr: source_attr(input.has_custom_placeholder),
        empty_label_source_attr: source_attr(input.has_custom_empty_label),
        aria_label_source_attr: source_attr(input.has_custom_aria_label),
        class_source_attr: source_attr(input.has_custom_class_name),
        action_source_attr: source_attr(input.has_custom_on_action),
        open_change_source_attr: source_attr(input.has_custom_on_open_change),
        default_open_source_attr: source_attr(input.has_custom_default_open),
        close_on_action_source_attr: source_attr(input.has_custom_close_on_action),
        disabled_source_attr: source_attr(input.has_custom_disabled),
        command_motion_source_attr: source_attr(input.has_custom_command_motion),
        overlay_motion_source_attr: source_attr(input.has_custom_overlay_motion),
    }
}

pub fn compose_class_name(
    base_class_name: Option<String>,
    state: CommandDialogPartState,
) -> String {
    let mut classes = vec![state.base_class.into()];

    if matches!(state.slot, CommandDialogSlot::Root) {
        if state.is_open {
            classes.push("ui-command-dialog--open".to_string());
        } else {
            classes.push("ui-command-dialog--closed".to_string());
        }

        if state.has_description {
            classes.push("ui-command-dialog--with-description".to_string());
        } else {
            classes.push("ui-command-dialog--title-only".to_string());
        }

        if state.close_on_action {
            classes.push("ui-command-dialog--close-on-action".to_string());
        } else {
            classes.push("ui-command-dialog--persistent".to_string());
        }

        if state.disabled {
            classes.push("ui-command-dialog--disabled".to_string());
        }

        if state.is_controlled {
            classes.push("ui-command-dialog--controlled".to_string());
        } else {
            classes.push("ui-command-dialog--uncontrolled".to_string());
        }

        if state.has_custom_id_base {
            classes.push("ui-command-dialog--custom-id".to_string());
        }

        if state.has_custom_title {
            classes.push("ui-command-dialog--custom-title".to_string());
        }

        if state.has_custom_description {
            classes.push("ui-command-dialog--custom-description".to_string());
        }

        if state.has_custom_placeholder {
            classes.push("ui-command-dialog--custom-placeholder".to_string());
        }

        if state.has_custom_empty_label {
            classes.push("ui-command-dialog--custom-empty-label".to_string());
        }

        if state.has_custom_aria_label {
            classes.push("ui-command-dialog--custom-aria-label".to_string());
        }

        if state.has_custom_on_action {
            classes.push("ui-command-dialog--custom-action".to_string());
        }

        if state.has_custom_on_open_change {
            classes.push("ui-command-dialog--custom-open-change".to_string());
        }

        if state.has_custom_default_open {
            classes.push("ui-command-dialog--custom-default-open".to_string());
        }

        if state.has_custom_close_on_action {
            classes.push("ui-command-dialog--custom-close-on-action".to_string());
        }

        if state.has_custom_disabled {
            classes.push("ui-command-dialog--custom-disabled".to_string());
        }

        if state.has_custom_command_motion {
            classes.push("ui-command-dialog--custom-command-motion".to_string());
        }

        if state.has_custom_overlay_motion {
            classes.push("ui-command-dialog--custom-overlay-motion".to_string());
        }

        if state.has_custom_class_name {
            classes.push("ui-command-dialog--custom-class".to_string());
            if let Some(base_class_name) = base_class_name {
                classes.push(base_class_name);
            }
        }
    } else if let Some(base_class_name) = normalize_optional_text(base_class_name) {
        classes.push(base_class_name);
    }

    classes.join(" ")
}

#[cfg(test)]
#[path = "../test/logic.rs"]
mod tests;
