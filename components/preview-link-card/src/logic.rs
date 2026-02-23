use leptos::prelude::*;

pub const DEFAULT_TITLE: &str = "Link preview image";
pub const DEFAULT_DESCRIPTION: &str = "Hover to preview the destination before opening.";
pub const DEFAULT_URL: &str = "https://example.com";
pub const DEFAULT_SITE_LABEL: &str = "example.com";
pub const DEFAULT_DISABLED: bool = false;
pub const DEFAULT_OPEN_DELAY_MS: u64 = 140;
pub const DEFAULT_CLOSE_DELAY_MS: u64 = 180;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum PreviewLinkCardSlot {
    Root,
    Trigger,
    Panel,
}

impl PreviewLinkCardSlot {
    pub(crate) fn as_attr(self) -> &'static str {
        match self {
            PreviewLinkCardSlot::Root => "preview-link-card",
            PreviewLinkCardSlot::Trigger => "preview-link-card-trigger",
            PreviewLinkCardSlot::Panel => "preview-link-card-panel",
        }
    }

    pub(crate) fn base_class(self) -> &'static str {
        match self {
            PreviewLinkCardSlot::Root => "ui-preview-link-card",
            PreviewLinkCardSlot::Trigger => "ui-preview-link-card__trigger",
            PreviewLinkCardSlot::Panel => "ui-preview-link-card__panel",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum PreviewLinkCardStateAttr {
    Open,
    Closed,
    Trigger,
    Panel,
}

impl PreviewLinkCardStateAttr {
    pub(crate) fn as_attr(self) -> &'static str {
        match self {
            PreviewLinkCardStateAttr::Open => "open",
            PreviewLinkCardStateAttr::Closed => "closed",
            PreviewLinkCardStateAttr::Trigger => "trigger",
            PreviewLinkCardStateAttr::Panel => "panel",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum PreviewLinkCardContentAttr {
    Trigger,
    Media,
    Text,
}

impl PreviewLinkCardContentAttr {
    pub(crate) fn as_attr(self) -> &'static str {
        match self {
            PreviewLinkCardContentAttr::Trigger => "trigger",
            PreviewLinkCardContentAttr::Media => "media",
            PreviewLinkCardContentAttr::Text => "text",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum PreviewLinkCardSourceAttr {
    Default,
    Custom,
}

impl PreviewLinkCardSourceAttr {
    pub(crate) fn as_attr(self) -> &'static str {
        match self {
            PreviewLinkCardSourceAttr::Default => "default",
            PreviewLinkCardSourceAttr::Custom => "custom",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum PreviewLinkCardOpenModeAttr {
    Controlled,
    Uncontrolled,
}

impl PreviewLinkCardOpenModeAttr {
    pub(crate) fn as_attr(self) -> &'static str {
        match self {
            PreviewLinkCardOpenModeAttr::Controlled => "controlled",
            PreviewLinkCardOpenModeAttr::Uncontrolled => "uncontrolled",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum PreviewLinkCardSiteLabelSourceAttr {
    Default,
    Derived,
    Custom,
}

impl PreviewLinkCardSiteLabelSourceAttr {
    pub(crate) fn as_attr(self) -> &'static str {
        match self {
            PreviewLinkCardSiteLabelSourceAttr::Default => "default",
            PreviewLinkCardSiteLabelSourceAttr::Derived => "derived",
            PreviewLinkCardSiteLabelSourceAttr::Custom => "custom",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct PreviewLinkCardPartStateInput {
    pub(crate) slot: PreviewLinkCardSlot,
    pub(crate) disabled: bool,
    pub(crate) has_image: bool,
    pub(crate) has_custom_class_name: bool,
    pub(crate) has_custom_delays: bool,
    pub(crate) has_custom_id: bool,
    pub(crate) has_custom_title: bool,
    pub(crate) has_custom_description: bool,
    pub(crate) has_custom_url: bool,
    pub(crate) site_label_source_attr: PreviewLinkCardSiteLabelSourceAttr,
    pub(crate) has_custom_motion: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct PreviewLinkCardPartState {
    pub(crate) slot: PreviewLinkCardSlot,
    pub(crate) slot_attr: &'static str,
    pub(crate) base_class: &'static str,
    pub(crate) state_attr: PreviewLinkCardStateAttr,
    pub(crate) content_attr: PreviewLinkCardContentAttr,
    pub(crate) is_disabled: bool,
    pub(crate) has_image: bool,
    pub(crate) has_custom_class_name: bool,
    pub(crate) has_custom_delays: bool,
    pub(crate) has_custom_id: bool,
    pub(crate) has_custom_title: bool,
    pub(crate) has_custom_description: bool,
    pub(crate) has_custom_url: bool,
    pub(crate) has_custom_motion: bool,
    pub(crate) class_source_attr: PreviewLinkCardSourceAttr,
    pub(crate) delay_source_attr: PreviewLinkCardSourceAttr,
    pub(crate) id_source_attr: PreviewLinkCardSourceAttr,
    pub(crate) title_source_attr: PreviewLinkCardSourceAttr,
    pub(crate) description_source_attr: PreviewLinkCardSourceAttr,
    pub(crate) url_source_attr: PreviewLinkCardSourceAttr,
    pub(crate) site_label_source_attr: PreviewLinkCardSiteLabelSourceAttr,
    pub(crate) motion_source_attr: PreviewLinkCardSourceAttr,
}

pub fn state_attr_for_open(is_open: bool) -> PreviewLinkCardStateAttr {
    if is_open {
        PreviewLinkCardStateAttr::Open
    } else {
        PreviewLinkCardStateAttr::Closed
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct OpenStateMarkersInput {
    pub is_open: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct OpenStateMarkers {
    pub state_attr: PreviewLinkCardStateAttr,
    pub open_attr: Option<&'static str>,
    pub closed_attr: Option<&'static str>,
}

pub fn resolve_open_state_markers(input: OpenStateMarkersInput) -> OpenStateMarkers {
    OpenStateMarkers {
        state_attr: state_attr_for_open(input.is_open),
        open_attr: input.is_open.then_some("true"),
        closed_attr: (!input.is_open).then_some("true"),
    }
}

pub fn content_attr(has_image: bool) -> PreviewLinkCardContentAttr {
    if has_image {
        PreviewLinkCardContentAttr::Media
    } else {
        PreviewLinkCardContentAttr::Text
    }
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.into())
    })
}

pub fn has_custom_delays(open_delay_ms: u64, close_delay_ms: u64) -> bool {
    open_delay_ms != DEFAULT_OPEN_DELAY_MS || close_delay_ms != DEFAULT_CLOSE_DELAY_MS
}

pub struct DelayInput {
    pub open_delay_ms: Option<u64>,
    pub close_delay_ms: Option<u64>,
}

pub struct Delays {
    pub open_delay_ms: u64,
    pub close_delay_ms: u64,
    pub has_custom_delays: bool,
}

pub fn normalize_delays(input: DelayInput) -> Delays {
    let open_delay_ms = input.open_delay_ms.unwrap_or(DEFAULT_OPEN_DELAY_MS);
    let close_delay_ms = input.close_delay_ms.unwrap_or(DEFAULT_CLOSE_DELAY_MS);

    Delays {
        open_delay_ms,
        close_delay_ms,
        has_custom_delays: has_custom_delays(open_delay_ms, close_delay_ms),
    }
}

pub struct OpenStateInput {
    pub is_open: Option<Signal<bool>>,
    pub open: Option<Signal<bool>>,
    pub default_open: Option<bool>,
    pub on_open_change: Option<Callback<bool>>,
}

pub struct OpenState {
    pub open: Option<Signal<bool>>,
    pub default_open: Option<bool>,
    pub on_open_change: Option<Callback<bool>>,
    pub is_controlled: bool,
}

pub fn normalize_open_state(input: OpenStateInput) -> OpenState {
    let open = input.is_open.or(input.open);
    let is_controlled = open.is_some();

    OpenState {
        open,
        default_open: input.default_open,
        on_open_change: input.on_open_change,
        is_controlled,
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct OpenStateSourceMarkersInput {
    pub is_controlled: bool,
    pub has_open_prop: bool,
    pub has_default_open: bool,
    pub has_on_open_change: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct OpenStateSourceMarkers {
    pub open_mode_attr: PreviewLinkCardOpenModeAttr,
    pub open_source_attr: PreviewLinkCardSourceAttr,
    pub default_open_source_attr: PreviewLinkCardSourceAttr,
    pub open_change_source_attr: PreviewLinkCardSourceAttr,
}

pub fn resolve_open_state_source_markers(
    input: OpenStateSourceMarkersInput,
) -> OpenStateSourceMarkers {
    OpenStateSourceMarkers {
        open_mode_attr: if input.is_controlled {
            PreviewLinkCardOpenModeAttr::Controlled
        } else {
            PreviewLinkCardOpenModeAttr::Uncontrolled
        },
        open_source_attr: source_attr(input.has_open_prop),
        default_open_source_attr: source_attr(input.has_default_open),
        open_change_source_attr: source_attr(input.has_on_open_change),
    }
}

pub fn resolve_id(custom_id: Option<String>, fallback_id: String) -> (String, bool) {
    if let Some(custom_id) = normalize_optional_text(custom_id) {
        return (custom_id, true);
    }

    (fallback_id, false)
}

pub fn resolve_title(value: Option<String>) -> (String, bool) {
    if let Some(value) = normalize_optional_text(value) {
        return (value, true);
    }

    (DEFAULT_TITLE.into(), false)
}

pub fn resolve_description(value: Option<String>) -> (String, bool) {
    if let Some(value) = normalize_optional_text(value) {
        return (value, true);
    }

    (DEFAULT_DESCRIPTION.into(), false)
}

pub fn resolve_url(value: Option<String>) -> (String, bool) {
    if let Some(value) = normalize_optional_text(value) {
        return (value, true);
    }

    (DEFAULT_URL.into(), false)
}

fn derive_site_label_from_url(url: &str) -> Option<String> {
    let trimmed = url.trim();
    if trimmed.is_empty() {
        return None;
    }

    let without_scheme = trimmed
        .split_once("://")
        .map(|(_, tail)| tail)
        .unwrap_or(trimmed);

    let host = without_scheme
        .split(['/', '?', '#'])
        .next()
        .unwrap_or("")
        .trim();

    if host.is_empty() {
        return None;
    }

    Some(host.trim_start_matches("www.").into())
}

pub fn resolve_site_label(
    site_label: Option<String>,
    resolved_url: &str,
) -> (String, PreviewLinkCardSiteLabelSourceAttr) {
    if let Some(site_label) = normalize_optional_text(site_label) {
        return (site_label, PreviewLinkCardSiteLabelSourceAttr::Custom);
    }

    if let Some(derived) = derive_site_label_from_url(resolved_url) {
        return (derived, PreviewLinkCardSiteLabelSourceAttr::Derived);
    }

    (
        DEFAULT_SITE_LABEL.into(),
        PreviewLinkCardSiteLabelSourceAttr::Default,
    )
}

pub fn resolve_image_src(image_src: Option<String>) -> Option<String> {
    normalize_optional_text(image_src)
}

fn source_attr(is_custom: bool) -> PreviewLinkCardSourceAttr {
    if is_custom {
        PreviewLinkCardSourceAttr::Custom
    } else {
        PreviewLinkCardSourceAttr::Default
    }
}

pub fn resolve_part_state(input: PreviewLinkCardPartStateInput) -> PreviewLinkCardPartState {
    PreviewLinkCardPartState {
        slot: input.slot,
        slot_attr: input.slot.as_attr(),
        base_class: input.slot.base_class(),
        state_attr: match input.slot {
            PreviewLinkCardSlot::Root => PreviewLinkCardStateAttr::Closed,
            PreviewLinkCardSlot::Trigger => PreviewLinkCardStateAttr::Trigger,
            PreviewLinkCardSlot::Panel => PreviewLinkCardStateAttr::Panel,
        },
        content_attr: match input.slot {
            PreviewLinkCardSlot::Trigger => PreviewLinkCardContentAttr::Trigger,
            PreviewLinkCardSlot::Root | PreviewLinkCardSlot::Panel => content_attr(input.has_image),
        },
        is_disabled: input.disabled,
        has_image: input.has_image,
        has_custom_class_name: input.has_custom_class_name,
        has_custom_delays: input.has_custom_delays,
        has_custom_id: input.has_custom_id,
        has_custom_title: input.has_custom_title,
        has_custom_description: input.has_custom_description,
        has_custom_url: input.has_custom_url,
        has_custom_motion: input.has_custom_motion,
        class_source_attr: source_attr(input.has_custom_class_name),
        delay_source_attr: source_attr(input.has_custom_delays),
        id_source_attr: source_attr(input.has_custom_id),
        title_source_attr: source_attr(input.has_custom_title),
        description_source_attr: source_attr(input.has_custom_description),
        url_source_attr: source_attr(input.has_custom_url),
        site_label_source_attr: input.site_label_source_attr,
        motion_source_attr: source_attr(input.has_custom_motion),
    }
}

pub fn compose_class_name(
    base_class_name: Option<String>,
    state: PreviewLinkCardPartState,
) -> String {
    let mut classes = vec![state.base_class.into()];

    if matches!(state.slot, PreviewLinkCardSlot::Root) {
        if state.is_disabled {
            classes.push("ui-preview-link-card--disabled".to_string());
        } else {
            classes.push("ui-preview-link-card--enabled".to_string());
        }

        if state.has_image {
            classes.push("ui-preview-link-card--media".to_string());
        } else {
            classes.push("ui-preview-link-card--text".to_string());
        }

        if state.has_custom_class_name {
            classes.push("ui-preview-link-card--custom-class".to_string());
        }

        if state.has_custom_delays {
            classes.push("ui-preview-link-card--custom-delay".to_string());
        }

        if state.has_custom_motion {
            classes.push("ui-preview-link-card--custom-motion".to_string());
        }

        if state.has_custom_id {
            classes.push("ui-preview-link-card--custom-id".to_string());
        }

        if state.has_custom_title {
            classes.push("ui-preview-link-card--custom-title".to_string());
        }

        if state.has_custom_description {
            classes.push("ui-preview-link-card--custom-description".to_string());
        }

        if state.has_custom_url {
            classes.push("ui-preview-link-card--custom-url".to_string());
        }

        if let Some(class_name) = normalize_optional_text(base_class_name) {
            classes.push(class_name);
        }

        return classes.join(" ");
    }

    if let Some(class_name) = normalize_optional_text(base_class_name) {
        classes.push(class_name);
    }

    classes.join(" ")
}

pub fn compose_panel_vars(top_px: f64, left_px: f64, anchor_width_px: f64) -> String {
    format!(
        "--ui-preview-link-card-top: {top_px}px; --ui-preview-link-card-left: {left_px}px; --ui-preview-link-card-anchor-width: {anchor_width_px}px;"
    )
}

#[cfg(test)]
#[path = "../test/logic.rs"]
mod tests;
