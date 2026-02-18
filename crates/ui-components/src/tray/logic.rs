pub use ui_state_primitives::tray::{
    DEFAULT_DISMISSABLE, DEFAULT_FIXED_HEIGHT, DEFAULT_ID_BASE, DEFAULT_KEYBOARD_DISMISS_DISABLED,
    DEFAULT_SHOW_CLOSE_BUTTON, DEFAULT_TITLE, TrayPartState, TraySlot, normalize_id_base,
    normalize_optional_text, normalize_required_text, resolve_state,
};

pub fn compose_class_name(base_class_name: Option<String>, state: TrayPartState) -> String {
    let _ = DEFAULT_ID_BASE;
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

        if state.has_custom_id_base {
            classes.push("ui-tray--custom-id".to_string());
        }

        if state.has_custom_title {
            classes.push("ui-tray--custom-title".to_string());
        }

        if state.has_custom_description {
            classes.push("ui-tray--custom-description".to_string());
        }

        if state.footer_source_attr == "custom" {
            classes.push("ui-tray--custom-footer".to_string());
        }

        if state.close_source_attr == "custom" {
            classes.push("ui-tray--custom-close".to_string());
        }

        if state.size_source_attr == "custom" {
            classes.push("ui-tray--custom-size".to_string());
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
    use crate::tray::TrayPartStateInput;

    #[test]
    fn compose_class_name_includes_state_markers() {
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

        let class_name = compose_class_name(Some("docs-tray".to_string()), state);

        for token in [
            "ui-tray",
            "ui-tray--with-description",
            "ui-tray--with-footer",
            "ui-tray--close-hidden",
            "ui-tray--fixed-height",
            "ui-tray--custom-id",
            "ui-tray--custom-title",
            "ui-tray--custom-description",
            "ui-tray--custom-footer",
            "ui-tray--custom-close",
            "ui-tray--custom-size",
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
