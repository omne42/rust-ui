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
        (!trimmed.is_empty()).then(|| trimmed.to_string())
    })
}

pub fn normalize_id_base(value: Option<String>) -> String {
    normalize_optional_text(value).unwrap_or_else(|| DEFAULT_ID_BASE.to_string())
}

pub fn normalize_title(value: Option<String>) -> String {
    normalize_optional_text(value).unwrap_or_else(|| DEFAULT_TITLE.to_string())
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
    let mut classes = vec![state.base_class.to_string()];

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
mod tests {
    use super::*;
    use crate::command_dialog::CommandDialogSlot;

    #[test]
    fn normalize_helpers_trim_and_fallback() {
        assert_eq!(normalize_optional_text(None), None);
        assert_eq!(normalize_optional_text(Some("  ".to_string())), None);
        assert_eq!(
            normalize_optional_text(Some(" docs-dialog ".to_string())),
            Some("docs-dialog".to_string())
        );

        assert_eq!(normalize_id_base(None), DEFAULT_ID_BASE);
        assert_eq!(
            normalize_id_base(Some(" docs-command-dialog ".to_string())),
            "docs-command-dialog"
        );

        assert_eq!(normalize_title(None), DEFAULT_TITLE);
        assert_eq!(
            normalize_title(Some(" Quick Actions ".to_string())),
            "Quick Actions"
        );
    }

    #[test]
    fn resolve_state_tracks_flags_and_sources() {
        let state = resolve_state(CommandDialogPartStateInput {
            slot: CommandDialogSlot::Root,
            is_open: true,
            has_description: true,
            close_on_action: false,
            disabled: true,
            is_controlled: true,
            has_custom_id_base: true,
            has_custom_title: true,
            has_custom_description: true,
            has_custom_placeholder: true,
            has_custom_empty_label: true,
            has_custom_aria_label: true,
            has_custom_class_name: true,
            has_custom_on_action: true,
            has_custom_on_open_change: true,
            has_custom_default_open: true,
            has_custom_close_on_action: true,
            has_custom_disabled: true,
            has_custom_command_motion: true,
            has_custom_overlay_motion: true,
        });

        assert_eq!(state.state_attr, "open");
        assert_eq!(state.description_attr, "present");
        assert_eq!(state.close_on_action_attr, "false");
        assert_eq!(state.disabled_attr, "true");
        assert_eq!(state.open_mode_attr, "controlled");
        assert_eq!(state.class_source_attr, "custom");
        assert_eq!(state.action_source_attr, "custom");
        assert_eq!(state.command_motion_source_attr, "custom");
        assert_eq!(state.overlay_motion_source_attr, "custom");
    }

    #[test]
    fn compose_class_name_contains_state_markers() {
        let class_name = compose_class_name(
            Some("docs-command-dialog".to_string()),
            resolve_state(CommandDialogPartStateInput {
                slot: CommandDialogSlot::Root,
                is_open: false,
                has_description: false,
                close_on_action: true,
                disabled: false,
                is_controlled: false,
                has_custom_id_base: true,
                has_custom_title: true,
                has_custom_description: false,
                has_custom_placeholder: true,
                has_custom_empty_label: true,
                has_custom_aria_label: true,
                has_custom_class_name: true,
                has_custom_on_action: true,
                has_custom_on_open_change: false,
                has_custom_default_open: true,
                has_custom_close_on_action: false,
                has_custom_disabled: false,
                has_custom_command_motion: true,
                has_custom_overlay_motion: false,
            }),
        );

        for token in [
            "ui-command-dialog",
            "ui-command-dialog--closed",
            "ui-command-dialog--title-only",
            "ui-command-dialog--close-on-action",
            "ui-command-dialog--uncontrolled",
            "ui-command-dialog--custom-command-motion",
            "ui-command-dialog--custom-class",
            "docs-command-dialog",
        ] {
            assert!(
                class_name.contains(token),
                "composed class should include `{token}`"
            );
        }
    }
}
