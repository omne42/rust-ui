use crate::tray::{TrayPartState, TrayPartStateInput, TraySlot};

pub const DEFAULT_ID_BASE: &str = "ui-tray";
pub const DEFAULT_TITLE: &str = "Tray";
pub const DEFAULT_SHOW_CLOSE_BUTTON: bool = true;
pub const DEFAULT_FIXED_HEIGHT: bool = false;
pub const DEFAULT_DISMISSABLE: bool = true;
pub const DEFAULT_KEYBOARD_DISMISS_DISABLED: bool = false;

pub fn state_attr(has_description: bool) -> &'static str {
    if has_description {
        "with-description"
    } else {
        "title-only"
    }
}

pub fn description_attr(has_description: bool) -> &'static str {
    if has_description { "present" } else { "absent" }
}

pub fn footer_attr(has_footer: bool) -> &'static str {
    if has_footer { "present" } else { "absent" }
}

pub fn close_button_attr(show_close_button: bool) -> &'static str {
    if show_close_button { "shown" } else { "hidden" }
}

pub fn size_attr(is_fixed_height: bool) -> &'static str {
    if is_fixed_height { "fixed" } else { "auto" }
}

pub fn dismiss_attr(is_dismissable: bool) -> &'static str {
    if is_dismissable {
        "dismissable"
    } else {
        "locked"
    }
}

pub fn keyboard_dismiss_attr(is_keyboard_dismiss_disabled: bool) -> &'static str {
    if is_keyboard_dismiss_disabled {
        "disabled"
    } else {
        "enabled"
    }
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.to_string())
    })
}

pub fn normalize_required_text(value: String, fallback: &'static str) -> String {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        fallback.to_string()
    } else {
        trimmed.to_string()
    }
}

pub fn normalize_id_base(value: String) -> String {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        DEFAULT_ID_BASE.to_string()
    } else {
        trimmed.to_string()
    }
}

fn source_attr(is_custom: bool) -> &'static str {
    if is_custom { "custom" } else { "default" }
}

pub fn resolve_state(input: TrayPartStateInput) -> TrayPartState {
    TrayPartState {
        slot: input.slot,
        slot_attr: input.slot.as_attr(),
        base_class: input.slot.base_class(),
        state_attr: state_attr(input.has_description),
        description_attr: description_attr(input.has_description),
        footer_attr: footer_attr(input.has_footer),
        close_button_attr: close_button_attr(input.show_close_button),
        size_attr: size_attr(input.is_fixed_height),
        dismiss_attr: dismiss_attr(input.is_dismissable),
        keyboard_dismiss_attr: keyboard_dismiss_attr(input.is_keyboard_dismiss_disabled),
        show_description: input.has_description,
        show_footer: input.has_footer,
        show_close_button: input.show_close_button,
        is_fixed_height: input.is_fixed_height,
        is_dismissable: input.is_dismissable,
        is_keyboard_dismiss_disabled: input.is_keyboard_dismiss_disabled,
        has_custom_id_base: input.has_custom_id_base,
        has_custom_title: input.has_custom_title,
        has_custom_description: input.has_custom_description,
        has_custom_class_name: input.has_custom_class_name,
        has_custom_motion: input.has_custom_motion,
        has_on_exit_complete: input.has_on_exit_complete,
        description_source_attr: source_attr(input.has_custom_description),
        footer_source_attr: source_attr(input.has_footer),
        close_source_attr: source_attr(input.show_close_button != DEFAULT_SHOW_CLOSE_BUTTON),
        size_source_attr: source_attr(input.is_fixed_height != DEFAULT_FIXED_HEIGHT),
        dismiss_source_attr: source_attr(input.is_dismissable != DEFAULT_DISMISSABLE),
        keyboard_dismiss_source_attr: source_attr(
            input.is_keyboard_dismiss_disabled != DEFAULT_KEYBOARD_DISMISS_DISABLED,
        ),
        id_source_attr: source_attr(input.has_custom_id_base),
        title_source_attr: source_attr(input.has_custom_title),
        class_source_attr: source_attr(input.has_custom_class_name),
        motion_source_attr: source_attr(input.has_custom_motion),
        exit_source_attr: source_attr(input.has_on_exit_complete),
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: TrayPartState) -> String {
    let mut classes = vec![state.base_class.to_string()];

    if state.slot == TraySlot::Root {
        if state.show_description {
            classes.push("ui-tray--with-description".to_string());
        } else {
            classes.push("ui-tray--title-only".to_string());
        }

        if state.show_footer {
            classes.push("ui-tray--with-footer".to_string());
        } else {
            classes.push("ui-tray--no-footer".to_string());
        }

        if state.show_close_button {
            classes.push("ui-tray--close-shown".to_string());
        } else {
            classes.push("ui-tray--close-hidden".to_string());
        }

        if state.is_fixed_height {
            classes.push("ui-tray--fixed-height".to_string());
        } else {
            classes.push("ui-tray--auto-height".to_string());
        }

        if state.has_custom_motion {
            classes.push("ui-tray--custom-motion".to_string());
        }

        if state.has_on_exit_complete {
            classes.push("ui-tray--custom-exit".to_string());
        }

        if state.dismiss_source_attr == "custom" {
            classes.push("ui-tray--custom-dismiss".to_string());
        }

        if state.keyboard_dismiss_source_attr == "custom" {
            classes.push("ui-tray--custom-keyboard-dismiss".to_string());
        }

        if state.has_custom_class_name {
            classes.push("ui-tray--custom-class".to_string());
            if let Some(base_class_name) = base_class_name {
                classes.push(base_class_name);
            }
        }
    }

    classes.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn state_attrs_follow_contract() {
        assert_eq!(state_attr(true), "with-description");
        assert_eq!(state_attr(false), "title-only");
        assert_eq!(description_attr(true), "present");
        assert_eq!(description_attr(false), "absent");
        assert_eq!(footer_attr(true), "present");
        assert_eq!(footer_attr(false), "absent");
        assert_eq!(close_button_attr(true), "shown");
        assert_eq!(close_button_attr(false), "hidden");
        assert_eq!(size_attr(true), "fixed");
        assert_eq!(size_attr(false), "auto");
        assert_eq!(dismiss_attr(true), "dismissable");
        assert_eq!(dismiss_attr(false), "locked");
        assert_eq!(keyboard_dismiss_attr(false), "enabled");
        assert_eq!(keyboard_dismiss_attr(true), "disabled");
    }

    #[test]
    fn normalize_helpers_trim_and_fallback() {
        assert_eq!(normalize_optional_text(None), None);
        assert_eq!(normalize_optional_text(Some("  ".to_string())), None);
        assert_eq!(
            normalize_optional_text(Some(" docs-tray ".to_string())),
            Some("docs-tray".to_string())
        );

        assert_eq!(
            normalize_required_text(" Tray ".to_string(), DEFAULT_TITLE),
            "Tray"
        );
        assert_eq!(
            normalize_required_text(" ".to_string(), DEFAULT_TITLE),
            DEFAULT_TITLE
        );

        assert_eq!(normalize_id_base(" docs-tray ".to_string()), "docs-tray");
        assert_eq!(normalize_id_base(" ".to_string()), DEFAULT_ID_BASE);
    }

    #[test]
    fn resolve_state_tracks_source_markers() {
        let state = resolve_state(TrayPartStateInput {
            slot: TraySlot::Root,
            has_description: true,
            has_footer: true,
            show_close_button: false,
            is_fixed_height: true,
            is_dismissable: false,
            is_keyboard_dismiss_disabled: true,
            has_custom_id_base: true,
            has_custom_title: true,
            has_custom_description: true,
            has_custom_class_name: true,
            has_custom_motion: true,
            has_on_exit_complete: true,
        });

        assert_eq!(state.slot_attr, "tray");
        assert_eq!(state.base_class, "ui-tray");
        assert_eq!(state.state_attr, "with-description");
        assert_eq!(state.description_attr, "present");
        assert_eq!(state.footer_attr, "present");
        assert_eq!(state.close_button_attr, "hidden");
        assert_eq!(state.size_attr, "fixed");
        assert_eq!(state.dismiss_attr, "locked");
        assert_eq!(state.keyboard_dismiss_attr, "disabled");
        assert_eq!(state.description_source_attr, "custom");
        assert_eq!(state.footer_source_attr, "custom");
        assert_eq!(state.close_source_attr, "custom");
        assert_eq!(state.size_source_attr, "custom");
        assert_eq!(state.dismiss_source_attr, "custom");
        assert_eq!(state.keyboard_dismiss_source_attr, "custom");
        assert_eq!(state.id_source_attr, "custom");
        assert_eq!(state.title_source_attr, "custom");
        assert_eq!(state.class_source_attr, "custom");
        assert_eq!(state.motion_source_attr, "custom");
        assert_eq!(state.exit_source_attr, "custom");
    }

    #[test]
    fn compose_class_name_includes_state_markers() {
        let state = resolve_state(TrayPartStateInput {
            slot: TraySlot::Root,
            has_description: false,
            has_footer: true,
            show_close_button: false,
            is_fixed_height: true,
            is_dismissable: false,
            is_keyboard_dismiss_disabled: true,
            has_custom_id_base: false,
            has_custom_title: false,
            has_custom_description: false,
            has_custom_class_name: true,
            has_custom_motion: true,
            has_on_exit_complete: true,
        });

        let class_name = compose_class_name(Some("docs-tray".to_string()), state);

        for token in [
            "ui-tray",
            "ui-tray--title-only",
            "ui-tray--with-footer",
            "ui-tray--close-hidden",
            "ui-tray--fixed-height",
            "ui-tray--custom-motion",
            "ui-tray--custom-exit",
            "ui-tray--custom-dismiss",
            "ui-tray--custom-keyboard-dismiss",
            "ui-tray--custom-class",
            "docs-tray",
        ] {
            assert!(
                class_name.contains(token),
                "composed class name should include `{token}`"
            );
        }
    }
}
