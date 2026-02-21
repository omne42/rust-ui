pub type AssetVariant = ui_state_primitives::asset::AssetVariant;
pub type AssetSize = ui_state_primitives::thumbnail::ThumbnailSize;
pub type AssetMotion = ui_thumbnail::ThumbnailMotion;
pub const ASSET_AGENT_SCHEMA: &'static str;

pub enum AssetAgentIntent {
    Display,
}

pub enum AssetAgentAction {
    StaticRender,
}

pub enum AssetInteractionSource {
    ExternalProp,
}

pub enum AssetMotionSource {
    Default,
    Custom,
}

pub enum AssetStreamSupport {
    Optional,
}

pub enum AssetStreamFallback {
    Snapshot,
}

pub enum AssetOutputStatus {
    Draft,
    Verified,
    Submittable,
}

pub fn Asset(
    variant: Option<AssetVariant>,
    label: Option<String>,
    lang: Option<String>,
    dir: Option<String>,
    size: Option<AssetSize>,
    is_selected: Option<bool>,
    is_focused: Option<bool>,
    motion: Option<AssetMotion>,
    class_name: Option<String>,
    children: Option<leptos::children::Children>,
) -> impl leptos::prelude::IntoView;
