use crate::icons::{IconsSet, IconsState, IconsStateInput};

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.into())
    })
}

pub fn parse_set_from_name(name: &str) -> Option<IconsSet> {
    let (prefix, _) = name.trim().split_once(':')?;

    match prefix {
        "workflow" => Some(IconsSet::Workflow),
        "ui" => Some(IconsSet::Ui),
        _ => None,
    }
}

pub fn resolve_set(name: &str, set: IconsSet) -> (IconsSet, bool) {
    let parsed = parse_set_from_name(name);
    (parsed.unwrap_or(set), parsed.is_some())
}

pub fn normalize_name(name: String, set: IconsSet) -> String {
    let trimmed = name.trim();

    if trimmed.is_empty() {
        return format!("{}:help", set.as_attr());
    }

    if trimmed.contains(':') {
        return trimmed.into();
    }

    format!("{}:{trimmed}", set.as_attr())
}

pub fn resolve_state(input: IconsStateInput) -> IconsState {
    IconsState {
        set: input.set,
        scale: input.scale,
        is_disabled: input.disabled,
        is_decorative: input.decorative,
        has_set_prefix_in_name: input.has_set_prefix_in_name,
        has_custom_set_prop: input.has_custom_set_prop,
        has_custom_aria_label: input.has_custom_aria_label,
        has_custom_class_name: input.has_custom_class_name,
        has_custom_glyphs: input.has_custom_glyphs,
        has_custom_tone: input.has_custom_tone,
        set_attr: input.set.as_attr(),
        scale_attr: input.scale.as_attr(),
        state_attr: if input.disabled {
            "disabled"
        } else if input.decorative {
            "decorative"
        } else {
            "ready"
        },
        set_source_attr: if input.has_set_prefix_in_name {
            "name"
        } else if input.has_custom_set_prop {
            "prop"
        } else {
            "default"
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
        tone_source_attr: if input.has_custom_tone {
            "custom"
        } else {
            "default"
        },
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: IconsState) -> String {
    let mut classes = vec![
        "ui-icons".to_string(),
        format!("ui-icons--set-{}", state.set_attr),
    ];

    classes.push(format!("ui-icons--scale-{}", state.scale_attr));

    if state.is_disabled {
        classes.push("ui-icons--disabled".to_string());
    }

    if state.is_decorative {
        classes.push("ui-icons--decorative".to_string());
    }

    if state.has_custom_glyphs {
        classes.push("ui-icons--custom-glyphs".to_string());
    }

    if state.has_custom_tone {
        classes.push("ui-icons--custom-tone".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-icons--custom-class".to_string());
        if let Some(base_class_name) = base_class_name {
            classes.push(base_class_name);
        }
    }

    classes.join(" ")
}

#[cfg(test)]
#[path = "../../test/icons/logic.rs"]
mod tests;
