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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ImageStatusEvent {
    LoadStarted,
    LoadSucceeded,
    LoadFailed,
    SourceCleared,
}

pub fn derive_initial_status(src: Option<&str>) -> ImageStatus {
    if src.is_some_and(|value| !value.trim().is_empty()) {
        ImageStatus::Loading
    } else {
        ImageStatus::Idle
    }
}

pub fn reduce_status(status: ImageStatus, event: ImageStatusEvent) -> ImageStatus {
    match (status, event) {
        (_, ImageStatusEvent::SourceCleared) => ImageStatus::Idle,
        (_, ImageStatusEvent::LoadStarted) => ImageStatus::Loading,
        (ImageStatus::Loading, ImageStatusEvent::LoadSucceeded) => ImageStatus::Loaded,
        (ImageStatus::Loaded, ImageStatusEvent::LoadSucceeded) => ImageStatus::Loaded,
        (ImageStatus::Idle | ImageStatus::Error, ImageStatusEvent::LoadSucceeded) => {
            ImageStatus::Loaded
        }
        (_, ImageStatusEvent::LoadFailed) => ImageStatus::Error,
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
#[path = "test/image.rs"]
mod tests;
