use crate::{
    PreviewCardMotion, PreviewCardPartState, PreviewCardPartStateInput, PreviewCardSiteLabelSource,
    PreviewCardSlot,
};
use ui_headless::PopoverPlacement;

pub const DEFAULT_TITLE: &str = "Link preview";
pub const DEFAULT_DESCRIPTION: &str = "Open this destination in a new tab.";
pub const DEFAULT_URL: &str = "https://example.com";
pub const DEFAULT_SITE_LABEL: &str = "example.com";
pub const DEFAULT_DISABLED: bool = false;
pub const DEFAULT_OPEN_DELAY_MS: u64 = 140;
pub const DEFAULT_CLOSE_DELAY_MS: u64 = 180;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PreviewCardRuntimeOptionsInput {
    pub is_disabled: Option<bool>,
    pub placement: Option<PopoverPlacement>,
    pub open_delay_ms: Option<u64>,
    pub close_delay_ms: Option<u64>,
    pub motion: Option<PreviewCardMotion>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PreviewCardRuntimeOptions {
    pub is_disabled: bool,
    pub placement: PopoverPlacement,
    pub open_delay_ms: u64,
    pub close_delay_ms: u64,
    pub motion: PreviewCardMotion,
    pub has_custom_delays: bool,
    pub has_custom_motion: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PreviewCardStateModelInput {
    pub class_name: Option<String>,
    pub is_disabled: bool,
    pub has_image: bool,
    pub has_custom_delays: bool,
    pub has_custom_id: bool,
    pub has_custom_title: bool,
    pub has_custom_description: bool,
    pub has_custom_url: bool,
    pub site_label_source: PreviewCardSiteLabelSource,
    pub has_custom_motion: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PreviewCardStateModel {
    pub root_state: PreviewCardPartState,
    pub trigger_state: PreviewCardPartState,
    pub panel_state: PreviewCardPartState,
    pub root_class: String,
    pub trigger_class: String,
    pub panel_class: String,
}

pub fn state_attr_for_open(is_open: bool) -> &'static str {
    if is_open { "open" } else { "closed" }
}

pub fn content_attr(has_image: bool) -> &'static str {
    if has_image { "media" } else { "text" }
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

pub fn resolve_runtime_options(input: PreviewCardRuntimeOptionsInput) -> PreviewCardRuntimeOptions {
    let is_disabled = input.is_disabled.unwrap_or(DEFAULT_DISABLED);
    let placement = input.placement.unwrap_or(PopoverPlacement::BottomStart);
    let open_delay_ms = input.open_delay_ms.unwrap_or(DEFAULT_OPEN_DELAY_MS);
    let close_delay_ms = input.close_delay_ms.unwrap_or(DEFAULT_CLOSE_DELAY_MS);
    let motion = input.motion.unwrap_or_default();

    PreviewCardRuntimeOptions {
        is_disabled,
        placement,
        open_delay_ms,
        close_delay_ms,
        motion,
        has_custom_delays: has_custom_delays(open_delay_ms, close_delay_ms),
        has_custom_motion: motion != PreviewCardMotion::default(),
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
) -> (String, PreviewCardSiteLabelSource) {
    if let Some(site_label) = normalize_optional_text(site_label) {
        return (site_label, PreviewCardSiteLabelSource::Custom);
    }

    if let Some(derived) = derive_site_label_from_url(resolved_url) {
        return (derived, PreviewCardSiteLabelSource::Derived);
    }

    (
        DEFAULT_SITE_LABEL.into(),
        PreviewCardSiteLabelSource::Default,
    )
}

pub fn resolve_image_src(image_src: Option<String>) -> Option<String> {
    normalize_optional_text(image_src)
}

fn source_attr(is_custom: bool) -> &'static str {
    if is_custom { "custom" } else { "default" }
}

pub fn resolve_part_state(input: PreviewCardPartStateInput) -> PreviewCardPartState {
    PreviewCardPartState {
        slot: input.slot,
        slot_attr: input.slot.as_attr(),
        base_class: input.slot.base_class(),
        state_attr: match input.slot {
            PreviewCardSlot::Root => "closed",
            PreviewCardSlot::Trigger => "trigger",
            PreviewCardSlot::Panel => "panel",
        },
        content_attr: match input.slot {
            PreviewCardSlot::Trigger => "trigger",
            PreviewCardSlot::Root | PreviewCardSlot::Panel => content_attr(input.has_image),
        },
        is_disabled: input.is_disabled,
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
        site_label_source: input.site_label_source,
        motion_source_attr: source_attr(input.has_custom_motion),
    }
}

pub fn resolve_state_model(input: PreviewCardStateModelInput) -> PreviewCardStateModel {
    let normalized_class_name = normalize_optional_text(input.class_name);
    let has_custom_class_name = normalized_class_name.is_some();

    let root_state = resolve_part_state(PreviewCardPartStateInput {
        slot: PreviewCardSlot::Root,
        is_disabled: input.is_disabled,
        has_image: input.has_image,
        has_custom_class_name,
        has_custom_delays: input.has_custom_delays,
        has_custom_id: input.has_custom_id,
        has_custom_title: input.has_custom_title,
        has_custom_description: input.has_custom_description,
        has_custom_url: input.has_custom_url,
        site_label_source: input.site_label_source,
        has_custom_motion: input.has_custom_motion,
    });

    let trigger_state = resolve_part_state(PreviewCardPartStateInput {
        slot: PreviewCardSlot::Trigger,
        is_disabled: input.is_disabled,
        has_image: input.has_image,
        has_custom_class_name: false,
        has_custom_delays: input.has_custom_delays,
        has_custom_id: input.has_custom_id,
        has_custom_title: input.has_custom_title,
        has_custom_description: input.has_custom_description,
        has_custom_url: input.has_custom_url,
        site_label_source: input.site_label_source,
        has_custom_motion: input.has_custom_motion,
    });

    let panel_state = resolve_part_state(PreviewCardPartStateInput {
        slot: PreviewCardSlot::Panel,
        is_disabled: input.is_disabled,
        has_image: input.has_image,
        has_custom_class_name: false,
        has_custom_delays: input.has_custom_delays,
        has_custom_id: input.has_custom_id,
        has_custom_title: input.has_custom_title,
        has_custom_description: input.has_custom_description,
        has_custom_url: input.has_custom_url,
        site_label_source: input.site_label_source,
        has_custom_motion: input.has_custom_motion,
    });

    let root_class = compose_class_name(normalized_class_name, root_state);
    let trigger_class = compose_class_name(None, trigger_state);
    let panel_class = compose_class_name(None, panel_state);

    PreviewCardStateModel {
        root_state,
        trigger_state,
        panel_state,
        root_class,
        trigger_class,
        panel_class,
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: PreviewCardPartState) -> String {
    let mut classes = vec![state.base_class.into()];

    if matches!(state.slot, PreviewCardSlot::Root) {
        if state.is_disabled {
            classes.push("ui-preview-card--disabled".to_string());
        } else {
            classes.push("ui-preview-card--enabled".to_string());
        }

        if state.has_image {
            classes.push("ui-preview-card--media".to_string());
        } else {
            classes.push("ui-preview-card--text".to_string());
        }

        if state.has_custom_class_name {
            classes.push("ui-preview-card--custom-class".to_string());
        }

        if state.has_custom_delays {
            classes.push("ui-preview-card--custom-delay".to_string());
        }

        if state.has_custom_motion {
            classes.push("ui-preview-card--custom-motion".to_string());
        }

        if state.has_custom_id {
            classes.push("ui-preview-card--custom-id".to_string());
        }

        if state.has_custom_title {
            classes.push("ui-preview-card--custom-title".to_string());
        }

        if state.has_custom_description {
            classes.push("ui-preview-card--custom-description".to_string());
        }

        if state.has_custom_url {
            classes.push("ui-preview-card--custom-url".to_string());
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
        "--ui-preview-card-top: {top_px}px; --ui-preview-card-left: {left_px}px; --ui-preview-card-anchor-width: {anchor_width_px}px;"
    )
}

#[cfg(test)]
#[path = "../test/logic.rs"]
mod tests;
