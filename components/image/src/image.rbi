pub type ImageStatus = ui_state_primitives::image::ImageStatus;
pub type ImageRadius = ui_state_primitives::image::ImageRadius;
pub type ImageShadow = ui_state_primitives::image::ImageShadow;
pub type ImageMotion = ui_image::ImageMotion;
pub type A11yDirection = ui_headless::A11yDirection;

pub const IMAGE_AGENT_SCHEMA: &'static str;

pub enum ImageAgentIntent {
    Display,
}

pub enum ImageAgentAction {
    InitialRender,
    ResourceEvent,
}

pub enum ImageAgentPropSource {
    ExternalProp,
}

pub enum ImageContentSource {
    Primary,
    Fallback,
    Empty,
}

pub enum ImageStreamSupport {
    Optional,
}

pub enum ImageStreamFallback {
    Snapshot,
}

pub enum ImageLlmRenderMode {
    Streaming,
    Snapshot,
}

pub enum ImageOutputStatus {
    Draft,
    Verified,
    Submittable,
}

pub fn Image(
    src: Option<String>,
    alt: String,
    fallback_src: Option<String>,
    is_skeleton_disabled: bool,
    is_blurred: bool,
    is_zoomed: bool,
    radius: ImageRadius,
    shadow: ImageShadow,
    motion: ImageMotion,
    class_name: Option<String>,
    lang: Option<String>,
    dir: Option<A11yDirection>,
) -> impl leptos::prelude::IntoView;
