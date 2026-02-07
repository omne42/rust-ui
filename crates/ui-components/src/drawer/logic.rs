pub type DrawerPlacement = crate::sheet::SheetPlacement;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DrawerStateInput {
    pub placement: DrawerPlacement,
    pub has_description: bool,
    pub has_footer: bool,
    pub show_close_button: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DrawerState {
    pub placement: DrawerPlacement,
    pub placement_class: &'static str,
    pub placement_attr: &'static str,
    pub show_description: bool,
    pub description_attr: &'static str,
    pub show_footer: bool,
    pub footer_attr: &'static str,
    pub show_close_button: bool,
    pub close_button_class: &'static str,
    pub close_button_attr: &'static str,
    pub state_class: &'static str,
    pub state_attr: &'static str,
    pub has_custom_class_name: bool,
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
        "ui-drawer".to_string()
    } else {
        trimmed.to_string()
    }
}

fn placement_class(placement: DrawerPlacement) -> &'static str {
    match placement {
        DrawerPlacement::Bottom => "ui-drawer--placement-bottom",
        DrawerPlacement::Left => "ui-drawer--placement-left",
        DrawerPlacement::Right => "ui-drawer--placement-right",
    }
}

fn placement_attr(placement: DrawerPlacement) -> &'static str {
    match placement {
        DrawerPlacement::Bottom => "bottom",
        DrawerPlacement::Left => "left",
        DrawerPlacement::Right => "right",
    }
}

pub fn resolve_state(input: DrawerStateInput) -> DrawerState {
    let (state_class, state_attr, description_attr) = if input.has_description {
        ("ui-drawer--with-description", "with-description", "present")
    } else {
        ("ui-drawer--title-only", "title-only", "absent")
    };

    let (close_button_class, close_button_attr) = if input.show_close_button {
        ("ui-drawer--close-shown", "shown")
    } else {
        ("ui-drawer--close-hidden", "hidden")
    };

    DrawerState {
        placement: input.placement,
        placement_class: placement_class(input.placement),
        placement_attr: placement_attr(input.placement),
        show_description: input.has_description,
        description_attr,
        show_footer: input.has_footer,
        footer_attr: if input.has_footer {
            "present"
        } else {
            "absent"
        },
        show_close_button: input.show_close_button,
        close_button_class,
        close_button_attr,
        state_class,
        state_attr,
        has_custom_class_name: input.has_custom_class_name,
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: DrawerState) -> String {
    let mut classes = vec![
        "ui-drawer".to_string(),
        state.placement_class.to_string(),
        state.state_class.to_string(),
        state.close_button_class.to_string(),
    ];

    if state.show_footer {
        classes.push("ui-drawer--with-footer".to_string());
    } else {
        classes.push("ui-drawer--no-footer".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-drawer--custom-class".to_string());
    }

    if state.has_custom_class_name
        && let Some(base_class_name) = base_class_name
    {
        classes.push(base_class_name);
    }

    classes.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

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
            normalize_required_text(" Drawer ".to_string(), "Drawer"),
            "Drawer"
        );
        assert_eq!(normalize_required_text(" ".to_string(), "Drawer"), "Drawer");
    }

    #[test]
    fn normalize_id_base_uses_default_for_blank_values() {
        assert_eq!(
            normalize_id_base(" docs-drawer ".to_string()),
            "docs-drawer"
        );
        assert_eq!(normalize_id_base("  ".to_string()), "ui-drawer");
    }

    #[test]
    fn resolve_state_tracks_flags_and_placement() {
        let state = resolve_state(DrawerStateInput {
            placement: DrawerPlacement::Left,
            has_description: true,
            has_footer: false,
            show_close_button: false,
            has_custom_class_name: true,
        });

        assert_eq!(state.placement, DrawerPlacement::Left);
        assert_eq!(state.placement_class, "ui-drawer--placement-left");
        assert_eq!(state.placement_attr, "left");

        assert!(state.show_description);
        assert_eq!(state.state_class, "ui-drawer--with-description");
        assert_eq!(state.state_attr, "with-description");
        assert_eq!(state.description_attr, "present");

        assert!(!state.show_footer);
        assert_eq!(state.footer_attr, "absent");

        assert!(!state.show_close_button);
        assert_eq!(state.close_button_class, "ui-drawer--close-hidden");
        assert_eq!(state.close_button_attr, "hidden");

        assert!(state.has_custom_class_name);
    }

    #[test]
    fn compose_class_name_includes_state_markers() {
        let class_name = compose_class_name(
            Some("docs-drawer".to_string()),
            resolve_state(DrawerStateInput {
                placement: DrawerPlacement::Right,
                has_description: false,
                has_footer: true,
                show_close_button: true,
                has_custom_class_name: true,
            }),
        );

        for token in [
            "ui-drawer",
            "ui-drawer--placement-right",
            "ui-drawer--title-only",
            "ui-drawer--close-shown",
            "ui-drawer--with-footer",
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
