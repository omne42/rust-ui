use super::AssetVariant;
use crate::thumbnail::ThumbnailSize;

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
mod tests {
    use super::*;

    #[test]
    fn resolve_label_uses_defaults_by_variant() {
        assert_eq!(resolve_label(None, AssetVariant::File), DEFAULT_FILE_LABEL);
        assert_eq!(
            resolve_label(Some("  ".to_string()), AssetVariant::Folder),
            DEFAULT_FOLDER_LABEL
        );
        assert_eq!(
            resolve_label(None, AssetVariant::Custom),
            DEFAULT_CUSTOM_LABEL
        );
        assert_eq!(
            resolve_label(
                Some("  Featured Artwork  ".to_string()),
                AssetVariant::Custom
            ),
            "Featured Artwork"
        );
    }

    #[test]
    fn resolve_state_tracks_sources_and_state_markers() {
        let state = resolve_state(AssetStateInput {
            variant: AssetVariant::Custom,
            size: ThumbnailSize::Size700,
            selected: true,
            focused: false,
            has_custom_label: true,
            has_custom_class_name: true,
            has_custom_content: true,
        });

        assert_eq!(state.variant_attr, "custom");
        assert_eq!(state.size_attr, "700");
        assert_eq!(state.data_state_attr, "selected");
        assert_eq!(state.label_source_attr, "custom");
        assert_eq!(state.class_source_attr, "custom");
        assert_eq!(state.content_source_attr, "custom-slot");
    }

    #[test]
    fn compose_class_name_exposes_state_markers() {
        let state = resolve_state(AssetStateInput {
            variant: AssetVariant::Folder,
            size: ThumbnailSize::Size600,
            selected: false,
            focused: true,
            has_custom_label: false,
            has_custom_class_name: true,
            has_custom_content: false,
        });

        let class_name = compose_class_name(Some("docs-asset".to_string()), state);
        for token in [
            "ui-asset",
            "ui-asset--variant-folder",
            "ui-asset--size-600",
            "ui-asset--focused",
            "ui-asset--custom-class",
            "docs-asset",
        ] {
            assert!(
                class_name.contains(token),
                "composed class name should include `{token}`"
            );
        }
    }
}
