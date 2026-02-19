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
mod tests {
    use super::*;
    use crate::preview_link_card::PreviewLinkCardPartStateInput;

    #[test]
    fn normalize_optional_text_trims_and_filters_blank_values() {
        assert_eq!(normalize_optional_text(None), None);
        assert_eq!(normalize_optional_text(Some(" \n\t ".to_string())), None);
        assert_eq!(
            normalize_optional_text(Some("  preview card  ".to_string())),
            Some("preview card".to_string())
        );
    }

    #[test]
    fn resolution_helpers_track_custom_and_default_sources() {
        assert_eq!(
            resolve_id(None, "generated-id".to_string()),
            ("generated-id".to_string(), false)
        );
        assert_eq!(
            resolve_id(Some(" docs-preview ".to_string()), "fallback".to_string()),
            ("docs-preview".to_string(), true)
        );

        assert_eq!(resolve_title(None), (DEFAULT_TITLE.into(), false));
        assert_eq!(
            resolve_title(Some("  GitHub  ".to_string())),
            ("GitHub".to_string(), true)
        );

        assert_eq!(
            resolve_description(Some("  Commit activity  ".to_string())),
            ("Commit activity".to_string(), true)
        );
        assert_eq!(
            resolve_description(None),
            (DEFAULT_DESCRIPTION.into(), false)
        );

        assert_eq!(resolve_url(None), (DEFAULT_URL.into(), false));
        assert_eq!(
            resolve_url(Some(" https://github.com/adobe/ui-baseline ".to_string())),
            ("https://github.com/adobe/ui-baseline".to_string(), true)
        );
    }

    #[test]
    fn resolve_site_label_supports_custom_derived_and_default_paths() {
        assert_eq!(
            resolve_site_label(Some(" Baseline ".to_string()), "https://example.com"),
            ("Baseline".to_string(), "custom")
        );

        assert_eq!(
            resolve_site_label(None, "https://www.github.com/adobe/ui-baseline"),
            ("github.com".to_string(), "derived")
        );

        assert_eq!(
            resolve_site_label(None, "   "),
            (DEFAULT_SITE_LABEL.into(), "default")
        );
    }

    #[test]
    fn resolve_part_state_tracks_slot_content_and_sources() {
        let root = resolve_part_state(PreviewLinkCardPartStateInput {
            slot: PreviewLinkCardSlot::Root,
            disabled: false,
            has_image: true,
            has_custom_class_name: true,
            has_custom_delays: true,
            has_custom_id: true,
            has_custom_title: true,
            has_custom_description: true,
            has_custom_url: true,
            site_label_source_attr: "derived",
            has_custom_motion: true,
        });

        assert_eq!(root.slot_attr, "preview-link-card");
        assert_eq!(root.content_attr, "media");
        assert_eq!(root.class_source_attr, "custom");
        assert_eq!(root.delay_source_attr, "custom");
        assert_eq!(root.site_label_source_attr, "derived");
        assert_eq!(root.motion_source_attr, "custom");

        let trigger = resolve_part_state(PreviewLinkCardPartStateInput {
            slot: PreviewLinkCardSlot::Trigger,
            disabled: false,
            has_image: true,
            has_custom_class_name: false,
            has_custom_delays: false,
            has_custom_id: false,
            has_custom_title: false,
            has_custom_description: false,
            has_custom_url: false,
            site_label_source_attr: "default",
            has_custom_motion: false,
        });

        assert_eq!(trigger.state_attr, "trigger");
        assert_eq!(trigger.content_attr, "trigger");
    }

    #[test]
    fn compose_class_name_includes_custom_and_content_markers() {
        let class_name = compose_class_name(
            Some("docs-preview-link-card".to_string()),
            resolve_part_state(PreviewLinkCardPartStateInput {
                slot: PreviewLinkCardSlot::Root,
                disabled: false,
                has_image: true,
                has_custom_class_name: true,
                has_custom_delays: true,
                has_custom_id: true,
                has_custom_title: true,
                has_custom_description: true,
                has_custom_url: true,
                site_label_source_attr: "derived",
                has_custom_motion: true,
            }),
        );

        for token in [
            "ui-preview-link-card",
            "ui-preview-link-card--enabled",
            "ui-preview-link-card--media",
            "ui-preview-link-card--custom-class",
            "ui-preview-link-card--custom-delay",
            "ui-preview-link-card--custom-motion",
            "ui-preview-link-card--custom-id",
            "ui-preview-link-card--custom-title",
            "ui-preview-link-card--custom-description",
            "ui-preview-link-card--custom-url",
            "docs-preview-link-card",
        ] {
            assert!(
                class_name.contains(token),
                "preview card class name should include `{token}`"
            );
        }
    }

    #[test]
    fn misc_helpers_keep_contracts_stable() {
        assert_eq!(state_attr_for_open(true), "open");
        assert_eq!(state_attr_for_open(false), "closed");
        assert_eq!(content_attr(true), "media");
        assert_eq!(content_attr(false), "text");

        assert!(!has_custom_delays(
            DEFAULT_OPEN_DELAY_MS,
            DEFAULT_CLOSE_DELAY_MS
        ));
        assert!(has_custom_delays(
            DEFAULT_OPEN_DELAY_MS + 1,
            DEFAULT_CLOSE_DELAY_MS
        ));

        assert_eq!(
            compose_panel_vars(12.5, 24.0, 180.0),
            "--ui-preview-link-card-top: 12.5px; --ui-preview-link-card-left: 24px; --ui-preview-link-card-anchor-width: 180px;"
        );

        assert!(should_handle_escape("Escape", true, false));
        assert!(!should_handle_escape("Escape", false, false));
        assert!(!should_handle_escape("Escape", true, true));
    }

    #[test]
    fn image_source_normalization_trims_and_drops_blank_values() {
        assert_eq!(resolve_image_src(None), None);
        assert_eq!(resolve_image_src(Some("  ".to_string())), None);
        assert_eq!(
            resolve_image_src(Some(" https://example.com/preview.png ".to_string())),
            Some("https://example.com/preview.png".to_string())
        );
    }
}
