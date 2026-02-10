use crate::drawer::{DrawerPartState, DrawerPartStateInput, DrawerSlot};

pub type DrawerPlacement = crate::sheet::SheetPlacement;

pub const DEFAULT_ID_BASE: &str = "ui-drawer";
pub const DEFAULT_TITLE: &str = "Drawer";
pub const DEFAULT_PLACEMENT: DrawerPlacement = DrawerPlacement::Right;
pub const DEFAULT_SHOW_CLOSE_BUTTON: bool = true;

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

pub fn placement_class(placement: DrawerPlacement) -> &'static str {
    match placement {
        DrawerPlacement::Bottom => "ui-drawer--placement-bottom",
        DrawerPlacement::Left => "ui-drawer--placement-left",
        DrawerPlacement::Right => "ui-drawer--placement-right",
    }
}

pub fn placement_attr(placement: DrawerPlacement) -> &'static str {
    match placement {
        DrawerPlacement::Bottom => "bottom",
        DrawerPlacement::Left => "left",
        DrawerPlacement::Right => "right",
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

pub fn resolve_state(input: DrawerPartStateInput) -> DrawerPartState {
    let has_custom_placement = input.placement != DEFAULT_PLACEMENT;

    DrawerPartState {
        slot: input.slot,
        slot_attr: input.slot.as_attr(),
        base_class: input.slot.base_class(),
        placement_attr: placement_attr(input.placement),
        placement_class: placement_class(input.placement),
        state_attr: state_attr(input.has_description),
        description_attr: description_attr(input.has_description),
        footer_attr: footer_attr(input.has_footer),
        close_button_attr: close_button_attr(input.show_close_button),
        show_description: input.has_description,
        show_footer: input.has_footer,
        show_close_button: input.show_close_button,
        has_custom_id_base: input.has_custom_id_base,
        has_custom_title: input.has_custom_title,
        has_custom_description: input.has_custom_description,
        has_custom_class_name: input.has_custom_class_name,
        has_custom_motion: input.has_custom_motion,
        has_on_exit_complete: input.has_on_exit_complete,
        placement_source_attr: source_attr(has_custom_placement),
        description_source_attr: source_attr(input.has_custom_description),
        footer_source_attr: source_attr(input.has_footer),
        close_source_attr: source_attr(input.show_close_button != DEFAULT_SHOW_CLOSE_BUTTON),
        id_source_attr: source_attr(input.has_custom_id_base),
        title_source_attr: source_attr(input.has_custom_title),
        class_source_attr: source_attr(input.has_custom_class_name),
        motion_source_attr: source_attr(input.has_custom_motion),
        exit_source_attr: source_attr(input.has_on_exit_complete),
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: DrawerPartState) -> String {
    let mut classes = vec![state.base_class.to_string()];

    if state.slot == DrawerSlot::Root {
        classes.push(state.placement_class.to_string());

        if state.show_description {
            classes.push("ui-drawer--with-description".to_string());
        } else {
            classes.push("ui-drawer--title-only".to_string());
        }

        if state.show_footer {
            classes.push("ui-drawer--with-footer".to_string());
        } else {
            classes.push("ui-drawer--no-footer".to_string());
        }

        if state.show_close_button {
            classes.push("ui-drawer--close-shown".to_string());
        } else {
            classes.push("ui-drawer--close-hidden".to_string());
        }

        if state.placement_source_attr == "custom" {
            classes.push("ui-drawer--custom-placement".to_string());
        }

        if state.has_custom_motion {
            classes.push("ui-drawer--custom-motion".to_string());
        }

        if state.has_on_exit_complete {
            classes.push("ui-drawer--custom-exit".to_string());
        }

        if state.has_custom_class_name {
            classes.push("ui-drawer--custom-class".to_string());
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
    }

    #[test]
    fn normalize_optional_text_trims_and_filters_blank_values() {
        assert_eq!(normalize_optional_text(None), None);
        assert_eq!(normalize_optional_text(Some("  \n\t".to_string())), None);
        assert_eq!(
            normalize_optional_text(Some(" docs-drawer ".to_string())),
            Some("docs-drawer".to_string())
        );
    }

    #[test]
    fn normalize_required_text_uses_fallback_for_blank_values() {
        assert_eq!(
            normalize_required_text(" Drawer ".to_string(), DEFAULT_TITLE),
            "Drawer"
        );
        assert_eq!(
            normalize_required_text(" ".to_string(), DEFAULT_TITLE),
            DEFAULT_TITLE
        );
    }

    #[test]
    fn normalize_id_base_uses_default_for_blank_values() {
        assert_eq!(
            normalize_id_base(" docs-drawer ".to_string()),
            "docs-drawer"
        );
        assert_eq!(normalize_id_base("  ".to_string()), DEFAULT_ID_BASE);
    }

    #[test]
    fn resolve_state_tracks_source_markers() {
        let state = resolve_state(DrawerPartStateInput {
            slot: DrawerSlot::Root,
            placement: DrawerPlacement::Left,
            has_description: true,
            has_footer: true,
            show_close_button: false,
            has_custom_id_base: true,
            has_custom_title: true,
            has_custom_description: true,
            has_custom_class_name: true,
            has_custom_motion: true,
            has_on_exit_complete: true,
        });

        assert_eq!(state.slot_attr, "drawer");
        assert_eq!(state.base_class, "ui-drawer");
        assert_eq!(state.placement_attr, "left");
        assert_eq!(state.state_attr, "with-description");
        assert_eq!(state.description_attr, "present");
        assert_eq!(state.footer_attr, "present");
        assert_eq!(state.close_button_attr, "hidden");
        assert_eq!(state.placement_source_attr, "custom");
        assert_eq!(state.description_source_attr, "custom");
        assert_eq!(state.footer_source_attr, "custom");
        assert_eq!(state.close_source_attr, "custom");
        assert_eq!(state.id_source_attr, "custom");
        assert_eq!(state.title_source_attr, "custom");
        assert_eq!(state.class_source_attr, "custom");
        assert_eq!(state.motion_source_attr, "custom");
        assert_eq!(state.exit_source_attr, "custom");
    }

    #[test]
    fn compose_class_name_includes_custom_markers() {
        let class_name = compose_class_name(
            Some("docs-drawer".to_string()),
            resolve_state(DrawerPartStateInput {
                slot: DrawerSlot::Root,
                placement: DrawerPlacement::Left,
                has_description: false,
                has_footer: true,
                show_close_button: false,
                has_custom_id_base: false,
                has_custom_title: false,
                has_custom_description: false,
                has_custom_class_name: true,
                has_custom_motion: true,
                has_on_exit_complete: true,
            }),
        );

        for token in [
            "ui-drawer",
            "ui-drawer--placement-left",
            "ui-drawer--title-only",
            "ui-drawer--with-footer",
            "ui-drawer--close-hidden",
            "ui-drawer--custom-placement",
            "ui-drawer--custom-motion",
            "ui-drawer--custom-exit",
            "ui-drawer--custom-class",
            "docs-drawer",
        ] {
            assert!(
                class_name.contains(token),
                "composed class name should include `{token}`"
            );
        }
    }
}
