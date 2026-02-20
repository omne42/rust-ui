use super::AssetVariant;
use ui_thumbnail::ThumbnailSize;

pub const DEFAULT_FILE_LABEL: &str = "File";
pub const DEFAULT_FOLDER_LABEL: &str = "Folder";
pub const DEFAULT_CUSTOM_LABEL: &str = "Asset";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AssetStateInput {
    pub variant: AssetVariant,
    pub size: ThumbnailSize,
    pub selected: bool,
    pub focused: bool,
    pub has_custom_label: bool,
    pub has_custom_class_name: bool,
    pub has_custom_content: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AssetState {
    pub variant_attr: &'static str,
    pub size_attr: &'static str,
    pub selected: bool,
    pub focused: bool,
    pub has_custom_label: bool,
    pub has_custom_class_name: bool,
    pub has_custom_content: bool,
    pub data_state_attr: &'static str,
    pub label_source_attr: &'static str,
    pub class_source_attr: &'static str,
    pub content_source_attr: &'static str,
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.into())
    })
}

pub fn resolve_label(label: Option<String>, variant: AssetVariant) -> String {
    if let Some(label) = normalize_optional_text(label) {
        return label;
    }

    match variant {
        AssetVariant::File => DEFAULT_FILE_LABEL.into(),
        AssetVariant::Folder => DEFAULT_FOLDER_LABEL.into(),
        AssetVariant::Custom => DEFAULT_CUSTOM_LABEL.into(),
    }
}

pub fn resolve_state(input: AssetStateInput) -> AssetState {
    let data_state_attr = if input.selected {
        "selected"
    } else if input.focused {
        "focused"
    } else {
        "default"
    };

    let label_source_attr = if input.has_custom_label {
        "custom"
    } else {
        "default"
    };

    let class_source_attr = if input.has_custom_class_name {
        "custom"
    } else {
        "default"
    };

    let content_source_attr = match (input.variant, input.has_custom_content) {
        (AssetVariant::Custom, true) => "custom-slot",
        (AssetVariant::Custom, false) => "fallback-icon",
        _ => "builtin-icon",
    };

    AssetState {
        variant_attr: input.variant.as_attr(),
        size_attr: input.size.as_attr(),
        selected: input.selected,
        focused: input.focused,
        has_custom_label: input.has_custom_label,
        has_custom_class_name: input.has_custom_class_name,
        has_custom_content: input.has_custom_content,
        data_state_attr,
        label_source_attr,
        class_source_attr,
        content_source_attr,
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: AssetState) -> String {
    let mut classes = vec![
        "ui-asset".to_string(),
        format!("ui-asset--variant-{}", state.variant_attr),
        format!("ui-asset--size-{}", state.size_attr),
    ];

    if state.selected {
        classes.push("ui-asset--selected".to_string());
    }

    if state.focused {
        classes.push("ui-asset--focused".to_string());
    }

    if state.has_custom_content {
        classes.push("ui-asset--custom-content".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-asset--custom-class".to_string());
        if let Some(base_class_name) = base_class_name {
            classes.push(base_class_name);
        }
    }

    classes.join(" ")
}

#[cfg(test)]
#[path = "../test/logic.rs"]
mod tests;
