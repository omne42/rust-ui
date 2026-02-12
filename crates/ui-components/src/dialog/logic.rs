use crate::dialog::{DialogPartState, DialogPartStateInput, DialogSlot};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum DialogSize {
    Sm,
    #[default]
    Md,
    Lg,
}

impl DialogSize {
    pub fn class_name(self) -> &'static str {
        match self {
            DialogSize::Sm => "ui-dialog--size-sm",
            DialogSize::Md => "ui-dialog--size-md",
            DialogSize::Lg => "ui-dialog--size-lg",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            DialogSize::Sm => "sm",
            DialogSize::Md => "md",
            DialogSize::Lg => "lg",
        }
    }
}

pub const DEFAULT_ID_BASE: &str = "ui-dialog";
pub const DEFAULT_TITLE: &str = "Dialog";
pub const DEFAULT_CLOSE_LABEL: &str = "Close";
pub const DEFAULT_SHOW_CLOSE_BUTTON: bool = true;
pub const DEFAULT_SIZE: DialogSize = DialogSize::Md;

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

pub fn resolve_state(input: DialogPartStateInput) -> DialogPartState {
    let has_custom_size = input.size != DEFAULT_SIZE;
    let has_custom_close =
        input.has_custom_close_label || input.show_close_button != DEFAULT_SHOW_CLOSE_BUTTON;

    DialogPartState {
        slot: input.slot,
        slot_attr: input.slot.as_attr(),
        base_class: input.slot.base_class(),
        size: input.size,
        size_attr: input.size.as_attr(),
        size_class: input.size.class_name(),
        state_attr: state_attr(input.has_description),
        description_attr: description_attr(input.has_description),
        footer_attr: footer_attr(input.has_footer),
        close_button_attr: close_button_attr(input.show_close_button),
        show_description: input.has_description,
        show_footer: input.has_footer,
        show_close_button: input.show_close_button,
        has_custom_size,
        has_custom_id_base: input.has_custom_id_base,
        has_custom_title: input.has_custom_title,
        has_custom_description: input.has_custom_description,
        has_custom_close_label: input.has_custom_close_label,
        has_custom_class_name: input.has_custom_class_name,
        has_custom_motion: input.has_custom_motion,
        has_on_exit_complete: input.has_on_exit_complete,
        size_source_attr: source_attr(has_custom_size),
        description_source_attr: source_attr(input.has_custom_description),
        footer_source_attr: source_attr(input.has_footer),
        close_source_attr: source_attr(has_custom_close),
        id_source_attr: source_attr(input.has_custom_id_base),
        title_source_attr: source_attr(input.has_custom_title),
        class_source_attr: source_attr(input.has_custom_class_name),
        motion_source_attr: source_attr(input.has_custom_motion),
        exit_source_attr: source_attr(input.has_on_exit_complete),
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: DialogPartState) -> String {
    let mut classes = vec![state.base_class.to_string()];

    if matches!(state.slot, DialogSlot::Root) {
        classes.push(state.size_class.to_string());

        if state.show_description {
            classes.push("ui-dialog--with-description".to_string());
        } else {
            classes.push("ui-dialog--title-only".to_string());
        }

        if state.show_footer {
            classes.push("ui-dialog--with-footer".to_string());
        } else {
            classes.push("ui-dialog--footer-absent".to_string());
        }

        if state.show_close_button {
            classes.push("ui-dialog--close-shown".to_string());
        } else {
            classes.push("ui-dialog--close-hidden".to_string());
        }

        if state.has_custom_size {
            classes.push("ui-dialog--custom-size".to_string());
        }

        if state.has_custom_id_base {
            classes.push("ui-dialog--custom-id".to_string());
        }

        if state.has_custom_title {
            classes.push("ui-dialog--custom-title".to_string());
        }

        if state.has_custom_description {
            classes.push("ui-dialog--custom-description".to_string());
        }

        if state.close_source_attr == "custom" {
            classes.push("ui-dialog--custom-close".to_string());
        }

        if state.has_custom_motion {
            classes.push("ui-dialog--custom-motion".to_string());
        }

        if state.has_on_exit_complete {
            classes.push("ui-dialog--custom-exit".to_string());
        }

        if state.has_custom_class_name {
            classes.push("ui-dialog--custom-class".to_string());
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
    use crate::dialog::DialogSlot;

    #[test]
    fn normalize_helpers_trim_values() {
        assert_eq!(normalize_optional_text(None), None);
        assert_eq!(normalize_optional_text(Some("  \n\t ".to_string())), None);
        assert_eq!(
            normalize_optional_text(Some("  docs-dialog  ".to_string())),
            Some("docs-dialog".to_string())
        );

        assert_eq!(
            normalize_required_text("  Confirm  ".to_string(), DEFAULT_TITLE),
            "Confirm"
        );
        assert_eq!(
            normalize_required_text("\n\t".to_string(), DEFAULT_TITLE),
            DEFAULT_TITLE
        );

        assert_eq!(
            normalize_id_base("  custom-dialog  ".to_string()),
            "custom-dialog"
        );
        assert_eq!(normalize_id_base("\n\t".to_string()), DEFAULT_ID_BASE);
    }

    #[test]
    fn resolve_state_tracks_size_description_and_sources() {
        let state = resolve_state(DialogPartStateInput {
            slot: DialogSlot::Root,
            size: DialogSize::Lg,
            has_description: true,
            has_footer: true,
            show_close_button: false,
            has_custom_id_base: true,
            has_custom_title: true,
            has_custom_description: true,
            has_custom_close_label: true,
            has_custom_class_name: true,
            has_custom_motion: true,
            has_on_exit_complete: true,
        });

        assert_eq!(state.size_attr, "lg");
        assert_eq!(state.state_attr, "with-description");
        assert_eq!(state.description_attr, "present");
        assert_eq!(state.footer_attr, "present");
        assert_eq!(state.close_button_attr, "hidden");
        assert_eq!(state.size_source_attr, "custom");
        assert_eq!(state.id_source_attr, "custom");
        assert_eq!(state.title_source_attr, "custom");
        assert_eq!(state.class_source_attr, "custom");
        assert_eq!(state.motion_source_attr, "custom");
        assert_eq!(state.exit_source_attr, "custom");
    }

    #[test]
    fn compose_class_name_adds_state_and_custom_markers() {
        let class_name = compose_class_name(
            Some("docs-dialog-custom".to_string()),
            resolve_state(DialogPartStateInput {
                slot: DialogSlot::Root,
                size: DialogSize::Lg,
                has_description: true,
                has_footer: true,
                show_close_button: true,
                has_custom_id_base: true,
                has_custom_title: true,
                has_custom_description: true,
                has_custom_close_label: true,
                has_custom_class_name: true,
                has_custom_motion: true,
                has_on_exit_complete: true,
            }),
        );

        for token in [
            "ui-dialog",
            "ui-dialog--size-lg",
            "ui-dialog--with-description",
            "ui-dialog--with-footer",
            "ui-dialog--close-shown",
            "ui-dialog--custom-size",
            "ui-dialog--custom-id",
            "ui-dialog--custom-title",
            "ui-dialog--custom-description",
            "ui-dialog--custom-close",
            "ui-dialog--custom-motion",
            "ui-dialog--custom-exit",
            "ui-dialog--custom-class",
            "docs-dialog-custom",
        ] {
            assert!(
                class_name.contains(token),
                "composed class should include `{token}`"
            );
        }
    }
}
