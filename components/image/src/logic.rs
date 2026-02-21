use std::borrow::Cow;

pub use ui_state_primitives::image::{
    ImageRadius, ImageShadow, ImageStatus, ImageStatusEvent, ImageViewState, derive_initial_status,
    normalize_optional_text, reduce_status, resolve_view_state,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageMotionSource {
    Default,
    Custom,
}

impl ImageMotionSource {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Custom => "custom",
        }
    }

    pub const fn is_custom(self) -> bool {
        matches!(self, Self::Custom)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageStatusSource {
    Initial,
    Event,
}

impl ImageStatusSource {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::Initial => "initial",
            Self::Event => "event",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImageNormalizeInput {
    pub src: Option<String>,
    pub fallback_src: Option<String>,
    pub class_name: Option<String>,
    pub lang: Option<String>,
    pub radius: ImageRadius,
    pub shadow: ImageShadow,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImageNormalizedProps {
    pub src: Option<String>,
    pub src_attr: String,
    pub fallback_src: Option<String>,
    pub fallback_src_attr: String,
    pub class_name: String,
    pub lang: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImageViewStateInput {
    pub src: Option<String>,
    pub fallback_src: Option<String>,
    pub status: ImageStatus,
    pub is_skeleton_disabled: bool,
    pub is_blurred: bool,
}

pub fn normalize_props(input: ImageNormalizeInput) -> ImageNormalizedProps {
    let src = normalize_optional_text(input.src);
    let fallback_src = normalize_optional_text(input.fallback_src);
    let custom_class_name = normalize_optional_text(input.class_name);
    let lang = normalize_optional_text(input.lang);

    ImageNormalizedProps {
        src: src.clone(),
        src_attr: src.unwrap_or_default(),
        fallback_src: fallback_src.clone(),
        fallback_src_attr: fallback_src.unwrap_or_default(),
        class_name: compose_class_name(input.radius, input.shadow, custom_class_name),
        lang,
    }
}

pub fn derive_view_state(input: ImageViewStateInput) -> ImageViewState {
    resolve_view_state(
        input.src.as_deref(),
        input.fallback_src.as_deref(),
        input.status,
        input.is_skeleton_disabled,
        input.is_blurred,
    )
}

pub fn apply_status_event(status: ImageStatus, event: ImageStatusEvent) -> ImageStatus {
    reduce_status(status, event)
}

pub fn resolve_motion_source(motion: crate::motion::ImageMotion) -> ImageMotionSource {
    if motion == crate::motion::ImageMotion::default() {
        ImageMotionSource::Default
    } else {
        ImageMotionSource::Custom
    }
}

pub fn compose_class_name(
    radius: ImageRadius,
    shadow: ImageShadow,
    class_name: Option<String>,
) -> String {
    let base_class = compose_base_class(radius, shadow);

    match class_name {
        Some(class_name) => {
            let mut composed = String::with_capacity(base_class.len() + 1 + class_name.len());
            composed.push_str(base_class.as_ref());
            composed.push(' ');
            composed.push_str(class_name.as_str());
            composed
        }
        None => base_class.into_owned(),
    }
}

fn compose_base_class(radius: ImageRadius, shadow: ImageShadow) -> Cow<'static, str> {
    match (radius, shadow) {
        (ImageRadius::Sm, ImageShadow::None) => {
            Cow::Borrowed("ui-image ui-image--radius-sm ui-image--shadow-none")
        }
        (ImageRadius::Sm, ImageShadow::Sm) => {
            Cow::Borrowed("ui-image ui-image--radius-sm ui-image--shadow-sm")
        }
        (ImageRadius::Sm, ImageShadow::Md) => {
            Cow::Borrowed("ui-image ui-image--radius-sm ui-image--shadow-md")
        }
        (ImageRadius::Md, ImageShadow::None) => {
            Cow::Borrowed("ui-image ui-image--radius-md ui-image--shadow-none")
        }
        (ImageRadius::Md, ImageShadow::Sm) => {
            Cow::Borrowed("ui-image ui-image--radius-md ui-image--shadow-sm")
        }
        (ImageRadius::Md, ImageShadow::Md) => {
            Cow::Borrowed("ui-image ui-image--radius-md ui-image--shadow-md")
        }
        (ImageRadius::Lg, ImageShadow::None) => {
            Cow::Borrowed("ui-image ui-image--radius-lg ui-image--shadow-none")
        }
        (ImageRadius::Lg, ImageShadow::Sm) => {
            Cow::Borrowed("ui-image ui-image--radius-lg ui-image--shadow-sm")
        }
        (ImageRadius::Lg, ImageShadow::Md) => {
            Cow::Borrowed("ui-image ui-image--radius-lg ui-image--shadow-md")
        }
        (ImageRadius::Full, ImageShadow::None) => {
            Cow::Borrowed("ui-image ui-image--radius-full ui-image--shadow-none")
        }
        (ImageRadius::Full, ImageShadow::Sm) => {
            Cow::Borrowed("ui-image ui-image--radius-full ui-image--shadow-sm")
        }
        (ImageRadius::Full, ImageShadow::Md) => {
            Cow::Borrowed("ui-image ui-image--radius-full ui-image--shadow-md")
        }
    }
}

#[cfg(test)]
#[path = "../test/logic.rs"]
mod tests;
