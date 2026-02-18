pub use crate::button::normalize_optional_text;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ImageStatus {
    #[default]
    Idle,
    Loading,
    Loaded,
    Error,
}

impl ImageStatus {
    pub fn as_attr(self) -> &'static str {
        match self {
            ImageStatus::Idle => "idle",
            ImageStatus::Loading => "loading",
            ImageStatus::Loaded => "loaded",
            ImageStatus::Error => "error",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ImageRadius {
    Sm,
    Md,
    #[default]
    Lg,
    Full,
}

impl ImageRadius {
    pub fn class_name(self) -> &'static str {
        match self {
            ImageRadius::Sm => "ui-image--radius-sm",
            ImageRadius::Md => "ui-image--radius-md",
            ImageRadius::Lg => "ui-image--radius-lg",
            ImageRadius::Full => "ui-image--radius-full",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            ImageRadius::Sm => "sm",
            ImageRadius::Md => "md",
            ImageRadius::Lg => "lg",
            ImageRadius::Full => "full",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ImageShadow {
    None,
    #[default]
    Sm,
    Md,
}

impl ImageShadow {
    pub fn class_name(self) -> &'static str {
        match self {
            ImageShadow::None => "ui-image--shadow-none",
            ImageShadow::Sm => "ui-image--shadow-sm",
            ImageShadow::Md => "ui-image--shadow-md",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            ImageShadow::None => "none",
            ImageShadow::Sm => "sm",
            ImageShadow::Md => "md",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ImageViewState {
    pub show_image: bool,
    pub show_fallback: bool,
    pub show_skeleton: bool,
    pub show_blurred: bool,
    pub is_loaded: bool,
    pub status: ImageStatus,
    pub status_attr: &'static str,
}

pub fn resolve_view_state(
    src: Option<&str>,
    fallback_src: Option<&str>,
    status: ImageStatus,
    disable_skeleton: bool,
    is_blurred: bool,
) -> ImageViewState {
    let has_src = src.is_some_and(|value| !value.trim().is_empty());
    let has_fallback = fallback_src.is_some_and(|value| !value.trim().is_empty());

    match status {
        ImageStatus::Idle => ImageViewState {
            show_image: has_src,
            show_fallback: !has_src && has_fallback,
            show_skeleton: has_src && !disable_skeleton,
            show_blurred: false,
            is_loaded: false,
            status,
            status_attr: status.as_attr(),
        },
        ImageStatus::Loading => ImageViewState {
            show_image: has_src,
            show_fallback: false,
            show_skeleton: has_src && !disable_skeleton,
            show_blurred: false,
            is_loaded: false,
            status,
            status_attr: status.as_attr(),
        },
        ImageStatus::Loaded => ImageViewState {
            show_image: has_src,
            show_fallback: false,
            show_skeleton: false,
            show_blurred: has_src && is_blurred,
            is_loaded: has_src,
            status,
            status_attr: status.as_attr(),
        },
        ImageStatus::Error => ImageViewState {
            show_image: false,
            show_fallback: has_fallback,
            show_skeleton: false,
            show_blurred: false,
            is_loaded: false,
            status,
            status_attr: status.as_attr(),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn status_attr_is_closed_set() {
        let expected = ["idle", "loading", "loaded", "error"];
        for status in [
            ImageStatus::Idle,
            ImageStatus::Loading,
            ImageStatus::Loaded,
            ImageStatus::Error,
        ] {
            assert!(expected.contains(&status.as_attr()));
        }
    }

    #[test]
    fn radius_and_shadow_contracts_are_stable() {
        assert_eq!(ImageRadius::Sm.class_name(), "ui-image--radius-sm");
        assert_eq!(ImageRadius::Full.class_name(), "ui-image--radius-full");
        assert_eq!(ImageRadius::Lg.as_attr(), "lg");

        assert_eq!(ImageShadow::None.class_name(), "ui-image--shadow-none");
        assert_eq!(ImageShadow::Md.class_name(), "ui-image--shadow-md");
        assert_eq!(ImageShadow::Sm.as_attr(), "sm");
    }

    #[test]
    fn shows_skeleton_while_loading() {
        let state = resolve_view_state(
            Some("https://example.com/a.png"),
            None,
            ImageStatus::Loading,
            false,
            false,
        );
        assert!(state.show_image);
        assert!(state.show_skeleton);
        assert!(!state.show_fallback);
        assert_eq!(state.status_attr, "loading");
    }

    #[test]
    fn shows_fallback_when_src_missing_or_error() {
        let state = resolve_view_state(None, Some("fallback.png"), ImageStatus::Idle, false, false);
        assert!(state.show_fallback);
        assert_eq!(state.status_attr, "idle");

        let state = resolve_view_state(
            Some("bad.png"),
            Some("fallback.png"),
            ImageStatus::Error,
            false,
            false,
        );
        assert!(state.show_fallback);
        assert!(!state.show_image);
        assert_eq!(state.status_attr, "error");
    }

    #[test]
    fn loaded_state_marks_loaded_only_for_non_empty_src() {
        let loaded = resolve_view_state(
            Some("photo.png"),
            Some("fallback.png"),
            ImageStatus::Loaded,
            false,
            true,
        );
        assert!(loaded.show_image);
        assert!(loaded.show_blurred);
        assert!(loaded.is_loaded);
        assert_eq!(loaded.status, ImageStatus::Loaded);

        let empty = resolve_view_state(
            Some("  "),
            Some("fallback.png"),
            ImageStatus::Loaded,
            false,
            true,
        );
        assert!(!empty.show_image);
        assert!(!empty.is_loaded);
    }
}
