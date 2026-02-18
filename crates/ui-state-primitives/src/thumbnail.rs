pub use crate::button::normalize_optional_text;

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
pub enum ThumbnailDataState {
    Default,
    Layer,
    Focused,
    Selected,
}

impl ThumbnailDataState {
    pub fn as_attr(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Layer => "layer",
            Self::Focused => "focused",
            Self::Selected => "selected",
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
    pub data_state: ThumbnailDataState,
}

fn is_allowed_background_char(ch: char) -> bool {
    matches!(
        ch,
        '#' | '('
            | ')'
            | ','
            | '.'
            | '%'
            | '-'
            | '/'
            | ' '
            | '['
            | ']'
            | '_'
            | 'a'..='z'
            | 'A'..='Z'
            | '0'..='9'
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
    let data_state = if input.selected {
        ThumbnailDataState::Selected
    } else if input.focused {
        ThumbnailDataState::Focused
    } else if input.layer {
        ThumbnailDataState::Layer
    } else {
        ThumbnailDataState::Default
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
        data_state,
    }
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
        assert_eq!(sanitize_background(Some(" ".to_string())), None);
    }

    #[test]
    fn resolve_state_tracks_priority_and_flags() {
        let selected = resolve_state(ThumbnailStateInput {
            size: ThumbnailSize::Size600,
            cover: true,
            layer: true,
            selected: true,
            focused: true,
            has_background: true,
            has_custom_class_name: true,
        });
        assert_eq!(selected.data_state, ThumbnailDataState::Selected);
        assert_eq!(selected.data_state.as_attr(), "selected");
        assert_eq!(selected.size_class, "ui-thumbnail--size-600");
        assert_eq!(selected.size_attr, "600");
        assert!(selected.cover);
        assert!(selected.layer);
        assert!(selected.selected);
        assert!(selected.focused);
        assert!(selected.has_background);
        assert!(selected.has_custom_class_name);

        let focused = resolve_state(ThumbnailStateInput {
            size: ThumbnailSize::Size500,
            cover: false,
            layer: true,
            selected: false,
            focused: true,
            has_background: false,
            has_custom_class_name: false,
        });
        assert_eq!(focused.data_state, ThumbnailDataState::Focused);
        assert_eq!(focused.data_state.as_attr(), "focused");

        let layer = resolve_state(ThumbnailStateInput {
            size: ThumbnailSize::Size500,
            cover: false,
            layer: true,
            selected: false,
            focused: false,
            has_background: false,
            has_custom_class_name: false,
        });
        assert_eq!(layer.data_state, ThumbnailDataState::Layer);
        assert_eq!(layer.data_state.as_attr(), "layer");

        let default_state = resolve_state(ThumbnailStateInput {
            size: ThumbnailSize::Size500,
            cover: false,
            layer: false,
            selected: false,
            focused: false,
            has_background: false,
            has_custom_class_name: false,
        });
        assert_eq!(default_state.data_state, ThumbnailDataState::Default);
        assert_eq!(default_state.data_state.as_attr(), "default");
    }
}
