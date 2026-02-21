use std::borrow::Cow;

use crate::icons_workflow::{IconsWorkflowState, IconsWorkflowStateInput, IconsetGlyph};

pub const DEFAULT_INNER_CLASS: &str = "ui-icons-workflow";

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.into())
    })
}

pub fn resolve_inner_aria_label(value: Option<String>) -> String {
    value.unwrap_or_default()
}

pub fn resolve_inner_class_name(value: Option<String>) -> String {
    value
        .map(|value| format!("{DEFAULT_INNER_CLASS} {value}"))
        .unwrap_or_else(|| DEFAULT_INNER_CLASS.into())
}

pub fn normalize_icon_reference(icon: String) -> (String, &'static str, bool, bool) {
    let icon = icon.trim();

    if icon.is_empty() {
        return ("workflow:help".into(), "default", false, true);
    }

    if icon.contains(':') {
        return (icon.into(), "explicit", true, false);
    }

    (format!("workflow:{icon}"), "prefixed", false, false)
}

pub fn default_workflow_glyphs() -> Vec<IconsetGlyph> {
    vec![
        IconsetGlyph::new("workflow:help", "?").with_aria_label("Workflow Help"),
        IconsetGlyph::new("workflow:success", "✓").with_aria_label("Workflow Success"),
        IconsetGlyph::new("workflow:warning", "⚠").with_aria_label("Workflow Warning"),
        IconsetGlyph::new("workflow:info", "ℹ").with_aria_label("Workflow Info"),
        IconsetGlyph::new("workflow:settings", "⚙").with_aria_label("Workflow Settings"),
    ]
}

pub fn resolve_state(input: IconsWorkflowStateInput) -> IconsWorkflowState {
    IconsWorkflowState {
        is_disabled: input.disabled,
        is_decorative: input.decorative,
        has_explicit_icon_reference: input.has_explicit_icon_reference,
        used_default_icon_reference: input.used_default_icon_reference,
        has_custom_aria_label: input.has_custom_aria_label,
        has_custom_class_name: input.has_custom_class_name,
        has_custom_glyphs: input.has_custom_glyphs,
        has_custom_size: input.has_custom_size,
        has_custom_tone: input.has_custom_tone,
        state_attr: if input.disabled {
            "disabled"
        } else if input.decorative {
            "decorative"
        } else {
            "ready"
        },
        icon_reference_source_attr: if input.used_default_icon_reference {
            "default"
        } else if input.has_explicit_icon_reference {
            "explicit"
        } else {
            "prefixed"
        },
        aria_source_attr: if input.has_custom_aria_label {
            "custom"
        } else {
            "default"
        },
        class_source_attr: if input.has_custom_class_name {
            "custom"
        } else {
            "default"
        },
        glyph_source_attr: if input.has_custom_glyphs {
            "custom"
        } else {
            "default"
        },
        size_source_attr: if input.has_custom_size {
            "custom"
        } else {
            "default"
        },
        tone_source_attr: if input.has_custom_tone {
            "custom"
        } else {
            "default"
        },
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: IconsWorkflowState) -> String {
    let mut classes: Vec<Cow<'static, str>> = vec![Cow::Borrowed("ui-icons-workflow")];

    if state.is_disabled {
        classes.push(Cow::Borrowed("ui-icons-workflow--disabled"));
    }

    if state.is_decorative {
        classes.push(Cow::Borrowed("ui-icons-workflow--decorative"));
    }

    if state.has_custom_glyphs {
        classes.push(Cow::Borrowed("ui-icons-workflow--custom-glyphs"));
    }

    if state.has_custom_size {
        classes.push(Cow::Borrowed("ui-icons-workflow--custom-size"));
    }

    if state.has_custom_tone {
        classes.push(Cow::Borrowed("ui-icons-workflow--custom-tone"));
    }

    if state.has_custom_class_name {
        classes.push(Cow::Borrowed("ui-icons-workflow--custom-class"));
        if let Some(base_class_name) = base_class_name {
            classes.push(Cow::Owned(base_class_name));
        }
    }

    classes
        .iter()
        .map(Cow::as_ref)
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
#[path = "../../test/workflow/logic.rs"]
mod tests;
