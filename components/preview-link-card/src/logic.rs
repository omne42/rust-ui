use crate::preview_link_card::{
    PreviewLinkCardPartState, PreviewLinkCardPartStateInput, PreviewLinkCardSlot,
};

pub const DEFAULT_TITLE: &str = "Link preview image";
pub const DEFAULT_DESCRIPTION: &str = "Hover to preview the destination before opening.";
pub const DEFAULT_URL: &str = "https://example.com";
pub const DEFAULT_SITE_LABEL: &str = "example.com";
pub const DEFAULT_DISABLED: bool = false;
pub const DEFAULT_OPEN_DELAY_MS: u64 = 140;
pub const DEFAULT_CLOSE_DELAY_MS: u64 = 180;

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
) -> (String, &'static str) {
    if let Some(site_label) = normalize_optional_text(site_label) {
        return (site_label, "custom");
    }

    if let Some(derived) = derive_site_label_from_url(resolved_url) {
        return (derived, "derived");
    }

    (DEFAULT_SITE_LABEL.into(), "default")
}

pub fn resolve_image_src(image_src: Option<String>) -> Option<String> {
    normalize_optional_text(image_src)
}

fn source_attr(is_custom: bool) -> &'static str {
    if is_custom { "custom" } else { "default" }
}

pub fn resolve_part_state(input: PreviewLinkCardPartStateInput) -> PreviewLinkCardPartState {
    PreviewLinkCardPartState {
        slot: input.slot,
        slot_attr: input.slot.as_attr(),
        base_class: input.slot.base_class(),
        state_attr: match input.slot {
            PreviewLinkCardSlot::Root => "closed",
            PreviewLinkCardSlot::Trigger => "trigger",
            PreviewLinkCardSlot::Panel => "panel",
        },
        content_attr: match input.slot {
            PreviewLinkCardSlot::Trigger => "trigger",
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

pub fn should_handle_escape(key: &str, is_open: bool, is_composing: bool) -> bool {
    key == "Escape" && is_open && !is_composing
}

#[cfg(test)]
#[path = "../test/logic.rs"]
mod tests;
