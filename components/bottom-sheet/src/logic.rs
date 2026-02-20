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
        "ui-bottom-sheet".to_string()
    } else {
        trimmed.into()
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
        state.state_class.into(),
        state.handle_class.into(),
        state.close_button_class.into(),
        state.detached_class.into(),
        state.inset_class.into(),
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
#[path = "../test/logic.rs"]
mod tests;
