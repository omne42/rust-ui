#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ImageStatus {
    #[default]
    Idle,
    Loading,
    Loaded,
    Error,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct ImageViewState {
    pub show_image: bool,
    pub show_fallback: bool,
    pub show_skeleton: bool,
    pub show_blurred: bool,
    pub is_loaded: bool,
}

pub fn resolve_view_state(
    src: Option<&str>,
    fallback_src: Option<&str>,
    status: ImageStatus,
    disable_skeleton: bool,
    is_blurred: bool,
) -> ImageViewState {
    let has_src = src.is_some_and(|v| !v.trim().is_empty());
    let has_fallback = fallback_src.is_some_and(|v| !v.trim().is_empty());

    match status {
        ImageStatus::Idle => ImageViewState {
            show_image: has_src,
            show_fallback: !has_src && has_fallback,
            show_skeleton: has_src && !disable_skeleton,
            show_blurred: false,
            is_loaded: false,
        },
        ImageStatus::Loading => ImageViewState {
            show_image: has_src,
            show_fallback: false,
            show_skeleton: has_src && !disable_skeleton,
            show_blurred: false,
            is_loaded: false,
        },
        ImageStatus::Loaded => ImageViewState {
            show_image: has_src,
            show_fallback: false,
            show_skeleton: false,
            show_blurred: has_src && is_blurred,
            is_loaded: true,
        },
        ImageStatus::Error => ImageViewState {
            show_image: false,
            show_fallback: has_fallback,
            show_skeleton: false,
            show_blurred: false,
            is_loaded: false,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
    }

    #[test]
    fn shows_fallback_when_src_missing_or_error() {
        let state = resolve_view_state(None, Some("fallback.png"), ImageStatus::Idle, false, false);
        assert!(state.show_fallback);

        let state = resolve_view_state(
            Some("bad.png"),
            Some("fallback.png"),
            ImageStatus::Error,
            false,
            false,
        );
        assert!(state.show_fallback);
        assert!(!state.show_image);
    }
}
