#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ThumbnailSize {
    Size50,
    Size75,
    Size100,
    Size200,
    Size300,
    Size400,
    #[default]
    Size500,
    Size600,
    Size700,
    Size800,
    Size900,
    Size1000,
}

impl ThumbnailSize {
    pub fn class_name(self) -> &'static str {
        match self {
            ThumbnailSize::Size50 => "ui-thumbnail--size-50",
            ThumbnailSize::Size75 => "ui-thumbnail--size-75",
            ThumbnailSize::Size100 => "ui-thumbnail--size-100",
            ThumbnailSize::Size200 => "ui-thumbnail--size-200",
            ThumbnailSize::Size300 => "ui-thumbnail--size-300",
            ThumbnailSize::Size400 => "ui-thumbnail--size-400",
            ThumbnailSize::Size500 => "ui-thumbnail--size-500",
            ThumbnailSize::Size600 => "ui-thumbnail--size-600",
            ThumbnailSize::Size700 => "ui-thumbnail--size-700",
            ThumbnailSize::Size800 => "ui-thumbnail--size-800",
            ThumbnailSize::Size900 => "ui-thumbnail--size-900",
            ThumbnailSize::Size1000 => "ui-thumbnail--size-1000",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            ThumbnailSize::Size50 => "50",
            ThumbnailSize::Size75 => "75",
            ThumbnailSize::Size100 => "100",
            ThumbnailSize::Size200 => "200",
            ThumbnailSize::Size300 => "300",
            ThumbnailSize::Size400 => "400",
            ThumbnailSize::Size500 => "500",
            ThumbnailSize::Size600 => "600",
            ThumbnailSize::Size700 => "700",
            ThumbnailSize::Size800 => "800",
            ThumbnailSize::Size900 => "900",
            ThumbnailSize::Size1000 => "1000",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ThumbnailStateInput {
    pub size: ThumbnailSize,
    pub cover: bool,
    pub layer: bool,
    pub selected: bool,
    pub focused: bool,
    pub has_background: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ThumbnailState {
    pub size: ThumbnailSize,
    pub size_class: &'static str,
    pub size_attr: &'static str,
    pub cover: bool,
    pub layer: bool,
    pub selected: bool,
    pub focused: bool,
    pub has_background: bool,
    pub has_custom_class_name: bool,
    pub data_state_attr: &'static str,
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.to_string())
    })
}

fn is_allowed_background_char(ch: char) -> bool {
    matches!(
        ch,
        '#' | '(' | ')' | ',' | '.' | '%' | '-' | '/' | ' ' | '[' | ']' | '_' | 'a'..='z' | 'A'..='Z' | '0'..='9'
    )
}

pub fn sanitize_background(value: Option<String>) -> Option<String> {
    let value = normalize_optional_text(value)?;
    if value.len() > 96 {
        return None;
    }
    if value.chars().all(is_allowed_background_char) {
        Some(value)
    } else {
        None
    }
}

pub fn resolve_state(input: ThumbnailStateInput) -> ThumbnailState {
    let data_state_attr = if input.selected {
        "selected"
    } else if input.focused {
        "focused"
    } else if input.layer {
        "layer"
    } else {
        "default"
    };

    ThumbnailState {
        size: input.size,
        size_class: input.size.class_name(),
        size_attr: input.size.as_attr(),
        cover: input.cover,
        layer: input.layer,
        selected: input.selected,
        focused: input.focused,
        has_background: input.has_background,
        has_custom_class_name: input.has_custom_class_name,
        data_state_attr,
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: ThumbnailState) -> String {
    let mut classes = vec!["ui-thumbnail".to_string(), state.size_class.to_string()];

    if state.cover {
        classes.push("ui-thumbnail--cover".to_string());
    }

    if state.layer {
        classes.push("ui-thumbnail--layer".to_string());
    }

    if state.selected {
        classes.push("ui-thumbnail--selected".to_string());
    }

    if state.focused {
        classes.push("ui-thumbnail--focused".to_string());
    }

    if state.has_background {
        classes.push("ui-thumbnail--background".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-thumbnail--custom-class".to_string());
        if let Some(base_class_name) = base_class_name {
            classes.push(base_class_name);
        }
    }

    classes.join(" ")
}

pub fn compose_inline_style(background: Option<&str>) -> Option<String> {
    background.map(|background| format!("--ui-thumbnail-background: {background};"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn size_contracts_are_stable() {
        assert_eq!(ThumbnailSize::Size50.class_name(), "ui-thumbnail--size-50");
        assert_eq!(ThumbnailSize::Size500.as_attr(), "500");
        assert_eq!(ThumbnailSize::Size1000.as_attr(), "1000");
    }

    #[test]
    fn sanitize_background_rejects_invalid_content() {
        assert_eq!(
            sanitize_background(Some("  #ff0000  ".to_string())),
            Some("#ff0000".to_string())
        );
        assert_eq!(
            sanitize_background(Some("javascript:alert(1)".to_string())),
            None
        );
    }

    #[test]
    fn state_and_class_composition_track_markers() {
        let state = resolve_state(ThumbnailStateInput {
            size: ThumbnailSize::Size600,
            cover: true,
            layer: true,
            selected: true,
            focused: false,
            has_background: true,
            has_custom_class_name: true,
        });

        assert_eq!(state.data_state_attr, "selected");

        let class_name = compose_class_name(Some("docs-thumbnail".to_string()), state);
        for token in [
            "ui-thumbnail",
            "ui-thumbnail--size-600",
            "ui-thumbnail--cover",
            "ui-thumbnail--layer",
            "ui-thumbnail--selected",
            "ui-thumbnail--background",
            "ui-thumbnail--custom-class",
            "docs-thumbnail",
        ] {
            assert!(
                class_name.contains(token),
                "composed class name should include `{token}`"
            );
        }

        assert_eq!(
            compose_inline_style(Some("#111827")),
            Some("--ui-thumbnail-background: #111827;".to_string())
        );
    }
}
