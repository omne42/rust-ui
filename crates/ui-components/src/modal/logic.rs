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
mod tests {
    use super::*;

    #[test]
    fn state_and_description_attrs_follow_contract() {
        assert_eq!(state_attr(true), "with-description");
        assert_eq!(state_attr(false), "title-only");
        assert_eq!(description_attr(true), "present");
        assert_eq!(description_attr(false), "absent");
    }

    #[test]
    fn normalize_optional_text_trims_and_filters_blank_values() {
        assert_eq!(normalize_optional_text(None), None);
        assert_eq!(normalize_optional_text(Some("  \n\t".to_string())), None);
        assert_eq!(
            normalize_optional_text(Some(" docs-modal ".to_string())),
            Some("docs-modal".to_string())
        );
    }

    #[test]
    fn normalize_required_text_falls_back_for_blank_values() {
        assert_eq!(
            normalize_required_text(" Confirm ".to_string(), DEFAULT_TITLE),
            "Confirm"
        );
        assert_eq!(
            normalize_required_text(" ".to_string(), DEFAULT_TITLE),
            DEFAULT_TITLE
        );
    }

    #[test]
    fn normalize_id_base_uses_default_for_blank_values() {
        assert_eq!(normalize_id_base(" docs-modal ".to_string()), "docs-modal");
        assert_eq!(normalize_id_base("  ".to_string()), DEFAULT_ID_BASE);
    }

    #[test]
    fn resolve_state_tracks_source_markers() {
        let state = resolve_state(ModalPartStateInput {
            slot: ModalSlot::Root,
            has_description: true,
            has_custom_id_base: true,
            has_custom_title: true,
            has_custom_description: true,
            has_custom_class_name: true,
            has_custom_motion: true,
            has_on_exit_complete: true,
        });

        assert_eq!(state.slot_attr, "modal");
        assert_eq!(state.base_class, "ui-modal");
        assert_eq!(state.state_attr, "with-description");
        assert_eq!(state.description_attr, "present");
        assert_eq!(state.id_source_attr, "custom");
        assert_eq!(state.title_source_attr, "custom");
        assert_eq!(state.description_source_attr, "custom");
        assert_eq!(state.class_source_attr, "custom");
        assert_eq!(state.motion_source_attr, "custom");
        assert_eq!(state.exit_source_attr, "custom");
    }

    #[test]
    fn compose_class_name_includes_custom_markers() {
        let class_name = compose_class_name(
            Some("docs-modal".to_string()),
            resolve_state(ModalPartStateInput {
                slot: ModalSlot::Root,
                has_description: true,
                has_custom_id_base: true,
                has_custom_title: true,
                has_custom_description: true,
                has_custom_class_name: true,
                has_custom_motion: true,
                has_on_exit_complete: true,
            }),
        );

        for token in [
            "ui-modal",
            "ui-modal--with-description",
            "ui-modal--custom-id",
            "ui-modal--custom-title",
            "ui-modal--custom-description",
            "ui-modal--custom-motion",
            "ui-modal--custom-exit",
            "ui-modal--custom-class",
            "docs-modal",
        ] {
            assert!(
                class_name.contains(token),
                "composed class name should include `{token}`"
            );
        }
    }
}
