use crate::modal::{ModalPartState, ModalPartStateInput, ModalSlot};

pub const DEFAULT_ID_BASE: &str = "ui-modal";
pub const DEFAULT_TITLE: &str = "Modal";

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

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.into())
    })
}

pub fn normalize_required_text(value: String, fallback: &'static str) -> String {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        fallback.into()
    } else {
        trimmed.into()
    }
}

pub fn normalize_id_base(value: String) -> String {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        DEFAULT_ID_BASE.into()
    } else {
        trimmed.into()
    }
}

fn source_attr(is_custom: bool) -> &'static str {
    if is_custom { "custom" } else { "default" }
}

pub fn resolve_state(input: ModalPartStateInput) -> ModalPartState {
    ModalPartState {
        slot: input.slot,
        slot_attr: input.slot.as_attr(),
        base_class: input.slot.base_class(),
        state_attr: state_attr(input.has_description),
        description_attr: description_attr(input.has_description),
        show_description: input.has_description,
        has_custom_id_base: input.has_custom_id_base,
        has_custom_title: input.has_custom_title,
        has_custom_description: input.has_custom_description,
        has_custom_class_name: input.has_custom_class_name,
        has_custom_motion: input.has_custom_motion,
        has_on_exit_complete: input.has_on_exit_complete,
        id_source_attr: source_attr(input.has_custom_id_base),
        title_source_attr: source_attr(input.has_custom_title),
        description_source_attr: source_attr(input.has_custom_description),
        class_source_attr: source_attr(input.has_custom_class_name),
        motion_source_attr: source_attr(input.has_custom_motion),
        exit_source_attr: source_attr(input.has_on_exit_complete),
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: ModalPartState) -> String {
    let mut classes = vec![state.base_class.into()];

    if state.slot == ModalSlot::Root {
        if state.show_description {
            classes.push("ui-modal--with-description".to_string());
        } else {
            classes.push("ui-modal--title-only".to_string());
        }

        if state.has_custom_id_base {
            classes.push("ui-modal--custom-id".to_string());
        }

        if state.has_custom_title {
            classes.push("ui-modal--custom-title".to_string());
        }

        if state.has_custom_description {
            classes.push("ui-modal--custom-description".to_string());
        }

        if state.has_custom_motion {
            classes.push("ui-modal--custom-motion".to_string());
        }

        if state.has_on_exit_complete {
            classes.push("ui-modal--custom-exit".to_string());
        }

        if state.has_custom_class_name {
            classes.push("ui-modal--custom-class".to_string());
            if let Some(base_class_name) = base_class_name {
                classes.push(base_class_name);
            }
        }
    }

    classes.join(" ")
}

#[cfg(test)]
#[path = "../test/logic.rs"]
mod tests;
