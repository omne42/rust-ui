use crate::bottom_sheet::{BottomSheetState, BottomSheetStateInput};

pub const DEFAULT_CLOSE_LABEL: &str = "Close bottom sheet";

#[cfg(test)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BottomSheetAgentContract {
    pub schema_attr: &'static str,
    pub intent_attr: &'static str,
    pub action_attr: &'static str,
    pub state_axis_attr: &'static str,
    pub source_axis_attr: &'static str,
    pub render_mode_attr: &'static str,
    pub streaming_attr: &'static str,
    pub fallback_attr: &'static str,
    pub output_status_attr: &'static str,
}

#[cfg(test)]
pub fn agent_contract() -> BottomSheetAgentContract {
    BottomSheetAgentContract {
        schema_attr: "bottom-sheet.v1",
        intent_attr: "overlay",
        action_attr: "dismiss",
        state_axis_attr: "visibility|description|footer|detached|inset",
        source_axis_attr: "default|custom",
        render_mode_attr: "snapshot",
        streaming_attr: "optional",
        fallback_attr: "snapshot",
        output_status_attr: "verified",
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
        "ui-bottom-sheet".to_string()
    } else {
        trimmed.to_string()
    }
}

pub fn normalize_bottom_inset_px(value: f64) -> f64 {
    if !value.is_finite() {
        return 0.0;
    }

    value.clamp(0.0, 240.0)
}

fn inset_bucket(detached: bool, bottom_inset_px: f64) -> (&'static str, &'static str) {
    if !detached || bottom_inset_px < 4.0 {
        return ("ui-bottom-sheet--inset-none", "none");
    }

    if bottom_inset_px < 12.0 {
        ("ui-bottom-sheet--inset-sm", "sm")
    } else if bottom_inset_px < 20.0 {
        ("ui-bottom-sheet--inset-md", "md")
    } else if bottom_inset_px < 28.0 {
        ("ui-bottom-sheet--inset-lg", "lg")
    } else {
        ("ui-bottom-sheet--inset-xl", "xl")
    }
}

pub fn resolve_state(input: BottomSheetStateInput) -> BottomSheetState {
    let (state_class, state_attr, description_attr) = if input.has_description {
        (
            "ui-bottom-sheet--with-description",
            "with-description",
            "present",
        )
    } else {
        ("ui-bottom-sheet--title-only", "title-only", "absent")
    };

    let (handle_class, handle_attr) = if input.show_handle {
        ("ui-bottom-sheet--handle-shown", "shown")
    } else {
        ("ui-bottom-sheet--handle-hidden", "hidden")
    };

    let (close_button_class, close_button_attr) = if input.show_close_button {
        ("ui-bottom-sheet--close-shown", "shown")
    } else {
        ("ui-bottom-sheet--close-hidden", "hidden")
    };

    let (detached_class, detached_attr) = if input.detached {
        ("ui-bottom-sheet--detached", "true")
    } else {
        ("ui-bottom-sheet--attached", "false")
    };

    let (inset_class, inset_attr) = inset_bucket(input.detached, input.bottom_inset_px);

    BottomSheetState {
        show_description: input.has_description,
        description_attr,
        show_footer: input.has_footer,
        footer_attr: if input.has_footer {
            "present"
        } else {
            "absent"
        },
        show_handle: input.show_handle,
        handle_class,
        handle_attr,
        show_close_button: input.show_close_button,
        close_button_class,
        close_button_attr,
        detached: input.detached,
        detached_class,
        detached_attr,
        inset_class,
        inset_attr,
        state_class,
        state_attr,
        class_source_attr: if input.has_custom_class_name {
            "custom"
        } else {
            "default"
        },
        has_custom_class_name: input.has_custom_class_name,
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: BottomSheetState) -> String {
    let mut classes = vec![
        "ui-bottom-sheet".to_string(),
        state.state_class.to_string(),
        state.handle_class.to_string(),
        state.close_button_class.to_string(),
        state.detached_class.to_string(),
        state.inset_class.to_string(),
    ];

    if state.show_footer {
        classes.push("ui-bottom-sheet--with-footer".to_string());
    } else {
        classes.push("ui-bottom-sheet--no-footer".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-bottom-sheet--custom-class".to_string());
        if let Some(base_class_name) = base_class_name {
            classes.push(base_class_name);
        }
    }

    classes.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_helpers_trim_and_fallback() {
        assert_eq!(normalize_optional_text(None), None);
        assert_eq!(normalize_optional_text(Some("  ".to_string())), None);
        assert_eq!(
            normalize_optional_text(Some(" docs-bottom-sheet ".to_string())),
            Some("docs-bottom-sheet".to_string())
        );

        assert_eq!(
            normalize_required_text(" Bottom sheet ".to_string(), "Bottom sheet"),
            "Bottom sheet"
        );
        assert_eq!(
            normalize_required_text(" ".to_string(), "Bottom sheet"),
            "Bottom sheet"
        );

        assert_eq!(
            normalize_id_base(" docs-bottom-sheet ".to_string()),
            "docs-bottom-sheet"
        );
        assert_eq!(normalize_id_base(" ".to_string()), "ui-bottom-sheet");

        assert_eq!(normalize_bottom_inset_px(-12.0), 0.0);
        assert_eq!(normalize_bottom_inset_px(999.0), 240.0);
        assert_eq!(normalize_bottom_inset_px(18.5), 18.5);
    }

    #[test]
    fn resolve_state_tracks_description_footer_handle_close_detached_and_inset() {
        let state = resolve_state(BottomSheetStateInput {
            has_description: true,
            has_footer: false,
            show_handle: true,
            show_close_button: false,
            detached: true,
            bottom_inset_px: 17.0,
            has_custom_class_name: true,
        });

        assert!(state.show_description);
        assert_eq!(state.state_class, "ui-bottom-sheet--with-description");
        assert_eq!(state.state_attr, "with-description");
        assert_eq!(state.description_attr, "present");

        assert!(!state.show_footer);
        assert_eq!(state.footer_attr, "absent");

        assert!(state.show_handle);
        assert_eq!(state.handle_class, "ui-bottom-sheet--handle-shown");
        assert_eq!(state.handle_attr, "shown");

        assert!(!state.show_close_button);
        assert_eq!(state.close_button_class, "ui-bottom-sheet--close-hidden");
        assert_eq!(state.close_button_attr, "hidden");

        assert!(state.detached);
        assert_eq!(state.detached_class, "ui-bottom-sheet--detached");
        assert_eq!(state.detached_attr, "true");
        assert_eq!(state.inset_class, "ui-bottom-sheet--inset-md");
        assert_eq!(state.inset_attr, "md");
        assert_eq!(state.class_source_attr, "custom");
    }

    #[test]
    fn resolve_state_maps_attached_to_no_inset() {
        let state = resolve_state(BottomSheetStateInput {
            has_description: false,
            has_footer: false,
            show_handle: false,
            show_close_button: true,
            detached: false,
            bottom_inset_px: 240.0,
            has_custom_class_name: false,
        });

        assert_eq!(state.inset_class, "ui-bottom-sheet--inset-none");
        assert_eq!(state.inset_attr, "none");
    }

    #[test]
    fn compose_class_name_includes_state_markers() {
        let state = resolve_state(BottomSheetStateInput {
            has_description: false,
            has_footer: true,
            show_handle: false,
            show_close_button: true,
            detached: false,
            bottom_inset_px: 24.0,
            has_custom_class_name: true,
        });

        let class_name = compose_class_name(Some("docs-bottom-sheet".to_string()), state);

        for token in [
            "ui-bottom-sheet",
            "ui-bottom-sheet--title-only",
            "ui-bottom-sheet--with-footer",
            "ui-bottom-sheet--handle-hidden",
            "ui-bottom-sheet--close-shown",
            "ui-bottom-sheet--attached",
            "ui-bottom-sheet--inset-none",
            "ui-bottom-sheet--custom-class",
            "docs-bottom-sheet",
        ] {
            assert!(
                class_name.contains(token),
                "composed class name should include `{token}`"
            );
        }
    }

    #[test]
    fn agent_contract_is_stable_and_machine_readable() {
        let contract = agent_contract();

        assert_eq!(contract.schema_attr, "bottom-sheet.v1");
        assert_eq!(contract.intent_attr, "overlay");
        assert_eq!(contract.action_attr, "dismiss");
        assert_eq!(
            contract.state_axis_attr,
            "visibility|description|footer|detached|inset"
        );
        assert_eq!(contract.source_axis_attr, "default|custom");
        assert_eq!(contract.render_mode_attr, "snapshot");
        assert_eq!(contract.streaming_attr, "optional");
        assert_eq!(contract.fallback_attr, "snapshot");
        assert_eq!(contract.output_status_attr, "verified");
    }
}
